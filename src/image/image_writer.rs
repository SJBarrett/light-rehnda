use std::fs::OpenOptions;
use std::io::{Error, Write};
use std::path::Path;
use image::{DynamicImage, ImageFormat, ImageResult, Rgb, Rgb32FImage, RgbImage};
use image::buffer::ConvertBuffer;

use crate::core::color::ColorRgb8;
use crate::image::image_buffer::ImageBuffer;

pub struct ImageFileWriter<'a> {
    pub output_file_path: &'a Path,
}

impl ImageFileWriter<'_> {
    pub fn write_image_buffer_to_ppm(&self, image_buffer: &ImageBuffer, num_samples: usize) -> Result<(), Error> {
        let mut out_file = OpenOptions::new()
            .write(true)
            .create(true)
            .read(false)
            .append(false)
            .open(self.output_file_path)?;

        writeln!(out_file, "P3")?;
        writeln!(out_file, "{} {}", image_buffer.image_width, image_buffer.image_height)?;
        writeln!(out_file, "255")?;
        for y in (0..image_buffer.image_height).rev() {
            for x in 0..image_buffer.image_width {
                let color_sample = image_buffer.get_color_sample_corrected(x, y, num_samples);
                let color_8_bit = ColorRgb8::from_color_rgb_f(color_sample);
                writeln!(out_file, "{} {} {}", color_8_bit.r, color_8_bit.g, color_8_bit.b)?;
            }
        }
        Ok(())
    }

    pub fn write_to_file(&self, image_buffer: &ImageBuffer, num_samples: usize) -> ImageResult<()> {
        let mut out_image = Rgb32FImage::new(image_buffer.image_width as u32, image_buffer.image_height as u32);
        for x in 0..image_buffer.image_width {
            for y in 0..image_buffer.image_height {
                let color_sample = image_buffer.get_color_sample_corrected(x, y, num_samples);
                out_image.put_pixel(x as u32, (image_buffer.image_height - 1 - y) as u32, Rgb([color_sample.x, color_sample.y, color_sample.z]));
            }
        }
        let rgb8 = DynamicImage::ImageRgb32F(out_image).into_rgb8();
        rgb8.save(self.output_file_path)?;
        Ok(())
    }
}