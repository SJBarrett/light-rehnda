use crate::core::color::ColorRgbF;
use crate::core::rehnda_math::Point3f;
use crate::texture::{Texture, Uv};

pub struct SolidTexture {
    pub albedo: ColorRgbF,
}

impl SolidTexture {
    pub fn new(r: f32, g: f32, b: f32) -> SolidTexture {
        SolidTexture {
            albedo: ColorRgbF::new(r, g, b),
        }
    }
}

impl Texture for SolidTexture {
    fn sample(&self, _uv: &Uv, _point: &Point3f) -> ColorRgbF {
        self.albedo
    }
}