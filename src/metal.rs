use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hitable::HitRecord;
use crate::material::Material;
use crate::material::ScatterResult;
use crate::utils::*;

pub struct Metal {
    albedo: Vec3,
    fuzz: f32
}

impl Metal {

    pub fn new(albedo: Vec3, f: f32) -> Box<dyn Material> {
        let fuzz = if f < 1.0 {
            f
        } else {
            1.0
        };

        Box::new(Metal {
            albedo,
            fuzz
        })
    }

}

impl Material for Metal {

    fn scatter(&self, r_in: &Ray, record: &HitRecord) -> (bool, Option<ScatterResult>) {
        let reflected = reflect(&r_in.direction().to_unit_vector(), &record.normal);
        let scattered = Ray::from_a_b(record.hit_point, reflected + self.fuzz * random_in_unit_sphere());
        let attenuation = self.albedo;

        let has_reflect = Vec3::dot(&scattered.direction(), &record.normal) > 0.0;

        (has_reflect, Some(ScatterResult { attenuation, scattered }))
    }

}