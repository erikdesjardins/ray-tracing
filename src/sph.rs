use std::ops::Range;

use hit::{HitRecord, Hittable};
use mat::Material;
use ray::Ray;
use vec::{dot, Vec3};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_range: Range<f32>) -> Option<HitRecord<'_>> {
        let oc = r.origin - self.center;
        let a = dot(r.direction, r.direction);
        let b = dot(oc, r.direction);
        let c = dot(oc, oc) - (self.radius * self.radius);
        let discr = (b * b) - (a * c);
        if discr > 0. {
            let temp = (-b - (b * b - a * c).sqrt()) / a;
            if t_range.start < temp && temp < t_range.end {
                let t = temp;
                let p = r.point_at(t);
                let normal = (p - self.center) / self.radius;
                return Some(HitRecord {
                    t,
                    p,
                    normal,
                    material: &self.material,
                });
            }
            let temp = (-b + (b * b - a * c).sqrt()) / a;
            if t_range.start < temp && temp < t_range.end {
                let t = temp;
                let p = r.point_at(t);
                let normal = (p - self.center) / self.radius;
                return Some(HitRecord {
                    t,
                    p,
                    normal,
                    material: &self.material,
                });
            }
        }
        None
    }
}
