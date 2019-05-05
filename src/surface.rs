use std::ops::RangeInclusive;
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hitable::*;
use crate::material::Material;
use crate::utils::nearly_equal;

pub struct Surface {
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    normal: Vec3,
    xrange: RangeInclusive<f32>,
    yrange: RangeInclusive<f32>,
    zrange: RangeInclusive<f32>,
    material: Box<dyn Material>
}

impl Surface {

    // ax + by + c - d = 0
    pub fn new(a: f32, b: f32, c: f32, d: f32,
               xrange: RangeInclusive<f32>, yrange: RangeInclusive<f32>, zrange: RangeInclusive<f32>,
               material: Box<dyn Material>) -> Box<dyn Hitable> {
        Box::new(Surface {
            a,
            b,
            c,
            d,
            normal: Vec3(a, b, c).to_unit_vector(),
            xrange,
            yrange,
            zrange,
            material
        })
    }

}

impl Hitable for Surface {

    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> (bool, Option<HitRecord>) {
        let normal_dot_direction = Vec3::dot(&self.normal, &ray.direction());
        if nearly_equal(normal_dot_direction, 0.0) {
            return (false, None);
        }

        let normal_dot_origin = Vec3::dot(&self.normal, &ray.origin());

        let t = -(self.d + normal_dot_origin) / (normal_dot_direction);
        if t <= t_min || t >= t_max {
            return (false, None);
        }

        let hit_point = ray.point_at_parameter(t);

        if self.xrange.contains(&hit_point.x()) &&
            self.yrange.contains(&hit_point.y()) &&
            self.zrange.contains(&hit_point.z()) {

            let normal = if normal_dot_direction < 0.0 {
                // cos(theta) < 0, 90 < theta < 180
                self.normal
            } else {
                // cos(theta) > 0, 0 < theta < 90
                -self.normal
            };

            (true, Some(HitRecord {
                t,
                hit_point,
                normal,
                material: self.material.as_ref()
            }))
        } else {
            (false, None)
        }
    }

}
