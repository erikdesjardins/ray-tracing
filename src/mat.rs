use rand::Rng;

use hit::HitRecord;
use ray::Ray;
use rnd::random_in_unit_sphere;
use vec::{dot, Vec3};

pub enum Material {
    Lambertian {
        albedo: Vec3,
    },
    Metal {
        albedo: Vec3,
        fuzz: f32, /* 0..1 */
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
        }
    }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2. * dot(v, n) * n
}
