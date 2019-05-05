use std::f32;
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::camera::Camera;

pub struct StandardCamera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl StandardCamera {

    pub fn new(look_from: Vec3, look_at: Vec3, vup: Vec3, vfov: f32, aspect: f32) -> StandardCamera {
        let theta = vfov * f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let w = (look_from - look_at).to_unit_vector();
        let u = Vec3::cross(&vup, &w).to_unit_vector();
        let v = Vec3::cross(&w, &u).to_unit_vector();

        StandardCamera {
            origin: look_from,
            lower_left_corner: look_from - half_width * u - half_height * v - w,
            horizontal: 2.0 * half_width * u,
            vertical: 2.0 * half_height * v
        }
    }

}

impl Camera for StandardCamera {

    fn get_ray(&self, s: f32, t: f32) -> Ray {
        Ray::from_a_b(
            self.origin,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin
        )
    }

}
