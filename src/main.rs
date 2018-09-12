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

fn random_in_unit_sphere(rng: &mut impl Rng) -> Vec3 {
    loop {
        let p = 2.0 * Vec3(
            rng.sample(Standard),
            rng.sample(Standard),
            rng.sample(Standard),
        ) - Vec3(1., 1., 1.);
        if p.squared_length() < 1. {
            return p;
        }
    }
}

fn color(rng: &mut impl Rng, r: &Ray, world: impl Hittable) -> Vec3 {
    match world.hit(r, 0.001..f32::MAX) {
        Some(rec) => {
            let target = rec.p + rec.normal + random_in_unit_sphere(rng);
            0.5 * color(
                rng,
                &Ray {
                    origin: rec.p,
                    direction: target - rec.p,
                },
                world,
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
                col += color(&mut rng, &r, &world);
            }
            let col = col / ns as f32;

            let col = Vec3(col.0.sqrt(), col.1.sqrt(), col.2.sqrt());
            let r = (255.99 * col.0) as u8;
            let g = (255.99 * col.1) as u8;
            let b = (255.99 * col.2) as u8;

            ppm.write_pixel(r, g, b)?;
        }
    }
    Ok(())
}
