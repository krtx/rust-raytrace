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
    fn hit(&self, ray: geometry::Ray) -> Option<obj::HitRecord> {
        // Is it impossible to use generics in closure?
        let take_min =
            |a: Option<obj::HitRecord>,
             b: Option<obj::HitRecord>| match (a, b) {
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

    fn trace(&self, ray: geometry::Ray, depth: i32) -> i32 {
        if depth <= 0 { return 0; }

        let hit = self.hit(ray);

        match hit {
            None      => 0,
            Some(hit) => {

                // println!("{}", hit.t);

                // match hit.material {
                //     obj::Material::Diffuse => {
                //         50
                //     },
                //     obj::Material::Specular => {
                //         100
                //     }
                // }

                match hit.material {
                    obj::Material::Diffuse  => {
                        let power_directional = geometry::dot(hit.normal, self.directional_light).max(0.0);

                        let shadow = self.positional_light.origin - hit.point;
                        let shadow_ray = geometry::Ray {
                            origin: hit.point,
                            unit_dir: shadow.normalize()
                        };
                        // println!("ray_to_light_dir = {}\nhit.pint = {}\nhit.normal = {}", ray_to_light_dir, hit.point, hit.normal);
                        let power_positional = match self.hit(shadow_ray) {
                            Some(h) => if h.t > shadow.size() {
                                // println!("{}, {}", hit.t, ray_to_light_dir.size());
                                // geometry::dot(hit.normal, -self.positional_light.unit_dir).max(0.0)
                                // println!("{}, {}", hit.normal, ray_to_light_dir);
                                geometry::dot(hit.normal, shadow_ray.unit_dir).max(0.0)
                            } else {
                                0.0
                            },
                            None      => {
                                geometry::dot(hit.normal, shadow_ray.unit_dir).max(0.0)
                            }
                        };
                        (power_directional * 127.0 + power_positional * 127.0) as i32
                    },
                    obj::Material::Specular => {
                        let dir = ray.unit_dir + 2.0 * geometry::dot(-ray.unit_dir, hit.normal) * hit.normal;
                        let ray = geometry::Ray { unit_dir: dir, origin: hit.point };
                        self.trace(ray, depth - 1)
                    }
                }
            }
        }
    }

    fn shot(&self, target: geometry::Vec3) -> geometry::Ray {
        geometry::Ray {
            origin: self.camera,
            unit_dir: (target - self.camera).normalize()
        }
    }
}

fn main() {
    let env = Env {
        camera: geometry::Vec3 { x: 0.0, y: 0.0, z: -10.0 },
        directional_light: geometry::Vec3 { x: -1.0, y: -2.0, z: -0.9 }.normalize(),
        positional_light: geometry::Ray {
            origin: geometry::Vec3 { x: 0.0, y: 0.0, z: 0.0 },
            unit_dir: geometry::Vec3 { x: -1.2, y: 1.5, z: 1.0 }.normalize()
        },
        spheres: vec![
            obj::sphere_new(7.0, 7.0, 5.0, 7.0, obj::Material::Diffuse),
            obj::sphere_new(5.0, -10.0, 5.0, 7.0, obj::Material::Specular),
            obj::sphere_new(-10.0, -10.0, 5.0, 4.0, obj::Material::Specular),
        ],
        planes: vec![
            obj::plane_new(0.0, 0.0, -1.0, -20.0, obj::Material::Diffuse),
            obj::plane_new(-1.0, 0.0, 0.0, -40.0, obj::Material::Diffuse),
            obj::plane_new(1.0, 0.0, 0.0, -40.0, obj::Material::Diffuse),
            obj::plane_new(0.0, -1.0, 0.0, -40.0, obj::Material::Diffuse),
            obj::plane_new(0.0, 1.0, 0.0, -40.0, obj::Material::Diffuse),
        ]
    };

    let left = -200;
    let right = 200;
    let bottom = -200;
    let top = 200;

    println!("P2");
    println!("{} {}", right - left, top - bottom);
    println!("255");

    // (-100, -100, 0) .. (99, 99, 0)
    for y in bottom..top {
        for x in left..right {
            let point = geometry::Vec3 { x: 0.1 * x as f64,
                                         y: 0.1 * y as f64,
                                         z: 0.0 };
            // println!("{}", point);
            let ray = env.shot(point);
            let power = env.trace(ray, 10);

            print!("{}", power);

            if x == right - 1 { println!(""); }
            else { print!(" "); }
        }
    }
}
