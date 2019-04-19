use rand::Rng;
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::camera::Camera;
use crate::hitable::*;
use crate::hitable_list::HitableList;

fn random_in_unit_sphere() -> Vec3 {
    let mut rnd = rand::thread_rng();
    let mut p;

    loop {
        p = 2.0 * Vec3(rnd.gen_range(0.0, 1.0), rnd.gen_range(0.0, 1.0), rnd.gen_range(0.0, 1.0));
        p = p - Vec3(1.0, 1.0, 1.0);

        if p.squared_length() < 1.0 {
            break;
        }
    }

    p
}

fn color(r: &Ray, hitable: &dyn Hitable) -> Vec3 {
    if let (true, Some(record)) = hitable.hit(r, 0.001, std::f32::MAX) {
        let target = record.hit_point + record.normal + random_in_unit_sphere();
        0.5 * color(&Ray::from_a_b(record.hit_point, target - record.hit_point), hitable)
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
    world.push(Box::new(Sphere::new(Vec3(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::new(Vec3(0.0, -100.5, -1.0), 100.0)));

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

            col = col / (ns as f32);
            col = Vec3(col.r().sqrt(), col.g().sqrt(), col.b().sqrt());

            let ir = (255.99 * col.0) as i32;
            let ig = (255.99 * col.1) as i32;
            let ib = (255.99 * col.2) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }

}
