use std::sync::Arc;
use crate::acceleration::aabb::Aabb;
use crate::core::ray::Ray;
use crate::core::rehnda_math::{Point3f, Vec3f};

use crate::hittable::{HitResult, Hittable};
use crate::material::Material;
use crate::texture::Uv;


pub struct XzRect {
    material: Arc<dyn Material>,
    x0: f32,
    x1: f32,
    z0: f32,
    z1: f32,
    k: f32,
}

impl XzRect {
    pub fn new(x0: f32, x1: f32, z0: f32, z1: f32, k: f32, material: Arc<dyn Material>) -> XzRect {
        XzRect {
            material,
            x0,
            x1,
            z0,
            z1,
            k,
        }
    }
}


impl Hittable for XzRect {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitResult> {
        let t = (self.k - ray.origin.y) / ray.direction.y;
        if t < t_min || t > t_max {
            return None
        }

        let x = ray.origin.x + t * ray.direction.x;
        let z = ray.origin.z + t * ray.direction.z;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }
        let uv = Uv::new((x - self.x0) / (self.x1 - self.x0), (z - self.z0) / (self.z1 - self.z0));
        let outward_normal = Vec3f::new(0.0, 1.0, 0.0);
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
            min_corner: Point3f::new(self.x0, self.k - 0.0001, self.z0),
            max_corner: Point3f::new(self.x1, self.k + 0.0001, self.z1),
        })
    }
}