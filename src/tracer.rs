use rand::Rng;
use scoped_threadpool::Pool;
use std::time::{Duration, Instant};
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hitable::*;
use crate::scene::Scene;
use crate::playground_scene::PlaygroundScene;
use crate::random_scene::RandomScene;
use crate::camera::Camera;
use crate::defocus_camera::DefocusCamera;
use crate::standard_camera::StandardCamera;
use crate::utils::as_u32_slice_mut;

fn color<T: Hitable>(r: &Ray, world: &T, depth: i32) -> Vec3 {
    if let (true, Some(record)) = world.hit(r, 0.001, std::f32::MAX) {
        if depth < 50 {
            if let (true, Some(scatter)) = record.material.scatter(r, &record) {
                scatter.attenuation * color(&scatter.scattered, world, depth + 1)
            } else {
                Vec3::default()
            }
        } else {
            Vec3::default()
        }
    } else {
        let unit_direction = r.direction().to_unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Vec3(1.0, 1.0, 1.0) + t * Vec3(0.5, 0.7, 1.0)
    }
}

pub fn trace(buffer: &mut [u8], pitch: usize, width: u32, height: u32) -> (u32, Duration) {
    let nx = width;
    let ny = height;
    let ns = 100;
    let threads: u32 = 8;

    let look_from = Vec3(13.0, 2.0, 3.0);
    let look_at = Vec3(0.0, 0.0, 0.0);
    let dist_to_focus: f32 = 10.0;
    let aperture: f32 = 0.1;

    let camera = DefocusCamera::new(
        look_from,
        look_at,
        Vec3(0.0, 1.0, 0.0),
        20.0,
        width as f32 / height as f32,
        aperture,
        dist_to_focus
    );

    let scene = RandomScene {};
    let world = scene.generate();

    let mut pool = Pool::new(threads);

    let now = Instant::now();

    pool.scoped(|scope| {
        let camera = &camera;
        let world = &world;
        let mut chunks = buffer.chunks_mut(pitch);

        for j in (0..ny).rev() {
            let y = j;
            let chunk = as_u32_slice_mut(chunks.next().unwrap());

            scope.execute(move || {
                let mut rng = rand::thread_rng();

                for x in 0..nx {
                    let mut col = Vec3(0.0, 0.0, 0.0);

                    for _s in 0..ns {
                        let u = (x as f32 + rng.gen::<f32>()) / nx as f32;
                        let v = (y as f32 + rng.gen::<f32>()) / ny as f32;
                        let r = camera.get_ray(u, v);
                        let _p = r.point_at_parameter(2.0);
                        col += color(&r, world, 0);
                    }

                    col /= ns as f32;
                    col = Vec3(col.r().sqrt(), col.g().sqrt(), col.b().sqrt());

                    let ir = (255.99 * col.0) as u32;
                    let ig = (255.99 * col.1) as u32;
                    let ib = (255.99 * col.2) as u32;

                    let value: u32 = ir | ig << 8 | ib << 16 | 255 << 24;
                    chunk[x as usize] = value;
                }
            });
        }
    });

    (threads, now.elapsed())
}
