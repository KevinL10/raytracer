use std::io;

pub use crate::vec3::Vec3;

pub use Vec3 as Color;

pub fn write_color<T: io::Write>(mut out: T, pixel: Color) {
    let r = (255.999 * pixel.x) as u8;
    let g = (255.999 * pixel.y) as u8;
    let b = (255.999 * pixel.z) as u8;
    writeln!(out, "{} {} {}", r, g, b).expect("Failed to write color");
}
