use crate::core::color::ColorRgbF;
use crate::core::rehnda_math::Point3f;

#[derive(Debug, Copy, Clone)]
pub struct Uv {
    pub u: f32,
    pub v: f32,
}

impl Uv {
    pub fn new(u: f32, v: f32) -> Uv {
        Uv {
            u,
            v,
        }
    }
    
    pub fn to_clamped_uv(&self) -> Uv {
        Uv {
            u: self.u.clamp(0.0, 1.0),
            v: self.v.clamp(0.0, 1.0),
        }
    }
}

pub trait Texture: Send + Sync {
    fn sample(&self, uv: &Uv, point: &Point3f) -> ColorRgbF;
}