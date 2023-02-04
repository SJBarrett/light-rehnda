use std::sync::Arc;
use crate::acceleration::aabb::Aabb;
use crate::core::ray::Ray;
use crate::core::rehnda_math::Vec3f;
use crate::hittable::{HitResult, Hittable};

pub struct Translate {
    contained_hittable: Arc<dyn Hittable>,
    offset: Vec3f,
}

impl Translate {
    pub fn new(hittable: Arc<dyn Hittable>, offset: &Vec3f) -> Translate {
        Translate {
            contained_hittable: hittable,
            offset: *offset,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitResult> {
        let moved_ray = Ray {
            origin: ray.origin - self.offset,
            direction: ray.direction,
            time: ray.time,
        };

        self.contained_hittable.hit(&moved_ray, t_min, t_max).map(|hit| {
            let (normal, front_face) = HitResult::is_hit_front_face(&moved_ray.direction, &hit.normal);
            HitResult {
                hit_location: hit.hit_location + self.offset,
                normal,
                t: hit.t,
                front_face,
                uv: hit.uv,
                material: hit.material,
            }
        })
    }

    fn bounding_box(&self, time_0: f32, time_1: f32) -> Option<Aabb> {
        self.contained_hittable.bounding_box(time_0, time_1)
            .map(|contained_bounding_box| Aabb {
                min_corner: contained_bounding_box.min_corner + self.offset,
                max_corner: contained_bounding_box.max_corner + self.offset,
            })
    }
}