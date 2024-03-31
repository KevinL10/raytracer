use crate::color::{write_color, Color};
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{Point, Vec3};

use std::f64::INFINITY;
use std::io;

pub struct Camera {
    image_width: i32,
    image_height: i32,
    center: Point,
    pixel00_loc: Point,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    samples_per_pixel: i32,
    max_depth: i32,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: i32,
        focal_length: f64,
        viewport_height: f64,
        samples_per_pixel: i32,
        max_depth: i32,
    ) -> Self {
        let image_height = ((image_width as f64) / aspect_ratio) as i32;

        // setup camera / viewport
        let viewport_width = viewport_height * (image_width as f64) / (image_height as f64);
        eprintln!(
            "viewport: height={}, width={}",
            viewport_height, viewport_width
        );

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / (image_width as f64);
        let pixel_delta_v = viewport_v / (image_height as f64);

        // compute camera center and upper-left pixel positions
        let center = Point::new(0.0, 0.0, 0.0);
        let viewport_upper_left =
            center - (viewport_u / 2.0) - (viewport_v / 2.0) - Vec3::new(0.0, 0.0, focal_length);
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Self {
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
            max_depth,
        }
    }

    pub fn render(&self, world: &dyn Hittable) {
        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        for i in 0..self.image_height {
            eprintln!("Scanlines remaining: {}", self.image_height - i);
            for j in 0..self.image_width {
                let mut agg_pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    agg_pixel_color += Camera::ray_color(ray, self.max_depth, world);
                }

                write_color(io::stdout(), agg_pixel_color, self.samples_per_pixel);
            }
        }

        eprintln!("Finished.");
    }

    fn ray_color(ray: Ray, depth: i32, world: &dyn Hittable) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        if let Some(hit_record) = world.hit(ray, Interval::new(0.001, INFINITY)) {
            // propagate the light ray if the ray is scattered. otherwise,
            // the material absorbs all of the light, and the color is black.
            if let Some(scattered_ray) = hit_record.material.scatter(ray, &hit_record) {
                return scattered_ray.attenuation
                    * Camera::ray_color(scattered_ray.ray, depth - 1, world);
            }
            return Color::new(0.0, 0.0, 0.0);
        };

        let y = 0.5 * (ray.direction.unit().y + 1.0);
        Color::new(1.0, 1.0, 1.0) * (1.0 - y) + Color::new(0.5, 0.7, 1.0) * y
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        // Return a ray to a random point within the square surrounding the (i, j)'th pixel
        let pixel_center = self.pixel00_loc + self.pixel_delta_u * j + self.pixel_delta_v * i;
        let pixel_sample = pixel_center
            + (-0.5 + rand::random::<f64>()) * self.pixel_delta_u
            + (-0.5 + rand::random::<f64>()) * self.pixel_delta_v;

        let ray_direction = pixel_sample - self.center;

        Ray::new(self.center, ray_direction)
    }
}
