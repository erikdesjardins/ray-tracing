#![cfg_attr(not(feature = "cargo-clippy"), allow(unknown_lints))]
#![allow(many_single_char_names)]

use std::f32;
use std::io::{self, stdout};
use std::time::Instant;

use rand::distributions::Standard;
use rand::{Rng, SeedableRng, XorShiftRng};

use cam::Camera;
use hit::Hittable;
use mat::{Material, Scatter};
use ray::Ray;
use sph::Sphere;
use vec::Vec3;

extern crate rand;

mod cam;
mod hit;
mod mat;
mod ppm;
mod ray;
mod rnd;
mod sph;
mod vec;

fn color(rng: &mut impl Rng, r: &Ray, world: &impl Hittable, depth: u32) -> Vec3 {
    match world.hit(r, 0.001..f32::MAX) {
        Some(rec) => match rec.material.scatter(rng, r, &rec) {
            Some(Scatter {
                attenuation,
                ref scattered,
            })
                if depth < 50 =>
            {
                attenuation * color(rng, scattered, world, depth + 1)
            }
            _ => Vec3(0., 0., 0.),
        },
        None => {
            let unit_direction = r.direction.unit_vector();
            let t = 0.5 * (unit_direction.y() + 1.);
            (1. - t) * Vec3(1., 1., 1.) + t * Vec3(0.5, 0.7, 1.)
        }
    }
}

fn main() -> Result<(), io::Error> {
    let start = Instant::now();

    let mut rng = XorShiftRng::from_seed([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);

    let nx = 400;
    let ny = 200;
    let ns = 100;

    let mut ppm = ppm::Writer::new(stdout(), nx, ny)?;

    let world = random_scene(&mut rng);

    let origin = Vec3(5., 1.5, 3.);
    let look_at = Vec3(0., -1., 0.);
    let dist_to_focus = (origin - look_at).length();
    let aperture = 0.1;
    let cam = Camera::new(
        origin,
        look_at,
        Vec3(0., 1., 0.),
        70.,
        nx as f32 / ny as f32,
        aperture,
        dist_to_focus,
    );

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3(0., 0., 0.);
            for _ in 0..ns {
                let s = (i as f32 + rng.sample::<f32, _>(Standard)) / nx as f32;
                let t = (j as f32 + rng.sample::<f32, _>(Standard)) / ny as f32;
                let r = cam.get_ray(&mut rng, s, t);
                col += color(&mut rng, &r, &world, 0);
            }
            let col = col / ns as f32;

            let col = Vec3(col.0.sqrt(), col.1.sqrt(), col.2.sqrt());
            let ir = (255.99 * col.0) as u8;
            let ig = (255.99 * col.1) as u8;
            let ib = (255.99 * col.2) as u8;

            ppm.write_pixel(ir, ig, ib)?;
        }
    }

    let elapsed = start.elapsed();
    eprintln!("{}.{:0>3}s", elapsed.as_secs(), elapsed.subsec_millis());

    Ok(())
}

fn random_scene(rng: &mut impl Rng) -> impl Hittable {
    let mut spheres = Vec::new();
    spheres.push(Sphere {
        center: Vec3(0., -1000., 0.),
        radius: 1000.,
        material: Material::Lambertian {
            albedo: Vec3(0.5, 0.5, 0.5),
        },
    });
    spheres.push(Sphere {
        center: Vec3(0., 1., 0.),
        radius: 1.,
        material: Material::Dielectric { ref_idx: 1.5 },
    });
    spheres.push(Sphere {
        center: Vec3(-4., 1., 0.),
        radius: 1.,
        material: Material::Lambertian {
            albedo: Vec3(0.4, 0.2, 0.1),
        },
    });
    spheres.push(Sphere {
        center: Vec3(4., 1., 0.),
        radius: 1.,
        material: Material::Metal {
            albedo: Vec3(0.7, 0.6, 0.5),
            fuzz: 0.,
        },
    });
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.sample::<f32, _>(Standard);
            let center = Vec3(
                a as f32 + 0.9 * rng.sample::<f32, _>(Standard),
                0.2,
                b as f32 + 0.9 * rng.sample::<f32, _>(Standard),
            );
            if (center - Vec3(4., 0.2, 0.)).length() > 0.9 {
                let material = if choose_mat < 0.8 {
                    Material::Lambertian {
                        albedo: Vec3(
                            rng.sample::<f32, _>(Standard) * rng.sample::<f32, _>(Standard),
                            rng.sample::<f32, _>(Standard) * rng.sample::<f32, _>(Standard),
                            rng.sample::<f32, _>(Standard) * rng.sample::<f32, _>(Standard),
                        ),
                    }
                } else if choose_mat < 0.95 {
                    Material::Metal {
                        albedo: Vec3(
                            0.5 * (1. + rng.sample::<f32, _>(Standard)),
                            0.5 * (1. + rng.sample::<f32, _>(Standard)),
                            0.5 * (1. + rng.sample::<f32, _>(Standard)),
                        ),
                        fuzz: 0.5 * rng.sample::<f32, _>(Standard),
                    }
                } else {
                    Material::Dielectric { ref_idx: 1.5 }
                };
                spheres.push(Sphere {
                    center,
                    radius: 0.2,
                    material,
                });
            }
        }
    }
    spheres
}
