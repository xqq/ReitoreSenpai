use std::ops::Deref;
use std::ops::DerefMut;
use crate::ray::Ray;
use crate::hitable::*;

#[derive(Default)]
pub struct HitableList {
    list: Vec<Box<dyn Hitable>>
}

impl Hitable for HitableList {

    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> (bool, Option<HitRecord>) {
        let mut hit_record = HitRecord::default();

        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.list {
            if let (true, Some(record)) = object.hit(ray, t_min, closest_so_far) {
                hit_anything = true;
                closest_so_far = record.t;
                hit_record = record;
            }
        }

        (hit_anything, Some(hit_record))
    }

}

impl Deref for HitableList {
    type Target = Vec<Box<dyn Hitable>>;

    #[inline(always)]
    fn deref(&self) -> &Vec<Box<dyn Hitable>> {
        &self.list
    }
}

impl DerefMut for HitableList {

    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Vec<Box<dyn Hitable>> {
        &mut self.list
    }

}
