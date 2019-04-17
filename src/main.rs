#![allow(non_snake_case)]

mod window;
mod vec3;
mod ray;

use vec3::Vec3;
use ray::Ray;

fn color(r: &Ray) -> Vec3 {
    let unit_direction = r.direction().to_unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3(1.0, 1.0, 1.0) + t * Vec3(0.5, 0.7, 1.0)
}

fn main() {
    let nx = 400;
    let ny = 200;

    println!("P3\n{} {}\n255", nx, ny);

    let lower_left_corner = Vec3(-2.0, -1.0, -1.0);
    let horizontal = Vec3(4.0, 0.0, 0.0);
    let vertical = Vec3(0.0, 2.0, 0.0);
    let origin = Vec3(0.0, 0.0, 0.0);

    for j in (0..=(ny - 1)).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;

            let r = Ray::from_a_b(origin, lower_left_corner + u * horizontal + v * vertical);
            let col = color(&r);

            let ir = (255.99 * col.0) as i32;
            let ig = (255.99 * col.1) as i32;
            let ib = (255.99 * col.2) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }

}
