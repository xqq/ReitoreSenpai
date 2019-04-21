use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hitable::HitRecord;

pub struct ScatterResult {
    pub attenuation: Vec3,
    pub scattered: Ray
}

pub trait Material : Send + Sync {

    fn scatter(&self, r_in: &Ray, record: &HitRecord) -> (bool, Option<ScatterResult>);

}
