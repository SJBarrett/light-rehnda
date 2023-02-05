use std::sync::Arc;
use log::{error, warn};
use crate::acceleration::aabb::Aabb;
use crate::core::color::ColorRgbF;
use crate::core::ray::Ray;
use crate::core::rehnda_math::{random, Vec3f};
use crate::hittable::{HitResult, Hittable};
use crate::material::isotropic::IsotropicMaterial;
use crate::material::Material;
use crate::texture::{Texture, Uv};

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    phase_function: Arc<dyn Material>,
    neg_inv_density: f32,
}

impl ConstantMedium {
    pub fn new_with_texture(boundary: Arc<dyn Hittable>, density: f32, texture: Arc<dyn Texture>) -> ConstantMedium {
        ConstantMedium {
            boundary,
            phase_function: Arc::new(IsotropicMaterial::new_with_texture(texture)),
            neg_inv_density: -1.0 / density,
        }
    }

    pub fn new_with_color(boundary: Arc<dyn Hittable>, density: f32, color: &ColorRgbF) -> ConstantMedium {
        ConstantMedium {
            boundary,
            phase_function: Arc::new(IsotropicMaterial::new_with_color(color)),
            neg_inv_density: -1.0 / density,
        }
    }
}

impl Hittable for ConstantMedium {
    ///
    /// This implementation does not support non convex volumes, as it expects to not
    /// hit itself again once it passes the boundary
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitResult> {
        let enable_debug = false;
        let debugging = enable_debug && random() < 0.00001;

        // do we ever encounter the medium?
        let mut hit_1 = self.boundary.hit(ray, f32::MIN, f32::MAX)?;
        // do we exit the medium, or was it just skimming the surface or have no depth
        let mut hit_2 = self.boundary.hit(ray, hit_1.t + 0.0001, f32::MAX)?;

        if debugging {
            warn!("t_min={}, t_max={}", hit_1.t, hit_2.t);
        }

        if hit_1.t < t_min {
            hit_1.t = t_min;
        }
        if hit_2.t > t_max {
            hit_2.t = t_max;
        }

        if hit_1.t >= hit_2.t {
            return None;
        }

        if hit_1.t < 0.0 {
            hit_1.t = 0.0;
        }

        let ray_length = ray.direction.length();
        let distance_inside_boundary = (hit_2.t - hit_1.t) * ray_length;
        // when does the ray hit the medium (based on density)
        let hit_distance = self.neg_inv_density * random().ln();

        // ray wasn't inside the medium long enough to hit it so passes through
        if hit_distance > distance_inside_boundary {
            return None;
        }

        let t = hit_1.t + hit_distance / ray_length;
        let hit_location = ray.at(t);

        if debugging {
            warn!("hit_distance = {}", hit_distance);
            warn!("t = {}", t);
            warn!("hit_location = {:?}", hit_location);
        }

        let normal = Vec3f::new(1.0, 0.0, 0.0);
        let front_face = true;

        Some(HitResult{
            hit_location,
            normal,
            t,
            front_face,
            uv: Uv::new(0.0, 0.0),
            material: self.phase_function.as_ref(),
        })
    }

    fn bounding_box(&self, time_0: f32, time_1: f32) -> Option<Aabb> {
        self.boundary.bounding_box(time_0, time_1)
    }
}
