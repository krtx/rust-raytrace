use geometry;

#[derive(Clone)]
pub enum Material {
    Diffuse(i32, i32, i32),
    Specular
}

pub struct Sphere {
    pub origin: geometry::Vec3,
    pub radius: f64,
    pub material: Material
}

pub fn sphere_new(x: f64, y: f64, z: f64, r: f64, m: Material) -> Sphere {
    Sphere { origin: geometry::Vec3 { x: x, y: y, z: z },
             radius: r,
             material: m }
}

pub struct Plane {
    pub normal: geometry::Vec3,
    pub distance: f64,
    pub material: Material
}

pub fn plane_new(x: f64, y: f64, z: f64, d: f64, m: Material) -> Plane {
    Plane { normal: geometry::Vec3 { x: x, y: y, z: z }.normalize(),
            distance: d,
            material: m }
}

pub struct HitRecord {
    pub t: f64,
    pub normal: geometry::Vec3,
    pub point: geometry::Vec3,
    pub material: Material
}

pub trait Hit {
    fn hit(&self, &geometry::Ray) -> Option<HitRecord>;
}

impl Hit for Sphere {
    fn hit(&self, ray: &geometry::Ray) -> Option<HitRecord> {
        let sray = self.origin.clone() - &ray.origin;

        let det = geometry::dot(&ray.unit_dir, &sray).powi(2) - sray.size().powi(2) + self.radius.powi(2);

        if det >= 0.0 {
            let t1 = geometry::dot(&ray.unit_dir, &sray) - det.sqrt();
            let t2 = geometry::dot(&ray.unit_dir, &sray) + det.sqrt();

            if t2 < 0.0 {
                None
            } else {
                let t = if t1 >= 0.0 { t1 } else { t2 };

                let point  = ray.origin.clone() + &(t * ray.unit_dir.clone());
                let normal = (point.clone() - &self.origin).normalize();
                Some(HitRecord { t: t,
                                 point: point + &(0.001 * normal.clone()),
                                 normal: normal,
                                 material: self.material.clone() })
            }
        } else { None }
    }
}

impl Hit for Plane {
    fn hit(&self, ray: &geometry::Ray) -> Option<HitRecord> {
        let t =
            (self.distance - geometry::dot(&ray.origin, &self.normal)) /
            geometry::dot(&ray.unit_dir, &self.normal);

        if t > 0.0 {
            Some(HitRecord{ t: t,
                            point: ray.origin.clone() + &(t * ray.unit_dir.clone()) + &(0.001 * self.normal.clone()),
                            normal: self.normal.clone(),
                            material: self.material.clone() })
        } else { None }
    }
}
