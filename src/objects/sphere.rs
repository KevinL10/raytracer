use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::{Point, Vec3};

pub struct Sphere {
    center: Point,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.orig - self.center;
        let a = ray.direction.length().powf(2.0);
        let b = Vec3::dot(ray.direction, oc);
        let c = oc.length().powf(2.0) - self.radius.powf(2.0);

        let discriminant = b * b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        // t1 is closer to the camera (t = 0) if it's in the correct (t_min, t_max) range
        let t1 = (-b - discriminant.sqrt()) / a;
        let t2 = (-b + discriminant.sqrt()) / a;

        let t = if t_min <= t1 && t1 <= t_max {
            t1
        } else if t_min <= t2 && t2 <= t_max {
            t2
        } else {
            return None;
        };

        Some(HitRecord {
            t,
            point: ray.at(t),
            normal: (ray.at(t) - self.center) / self.radius,
        })
    }
}
