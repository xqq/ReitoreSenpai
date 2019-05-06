use rand::Rng;
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hitable::HitRecord;
use crate::material::Material;
use crate::material::ScatterResult;
use crate::utils::*;

pub struct Dielectric {
    ref_idx: f32
}

impl Dielectric {

    pub fn new(ref_idx: f32) -> Box<dyn Material> {
        Box::new(Dielectric {
            ref_idx
        })
    }

}

impl Material for Dielectric {

    fn scatter(&self, r_in: &Ray, record: &HitRecord) -> (bool, Option<ScatterResult>) {
        let mut refracted = Vec3::default();
        let outward_normal: Vec3;
        let ni_over_nt;
        let reflect_prob: f32;
        let cosine: f32;

        if Vec3::dot(&r_in.direction(), &record.normal) > 0.0 {
            outward_normal = -record.normal;
            ni_over_nt = self.ref_idx;
            cosine = self.ref_idx * Vec3::dot(&r_in.direction(), &record.normal) / r_in.direction().length();
        } else {
            outward_normal = record.normal;
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = -Vec3::dot(&r_in.direction(), &record.normal) / r_in.direction().length();
        }

        if let Some(refract) = refract(&r_in.direction(), &outward_normal, ni_over_nt) {
            refracted = refract;
            reflect_prob = schlick(cosine, self.ref_idx);
        } else {
            reflect_prob = 1.0;
        }

        let rng = fast_thread_rng();

        if rng.gen::<f32>() < reflect_prob {
            let reflected = reflect(&r_in.direction(), &record.normal);
            (true, Some(ScatterResult {
                attenuation: Vec3(1.0, 1.0, 1.0),
                scattered: Ray::from_a_b(record.hit_point, reflected)
            }))
        } else {
            (true, Some(ScatterResult {
                attenuation: Vec3(1.0, 1.0, 1.0),
                scattered: Ray::from_a_b(record.hit_point, refracted)
            }))
        }
    }

}
