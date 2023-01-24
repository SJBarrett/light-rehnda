use crate::core::ray::Ray;
use crate::core::rehnda_math::{Point3f, random_in_range, Vec3Ext, Vec3f};

#[derive(Debug, Copy, Clone)]
pub struct Camera {
    aspect_ratio: f32,
    origin: Point3f,
    lower_left_corner: Point3f,
    look_direction: Vec3f,
    horizontal: Vec3f,
    vertical: Vec3f,
    u_axis: Vec3f,
    v_axis: Vec3f,
    lens_radius: f32,
    shutter_open_time: f32,
    shutter_close_time: f32,
}

pub struct CameraCreateInfo {
    pub look_from: Vec3f,
    pub look_at: Vec3f,
    pub up: Vec3f,
    pub vertical_fov_degrees: f32,
    pub aspect_ratio: f32,
    pub aperture: f32,
    pub focus_distance: f32,
    pub time_0: f32,
    pub time_1: f32
}

impl Camera {
    pub fn new(create_info: &CameraCreateInfo) -> Camera {
        let theta = create_info.vertical_fov_degrees.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = create_info.aspect_ratio * viewport_height;

        let look_direction = (create_info.look_from - create_info.look_at).unit_vector();
        let u_axis = create_info.up.cross(look_direction).unit_vector();
        let v_axis = look_direction.cross(u_axis);

        let origin = create_info.look_from;
        let horizontal = create_info.focus_distance * viewport_width * u_axis;
        let vertical = create_info.focus_distance * viewport_height * v_axis;

        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - look_direction * create_info.focus_distance;
        let lens_radius = create_info.aperture / 2.0;

        Camera {
            aspect_ratio: create_info.aspect_ratio,
            origin,
            lower_left_corner,
            look_direction,
            horizontal,
            vertical,
            u_axis,
            v_axis,
            lens_radius,
            shutter_open_time: create_info.time_0,
            shutter_close_time: create_info.time_1,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let random_offset = self.lens_radius * Vec3f::random_vec_in_unit_disk();
        let camera_frame_offset = self.u_axis * random_offset.x + self.v_axis * random_offset.y;
        Ray {
            origin: self.origin + camera_frame_offset,
            direction: self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - camera_frame_offset,
            time: random_in_range(self.shutter_open_time, self.shutter_close_time)
        }
    }
}