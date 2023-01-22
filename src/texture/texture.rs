use crate::core::color::ColorRgbF;
use crate::core::rehnda_math::Point3f;

#[derive(Debug, Copy, Clone)]
pub struct Uv(pub f32, pub f32);

pub trait Texture {
    fn sample(&self, uv: &Uv, point: &Point3f) -> ColorRgbF;
}