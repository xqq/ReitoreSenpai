use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::material::Material;

pub struct HitRecord<'a> {
    pub t: f32,
    pub hit_point: Vec3,
    pub normal: Vec3,
    pub material: &'a dyn Material
}

pub trait Hitable : Send + Sync {

    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> (bool, Option<HitRecord>);

}
