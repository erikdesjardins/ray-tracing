use std::io::{self, stdout};

use ray::Ray;
use vec::{dot, Vec3};

mod ppm;
mod ray;
mod vec;

fn hit_sphere(center: Vec3, radius: f32, r: &Ray) -> f32 {
    let oc = r.origin - center;
    let a = dot(r.direction, r.direction);
    let b = 2. * dot(oc, r.direction);
    let c = dot(oc, oc) - (radius * radius);
    let discr = (b * b) - (4. * a * c);
    if discr < 0. {
        -1.
    } else {
        (-b - discr.sqrt()) / (2. * a)
    }
}

fn color(r: &Ray) -> Vec3 {
    let t = hit_sphere(Vec3(0., 0., -1.), 0.5, r);
    if t > 0. {
        let n = (r.point_at(t) - Vec3(0., 0., -1.)).unit_vector();
        return 0.5 * Vec3(n.x() + 1., n.y() + 1., n.z() + 1.);
    }
    let unit_direction = r.direction.unit_vector();
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
