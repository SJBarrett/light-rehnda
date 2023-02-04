use std::cmp::Ordering;
use std::sync::Arc;
use crate::acceleration::aabb::Aabb;
use crate::core::ray::Ray;
use crate::core::rehnda_math::random_int_in_range;
use crate::hittable::{HitResult, Hittable};

pub struct BvhNode {
    pub left: Arc<dyn Hittable>,
    pub right: Arc<dyn Hittable>,
    pub node_bounding_box: Aabb,
}

impl BvhNode {
    pub fn new(src_objects: &[Arc<dyn Hittable>], time_0: f32, time_1: f32) -> BvhNode {
        let mut objects = src_objects.to_vec();

        let split_axis = random_int_in_range(0, 2);
        let comparator_func = match split_axis {
            0 => BvhNode::box_x_compare,
            1 => BvhNode::box_y_compare,
            2 => BvhNode::box_z_compare,
            _ => unreachable!(),
        };

        let objects_span = src_objects.len();
        let (left, right) = match objects_span {
            1 => (objects[0].clone(), objects[0].clone()),
            2 => unsafe {
                if comparator_func(objects.get_unchecked(0), objects.get_unchecked(1)) == Ordering::Greater {
                    (objects[0].clone(), objects[1].clone())
                } else {
                    (objects[1].clone(), objects[0].clone())
                }
            },
            _ => {
                // TODO only sort slice
                objects.sort_by(comparator_func);
                let mid = src_objects.len() / 2;
                let left_slice = &objects[0..mid];
                let right_slice = &objects[mid..objects_span];
                (
                    Arc::new(BvhNode::new(left_slice, time_0, time_1)) as Arc<dyn Hittable>,
                    Arc::new(BvhNode::new(right_slice, time_0, time_1)) as Arc<dyn Hittable>
                )
            }
        };

        let bounding_box = if let Some(left_box) = left.bounding_box(time_0, time_1) {
            if let Some(right_box) = right.bounding_box(time_0, time_1) {
                Aabb::surrounding_box(&left_box, &right_box)
            } else { unreachable!() }
        } else { unreachable!() };

        BvhNode {
            left,
            right,
            node_bounding_box: bounding_box,
        }
    }

    fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        BvhNode::box_compare(a, b, 0)
    }

    fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        BvhNode::box_compare(a, b, 1)
    }

    fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        BvhNode::box_compare(a, b, 2)
    }

    fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis: usize) -> Ordering {
        if let Some(box_a) = a.bounding_box(0f32, 0f32) {
            if let Some(box_b) = b.bounding_box(0f32, 0f32) {
                return box_a.min_corner[axis].total_cmp(&box_b.min_corner[axis]);
            }
        }
        unreachable!("Can't compare boxes when no bounding boxes present")
    }
}

impl Hittable for BvhNode {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitResult> {
        if !self.node_bounding_box.does_hit(ray, t_min, t_max) {
            return None;
        }

        let left_hit = self.left.hit(ray, t_min, t_max);
        let right_hit = self.right.hit(ray, t_min, left_hit.map_or(t_max, |hit| hit.t));

        if right_hit.is_some() {
            right_hit
        } else if left_hit.is_some() {
            left_hit
        } else {
            None
        }
    }

    fn bounding_box(&self, _time_0: f32, _time_1: f32) -> Option<Aabb> {
        Some(self.node_bounding_box)
    }
}