use glam::{Vec3};

pub type Vec3f = Vec3;

pub trait Vec3Ext {
    fn is_near_zero(&self) -> bool;
}

impl Vec3Ext for Vec3 {
    fn is_near_zero(&self) -> bool {
        const S: f32 = 1e-8f32;
        self.x.abs() < S && self.y.abs() < S && self.z.abs() < S
    }
}
