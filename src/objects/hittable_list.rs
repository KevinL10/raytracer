use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;

use std::sync::Arc;

pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new()
        }
    }

    pub fn add(&mut self, hittable: Arc<dyn Hittable>) {
        self.objects.push(Arc::clone(&hittable));
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: Ray, ray_t: Interval) -> Option<HitRecord> {
        // TODO: fold over current closest point instead of checking
        // hit on the entire original interval
        self.objects
            .iter()
            .map(|object| object.hit(ray, ray_t))
            .filter_map(|hit_record| hit_record)
            .min_by(|h1, h2| h1.t.partial_cmp(&h2.t).unwrap())
    }
}
