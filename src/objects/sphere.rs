use crate::maths::vector3f::Vector3f;
use crate::objects::color4u8::Color4u8;
use crate::objects::intersectable::Intersectable;
use crate::objects::ray::Ray;

pub struct Sphere {
    pub center: Vector3f,
    pub radius: f32,
    pub color: Color4u8,
    pub albedo: f32
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<f32> {
        let l = ray.origin - self.center;
        let adj = Vector3f::dot(&l, &ray.direction);
        let d2 = Vector3f::dot(&l, &l) - (adj * adj);
        let radius2 = self.radius * self.radius;
        if d2 > radius2 {
            return None;
        }

        let thc = (radius2 - d2).sqrt();
        let t0 = adj - thc;
        let t1 = adj + thc;

        if t0 < 0.0 && t1 < 0.0 {
            return None;
        }

        let distance = if t0 < t1 {
            t0
        } else {
            t1
        };
        Some(distance)
    }

    fn get_albedo(&self) -> f32 {
        self.albedo
    }

    fn get_color(&self) -> &Color4u8 {
        return &self.color;
    }

    fn surface_normal(&self, hit_point: &Vector3f) -> Vector3f {
        (*hit_point - self.center).normalize()
    }
}