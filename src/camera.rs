use crate::ray::Ray;

pub trait Camera {

    fn get_ray(&self, s: f32, t: f32) -> Ray;

}
