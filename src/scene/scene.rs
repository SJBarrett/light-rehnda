use std::sync::Arc;
use crate::hittable::Hittable;
use crate::scene::camera::Camera;

#[derive(Clone)]
pub struct Scene {
    pub camera: Camera,
    pub world: Arc<dyn Hittable>,
}