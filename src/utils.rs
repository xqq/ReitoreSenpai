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

pub fn reflect(v: &Vec3, normal: &Vec3) -> Vec3 {
    *v - 2.0 * Vec3::dot(v, normal) * normal
}

pub fn split_chunk_from_mut<T: Sized>(slice: &[T], from: usize, length: usize) -> &mut [T] {
    let ptr = slice.as_ptr() as *mut T;

    unsafe {
        assert!(from + length <= slice.len());

        slice::from_raw_parts_mut(ptr.add(from), length)
    }
}
