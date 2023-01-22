use crate::acceleration::aabb::Aabb;
use crate::core::ray::Ray;
use crate::core::rehnda_math::{Point3f, Vec3f};
use crate::texture::Uv;

#[derive(Debug, Copy, Clone)]
pub struct HitResult {
    pub hit_location: Point3f,
    pub normal: Vec3f,
    pub t: f32,
    pub front_face: bool,
    pub uv: Uv,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitResult>;

    fn bounding_box(&self, time_0: f32, time_1: f32) -> Option<Aabb>;
}