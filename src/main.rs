use std::io::{self, stdout};

use vec::Vec3;

mod ppm;
mod vec;

fn main() -> Result<(), io::Error> {
    let x = 200;
    let y = 100;
    let mut ppm = ppm::Writer::new(stdout(), x, y)?;
    for j in (0..y).rev() {
        for i in 0..x {
            let r = i as f32 / x as f32;
            let g = j as f32 / y as f32;
            let b = 0.2;
            let color = Vec3 {
                x: (255.99 * r) as u8,
                y: (255.99 * g) as u8,
                z: (255.99 * b) as u8,
            };
            ppm.write_pixel(&color)?;
        }
    }
    Ok(())
}
