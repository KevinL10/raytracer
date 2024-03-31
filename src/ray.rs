use crate::vec3::{Point, Vec3};
use crate::color::Color;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Ray {
    pub orig: Point,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(orig: Point, direction: Vec3) -> Self {
        Self { orig, direction }
    }

    pub fn at(self, t: f64) -> Point {
        self.orig + t * self.direction
    }
}


#[derive(PartialEq, Debug, Clone, Copy)]
pub struct ScatteredRay {
    pub attenuation: Color,
    pub ray: Ray,
}
