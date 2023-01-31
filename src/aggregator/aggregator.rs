use std::fmt::Write;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};

use crate::core::color::ColorRgbF;
use crate::core::ray::Ray;
use crate::core::rehnda_math::random_in_range;
use crate::image::image_buffer::ImageBuffer;
use crate::scene::Scene;

#[derive(Debug, Copy, Clone)]
pub struct AggregationConfig {
    pub samples_per_pixel: usize,
    pub max_sample_depth: usize,
}

fn sample_ray(ray: &Ray, scene: &Scene, depth: usize) -> ColorRgbF {
    if depth == 0 {
        return ColorRgbF::ZERO;
    }

    return if let Some(hit_result) = scene.world.hit(ray, 0.001, f32::MAX) {
        let emitted = hit_result.material.emitted(&hit_result.uv, &hit_result.hit_location);
        if let Some(scatter) = hit_result.material.scatter(ray, &hit_result) {
            emitted + scatter.attenuation * sample_ray(&scatter.scattered_ray, scene, depth - 1)
        } else {
            // no scatter, so only return the emitted light
            emitted
        }
    } else {
        // hit nothing in the world so return a background color
        scene.background
    }
}

pub fn sample_pixels(aggregation_config: &AggregationConfig, scene: &Scene, out_image_buffer: &mut ImageBuffer, print_progress: bool) {
    let progress_bar: Option<ProgressBar> = if print_progress {
        let pb = ProgressBar::new(100u64);
        pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .unwrap()
            .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
            .progress_chars("#>-"));
        Some(pb)
    } else {
        None
    };
    for j in (0..out_image_buffer.image_height).rev() {
        if let Some(pb) = progress_bar.as_ref() {
            pb.set_position((((out_image_buffer.image_height - j) as f32 / out_image_buffer.image_height as f32) * 100f32) as u64);
        }
        for i in 0..out_image_buffer.image_width {
            for _s in 0..aggregation_config.samples_per_pixel {
                let u = (i as f32 + random_in_range(0.0, 1.0)) / (out_image_buffer.image_width - 1) as f32;
                let v = (j as f32 + random_in_range(0.0, 1.0)) / (out_image_buffer.image_height - 1) as f32;
                let ray = scene.camera.get_ray(u, v);
                out_image_buffer.write_color_sample(i, j, sample_ray(&ray, scene, aggregation_config.max_sample_depth));
            }
        }
    }
}