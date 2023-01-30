use std::sync::Arc;
use crate::core::color::ColorRgbF;
use crate::core::rehnda_math::Point3f;
use crate::texture::{Texture, Uv};

pub struct CheckerTexture {
    even_texture: Arc<dyn Texture>,
    odd_texture: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(even_texture: Arc<dyn Texture>, odd_texture: Arc<dyn Texture>) -> CheckerTexture {
        CheckerTexture {
            even_texture,
            odd_texture,
        }
    }
}

impl Texture for CheckerTexture {
    fn sample(&self, uv: &Uv, point: &Point3f) -> ColorRgbF {
        let sines = (10.0 * point.x).sin() * (10.0 * point.y).sin() * (10.0 * point.z).sin();

        if sines < 0.0 {
            self.odd_texture.sample(uv, point)
        } else {
            self.even_texture.sample(uv, point)
        }
    }
}