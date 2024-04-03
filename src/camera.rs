use crate::color::{write_color, Color};
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{random_in_unit_disk, Point, Vec3};

use rayon::prelude::*;
use std::f64::INFINITY;
use std::io;
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct Camera {
    image_width: i32,
    image_height: i32,
    center: Point,
    pixel00_loc: Point,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    samples_per_pixel: i32,
    max_depth: i32,

    defocus_angle: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: i32,
        vfov: f64,
        samples_per_pixel: i32,
        max_depth: i32,
        lookfrom: Point,
        lookat: Point,
        vup: Vec3,
        defocus_angle: f64,
        focus_dist: f64,
    ) -> Self {
        let image_height = ((image_width as f64) / aspect_ratio) as i32;

        // setup camera
        let w = (lookfrom - lookat).unit();
        let u = Vec3::cross(vup, w).unit();
        let v = Vec3::cross(w, u).unit();

        // setup viewport
        // tan(theta / 2) = h / focus_dist
        let h = (vfov.to_radians() / 2.0).tan() * focus_dist;
        let viewport_height = 2.0 * h;
        let viewport_width = viewport_height * (image_width as f64) / (image_height as f64);
        eprintln!(
            "viewport: height={}, width={}",
            viewport_height, viewport_width
        );

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v; // viewport_v should point downward

        let pixel_delta_u = viewport_u / (image_width as f64);
        let pixel_delta_v = viewport_v / (image_height as f64);

        // compute camera center and upper-left pixel positions
        let center = lookfrom;
        let viewport_upper_left = center - (viewport_u / 2.0) - (viewport_v / 2.0) - focus_dist * w;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let defocus_radius = (defocus_angle / 2.0).to_radians().tan() * focus_dist;
        let defocus_disk_u = defocus_radius * u;
        let defocus_disk_v = defocus_radius * v;

        eprintln!("defocus radius {}", defocus_radius);
        eprintln!("f/{}", focus_dist / (2.0 * defocus_radius));

        Self {
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
            max_depth,
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    pub fn render(&self, world: &dyn Hittable) {
        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        let start = std::time::Instant::now();

        let mut pixels = vec![
            vec![Color::new(0.0, 0.0, 0.0); self.image_width as usize];
            self.image_height as usize
        ];
        let lines_scanned = AtomicUsize::new(0);

        pixels.par_iter_mut().enumerate().for_each(|(i, row)| {
            for j in 0..self.image_width {
                let mut agg_pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i as i32, j);
                    agg_pixel_color += Camera::ray_color(ray, self.max_depth, world);
                }
                row[j as usize] = agg_pixel_color;
            }
            let cnt = lines_scanned.fetch_add(1, Ordering::SeqCst) + 1; // fetch_add returns the previous value
            eprintln!("Scanlines remaining: {}", self.image_height - cnt as i32);
        });

        for i in 0..self.image_height {
            for j in 0..self.image_width {
                write_color(
                    io::stdout(),
                    pixels[i as usize][j as usize],
                    self.samples_per_pixel,
                );
            }
        }

        eprintln!("Finished in {:?} seconds", start.elapsed());
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
        

        // blue sky 
        Color::new(1.0, 1.0, 1.0) * (1.0 - y) + Color::new(0.5, 0.7, 1.0) * y
        // sunset
        // Color::new(1.0, 1.0, 1.0) * (1.0 - y) + Color::new(0.93, 0.69, 0.38) * y
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        // Return a ray to a random point within the square surrounding the (i, j)'th pixel
        let pixel_center = self.pixel00_loc + self.pixel_delta_u * j + self.pixel_delta_v * i;
        let pixel_sample = pixel_center
            + (-0.5 + rand::random::<f64>()) * self.pixel_delta_u
            + (-0.5 + rand::random::<f64>()) * self.pixel_delta_v;

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn defocus_disk_sample(&self) -> Point {
        // return a random point in the disk around the camera center
        let disk = random_in_unit_disk();
        self.center + disk.x * self.defocus_disk_u + disk.y * self.defocus_disk_v
    }
}
