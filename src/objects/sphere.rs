use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point, Vec3};
use std::rc::Rc;

pub struct Sphere {
    center: Point,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point, radius: f64, material: Rc<dyn Material>) -> Self {
        Self { center, radius, material }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, ray_t: Interval) -> Option<HitRecord> {
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

        let t = if ray_t.surrounds(t1) {
            t1
        } else if ray_t.surrounds(t2) {
            t2
        } else {
            return None;
        };

        let point = ray.at(t);
        let outward_normal = (point - self.center) / self.radius;
        Some(HitRecord::new(point, outward_normal, ray, t, Rc::clone(&self.material)))
    }
}
