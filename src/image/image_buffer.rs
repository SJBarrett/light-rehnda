use crate::core::color::ColorRgbF;
use crate::core::rehnda_math::Vec3Ext;

pub struct ImageBuffer {
    pub image_width: usize,
    pub image_height: usize,
    pixels: Vec<ColorRgbF>
}

impl ImageBuffer {
    pub fn new(width: usize, height: usize) -> ImageBuffer {
        let mut pixels = Vec::new();
        pixels.resize(width * height, ColorRgbF::ZERO);
        ImageBuffer {
            image_width: width,
            image_height: height,
            pixels
        }
    }

    pub fn write_color_sample(&mut self , x: usize, y: usize, color: ColorRgbF) {
        self.pixels[x + self.image_width * y] += color;
    }

    pub fn add_buffer(&mut self, other_buffer: &ImageBuffer) {
        for x in 0..self.image_width {
            for y in 0..self.image_height {
                self.write_color_sample(x, y, other_buffer.pixels[x + self.image_width * y]);
            }
        }
    }

    pub fn get_color_sample_corrected(&self, x: usize, y: usize, num_samples: usize) -> ColorRgbF {
        let color = self.pixels[x + self.image_width * y];
        if color.is_near_zero() {
            return ColorRgbF::ZERO;
        }

        let mut r = color.x;
        let mut g = color.y;
        let mut b = color.z;

        let scale: f32 = 1.0 / num_samples as f32;
        r = ImageBuffer::perform_gamma_correction(scale * r);
        g = ImageBuffer::perform_gamma_correction(scale * g);
        b = ImageBuffer::perform_gamma_correction(scale * b);

        ColorRgbF {
            x: r,
            y: g,
            z: b
        }
    }

    fn perform_gamma_correction(value: f32) -> f32 {
        return value.sqrt();
    }
}
