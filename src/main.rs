use std::f32;
use std::io::{self, stdout};

use rand::distributions::Standard;
use rand::{Rng, SeedableRng, XorShiftRng};

use cam::Camera;
use hit::Hittable;
use ray::Ray;
use sph::Sphere;
use vec::Vec3;

extern crate rand;

mod cam;
mod hit;
mod ppm;
mod ray;
mod sph;
mod vec;

fn color(r: &Ray, world: impl Hittable) -> Vec3 {
    match world.hit(r, 0.0..f32::MAX) {
        Some(rec) => {
            0.5 * Vec3(
                rec.normal.x() + 1.,
                rec.normal.y() + 1.,
                rec.normal.z() + 1.,
            )
        }
        None => {
            let unit_direction = r.direction.unit_vector();
            let t = 0.5 * (unit_direction.y() + 1.);
            (1. - t) * Vec3(1., 1., 1.) + t * Vec3(0.5, 0.7, 1.)
        }
    }
}

fn main() -> Result<(), io::Error> {
    let mut rng = XorShiftRng::from_seed([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);

    let nx = 200;
    let ny = 100;
    let ns = 100;

    let mut ppm = ppm::Writer::new(stdout(), nx, ny)?;

    let world = vec![
        Sphere {
            center: Vec3(0., 0., -1.),
            radius: 0.5,
        },
        Sphere {
            center: Vec3(0., -100.5, -1.),
            radius: 100.,
        },
    ];

    let cam = Camera {
        origin: Vec3(0., 0., 0.),
        lower_left_corner: Vec3(-2., -1., -1.),
        horizontal: Vec3(4., 0., 0.),
        vertical: Vec3(0., 2., 0.),
    };

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3(0., 0., 0.);
            for _ in 0..ns {
                let u = (i as f32 + rng.sample::<f32, _>(Standard)) / nx as f32;
                let v = (j as f32 + rng.sample::<f32, _>(Standard)) / ny as f32;
                let r = cam.get_ray(u, v);
                col += color(&r, &world);
            }
            col /= ns as f32;

            let r = (255.99 * col.0) as u8;
            let g = (255.99 * col.1) as u8;
            let b = (255.99 * col.2) as u8;

            ppm.write_pixel(r, g, b)?;
        }
    }
    Ok(())
}
