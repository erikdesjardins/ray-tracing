use std::f32::consts::PI;

use ray::Ray;
use vec::{cross, Vec3};

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(
        origin: Vec3,
        look_at: Vec3,
        vup: Vec3,
        vfov: f32, /* deg */
        aspect: f32,
    ) -> Self {
        let theta = vfov * PI / 180.;
        let half_height = (theta / 2.).tan();
        let half_width = aspect * half_height;
        let w = (origin - look_at).unit_vector();
        let u = cross(vup, w).unit_vector();
        let v = cross(w, u);
        Self {
            origin,
            lower_left_corner: origin - half_width * u - half_height * v - w,
            horizontal: 2. * half_width * u,
            vertical: 2. * half_height * v,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + (u * self.horizontal) + (v * self.vertical)
                - self.origin,
        }
    }
}
