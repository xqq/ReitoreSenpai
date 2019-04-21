use rand::Rng;
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::camera::Camera;
use crate::hitable::*;
use crate::hitable_list::HitableList;
use crate::lambertian::Lambertian;
use crate::metal::Metal;
use crate::utils::*;

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

pub fn trace() {
    let nx = 800;
    let ny = 400;
    let ns = 100;

    println!("P3\n{} {}\n255", nx, ny);

    let mut rng = rand::thread_rng();
    let camera = Camera::new();
    let mut world = HitableList::default();
    for j in (0..=(ny - 1)).rev() {
        for i in 0..nx {
            let mut col = Vec3(0.0, 0.0, 0.0);

            for _s in 0..ns {
                let u = (i as f32 + rng.gen_range(0.0, 1.0)) / nx as f32;
                let v = (j as f32 + rng.gen_range(0.0, 1.0)) / ny as f32;
                let r = camera.get_ray(u, v);
                let _p = r.point_at_parameter(2.0);
                col = col + color(&r, &world);
            }
    world.push(Box::new(Sphere::new(
        Vec3(0.0, 0.0, -1.0),
        0.5,
        Box::new(Lambertian { albedo: Vec3(0.8, 0.3, 0.3) })
    )));
    world.push(Box::new(Sphere::new(
        Vec3(0.0, -100.5, -1.0),
        100.0,
        Box::new( Lambertian { albedo: Vec3(0.8, 0.8, 0.0) })
    )));
    world.push(Box::new(Sphere::new(
        Vec3(1.0, 0.0, -1.0),
        0.5,
        Box::new( Metal::new(Vec3(0.8, 0.6, 0.2), 0.2))
    )));
    world.push(Box::new(Sphere::new(
        Vec3(-1.0, 0.0, -1.0),
        0.5,
        Box::new( Metal::new(Vec3(0.8, 0.8, 0.8), 0.4))
    )));

            col = col / (ns as f32);
            col = Vec3(col.r().sqrt(), col.g().sqrt(), col.b().sqrt());

            let ir = (255.99 * col.0) as i32;
            let ig = (255.99 * col.1) as i32;
            let ib = (255.99 * col.2) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }

}
