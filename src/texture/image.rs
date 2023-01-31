use std::path::Path;
use image::RgbImage;
use crate::core::color::ColorRgbF;
use crate::core::rehnda_math::Point3f;
use crate::texture::{Texture, Uv};

pub struct ImageTexture {
    rgb_img: RgbImage
}

impl ImageTexture {
    pub fn new_from_image_file(image_path: &Path) -> ImageTexture {
        let img = image::open(image_path).unwrap();
        let rgb_img = img.to_rgb8();

        ImageTexture {
            rgb_img
        }
    }
}

impl Texture for ImageTexture {
    fn sample(&self, uv: &Uv, point: &Point3f) -> ColorRgbF {
        let clamped_uv = uv.to_clamped_uv();
        let v_flipped = 1.0 - clamped_uv.v;

        let mut i = (clamped_uv.u * self.rgb_img.width() as f32) as u32;
        let mut j = (v_flipped * self.rgb_img.height() as f32) as u32;

        if i >= self.rgb_img.width() {
            i = self.rgb_img.width() - 1;
        }
        if j >= self.rgb_img.height() {
            j = self.rgb_img.height() - 1;
        }

        let color_scale = 1.0 / 255.0;
        let pixel = self.rgb_img.get_pixel(i, j);

        let r = pixel.0[0];
        let g = pixel.0[1];
        let b = pixel.0[2];

        ColorRgbF::new(
            r as f32 * color_scale,
            g as f32 * color_scale,
            b as f32 * color_scale,
        )
    }
}