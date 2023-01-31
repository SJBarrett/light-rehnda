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

impl<'a> HitResult<'a> {
    pub fn is_hit_front_face(incident_ray_dir: &Vec3f, outward_normal: &Vec3f) -> (Vec3f, bool) {
        let mut normal = *outward_normal;
        let mut front_face = true;
        if incident_ray_dir.dot(normal) > 0.0 {
            normal = -normal;
            front_face = false;
        }

        (normal, front_face)
    }
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