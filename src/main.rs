mod color;
mod vec3;

use std::io;

use crate::color::write_color;
use crate::vec3::Vec3;

fn main() {
    let image_height = 256;
    let image_width = 256;

    println!("P3\n{image_height} {image_width}\n255");

    for i in 0..image_height {
        eprintln!("Scanlines remaining: {}", image_height - i);
        for j in 0..image_width {
            let r = j as f64 / (image_width as f64 - 1.0);
            let g = i as f64 / (image_height as f64 - 1.0);
            let b = 0f64;

            write_color(io::stdout(), Vec3::new(r, g, b));
        }
    }

    eprintln!("Finished.");
}
