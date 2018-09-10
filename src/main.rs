use std::io::{self, stdout};

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
            let r = (255.99 * r) as u8;
            let g = (255.99 * g) as u8;
            let b = (255.99 * b) as u8;
            ppm.write_pixel(r, g, b)?;
        }
    }
    Ok(())
}
