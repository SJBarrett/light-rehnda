use glam::{Vec3};
use rand::prelude::*;

pub type Vec3f = Vec3;
pub type Point3f = Vec3;

pub fn random_int_in_range(min: i32, max: i32) -> i32 {
    (min as f32 + random::<f32>() * (max - min) as f32) as i32
}

pub fn random_in_range(min: f32, max: f32) -> f32 {
    min + random::<f32>() * (max - min)
}

pub trait Vec3Ext {
    fn is_near_zero(&self) -> bool;
    fn unit_vector(&self) -> Vec3f;
    fn random_vec_in_unit_sphere() -> Vec3f;
    fn random_vec_in_unit_disk() -> Vec3f;
    fn random_vec_in_range(min: f32, max: f32) -> Vec3f;
    fn random_unit_vector() -> Vec3f;
    fn reflect(&self, normal: Vec3f) -> Vec3f;
}

impl Vec3Ext for Vec3 {
    fn is_near_zero(&self) -> bool {
        const S: f32 = 1e-8f32;
        self.x.abs() < S && self.y.abs() < S && self.z.abs() < S
    }

    fn unit_vector(&self) -> Vec3f {
        *self / self.length()
    }

    fn random_vec_in_unit_disk() -> Vec3f {
        loop {
            let candidate = Vec3f::new(random_in_range(-1.0, 1.0), random_in_range(-1.0, 1.0), 0.0);
            if candidate.length_squared() < 1.0 {
                return candidate;
            }
        }
    }

    fn random_vec_in_unit_sphere() -> Vec3f {
        loop {
            let candidate = Self::random_vec_in_range(-1f32, 1f32);
            if candidate.length_squared() < 1f32 {
                return candidate
            }
        }
    }

    fn random_vec_in_range(min: f32, max: f32) -> Vec3f {
        Vec3f::new(random_in_range(min, max), random_in_range(min, max), random_in_range(min, max))
    }

    fn random_unit_vector() -> Vec3f {
        Self::random_vec_in_unit_sphere()
    }

    fn reflect(&self, normal: Vec3f) -> Vec3f {
        *self - 2.0 * self.dot(normal) * normal
    }
}





