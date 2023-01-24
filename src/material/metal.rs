use crate::core::color::ColorRgbF;
use crate::core::ray::Ray;
use crate::core::rehnda_math::{Vec3Ext, Vec3f};
use crate::hittable::HitResult;
use crate::material::{Material, Scatter};

#[derive(Debug, Copy, Clone)]
pub struct MetalMaterial {
    pub albedo: ColorRgbF,
    pub fuzz: f32,
}

impl Material for MetalMaterial {
    fn scatter(&self, ray_in: &Ray, hit_result: &HitResult) -> Option<Scatter> {
        let reflect_dir = ray_in.direction.unit_vector().reflect(hit_result.normal);
        let scatter_dir = reflect_dir + self.fuzz * Vec3f::random_vec_in_unit_sphere();
        Some(Scatter {
            scattered_ray: Ray { origin: hit_result.hit_location, direction: scatter_dir, time: ray_in.time },
            attenuation: self.albedo,
        })
    }
}