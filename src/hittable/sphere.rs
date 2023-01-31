use std::f32::consts::PI;
use std::sync::Arc;
use crate::acceleration::aabb::Aabb;
use crate::core::ray::Ray;
use crate::core::rehnda_math::{Point3f, Vec3f};
use crate::hittable::{HitResult, Hittable};
use crate::material::Material;
use crate::texture::Uv;

pub struct Sphere {
    pub centre: Point3f,
    pub radius: f32,
    pub material: Arc<dyn Material>,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitResult> {
        let oc = ray.origin - self.centre;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0f32 {
            return None;
        }

        let sqrt_dist = discriminant.sqrt();
        let ia = 1f32 / a;
        let mut root = (-half_b - sqrt_dist) * ia;
        if root < t_min || root > t_max {
            root = (-half_b + sqrt_dist) * ia;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let hit_location = ray.at(root);
        let outward_normal = (hit_location - self.centre) / self.radius;
        let (normal, front_face) = HitResult::is_hit_front_face(&ray.direction, &outward_normal);

        Some(HitResult {
            hit_location,
            normal,
            t: root,
            front_face,
            uv: Sphere::get_unit_sphere_uv(&outward_normal),
            material: &*self.material,
        })
    }

    fn bounding_box(&self, _time_0: f32, _time_1: f32) -> Option<Aabb> {
        Some(Aabb {
            min_corner: self.centre - Vec3f::splat(self.radius),
            max_corner: self.centre + Vec3f::splat(self.radius),
        })
    }
}

impl Sphere {
    fn get_unit_sphere_uv(surface_point: &Point3f) -> Uv {
        let theta = (-surface_point.y).acos();
        let phi = (-surface_point.z).atan2(surface_point.x) + PI;
        Uv::new(phi / (2f32 * PI), theta / PI)
    }
}