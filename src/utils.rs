#![allow(unused)]

use rand::Rng;
use std::slice;
use crate::vec3::Vec3;

pub fn random_in_unit_sphere() -> Vec3 {
    let mut rnd = rand::thread_rng();
    let mut p;

    loop {
        p = 2.0 * Vec3(rnd.gen_range(0.0, 1.0), rnd.gen_range(0.0, 1.0), rnd.gen_range(0.0, 1.0));
        p -= Vec3(1.0, 1.0, 1.0);

        if p.squared_length() < 1.0 {
            break;
        }
    }

    p
}

pub fn random_in_unit_disk() -> Vec3 {
    let mut rnd = rand::thread_rng();
    let mut p;

    loop {
        p = 2.0 * Vec3(rnd.gen_range(0.0, 1.0), rnd.gen_range(0.0, 1.0), 0.0) - Vec3(1.0, 1.0, 0.0);

        if Vec3::dot(&p, &p) < 1.0 {
            break;
        }
    }

    p
}

pub fn reflect(v: &Vec3, normal: &Vec3) -> Vec3 {
    *v - 2.0 * Vec3::dot(v, normal) * normal
}

pub fn refract(v: &Vec3, normal: &Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = v.to_unit_vector();
    let dt = Vec3::dot(&uv, normal);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);

    if discriminant > 0.0 {
        Some(ni_over_nt * (uv - *normal * dt) - *normal * discriminant.sqrt())
    } else {
        None
    }
}

pub fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

pub fn split_chunk_from_mut<T: Sized>(slice: &[T], from: usize, length: usize) -> &mut [T] {
    let ptr = slice.as_ptr() as *mut T;

    unsafe {
        assert!(from + length <= slice.len());

        slice::from_raw_parts_mut(ptr.add(from), length)
    }
}
