mod camera;
mod color;
mod hittable;
mod interval;
mod material;
mod objects;
mod ray;
mod vec3;

use std::sync::Arc;

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
    let image_width = 1200;

    // camera settings
    let vfov = 30.0; // vertical view angle
    let samples_per_pixel = 200;
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
    let center: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let ground: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let left: Arc<dyn Material> = Arc::new(Dielectric::new(1.5));
    let right: Arc<dyn Material> = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.1));

    let high: Arc<dyn Material> = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.1));

    // create objects
    let mut world = HittableList::new();
    world.add(Arc::new(Sphere::new(
        Point::new(0.0, 0.0, -1.0),
        0.5,
        Arc::clone(&center),
    )));
    world.add(Arc::new(Sphere::new(
        Point::new(-1.0, 0.0, -1.0),
        -0.4,
        Arc::clone(&left),
    )));
    world.add(Arc::new(Sphere::new(
        Point::new(-1.0, 0.0, -1.0),
        0.5,
        Arc::clone(&left),
    )));
    world.add(Arc::new(Sphere::new(
        Point::new(1.0, 0.0, -1.0),
        0.5,
        Arc::clone(&right),
    )));
    world.add(Arc::new(Sphere::new(
        Point::new(3.0, 3.0, -5.0),
        1.0,
        Arc::clone(&high),
    )));
    world.add(Arc::new(Sphere::new(
        Point::new(0.0, -100.5, -1.0),
        100.0,
        Arc::clone(&ground),
    )));

    camera.render(&world);
}

fn book_cover() {
    let mut world = HittableList::new();
    world.add(Arc::new(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5))),
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
                    world.add(Arc::new(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Lambertian::new(albedo)),
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Vec3::random_range(0.5, 1.0);
                    let fuzz = rand::random::<f64>() / 2.0;
                    world.add(Arc::new(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Metal::new(albedo, fuzz)),
                    )));
                } else {
                    // glass
                    world.add(Arc::new(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Dielectric::new(1.5)),
                    )));
                }
            }
        }
    }

    world.add(Arc::new(Sphere::new(
        Point::new(0.0, 1.0, 0.0),
        1.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    world.add(Arc::new(Sphere::new(
        Point::new(-4.0, 1.0, 0.0),
        1.0,
        Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1))),
    )));
    world.add(Arc::new(Sphere::new(
        Point::new(4.0, 1.0, 0.0),
        1.0,
        Arc::new(Metal::new(Color::new(0.4, 0.2, 0.1), 0.0)),
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

fn pool_table() {
    // pool table with solid colors, low-angle shot
    // inner table spans x-axis (-8, 8) and z-axis (-4, 4)
    let blue: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.1, 0.1, 0.8)));
    let brown: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.3, 0.16, 0.09)));
    let rail_green: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.0, 0.3, 0.1)));
    let white: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.08, 0.34, 0.26)));

    let ball_colors: Vec<Arc<dyn Material>> = vec![
        Arc::new(Lambertian::new(Color::new(0.7, 0.0, 0.0))), // red
        Arc::new(Lambertian::new(Color::new(0.0, 0.17, 0.7))), // blue
        Arc::new(Lambertian::new(Color::new(1.0, 0.6, 0.7))), // orange
        Arc::new(Lambertian::new(Color::new(0.0, 0.6, 0.2))), // green
    ];

    let mut world = HittableList::new();
    // floor
    world.add(Arc::new(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::clone(&rail_green)
    )));

    // balls
    for color in ball_colors.iter() {
        world.add(Arc::new(Sphere::new(
            Point::new(
                -6.0 * (rand::random::<f64>() * 2.0 - 1.0),
                0.5,
                -2.0 * (rand::random::<f64>() * 2.0 - 1.0),
            ),
            0.5,
            Arc::clone(&color),
        )));
    }

    // white ball
    world.add(Arc::new(Sphere::new(
        Point::new(2.0, 0.5, 0.0),
        0.5,
        Arc::clone(&white),
    )));

    // table top rail (formed from adjacent spheres of radius 0.7)
    let rail_radius = 0.7;
    let rail_spacing = 0.2;
    for i in (1.0 / rail_spacing) as i32..(8.0 / rail_spacing) as i32 {
        world.add(Arc::new(Sphere::new(
            Point::new(-8.0 + rail_spacing * i as f64, rail_radius, -4.0),
            rail_radius,
            Arc::clone(&rail_green),
        )));
    }

    // table left rail (formed from adjacent spheres of radius 0.7)
    for i in (1.0 / rail_spacing) as i32..(8.0 / rail_spacing) as i32 {
        world.add(Arc::new(Sphere::new(
            Point::new(-8.0, rail_radius, -4.0 + rail_spacing * i as f64),
            rail_radius,
            Arc::clone(&rail_green),
        )));
    }

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    // camera settings
    let vfov = 20.0;
    let samples_per_pixel = 100;
    // max number of ray bounces
    let max_depth = 10;
    // look from center-right to the top left pocket
    let lookfrom = Point::new(8.0, 1.5, 2.0);
    let lookat = Point::new(2.0, 0.5, 0.0);

    // DEBUG: camera from high up
    // let lookfrom = Point::new(0.0, 10.0, 1.0);
    // let lookat = Point::new(0.0, 0.0, 0.0);

    let vup = Vec3::new(0.0, 1.0, 0.0);
    let defocus_angle: f64 = 0.0;

    // for our raytracer, focus_dist is the same as focal_length
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
    pool_table();
}
