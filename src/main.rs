use std::io::{self, stdout};

use ray::Ray;
use vec::{dot, Vec3};

mod ppm;
mod ray;
mod vec;

fn hit_sphere(center: Vec3, radius: f32, ray: &Ray) -> bool {
    let oc = ray.origin - center;
    let a = dot(ray.direction, ray.direction);
    let b = 2. * dot(oc, ray.direction);
    let c = dot(oc, oc) - (radius * radius);
    let discr = (b * b) - (4. * a * c);
    discr > 0.
}

fn color(ray: &Ray) -> Vec3 {
    if hit_sphere(Vec3(0., 0., -1.), 0.5, ray) {
        return Vec3(1., 0., 0.);
    }
    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.);
    (1. - t) * Vec3(1., 1., 1.) + t * Vec3(0.5, 0.7, 1.)
}

fn main() -> Result<(), io::Error> {
    let nx = 200;
    let ny = 100;
    let mut ppm = ppm::Writer::new(stdout(), nx, ny)?;

    let lower_left_corner = Vec3(-2., -1., -1.);
    let horizontal = Vec3(4., 0., 0.);
    let vertical = Vec3(0., 2., 0.);
    let origin = Vec3(0., 0., 0.);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;

            let r = Ray {
                origin,
                direction: lower_left_corner + (u * horizontal) + (v * vertical),
            };
            let c = color(&r);

            let r = (255.99 * c.0) as u8;
            let g = (255.99 * c.1) as u8;
            let b = (255.99 * c.2) as u8;

            ppm.write_pixel(r, g, b)?;
        }
    }
    Ok(())
}
