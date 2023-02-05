use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use crate::core::color::ColorRgbF;
use crate::core::ray::Ray;
use crate::core::rehnda_math::{Vec3Ext, Vec3f};
use crate::hittable::HitResult;
use crate::material::{Material, Scatter};
use crate::texture::solid::SolidTexture;
use crate::texture::Texture;

pub struct IsotropicMaterial {
    albedo: Arc<dyn Texture>
}

impl IsotropicMaterial {
    pub fn new_with_color(color: &ColorRgbF) -> IsotropicMaterial {
        IsotropicMaterial {
            albedo: Arc::new(SolidTexture { albedo: *color })
        }
    }

    pub fn new_with_texture(texture: Arc<dyn Texture>) -> IsotropicMaterial {
        IsotropicMaterial {
            albedo: texture,
        }
    }
}

impl Material for IsotropicMaterial {
    fn scatter(&self, ray_in: &Ray, hit_result: &HitResult) -> Option<Scatter> {
        let scattered_ray = Ray {
            origin: hit_result.hit_location,
            direction: Vec3f::random_vec_in_unit_sphere(),
            time: ray_in.time,
        };
        let attenuation = self.albedo.sample(&hit_result.uv, &hit_result.hit_location);
        Some(Scatter {
            scattered_ray,
            attenuation,
        })
    }
}

impl Debug for IsotropicMaterial {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "IsotropicMaterial")
    }
}
