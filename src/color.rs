use std::io;

pub use crate::vec3::Vec3;

pub use Vec3 as Color;

fn linear_to_gamma(x: f64) -> f64 {
    x.sqrt()
}

pub fn write_color<T: io::Write>(mut out: T, agg_pixel_color: Color, num_samples: i32) {
    let r = linear_to_gamma(agg_pixel_color.x / (num_samples as f64));
    let g = linear_to_gamma(agg_pixel_color.y / (num_samples as f64));
    let b = linear_to_gamma(agg_pixel_color.z / (num_samples as f64));
    writeln!(
        out,
        "{} {} {}",
        (256.0 * r.clamp(0.0, 0.999)) as u8,
        (256.0 * g.clamp(0.0, 0.999)) as u8,
        (256.0 * b.clamp(0.0, 0.999)) as u8,
    )
    .expect("Failed to write color");
}
