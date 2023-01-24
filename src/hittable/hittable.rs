use std::sync::Arc;
use crate::acceleration::aabb::Aabb;
use crate::core::ray::Ray;
use crate::core::rehnda_math::{Point3f, Vec3f};
use crate::material::Material;
use crate::texture::Uv;

#[derive(Debug, Copy, Clone)]
pub struct HitResult<'a> {
    pub hit_location: Point3f,
    pub normal: Vec3f,
    pub t: f32,
    pub front_face: bool,
    pub uv: Uv,
    pub material: &'a dyn Material
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitResult>;

    fn bounding_box(&self, time_0: f32, time_1: f32) -> Option<Aabb>;
}

impl Hittable for Vec<Arc<dyn Hittable>> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitResult> {
        let mut closest_hit = t_max;
        let mut closest_hit_object: Option<HitResult> = None;
        for obj in self {
            if let Some(hit) = obj.hit(ray, t_min, closest_hit) {
                closest_hit_object = Some(hit);
                closest_hit = hit.t;
            }
        }

        closest_hit_object
    }

    fn bounding_box(&self, time_0: f32, time_1: f32) -> Option<Aabb> {
        todo!()
    }
}