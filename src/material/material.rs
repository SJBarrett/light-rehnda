use crate::core::color::ColorRgbF;
use crate::core::ray::Ray;
use crate::hittable::HitResult;

pub struct Scatter {
    pub scattered_ray: Ray,
    pub attenuation: ColorRgbF,
}

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_result: &HitResult) -> Option<Scatter>;
}