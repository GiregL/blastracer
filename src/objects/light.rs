use crate::maths::vector3f::Vector3f;
use crate::objects::color4u8::Color4u8;

pub struct DirectionalLight {
    pub direction: Vector3f,
    pub color: Color4u8,
    pub intensity: f32
}