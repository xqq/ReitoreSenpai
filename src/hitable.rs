use crate::vec3::Vec3;
use crate::ray::Ray;

#[derive(Default)]
pub struct HitRecord {
    pub t: f32,
    pub hit_point: Vec3,
    pub normal: Vec3
}

pub trait Hitable {

    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> (bool, Option<HitRecord>);

}
