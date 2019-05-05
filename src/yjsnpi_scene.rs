use crate::vec3::Vec3;
use crate::hitable_list::HitableList;
use crate::sphere::Sphere;
use crate::surface::Surface;
use crate::lambertian::Lambertian;
use crate::dielectric::Dielectric;
use crate::metal::Metal;
use crate::scene::Scene;

pub struct YjsnpiScene {}

impl Scene for YjsnpiScene {

    fn generate(&self) -> HitableList {
        let mut world = HitableList::default();

        // x-y plane
        world.push(Surface::new(
            0.0,
            0.0,
            1.0,
            0.0,
            -0.5..=15.0,
            -0.5..=15.0,
            -0.5..=15.0,
            Lambertian::new(Vec3(0.615, 0.555, 0.504))
        ));

        // y-z plane
        world.push(Surface::new(
            1.0,
            0.0,
            0.0,
            0.0,
            -0.5..=15.0,
            -0.5..=15.0,
            -0.5..=15.0,
            Lambertian::new(Vec3(0.346, 0.389, 0.365))
        ));

        // x-z plane
        world.push(Surface::new(
            0.0,
            1.0,
            0.0,
            0.0,
            -0.5..=15.0,
            -0.5..=15.0,
            -0.5..=15.0,
            Metal::new(Vec3(0.061, 0.04, 0.063), 0.0)
        ));

        // senpai ball
        world.push(Sphere::new(
            Vec3(4.0, 3.0, 4.0),
            3.0,
            Lambertian::new(
                Vec3(0.398, 0.200, 0.173),
            )
        ));

        world
    }

}
