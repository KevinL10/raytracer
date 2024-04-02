use rand::Rng;
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub use Vec3 as Point;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn length(self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn unit(self) -> Self {
        self / self.length()
    }

    pub fn dot(v: Vec3, u: Vec3) -> f64 {
        return v.x * u.x + v.y * u.y + v.z * u.z;
    }

    pub fn cross(v: Vec3, u: Vec3) -> Vec3 {
        Vec3::new(
            v.y * u.z - v.z * u.y,
            v.z * u.x - v.x * u.z,
            v.x * u.y - v.y * u.x,
        )
    }

    pub fn random() -> Vec3 {
        Vec3 {
            x: rand::random(),
            y: rand::random(),
            z: rand::random(),
        }
    }

    pub fn random_range(min: f64, max: f64) -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3 {
            x: rng.gen_range(min..=max),
            y: rng.gen_range(min..=max),
            z: rng.gen_range(min..=max),
        }
    }

    pub fn random_disk(min: f64, max: f64) -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3 {
            x: rng.gen_range(min..=max),
            y: rng.gen_range(min..=max),
            z: rng.gen_range(min..=max),
        }
    }

    pub fn near_zero(&self) -> bool {
        let eps = 1e-8;
        self.x.abs() < eps && self.y.abs() < eps && self.z.abs() < eps
    }
}

// Implement simple addition/multiplication/etc operations for Vec3
// TODO: clean up impls below
impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Self) -> Self::Output {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Self) -> Self::Output {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<i32> for Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: i32) -> Self::Output {
        self * (scalar as f64)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: f64) -> Self::Output {
        Vec3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Self) -> Self::Output {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, scalar: f64) -> Self::Output {
        self * (1.0 / scalar)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        self * -1.0
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Self::Output {
        other * self
    }
}

// useful functions

// produce the vector when the ray hits a mirror reflective surface
// assumes that direction and normal are unit vectors
pub fn reflect(direction: Vec3, normal: Vec3) -> Vec3 { 
    direction + 2.0 * normal * Vec3::dot(-direction, normal)
} 

pub fn refract(direction: Vec3, normal: Vec3, refraction_ratio: f64) -> Vec3 {
    let cos_theta = Vec3::dot(direction, -normal);
    let r_perp = refraction_ratio * (direction + cos_theta * normal);
    let r_parallel  = -normal * (1.0 - r_perp.length().powf(2.0)).sqrt();


    r_perp + r_parallel
}


// note: we first sample points from within the sphere and then
// normalize them. We do this to avoid oversampling from points
// toward the "corners" of the sphere (~ 45 degrees).
fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random_range(-1.0, 1.0);
        if p.length() < 1.0 {
            return p;
        }
    }
}

pub fn random_unit_vector() -> Vec3 {
    random_in_unit_sphere().unit()
}

pub fn random_in_unit_disk() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = Vec3::new(rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0), 0.0);
        if p.length() < 1.0 {
            return p;
        }
    }
}

#[allow(dead_code)]
pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
    let on_unit_sphere = random_unit_vector();
    if Vec3::dot(on_unit_sphere, normal) > 0.0 {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_double_eq(a: f64, b: f64) {
        assert!((a - b).abs() <= 0.000001,);
    }
    #[test]
    fn init_vector() {
        let vector = Vec3::new(2.0, 3.0, 4.0);
        assert_double_eq(vector.x, 2.0);
        assert_double_eq(vector.y, 3.0);
        assert_double_eq(vector.z, 4.0);
    }

    #[test]
    fn vector_add() {
        let v1 = Vec3::new(1.0, -5.0, 10.0);
        let v2 = Vec3::new(-4.0, 9.0, 1.0);
        let v3 = v1 + v2;
        assert_double_eq(v1.x, 1.0);
        assert_double_eq(v2.y, 9.0);
        assert_double_eq(v3.x, -3.0);
        assert_double_eq(v3.y, 4.0);
        assert_double_eq(v3.z, 11.0);
    }

    #[test]
    fn vector_mul() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = v1 * -4.0;
        assert_double_eq(v2.x, -4.0);
        assert_double_eq(v2.y, -8.0);
        assert_double_eq(v2.z, -12.0);
    }

    #[test]
    fn vector_length() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        assert_double_eq(v1.length(), 3.7416573);
    }

    #[test]
    fn vector_dot() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 1.0, 3.0);
        assert_double_eq(Vec3::dot(v1, v2), 15.0);
    }

    #[test]
    fn vector_cross() {
        let v1 = Vec3::new(3.0, -3.0, 1.0);
        let v2 = Vec3::new(4.0, 9.0, 2.0);
        let v3 = Vec3::new(-15.0, -2.0, 39.0);
        assert_eq!(Vec3::cross(v1, v2), v3);
    }
}
