mod camera;
mod color;
mod hittable;
mod interval;
mod objects;
mod ray;
mod vec3;

use std::rc::Rc;

use crate::camera::Camera;
use crate::objects::HittableList;
use crate::objects::Sphere;
use crate::vec3::Point;

fn main() {
    // aspect ratio: width / height;
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    // camera settings
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let samples_per_pixel = 100;
    // max number of ray bounces
    let max_depth = 10;
    let camera = Camera::new(aspect_ratio, image_width, focal_length, viewport_height, samples_per_pixel, max_depth);

    // create world
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point::new(2.0, 2.0, -5.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)));

    camera.render(&world);
}
