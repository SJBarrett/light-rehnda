use crate::core::rehnda_math::{Point3f, Vec3f};

pub struct Ray {
    pub origin: Point3f,
    pub direction: Vec3f,
    pub time: f32,
}

impl Ray {
    pub fn at(&self, t: f32) -> Point3f {
        self.origin + t * self.direction
    }
}

