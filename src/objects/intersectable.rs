use super::ray::Ray;
use crate::objects::color4u8::Color4u8;
use crate::maths::vector3f::Vector3f;

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<f32>;
    fn get_albedo(&self) -> f32;
    fn get_color(&self) -> &Color4u8;
    fn surface_normal(&self, hit_point: &Vector3f) -> Vector3f;
}