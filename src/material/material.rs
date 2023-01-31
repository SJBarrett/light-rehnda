use std::fmt::{Debug};
use crate::core::color::ColorRgbF;
use crate::core::ray::Ray;
use crate::core::rehnda_math::Point3f;
use crate::hittable::HitResult;
use crate::texture::Uv;

pub struct Scatter {
    pub scattered_ray: Ray,
    pub attenuation: ColorRgbF,
}


pub trait Material: Debug + Send + Sync {
    fn scatter(&self, ray_in: &Ray, hit_result: &HitResult) -> Option<Scatter>;

    fn emitted(&self, uv: &Uv, point: &Point3f) -> ColorRgbF {
        ColorRgbF::ZERO
    }
}
