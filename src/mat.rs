use rand::distributions::Standard;
use rand::Rng;

use crate::hit::HitRecord;
use crate::ray::Ray;
use crate::rnd::random_in_unit_sphere;
use crate::vec::{dot, Vec3};

pub enum Material {
    Lambertian {
        albedo: Vec3,
    },
    Metal {
        albedo: Vec3,
        fuzz: f32, /* 0..1 */
    },
    Dielectric {
        ref_idx: f32,
    },
}

pub struct Scatter {
    pub attenuation: Vec3,
    pub scattered: Ray,
}

impl Material {
    pub fn scatter(&self, rng: &mut impl Rng, r_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
        match self {
            Material::Lambertian { albedo } => {
                let target = rec.p + rec.normal + random_in_unit_sphere(rng);
                let scattered = Ray {
                    origin: rec.p,
                    direction: target - rec.p,
                };
                Some(Scatter {
                    scattered,
                    attenuation: *albedo,
                })
            }
            Material::Metal { albedo, fuzz } => {
                let reflected = reflect(r_in.direction.unit_vector(), rec.normal);
                let scattered = Ray {
                    origin: rec.p,
                    direction: reflected + *fuzz * random_in_unit_sphere(rng),
                };
                if dot(scattered.direction, rec.normal) > 0. {
                    Some(Scatter {
                        scattered,
                        attenuation: *albedo,
                    })
                } else {
                    None
                }
            }
            Material::Dielectric { ref_idx } => {
                let reflected = reflect(r_in.direction, rec.normal);
                let outward_normal;
                let ni_over_nt;
                let cosine;
                if dot(r_in.direction, rec.normal) > 0. {
                    outward_normal = -rec.normal;
                    ni_over_nt = *ref_idx;
                    cosine = ref_idx * dot(r_in.direction, rec.normal) / r_in.direction.length();
                } else {
                    outward_normal = rec.normal;
                    ni_over_nt = 1. / ref_idx;
                    cosine = -dot(r_in.direction, rec.normal) / r_in.direction.length();
                };
                let direction = match refract(r_in.direction, outward_normal, ni_over_nt) {
                    Some(refracted) => {
                        let reflect_prob = schlick(cosine, *ref_idx);
                        if rng.sample::<f32, _>(Standard) < reflect_prob {
                            reflected
                        } else {
                            refracted
                        }
                    }
                    None => reflected,
                };
                let scattered = Ray {
                    origin: rec.p,
                    direction,
                };
                Some(Scatter {
                    scattered,
                    attenuation: Vec3(1., 1., 1.),
                })
            }
        }
    }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2. * dot(v, n) * n
}

fn refract(v: Vec3, n: Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = v.unit_vector();
    let dt = dot(uv, n);
    let discriminant = 1. - ni_over_nt * ni_over_nt * (1. - dt * dt);
    if discriminant > 0. {
        Some(ni_over_nt * (uv - dt * n) - discriminant.sqrt() * n)
    } else {
        None
    }
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = ((1. - ref_idx) / (1. + ref_idx)).powi(2);
    r0 + (1. - r0) * (1. - cosine).powf(5.)
}
