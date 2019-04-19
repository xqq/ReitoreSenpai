use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hitable::*;

pub struct Sphere {
    center: Vec3,
    radius: f32
}

impl Sphere {

    pub fn new(center: Vec3, radius: f32) -> Sphere {
        Sphere {
            center,
            radius
        }
    }

}

impl Hitable for Sphere {

    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> (bool, Option<HitRecord>) {
        let oc = ray.origin() - self.center;

        let a = Vec3::dot(&ray.direction(), &ray.direction());
        let b = Vec3::dot(&oc, &ray.direction());
        let c = Vec3::dot(&oc, &oc) - self.radius * self.radius;

        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let temp = (-b - discriminant.sqrt()) / a;
            if temp > t_min && temp < t_max {
                let hit_point = ray.point_at_parameter(temp);
                return (true, Some(HitRecord {
                    t: temp,
                    hit_point,
                    normal: (hit_point - self.center) / self.radius
                }))
            }

            let temp = (-b + discriminant.sqrt()) / a;
            if temp > t_min && temp < t_max {
                let hit_point = ray.point_at_parameter(temp);
                return (true, Some(HitRecord {
                    t: temp,
                    hit_point,
                    normal: (hit_point - self.center) / self.radius
                }))
            }
        }

        (false, None)
    }

}
