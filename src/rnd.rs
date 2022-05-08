use rand::distributions::Standard;
use rand::Rng;

use crate::vec::Vec3;

pub fn random_in_unit_sphere(rng: &mut impl Rng) -> Vec3 {
    loop {
        let p =
            2.0 * Vec3(
                rng.sample(Standard),
                rng.sample(Standard),
                rng.sample(Standard),
            ) - Vec3(1., 1., 1.);
        if p.squared_length() < 1. {
            return p;
        }
    }
}

pub fn random_in_unit_disk(rng: &mut impl Rng) -> Vec3 {
    loop {
        let p = 2.0 * Vec3(rng.sample(Standard), rng.sample(Standard), 0.) - Vec3(1., 1., 0.);
        if p.squared_length() < 1. {
            return p;
        }
    }
}
