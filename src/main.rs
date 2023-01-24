use std::num::NonZeroUsize;
use std::path::Path;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;
use std::time::Instant;
use log::info;

use simplelog::*;
use crate::acceleration::bvh::BvhNode;
use crate::aggregator::{AggregationConfig, sample_pixels};

use crate::core::color::ColorRgbF;
use crate::core::rehnda_math::Point3f;
use crate::hittable::Hittable;
use crate::hittable::sphere::Sphere;
use crate::image::image_buffer::ImageBuffer;
use crate::image::image_writer::ImageFileWriter;
use crate::material::lambertian::LambertianMaterial;
use crate::material::metal::MetalMaterial;
use crate::scene::camera::{Camera, CameraCreateInfo};
use crate::scene::Scene;
use crate::texture::solid::SolidTexture;

mod acceleration;
mod aggregator;
mod core;
mod hittable;
mod image;
mod material;
mod scene;
mod texture;

fn main() {
    TermLogger::init(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto).unwrap();
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 600;
    let image_height = (image_width as f32 / aspect_ratio) as usize;

    let test_scene = test_scene();
    let num_samples = 4;
    let num_threads = thread::available_parallelism().map(NonZeroUsize::get).unwrap_or(1);

    let aggregation_config = AggregationConfig {
        samples_per_pixel: num_samples,
        max_sample_depth: 30,
    };
    info!("Rendering using {} threads", num_threads);
    let mut render_thread_handles: Vec<JoinHandle<ImageBuffer>> = Vec::new();
    for _i in 1..num_threads {
        render_thread_handles.push(spawn_render_thread(aggregation_config, test_scene.clone(), image_width, image_height));
    }
    let mut main_buffer = ImageBuffer::new(image_width, image_height);

    let render_start = Instant::now();
    info!("Main thread starting rendering");
    sample_pixels(&aggregation_config, &test_scene, &mut main_buffer, true);
    info!("Main thread done. Main thread took: {:?}", render_start.elapsed());

    render_thread_handles.into_iter().for_each(|x| main_buffer.add_buffer(&x.join().unwrap()));

    let render_duration = render_start.elapsed();
    info!("All threads done. Took {:?}", render_duration);

    let out_path = Path::new("out.ppm");
    let image_writer = ImageFileWriter {
        output_file_path: out_path,
    };
    image_writer.write_image_buffer_to_ppm(&main_buffer, num_samples * num_threads).unwrap();
    info!("Done!");
}

fn spawn_render_thread(aggregation_config: AggregationConfig, scene: Scene, image_width: usize, image_height: usize) -> JoinHandle<ImageBuffer> {
    thread::spawn(move || {
        let mut img_buffer = ImageBuffer::new(image_width, image_height);
        sample_pixels(&aggregation_config, &scene, &mut img_buffer, false);
        img_buffer
    })
}

fn test_scene() -> Scene {
    let sphere_mat = Arc::new(LambertianMaterial {
        texture: Arc::new(SolidTexture { albedo: ColorRgbF::new(0.7, 0.1, 0.7) }),
    });
    let metal_mat = Arc::new(MetalMaterial {
        albedo: ColorRgbF::new(0.1, 0.6, 0.2),
        fuzz: 0.0,
    });
    let sphere_mat_2 = Arc::new(LambertianMaterial {
        texture: Arc::new(SolidTexture { albedo: ColorRgbF::new(0.1, 0.6, 0.2) }),
    });
    let sphere_1 = Sphere {
        centre: Point3f::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: sphere_mat,
    };
    let sphere_2 = Sphere {
        centre: Point3f::new(0.0, 2.0, 0.0),
        radius: 2.0,
        material: metal_mat,
    };
    let objects: Vec<Arc<dyn Hittable>> = vec![Arc::new(sphere_1), Arc::new(sphere_2)];

    let world = BvhNode::new(objects.as_slice(), 0, objects.len(), 0.0, 1.0);
    let cam_create_info = CameraCreateInfo {
        look_from: Point3f::new(13.0, 2.0, 3.0),
        look_at: Point3f::splat(0.0),
        up: Point3f::new(0.0, 1.0, 0.0),
        vertical_fov_degrees: 20.0,
        aspect_ratio: 16.0 / 9.0,
        aperture: 0.1,
        focus_distance: 10.0,
        time_0: 0.0,
        time_1: 1.0,
    };
    let camera = Camera::new(&cam_create_info);

    Scene {
        world: Arc::new(world),
        camera,
    }
}