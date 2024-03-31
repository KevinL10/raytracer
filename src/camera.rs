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
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: i32, focal_length: f64, viewport_height: f64) -> Self {
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
        }
    }

    pub fn render(&self, world: &dyn Hittable) {
        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        for i in 0..self.image_height {
            // eprintln!("Scanlines remaining: {}", image_height - i);
            for j in 0..self.image_width {
                let pixel_center =
                    self.pixel00_loc + self.pixel_delta_u * j + self.pixel_delta_v * i;
                let ray_direction = pixel_center - self.center;

                let ray = Ray::new(self.center, ray_direction);
                let color = Camera::ray_color(ray, world);
                write_color(io::stdout(), color);
            }
        }

        eprintln!("Finished.");
    }

    fn ray_color(ray: Ray, world: &dyn Hittable) -> Color {
        if let Some(hit_record) = world.hit(ray, Interval::new(0.0, INFINITY)) {
            return 0.5 * (hit_record.normal + Vec3::new(1.0, 1.0, 1.0));
        };

        let y = 0.5 * (ray.direction.unit().y + 1.0);
        Color::new(1.0, 1.0, 1.0) * (1.0 - y) + Color::new(0.5, 0.7, 1.0) * y
    }
}
