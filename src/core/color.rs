use glam::Vec3;

/// Color type where colors are encoded as a unit interval (0 to 1)
pub type ColorRgbF = Vec3;

/// Color represented by an unsigned 8 bit integer per channel
pub struct ColorRgb8 {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl ColorRgb8 {
    pub fn from_color_rgb_f(color_rgb: ColorRgbF) -> ColorRgb8 {
        ColorRgb8 {
            r: (256.0f32 * color_rgb.x.clamp(0.0, 0.999)) as u8,
            g: (256.0f32 * color_rgb.y.clamp(0.0, 0.999)) as u8,
            b: (256.0f32 * color_rgb.z.clamp(0.0, 0.999)) as u8,
        }
    }
}