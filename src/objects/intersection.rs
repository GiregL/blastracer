use crate::objects::intersectable::Intersectable;

pub struct Intersection<'a> {
    pub distance: f32,
    pub object: &'a Box<Intersectable>
}