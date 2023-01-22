use std::fs::OpenOptions;
use std::io::{Error, Write};
use std::path::Path;

use crate::color::ColorRgb8;
use crate::image_buffer::ImageBuffer;

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
}