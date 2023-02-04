use std::sync::Arc;
use crate::acceleration::aabb::Aabb;
use crate::core::ray::Ray;
use crate::core::rehnda_math::{Point3f, Vec3f};
use crate::hittable::{HitResult, Hittable};

pub struct RotateY {
    contained_hittable: Arc<dyn Hittable>,
    sin_theta: f32,
    cos_theta: f32,
    bounding_box: Option<Aabb>,
}

impl RotateY {
    pub fn new(hittable: Arc<dyn Hittable>, angle: f32) -> RotateY {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let contained_box = hittable.bounding_box(0.0, 1.0);


        let bounding_box = contained_box.map(
            |contained_bbox| {
                let mut min_corner = Point3f::new(f32::MAX, f32::MAX, f32::MAX);
                let mut max_corner = Point3f::new(f32::MIN, f32::MIN, f32::MIN);
                for i in 0..2 {
                    for j in 0..2 {
                        for k in 0..2 {
                            let x = i as f32 * contained_bbox.max_corner.x + (1 - i) as f32 * contained_bbox.min_corner.x;
                            let y = j as f32 * contained_bbox.max_corner.y + (1 - j) as f32 * contained_bbox.min_corner.y;
                            let z = k as f32 * contained_bbox.max_corner.z + (1 - k) as f32 * contained_bbox.min_corner.z;

                            let new_x = cos_theta * x + sin_theta * z;
                            let new_z = -sin_theta * x + cos_theta * z;

                            let tester = Vec3f::new(new_x, y, new_z);
                            for c in 0..3 {
                                min_corner[c] = min_corner[c].min(tester[c]);
                                max_corner[c] = max_corner[c].max(tester[c]);
                            }
                        }
                    }
                }
                Aabb {
                    min_corner,
                    max_corner,
                }
            });

        RotateY {
            contained_hittable: hittable,
            sin_theta,
            cos_theta,
            bounding_box,
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitResult> {
        let mut origin = ray.origin;
        let mut direction = ray.direction;

        origin[0] = self.cos_theta * ray.origin[0] - self.sin_theta * ray.origin[2];
        origin[2] = self.sin_theta * ray.origin[0] + self.cos_theta * ray.origin[2];

        direction[0] = self.cos_theta * ray.direction[0] - self.sin_theta * ray.direction[2];
        direction[2] = self.sin_theta * ray.direction[0] + self.cos_theta * ray.direction[2];

        let rotated_ray = Ray {
            origin,
            direction,
            time: ray.time,
        };

        self.contained_hittable.hit(&rotated_ray, t_min, t_max).map(|hit| {
            let mut hit_location = hit.hit_location;
            let mut normal = hit.normal;

            hit_location[0] = self.cos_theta * hit.hit_location[0] + self.sin_theta * hit.hit_location[2];
            hit_location[2] = -self.sin_theta * hit.hit_location[0] + self.cos_theta * hit.hit_location[2];

            normal[0] = self.cos_theta * hit.normal[0] + self.sin_theta * hit.normal[2];
            normal[2] = -self.sin_theta * hit.normal[0] + self.cos_theta * hit.normal[2];

            let (face_normal, front_face) = HitResult::is_hit_front_face(&rotated_ray.direction, &normal);

            HitResult {
                hit_location,
                normal: face_normal,
                t: hit.t,
                front_face,
                uv: hit.uv,
                material: hit.material,
            }
        })
    }

    fn bounding_box(&self, _time_0: f32, _time_1: f32) -> Option<Aabb> {
        self.bounding_box
    }
}