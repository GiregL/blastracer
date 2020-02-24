use crate::objects::intersectable::Intersectable;
use crate::objects::intersection::Intersection;
use crate::objects::ray::Ray;
use crate::objects::light::DirectionalLight;

pub struct Scene {

    pub width: usize,
    pub height: usize,
    pub fov: f32,
    pub shapes: Vec<Box<Intersectable>>,
    pub light: DirectionalLight

}

impl Scene {
    pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
        self.shapes
            .iter()
            .filter_map(|shape| {
                shape.intersect(&ray).map(
                    |distance| {
                        Intersection {distance, object: shape}
                    })
            })
            .min_by(|i1, i2| {
                i1.distance.partial_cmp(&i2.distance).unwrap()
            })
    }
}