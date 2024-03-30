use crate::vec3::{Point, Vec3};
use crate::ray::{Ray};

pub struct HitRecord {
    // Relevant information for when a ray hits an object.
    pub point: Point,
    pub normal: Vec3,
    pub t: f64
}


pub trait Hittable {
    // Return the HitRecord at which the ray intersects the object
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
