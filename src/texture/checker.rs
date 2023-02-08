use std::sync::Arc;
use crate::core::color::ColorRgbF;
use crate::core::rehnda_math::Point3f;
use crate::texture::{Texture, Uv};

pub struct CheckerTexture {
    even_texture: Arc<dyn Texture>,
    odd_texture: Arc<dyn Texture>,
    scale: f32,
}

impl CheckerTexture {
    pub fn new(scale: f32, even_texture: Arc<dyn Texture>, odd_texture: Arc<dyn Texture>) -> CheckerTexture {
        CheckerTexture {
            even_texture,
            odd_texture,
            scale,
        }
    }
}

impl Texture for CheckerTexture {
    fn sample(&self, uv: &Uv, point: &Point3f) -> ColorRgbF {
        let sines = (self.scale * point.x).sin() * (self.scale * point.y).sin() * (self.scale * point.z).sin();

        if sines < 0.0 {
            self.odd_texture.sample(uv, point)
        } else {
            self.even_texture.sample(uv, point)
        }
    }
}