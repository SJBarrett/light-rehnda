use std::sync::Arc;
use crate::acceleration::aabb::Aabb;
use crate::core::ray::Ray;
use crate::core::rehnda_math::{Point3f, Vec3f};

use crate::hittable::{HitResult, Hittable};
use crate::material::Material;
use crate::texture::Uv;

pub struct YzRect {
    material: Arc<dyn Material>,
    y0: f32,
    y1: f32,
    z0: f32,
    z1: f32,
    k: f32,
}

impl YzRect {
    pub fn new(y0: f32, y1: f32, z0: f32, z1: f32, k: f32, material: Arc<dyn Material>) -> YzRect {
        YzRect {
            material,
            y0,
            y1,
            z0,
            z1,
            k,
        }
    }
}


impl Hittable for YzRect {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitResult> {
        let t = (self.k - ray.origin.x) / ray.direction.x;
        if t < t_min || t > t_max {
            return None
        }

        let y = ray.origin.y + t * ray.direction.y;
        let z = ray.origin.z + t * ray.direction.z;
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }
        let uv = Uv::new((y - self.y0) / (self.y1 - self.y0), (z - self.z0) / (self.z1 - self.z0));
        let outward_normal = Vec3f::new(1.0, 0.0, 0.0);
        let (normal, front_face) = HitResult::is_hit_front_face(&ray.direction, &outward_normal);

        Some(HitResult {
            hit_location: ray.at(t),
            normal,
            t,
            front_face,
            uv,
            material: &*self.material,
        })
    }

    fn bounding_box(&self, time_0: f32, time_1: f32) -> Option<Aabb> {
        Some(Aabb {
            min_corner: Point3f::new(self.k - 0.0001, self.y0, self.z0),
            max_corner: Point3f::new(self.k + 0.0001, self.y1, self.z1),
        })
    }
}