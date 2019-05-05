#![allow(non_snake_case)]
#![feature(maybe_uninit)]
#![feature(duration_float)]
#![feature(range_contains)]
#![feature(nll)]

extern crate rand;
extern crate sdl2;
extern crate scoped_threadpool;

mod window;
mod vec3;
mod ray;
mod hitable;
mod sphere;
mod surface;
mod tracer;
mod hitable_list;
mod camera;
mod standard_camera;
mod defocus_camera;
mod material;
mod metal;
mod lambertian;
mod dielectric;
mod utils;
mod scene;
mod random_scene;
mod playground_scene;
mod yjsnpi_scene;

fn main() {
    window::window_main_loop()
}
