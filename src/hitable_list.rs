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
        let hit_record = std::mem::MaybeUninit::<HitRecord>::uninit();
        let mut hit_record = unsafe { hit_record.assume_init() };

        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.list {
            if let (true, Some(record)) = object.hit(ray, t_min, closest_so_far) {
                hit_anything = true;
                closest_so_far = record.t;
                hit_record = record;
            }
        }

        if hit_anything {
            (true, Some(hit_record))
        } else {
            (false, None)
        }
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
