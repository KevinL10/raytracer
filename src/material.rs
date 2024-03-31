use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::{Ray, ScatteredRay};
use crate::vec3::{random_unit_vector, Vec3};

pub trait Material {
    // describe the scattered ray (ray and attenuation color) off of the
    // material, given an incident ray with hit_record hit
    fn scatter(&self, ray: Ray, hit_record: &HitRecord) -> Option<ScatteredRay>;
}

// a diffuse surface material that scatters rays with a cos distribution
// toward the normal
pub struct Lambertian {
    // describes the reduction in intensity/color of a ray that hits the surface
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: Ray, hit_record: &HitRecord) -> Option<ScatteredRay> {
        let mut direction = hit_record.normal + random_unit_vector();

        // drop the case where the random unit sphere points backward
        // and results in a zero scatter vector
        if direction.near_zero() {
            direction = hit_record.normal;
        }

        Some(ScatteredRay {
            ray: Ray::new(hit_record.point, direction),
            attenuation: self.albedo,
        })
    }
}

// a material that mirror reflects all of the light that hits the surface
pub struct Metal {
    // reduction in intensity/color of a ray that hits the surface
    albedo: Color,
    // magnitude of a random unit vector added to the scatter vector
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: fuzz.clamp(0.0, 1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: Ray, hit_record: &HitRecord) -> Option<ScatteredRay> {
        // mirror the ray that hits the object against the normal of the hit
        let reflected = ray.direction.unit()
            - 2.0 * hit_record.normal * Vec3::dot(ray.direction.unit(), hit_record.normal)
            + self.fuzz * random_unit_vector();

        Some(ScatteredRay {
            ray: Ray::new(hit_record.point, reflected),
            attenuation: self.albedo,
        })
    }
}
