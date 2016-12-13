extern crate raytrace;

use std::iter::Iterator;
use raytrace::geometry;
use raytrace::obj;
use raytrace::obj::Hit;

struct Env {
    camera: geometry::Vec3,
    directional_light: geometry::Vec3,
    positional_light: geometry::Ray,
    spheres: Vec<obj::Sphere>,
    planes: Vec<obj::Plane>
}

impl Env {
    fn hit<'a>(&'a self, ray: &geometry::Ray) -> Option<obj::HitRecord<'a>> {
        // Is it impossible to use generics in closure?
        let take_min =
            |a: Option<obj::HitRecord<'a>>,
             b: Option<obj::HitRecord<'a>>| match (a, b) {
            (Some(a), Some(b)) => if a.t < b.t { Some(a) } else { Some(b) },
            (Some(a), None)    => Some(a),
            (None   , Some(b)) => Some(b),
            (None   , None)    => None
        };
        let hit = self.spheres.iter().fold(
            None,
            |acc, sphere| take_min(acc, sphere.hit(ray))
        );
        let hit = self.planes.iter().fold(
            hit,
            |acc, plane| take_min(acc, plane.hit(ray))
        );
        hit
    }

    fn trace(&self, ray: &geometry::Ray, depth: i32) -> (i32, i32, i32) {
        if depth <= 0 { return (0, 0, 0); }

        match self.hit(ray) {
            None      => (0, 0, 0),
            Some(hit) => {
                match hit.material {
                    &obj::Material::Diffuse(r, g, b)  => {
                        // let power_directional = geometry::dot(hit.normal, self.directional_light).max(0.0);

                        let shadow = self.positional_light.origin.clone() - &hit.point;
                        let shadow_size = shadow.size();
                        let shadow_ray = geometry::Ray {
                            origin: hit.point,
                            unit_dir: shadow.normalize()
                        };
                        let power_coefficient = match self.hit(&shadow_ray) {
                            Some(h) => if h.t > shadow_size { 1.0 } else { 0.2 },
                            None    => 1.0
                        };
                        let power_positional =
                            (power_coefficient *
                             geometry::dot(&hit.normal, &shadow_ray.unit_dir).max(0.0) /
                             (shadow_size + 0.001).powf(1.5) * 150.0).min(1.0).max(0.0);
                        ((power_positional * (r as f64)) as i32,
                         (power_positional * (g as f64)) as i32,
                         (power_positional * (b as f64)) as i32)
                    },
                    &obj::Material::Specular => {
                        let dir = ray.unit_dir.clone() + &(2.0 * geometry::dot(&-ray.unit_dir.clone(), &hit.normal) * hit.normal);
                        let ray = geometry::Ray { unit_dir: dir, origin: hit.point };
                        self.trace(&ray, depth - 1)
                    }
                }
            }
        }
    }

    fn shot(&self, target: geometry::Vec3) -> geometry::Ray {
        geometry::Ray {
            origin: self.camera.clone(),
            unit_dir: (target - &self.camera).normalize()
        }
    }
}

fn main() {
    let env = Env {
        camera: geometry::Vec3 { x: 0.0, y: 0.0, z: -50.0 },
        directional_light: geometry::Vec3 { x: -1.0, y: 1.0, z: 0.0 }.normalize(),
        positional_light: geometry::Ray {
            origin: geometry::Vec3 { x: 0.0, y: -24.0, z: -5.0 },
            unit_dir: geometry::Vec3 { x: -1.2, y: 1.5, z: 1.0 }.normalize() // unused
        },
        spheres: vec![
            obj::sphere_new( 12.0,   0.0, -2.0, 2.0, obj::Material::Diffuse(255,255,255)),
            obj::sphere_new( -4.0,  13.0,  9.0, 2.0, obj::Material::Diffuse(255,255,255)),
            obj::sphere_new( 16.0,  -5.0, -2.0, 2.0, obj::Material::Diffuse(255,255,255)),
            obj::sphere_new(  0.0,   4.0,  8.0, 2.0, obj::Material::Diffuse(255,255,255)),
            obj::sphere_new(-13.0, -13.0, -4.0, 3.0, obj::Material::Diffuse(255,255,255)),
            obj::sphere_new(-10.0, -16.0,  3.0, 2.0, obj::Material::Diffuse(255,255,255)),
            obj::sphere_new(-14.0,  18.0,  5.0, 8.0, obj::Material::Specular), // left sphere
            obj::sphere_new( 13.0,  18.0,  0.0, 8.0, obj::Material::Specular), // right sphere
        ],
        planes: vec![
            obj::plane_new( 0.0,  0.0, -1.0, -20.0, obj::Material::Specular), // bottom
            obj::plane_new(-1.0,  0.0,  0.0, -25.0, obj::Material::Diffuse(128,128,255)), // right wall
            obj::plane_new( 1.0,  0.0,  0.0, -25.0, obj::Material::Diffuse(255,128,128)), // left wall
            obj::plane_new( 0.0, -1.0,  0.0, -25.0, obj::Material::Diffuse(255,255,100)), // floor
            obj::plane_new( 0.0,  1.0,  0.0, -25.0, obj::Material::Diffuse(255,255,255)), // ceil
        ]
    };

    let left = -300;
    let right = 300;
    let bottom = -300;
    let top = 300;

    let mut pixels: Vec<Vec<(i32, i32, i32)>> = Vec::new();

    for y in bottom..top {
        let mut row = Vec::new();
        for x in left..right {
            let point = geometry::Vec3 { x: 0.08 * x as f64, y: 0.08 * y as f64, z: -10.0 };
            let ray = env.shot(point);
            let power = env.trace(&ray, 5);

            row.push(power);
        }
        pixels.push(row);
    }

    println!("P3");
    println!("{} {}", right - left, top - bottom);
    println!("255");

    for row in pixels.iter() {
        for color in row.iter() { println!("{} {} {}", color.0, color.1, color.2); }
    }
    // for row in pixels.iter() {
    //     for color in row.iter() { print!("{} ", color.1); }
    //     println!("");
    // }
    // for row in pixels.iter() {
    //     for color in row.iter() { print!("{} ", color.2); }
    //     println!("");
    // }
}
