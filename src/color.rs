use std::io;

pub use crate::vec3::Vec3;

pub use Vec3 as Color;

pub fn write_color<T: io::Write>(mut out: T, agg_pixel_color: Color, num_samples: i32) {
    let r = (256.0 * agg_pixel_color.x / (num_samples as f64)) as u8;
    let g = (256.0 * agg_pixel_color.y / (num_samples as f64)) as u8;
    let b = (256.0 * agg_pixel_color.z / (num_samples as f64)) as u8;
    writeln!(
        out,
        "{} {} {}",
        r.clamp(0, 255),
        g.clamp(0, 255),
        b.clamp(0, 255)
    )
    .expect("Failed to write color");
}
