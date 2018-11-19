use std::f32::consts::PI;

use rand::Rng;

use ray::Ray;
use rnd::random_in_unit_disk;
use vec::{cross, Vec3};

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        origin: Vec3,
        look_at: Vec3,
        vup: Vec3,
        vfov: f32, /* deg */
        aspect: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Self {
        let theta = vfov * PI / 180.;
        let half_height = (theta / 2.).tan();
        let half_width = aspect * half_height;
        let w = (origin - look_at).unit_vector();
        let u = cross(vup, w).unit_vector();
        let v = cross(w, u);
        Self {
            origin,
            lower_left_corner: origin
                - half_width * focus_dist * u
                - half_height * focus_dist * v
                - focus_dist * w,
            horizontal: 2. * half_width * focus_dist * u,
            vertical: 2. * half_height * focus_dist * v,
            u,
            v,
            lens_radius: aperture / 2.,
        }
    }

    pub fn get_ray(&self, rng: &mut impl Rng, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk(rng);
        let offset = rd.x() * self.u + rd.y() * self.v;
        Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner + (s * self.horizontal) + (t * self.vertical)
                - self.origin
                - offset,
        }
    }
}
