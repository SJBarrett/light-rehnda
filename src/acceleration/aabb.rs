use std::mem::swap;
use crate::core::ray::Ray;
use crate::core::rehnda_math::Point3f;

#[derive(Debug, Copy, Clone)]
pub struct Aabb {
    pub min_corner: Point3f,
    pub max_corner: Point3f,
}

impl Aabb {
    /**
     * Could use the "slab" method, but the below is copied from Andrew Kensler's (from
     * Pixar) optimised version that compiles well on many compilers.
     */
    pub fn does_hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
        for a in 0..3 {
            let inv_d = 1.0f32 / ray.direction[a];
            let mut t_0 = (self.min_corner[a] - ray.origin[a]) * inv_d;
            let mut t_1 = (self.max_corner[a] - ray.origin[a]) * inv_d;
            if inv_d < 0f32 {
                swap(&mut t_0, &mut t_1);
            }
            let new_t_min = if t_0 > t_min { t_0 } else { t_min };
            let new_t_max = if t_1 < t_max { t_1 } else { t_max };
            if new_t_max <= new_t_min {
                return false;
            }
        }
        true
    }

    pub fn surrounding_box(box_0: &Aabb, box_1: &Aabb) -> Aabb {
        let min_corner = Point3f::new(
            box_0.min_corner.x.min(box_1.min_corner.x),
            box_0.min_corner.y.min(box_1.min_corner.y),
            box_0.min_corner.z.min(box_1.min_corner.z),
        );
        let max_corner = Point3f::new(
            box_0.max_corner.x.max(box_1.max_corner.x),
            box_0.max_corner.y.max(box_1.max_corner.y),
            box_0.max_corner.z.max(box_1.max_corner.z),
        );
        Aabb {
            min_corner,
            max_corner,
        }
    }
}

