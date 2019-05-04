use crate::hitable_list::HitableList;

pub trait Scene {

    fn generate(&self) -> HitableList;

}
