#![allow(unused)]

use rand::Rng;
use rand::SeedableRng;
use rand_xorshift::XorShiftRng;
use png::HasParameters;
use std::cell::UnsafeCell;
use std::slice;
use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use crate::vec3::Vec3;

thread_local!(
    static THREAD_XORSHIFT_RNG: UnsafeCell<XorShiftRng> = {
        let mut rng = XorShiftRng::from_rng(rand::thread_rng()).unwrap();

        UnsafeCell::new(rng)
    }
);

pub fn fast_thread_rng() -> &'static mut XorShiftRng {
    let rng = THREAD_XORSHIFT_RNG.with(|t| {
        unsafe {
            &mut *t.get()
        }
    });
    rng
}

pub fn random_in_unit_sphere() -> Vec3 {
    let rng = fast_thread_rng();
    let mut p;

    loop {
        p = 2.0 * Vec3(rng.gen(), rng.gen(), rng.gen());
        p -= Vec3(1.0, 1.0, 1.0);

        if p.squared_length() < 1.0 {
            break;
        }
    }

    p
}

pub fn random_in_unit_disk() -> Vec3 {
    let rng = fast_thread_rng();
    let mut p;

    loop {
        p = 2.0 * Vec3(rng.gen(), rng.gen(), 0.0) - Vec3(1.0, 1.0, 0.0);

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

pub fn nearly_equal(a: f32, b: f32) -> bool {
    const EPSILON: f32 = 1e-6;
    (a - EPSILON <= b) && (b <= a + EPSILON)
}

pub fn split_chunk_from_mut<T: Sized>(slice: &[T], from: usize, length: usize) -> &mut [T] {
    let ptr = slice.as_ptr() as *mut T;

    unsafe {
        assert!(from + length <= slice.len());

        slice::from_raw_parts_mut(ptr.add(from), length)
    }
}

pub fn as_u32_slice_mut(slice: &mut [u8]) -> &mut [u32] {
    unsafe {
        std::slice::from_raw_parts_mut(
            slice.as_mut_ptr() as *mut u32,
            slice.len() / 4
        )
    }
}

pub fn write_png_file(file_path: String, width: u32, height: u32, buffer: &[u8]) {
    let path = Path::new(&file_path);
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width, height);
    encoder
        .set(png::ColorType::RGBA)
        .set(png::BitDepth::Eight)
        .set(png::Compression::Best);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&buffer).unwrap();
}
