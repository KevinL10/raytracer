use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{Point, Vec3};

pub struct HitRecord {
    // Relevant information for when a ray hits an object.
    pub point: Point,
    pub normal: Vec3,
    pub t: f64,
    // true if ray hits the object on the outside surface
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(point: Point, outward_normal: Vec3, ray: Ray, t: f64) -> Self {
        let front_face = Vec3::dot(ray.direction, outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        assert!((normal.length() - 1.0).abs() < 0.0001);
        Self {
            point,
            normal,
            t,
            front_face,
        }
    }
}

pub trait Hittable {
    // Return the HitRecord at which the ray intersects the object
    fn hit(&self, ray: Ray, ray_t: Interval) -> Option<HitRecord>;
}
