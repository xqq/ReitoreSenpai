#![allow(dead_code)]

use crate::vec3::Vec3;

#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct Ray {
    a: Vec3,
    b: Vec3
}

impl Ray {

    pub fn from_a_b(a: Vec3, b: Vec3) -> Ray {
        Ray {
            a,
            b
        }
    }

    pub fn origin(&self) -> Vec3 {
        self.a
    }

    pub fn direction(&self) -> Vec3 {
        self.b
    }

    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.a + t * self.b
    }

}
