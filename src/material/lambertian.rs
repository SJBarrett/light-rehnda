use std::sync::Arc;
use crate::core::ray::Ray;
use crate::core::rehnda_math::{Vec3f, Vec3Ext};
use crate::hittable::HitResult;
use crate::material::{Material, Scatter};
use crate::texture::Texture;

pub struct LambertianMaterial<T: Texture> {
    texture: Arc<T>,
}

impl<T: Texture> Material for LambertianMaterial<T> {
    fn scatter(&self, ray_in: &Ray, hit_result: &HitResult) -> Option<Scatter> {
        let mut scatter_direction = hit_result.normal + Vec3f::random_unit_vector();

        if scatter_direction.is_near_zero() {
            scatter_direction = hit_result.normal;
        }

        Some(Scatter{
            scattered_ray: Ray{origin: hit_result.hit_location, direction: scatter_direction, time: ray_in.time},
            attenuation: self.texture.sample(&hit_result.uv, &hit_result.hit_location),
        })
    }
}