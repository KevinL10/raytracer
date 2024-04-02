use rand::seq::index;

use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::{Ray, ScatteredRay};
use crate::vec3::{random_unit_vector, reflect, refract, Vec3};

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
        let reflected =
            reflect(ray.direction.unit(), hit_record.normal) + self.fuzz * random_unit_vector();

        Some(ScatteredRay {
            ray: Ray::new(hit_record.point, reflected),
            attenuation: self.albedo,
        })
    }
}

pub struct Dielectric {
    ir: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            ir: index_of_refraction 
        }
    }

    // note: Shlick's approximation takes in the *relative* ratio of 
    // refraction instead of the absolute index
    fn reflectance(cosine: f64, ref_index: f64) -> f64 {
        let r0 = ((1.0 - ref_index) / (1.0 + ref_index)).powf(2.0);
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

impl Material for Dielectric {

    fn scatter(&self, ray: Ray, hit_record: &HitRecord) -> Option<ScatteredRay> {
        // when the ray intersects on the front face, the ratio will be
        // that of outside air (1.0) to the material (self.ir)
        let unit_direction = ray.direction.unit();
        let cos_theta = Vec3::dot(unit_direction, -hit_record.normal);
        let sin_theta = (1.0 - cos_theta.powf(2.0)).sqrt();

        let refraction_ratio = if hit_record.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        // if the incident angle exceeds the critical angle, the ray will
        // completely reflect. otherwise, the ray partially reflects/refracts
        // with probability given by Shlick's approximation
        let cannot_refract = sin_theta * refraction_ratio > 1.0;

        let direction = if cannot_refract || (Dielectric::reflectance(cos_theta, refraction_ratio) > rand::random()) {
            reflect(unit_direction, hit_record.normal)
        } else {
            refract(unit_direction, hit_record.normal, refraction_ratio)
        };

        Some(ScatteredRay {
            ray: Ray::new(hit_record.point, direction),
            attenuation: Color::new(1.0, 1.0, 1.0),
        })
    }
}
