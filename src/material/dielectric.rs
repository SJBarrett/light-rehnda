use crate::core::color::ColorRgbF;
use crate::core::ray::Ray;
use crate::core::rehnda_math::{random, random_in_range, Vec3Ext};
use crate::hittable::HitResult;
use crate::material::{Material, Scatter};

#[derive(Debug, Copy, Clone)]
pub struct DielectricMaterial {
    pub refractive_index: f32,
}

impl Material for DielectricMaterial {
    fn scatter(&self, ray_in: &Ray, hit_result: &HitResult) -> Option<Scatter> {
        let attenuation = ColorRgbF::splat(1.0);

        let refraction_ratio = if hit_result.front_face {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };
        let unit_dir = ray_in.direction.unit_vector();

        let cos_theta = (-unit_dir).dot(hit_result.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let scatter_direction = if cannot_refract || reflectance(cos_theta, refraction_ratio) > random() {
            unit_dir.reflect(hit_result.normal)
        } else {
            unit_dir.refract(hit_result.normal, refraction_ratio)
        };

        let scattered_ray = Ray {
            origin: hit_result.hit_location,
            direction: scatter_direction,
            time: ray_in.time,
        };
        Some(Scatter{
            scattered_ray,
            attenuation,
        })
    }
}

fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
    let mut r_0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r_0 = r_0 * r_0;
    r_0 + (1.0 - r_0) * ((1.0 - cosine).powi(5))
}