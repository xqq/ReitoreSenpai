use crate::vec3::Vec3;
use crate::hitable_list::HitableList;
use crate::sphere::Sphere;
use crate::lambertian::Lambertian;
use crate::dielectric::Dielectric;
use crate::metal::Metal;
use crate::scene::Scene;

pub struct PlaygroundScene {}

impl Scene for PlaygroundScene {

    fn generate(&self) -> HitableList {
        let mut world = HitableList::default();

        world.push(Sphere::new(
            Vec3(0.0, 0.0, -1.0),
            0.5,
            Box::new(Lambertian { albedo: Vec3(0.1, 0.2, 0.5) })
        ));
        world.push(Sphere::new(
            Vec3(0.0, -100.5, -1.0),
            100.0,
            Box::new( Lambertian { albedo: Vec3(0.8, 0.8, 0.0) })
        ));
        world.push(Sphere::new(
            Vec3(1.0, 0.0, -1.0),
            0.5,
            Box::new( Metal::new(Vec3(0.8, 0.6, 0.2), 1.0))
        ));
        world.push(Sphere::new(
            Vec3(-1.0, 0.0, -1.0),
            0.5,
            Box::new( Dielectric { ref_idx: 1.5 })
        ));
        world.push(Sphere::new(
            Vec3(-1.0, 0.0, -1.0),
            -0.45,
            Box::new( Dielectric { ref_idx: 1.5 })
        ));

        world
    }

}
