use crate::maths::vector3f::Vector3f;
use crate::objects::color4u8::Color4u8;
use crate::objects::intersectable::Intersectable;
use crate::objects::ray::Ray;

pub struct Plane {
    pub p0: Vector3f,
    pub normal: Vector3f,
    pub color: Color4u8,
    pub albedo: f32
}

impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<f32> {
        let normal = &self.normal;
        let denom = Vector3f::dot(normal, &ray.direction);
        if denom > 1e-6f32 {
            let v = self.p0 - ray.origin;
            let distance = Vector3f::dot(&v, &normal) / denom;
            if distance >= 0.0 {
                return Some(distance)
            }
        }
        None
    }

    fn get_albedo(&self) -> f32 {
        self.albedo
    }

    fn get_color(&self) -> &Color4u8 {
        &self.color
    }

    fn surface_normal(&self, _hit_point: &Vector3f) -> Vector3f {
        self.normal * (-1.0)
    }
}