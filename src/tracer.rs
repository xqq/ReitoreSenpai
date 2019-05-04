use rand::Rng;
use scoped_threadpool::Pool;
use std::time::{Duration, Instant};
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::camera::Camera;
use crate::hitable::*;
use crate::hitable_list::HitableList;
use crate::lambertian::Lambertian;
use crate::metal::Metal;
use crate::dielectric::Dielectric;

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

    let look_from = Vec3(3.0, 3.0, 2.0);
    let look_at = Vec3(0.0, 0.0, -1.0);
    let dist_to_focus = (look_from - look_at).length();
    let aperture: f32 = 2.0;

    let camera = Camera::new(
        look_from,
        look_at,
        Vec3(0.0, 1.0, 0.0),
        20.0,
        width as f32 / height as f32,
        aperture,
        dist_to_focus
    );

    let mut world = HitableList::default();
    world.push(Box::new(Sphere::new(
        Vec3(0.0, 0.0, -1.0),
        0.5,
        Box::new(Lambertian { albedo: Vec3(0.1, 0.2, 0.5) })
    )));
    world.push(Box::new(Sphere::new(
        Vec3(0.0, -100.5, -1.0),
        100.0,
        Box::new( Lambertian { albedo: Vec3(0.8, 0.8, 0.0) })
    )));
    world.push(Box::new(Sphere::new(
        Vec3(1.0, 0.0, -1.0),
        0.5,
        Box::new( Metal::new(Vec3(0.8, 0.6, 0.2), 1.0))
    )));
    world.push(Box::new(Sphere::new(
        Vec3(-1.0, 0.0, -1.0),
        0.5,
        Box::new( Dielectric { ref_idx: 1.5 })
    )));
    world.push(Box::new(Sphere::new(
        Vec3(-1.0, 0.0, -1.0),
        -0.45,
        Box::new( Dielectric { ref_idx: 1.5 })
    )));

    let mut pool = Pool::new(threads);

    let now = Instant::now();

    pool.scoped(|scope| {
        let camera = &camera;
        let world = &world;
        let mut chunks = buffer.chunks_mut(pitch);

        for j in 0..ny {
            let y = j;
            let chunk = chunks.next().unwrap();

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

                    let ir = (255.99 * col.0) as u8;
                    let ig = (255.99 * col.1) as u8;
                    let ib = (255.99 * col.2) as u8;

                    let offset = x as usize * 3;
                    chunk[offset] = ir;
                    chunk[offset + 1] = ig;
                    chunk[offset + 2] = ib;
                }
            });
        }
    });

    (threads, now.elapsed())
}
