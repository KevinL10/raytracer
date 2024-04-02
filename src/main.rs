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

fn main() {
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
