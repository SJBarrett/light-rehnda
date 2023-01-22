use std::path::Path;
use crate::color::{ColorRgb8, ColorRgbF};
use crate::core::color::ColorRgbF;
use crate::image::image_buffer::ImageBuffer;
use crate::image::image_writer::ImageFileWriter;
use crate::image_buffer::ImageBuffer;
use crate::image_writer::ImageFileWriter;

mod core;
mod image;

fn main() {
    println!("Hello, world!");
    let image_width = 256;
    let image_height = 256;

    let mut image_buffer = ImageBuffer::new(image_width, image_height);
    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let r = (i as f32) / ((image_width - 1) as f32) ;
            let g = (j as f32) / ((image_height - 1) as f32);
            let b = 0.25f32;

            image_buffer.write_color_sample(i, j, ColorRgbF::new(r, g, b))
        }
    }

    let out_path = Path::new("out.ppm");
    let image_writer = ImageFileWriter {
        output_file_path: out_path,
    };
    image_writer.write_image_buffer_to_ppm(&image_buffer, 1).unwrap();
    println!("Done!");
}
