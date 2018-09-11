use std::ops::Range;

use ray::Ray;
use vec::Vec3;

pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_range: Range<f32>) -> Option<HitRecord>;
}

impl<'a, T: Hittable> Hittable for &'a T {
    fn hit(&self, r: &Ray, t_range: Range<f32>) -> Option<HitRecord> {
        <T as Hittable>::hit(self, r, t_range)
    }
}

impl<'a, T: Hittable> Hittable for &'a mut T {
    fn hit(&self, r: &Ray, t_range: Range<f32>) -> Option<HitRecord> {
        <T as Hittable>::hit(self, r, t_range)
    }
}

impl<T: Hittable> Hittable for Vec<T> {
    fn hit(&self, r: &Ray, t_range: Range<f32>) -> Option<HitRecord> {
        let mut closest_so_far = t_range.end;
        let mut hit_rec = None;
        for h in self {
            if let Some(rec) = h.hit(r, t_range.start..closest_so_far) {
                closest_so_far = rec.t;
                hit_rec = Some(rec);
            }
        }
        hit_rec
    }
}
