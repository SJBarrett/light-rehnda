use std::sync::Arc;

use crate::acceleration::aabb::Aabb;
use crate::core::ray::Ray;
use crate::core::rehnda_math::Point3f;
use crate::hittable::{HitResult, Hittable};
use crate::hittable::xy_rect::XyRect;
use crate::hittable::xz_rect::XzRect;
use crate::hittable::yz_rect::YzRect;
use crate::material::Material;

pub struct BoxHittable {
    min_corner: Point3f,
    max_corner: Point3f,
    sides: Vec<Arc<dyn Hittable>>,
}

impl BoxHittable {
    pub fn new(min_corner: &Point3f, max_corner: &Point3f, material: Arc<dyn Material>) -> BoxHittable {
        let mut sides: Vec<Arc<dyn Hittable>> = vec![
            Arc::new(XyRect::new(min_corner.x, max_corner.x, min_corner.y, max_corner.y, max_corner.z, material.clone())),
            Arc::new(XyRect::new(min_corner.x, max_corner.x, min_corner.y, max_corner.y, min_corner.z, material.clone())),
            Arc::new(XzRect::new(min_corner.x, max_corner.x, min_corner.z, max_corner.z, max_corner.y, material.clone())),
            Arc::new(XzRect::new(min_corner.x, max_corner.x, min_corner.z, max_corner.z, min_corner.y, material.clone())),
            Arc::new(YzRect::new(min_corner.y, max_corner.y, min_corner.z, max_corner.z, max_corner.x, material.clone())),
            Arc::new(YzRect::new(min_corner.y, max_corner.y, min_corner.z, max_corner.z, min_corner.x, material)),
        ];

        BoxHittable {
            min_corner: *min_corner,
            max_corner: *max_corner,
            sides,
        }
    }
}

impl Hittable for BoxHittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitResult> {
        self.sides.hit(ray, t_min, t_max)
    }

    fn bounding_box(&self, time_0: f32, time_1: f32) -> Option<Aabb> {
        Some(Aabb { min_corner: self.min_corner, max_corner: self.max_corner })
    }
}