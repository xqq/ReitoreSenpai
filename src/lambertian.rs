use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hitable::HitRecord;
use crate::material::*;
use crate::utils::*;

pub struct Lambertian {
    pub albedo: Vec3
}

impl Material for Lambertian {

    fn scatter(&self, _r_in: &Ray, record: &HitRecord) -> (bool, Option<ScatterResult>) {
        let target = record.hit_point + record.normal + random_in_unit_sphere();

        (true, Some(ScatterResult {
           attenuation: self.albedo,
            scattered: Ray::from_a_b(record.hit_point, target - record.hit_point)
        }))
    }

}
