use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3
}

impl Camera {

    pub fn new() -> Camera {
        Camera {
            origin: Vec3(0.0, 0.0, 0.0),
            lower_left_corner: Vec3(-2.0, -1.0, -1.0),
            horizontal: Vec3(4.0, 0.0, 0.0),
            vertical: Vec3(0.0, 2.0, 0.0)
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::from_a_b(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical
        )
    }

}
