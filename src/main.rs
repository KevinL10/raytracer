mod camera;
mod color;
mod hittable;
mod interval;
mod material;
mod objects;
mod ray;
mod vec3;

use std::rc::Rc;

use material::Dielectric;

use crate::camera::Camera;
use crate::color::Color;
use crate::material::{Lambertian, Material, Metal};
use crate::objects::HittableList;
use crate::objects::Sphere;
use crate::vec3::{Point, Vec3};

fn basic_world() {
    // aspect ratio: width / height;
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    // camera settings
    let vfov = 30.0; // vertical view angle
    let samples_per_pixel = 20;
    // max number of ray bounces
    let max_depth = 10;
    let lookfrom = Point::new(-2.0, 2.0, 1.0);
    let lookat = Point::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let defocus_angle: f64 = 10.0;

    // for our raytracer, focus_dist is the same as focal_length
    let focus_dist = 3.4;
    let camera = Camera::new(
        aspect_ratio,
        image_width,
        vfov,
        samples_per_pixel,
        max_depth,
        lookfrom,
        lookat,
        vup,
        defocus_angle,
        focus_dist,
    );

    // create materials
    let center: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let ground: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let left: Rc<dyn Material> = Rc::new(Dielectric::new(1.5));
    let right: Rc<dyn Material> = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.1));

    let high: Rc<dyn Material> = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.1));

    // create objects
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(
        Point::new(0.0, 0.0, -1.0),
        0.5,
        Rc::clone(&center),
    )));
    world.add(Rc::new(Sphere::new(
        Point::new(-1.0, 0.0, -1.0),
        -0.4,
        Rc::clone(&left),
    )));
    world.add(Rc::new(Sphere::new(
        Point::new(-1.0, 0.0, -1.0),
        0.5,
        Rc::clone(&left),
    )));
    world.add(Rc::new(Sphere::new(
        Point::new(1.0, 0.0, -1.0),
        0.5,
        Rc::clone(&right),
    )));
    world.add(Rc::new(Sphere::new(
        Point::new(3.0, 3.0, -5.0),
        1.0,
        Rc::clone(&high),
    )));
    world.add(Rc::new(Sphere::new(
        Point::new(0.0, -100.5, -1.0),
        100.0,
        Rc::clone(&ground),
    )));

    camera.render(&world);
}

fn book_cover() {
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5))),
    )));

    for a in -11..=11 {
        for b in -11..=11 {
            let choose_mat = rand::random::<f64>();
            let center = Point::new(
                a as f64 + 0.9 * rand::random::<f64>(),
                0.2,
                b as f64 + 0.9 * rand::random::<f64>(),
            );

            if (center - Point::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Vec3::random() * Vec3::random();
                    world.add(Rc::new(Sphere::new(
                        center,
                        0.2,
                        Rc::new(Lambertian::new(albedo)),
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Vec3::random_range(0.5, 1.0);
                    let fuzz = rand::random::<f64>() / 2.0;
                    world.add(Rc::new(Sphere::new(
                        center,
                        0.2,
                        Rc::new(Metal::new(albedo, fuzz)),
                    )));
                } else {
                    // glass
                    world.add(Rc::new(Sphere::new(
                        center,
                        0.2,
                        Rc::new(Dielectric::new(1.5)),
                    )));
                }
            }
        }
    }

    world.add(Rc::new(Sphere::new(
        Point::new(0.0, 1.0, 0.0),
        1.0,
        Rc::new(Dielectric::new(1.5)),
    )));
    world.add(Rc::new(Sphere::new(
        Point::new(-4.0, 1.0, 0.0),
        1.0,
        Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1))),
    )));
    world.add(Rc::new(Sphere::new(
        Point::new(4.0, 1.0, 0.0),
        1.0,
        Rc::new(Metal::new(Color::new(0.4, 0.2, 0.1), 0.0)),
    )));

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1200;
    let samples_per_pixel = 500;
    let max_depth = 50;

    let vfov = 20.0;
    let lookfrom = Point::new(13.0, 2.0, 3.0);
    let lookat = Point::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let defocus_angle = 0.6;
    let focus_dist = 10.0;

    let camera = Camera::new(
        aspect_ratio,
        image_width,
        vfov,
        samples_per_pixel,
        max_depth,
        lookfrom,
        lookat,
        vup,
        defocus_angle,
        focus_dist,
    );

    camera.render(&world);
}

fn main() {
    book_cover();
}
