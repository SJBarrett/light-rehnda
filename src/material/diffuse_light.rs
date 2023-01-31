use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use crate::core::color::ColorRgbF;
use crate::core::ray::Ray;
use crate::core::rehnda_math::Point3f;
use crate::hittable::HitResult;
use crate::material::{Material, Scatter};
use crate::texture::solid::SolidTexture;
use crate::texture::{Texture, Uv};

pub struct DiffuseLight {
    texture: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new_solid_light(color: &ColorRgbF) -> DiffuseLight {
        DiffuseLight {
            texture: Arc::new(SolidTexture::new(color.x, color.y, color.z)),
        }
    }
}

impl Debug for DiffuseLight {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "DiffuseLight")
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, ray_in: &Ray, hit_result: &HitResult) -> Option<Scatter> {
        None
    }

    fn emitted(&self, uv: &Uv, point: &Point3f) -> ColorRgbF {
        self.texture.sample(uv, point)
    }
}