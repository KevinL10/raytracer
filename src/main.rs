mod color;
mod ray;
mod vec3;

use std::io;

use crate::color::{write_color, Color};
use crate::ray::Ray;
use crate::vec3::{Point, Vec3};

fn ray_color(ray: Ray) -> Color {
    if hit_sphere(Point::new(0.0, 4.0, -10.0), 1.0, ray) {
        return Color::new(1.0, 0.0, 0.0);
    }

    let y = 0.5 * (ray.direction.unit().y + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - y) + Color::new(0.5, 0.7, 1.0) * y
}

fn hit_sphere(center: Point, radius: f64, ray: Ray) -> bool {
    let oc = ray.orig - center;
    let a = ray.direction.length().powf(2.0);
    let b = 2.0 * Vec3::dot(ray.direction, oc);
    let c = oc.length().powf(2.0) - radius.powf(2.0);

    let discriminant = b * b - 4.0 * a * c;
    discriminant >= 0.0
}

fn main() {
    // aspect ratio: width / height;
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = ((image_width as f64) / aspect_ratio) as i32;

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64) / (image_height as f64);
    eprintln!(
        "viewport: height={}, width={}",
        viewport_height, viewport_width
    );

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / (image_width as f64);
    let pixel_delta_v = viewport_v / (image_height as f64);

    let camera_center = Point::new(0.0, 0.0, 0.0);
    let viewport_upper_left =
        camera_center - (viewport_u / 2.0) - (viewport_v / 2.0) - Vec3::new(0.0, 0.0, focal_length);
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    println!("P3\n{image_width} {image_height}\n255");

    for i in 0..image_height {
        // eprintln!("Scanlines remaining: {}", image_height - i);
        for j in 0..image_width {
            let pixel_center = pixel00_loc + pixel_delta_u * j + pixel_delta_v * i;
            let ray_direction = pixel_center - camera_center;

            let ray = Ray::new(pixel_center, ray_direction);

            write_color(io::stdout(), ray_color(ray));
        }
    }

    eprintln!("Finished.");
}
