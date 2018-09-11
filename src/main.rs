use std::f32;
use std::io::{self, stdout};

use hit::Hittable;
use ray::Ray;
use sphere::Sphere;
use vec::Vec3;

mod hit;
mod ppm;
mod ray;
mod sphere;
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
    let nx = 200;
    let ny = 100;
    let mut ppm = ppm::Writer::new(stdout(), nx, ny)?;

    let lower_left_corner = Vec3(-2., -1., -1.);
    let horizontal = Vec3(4., 0., 0.);
    let vertical = Vec3(0., 2., 0.);
    let origin = Vec3(0., 0., 0.);

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

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;

            let r = Ray {
                origin,
                direction: lower_left_corner + (u * horizontal) + (v * vertical),
            };
            let c = color(&r, &world);

            let r = (255.99 * c.0) as u8;
            let g = (255.99 * c.1) as u8;
            let b = (255.99 * c.2) as u8;

            ppm.write_pixel(r, g, b)?;
        }
    }
    Ok(())
}
