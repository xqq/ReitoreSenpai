use crate::vec3::Vec3;
use crate::hitable_list::HitableList;
use rand::{thread_rng, Rng};
use crate::sphere::Sphere;
use crate::lambertian::Lambertian;
use crate::dielectric::Dielectric;
use crate::metal::Metal;
use crate::scene::Scene;

pub struct RandomScene {

}

impl Scene for RandomScene {

    fn generate(&self) -> HitableList {
        let n: usize = 500;
        let mut list = HitableList::default();
        list.reserve(n + 1);
        list.push(Sphere::new(
            Vec3(0.0, -1000.0, 0.0),
            1000.0,
            Lambertian::new(
                Vec3(0.5, 0.5, 0.5)
            )
        ));

        let mut rng = thread_rng();

        for a in -11..11 {
            for b in -11..11 {
                let choose_mat: f32 = rng.gen();
                let center = Vec3(
                    a as f32 + 0.9 * rng.gen::<f32>(),
                    0.2,
                    b as f32 + 0.9 * rng.gen::<f32>()
                );

                if (center - Vec3(4.0, 0.2, 0.0)).length() > 0.9 {
                    if choose_mat < 0.8 {  // diffuse
                        list.push(Sphere::new(
                            center,
                            0.2,
                            Lambertian::new(
                                Vec3(
                                    rng.gen::<f32>() * rng.gen::<f32>(),
                                    rng.gen::<f32>() * rng.gen::<f32>(),
                                    rng.gen::<f32>() * rng.gen::<f32>()
                                )
                            )
                        ));
                    } else if choose_mat < 0.95 {  // metal
                        list.push(Sphere::new(
                            center,
                            0.2,
                            Metal::new(
                                Vec3(
                                    0.5 * (1.0 + rng.gen::<f32>()),
                                    0.5 * (1.0 + rng.gen::<f32>()),
                                    0.5 * (1.0 + rng.gen::<f32>())
                                ),
                                0.5 * rng.gen::<f32>()
                            )
                        ));
                    } else {  // glass
                        list.push(Sphere::new(
                            center,
                            0.2,
                            Dielectric::new(
                                1.5
                            )
                        ));
                    }
                }
            }
        }

        list.push(Sphere::new(
            Vec3(0.0, 1.0, 0.0),
            1.0,
            Dielectric::new(
                1.5
            )
        ));
        list.push(Sphere::new(
            Vec3(-4.0, 1.0, 0.0),
            1.0,
            Lambertian::new(
                Vec3(0.4, 0.2, 0.1)
            )
        ));
        list.push(Sphere::new(
            Vec3(4.0, 1.0, 0.0),
            1.0,
            Metal::new(
                Vec3(0.7, 0.6, 0.5),
                0.0
            )
        ));

        list
    }

}
