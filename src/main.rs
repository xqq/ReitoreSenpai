#![allow(non_snake_case)]

extern crate rand;

mod window;
mod vec3;
mod ray;
mod hitable;
mod sphere;
mod tracer;
mod hitable_list;
mod camera;

fn main() {
    tracer::trace()
}
