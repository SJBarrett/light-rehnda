use crate::core::color::ColorRgbF;
use crate::core::rehnda_math::Point3f;
use crate::texture::{Texture, Uv};
use crate::util::perlin::PERLIN;

pub struct NoiseTexture {
    pub scale: f32,
}

impl Texture for NoiseTexture {
    fn sample(&self, uv: &Uv, point: &Point3f) -> ColorRgbF {
        ColorRgbF::ONE * 0.5 * (1.0 + (self.scale * point.z + 10.0 * PERLIN.turbulence(point, 7)).sin())
    }
}