use std::path::Path;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;
use std::time::Instant;
use log::info;

use simplelog::*;
use clap::Parser;
use serde::de::Unexpected::Str;
use crate::acceleration::bvh::BvhNode;
use crate::aggregator::{AggregationConfig, sample_pixels};

use crate::core::color::ColorRgbF;
use crate::core::rehnda_math::Point3f;
use crate::hittable::Hittable;
use crate::hittable::sphere::Sphere;
use crate::image::image_buffer::ImageBuffer;
use crate::image::image_writer::ImageFileWriter;
use crate::material::dielectric::DielectricMaterial;
use crate::material::lambertian::LambertianMaterial;
use crate::material::metal::MetalMaterial;
use crate::scene::camera::{Camera, CameraCreateInfo};
use crate::scene::Scene;
use crate::scene::scene_builder::load_scene;
use crate::scene::settings::RehndaSettings;
use crate::texture::solid::SolidTexture;

mod acceleration;
mod aggregator;
mod core;
mod hittable;
mod image;
mod material;
mod scene;
mod texture;
mod util;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    output_file: Option<String>,
    #[arg(short, long)]
    settings_file: String,
}

fn main() {
    TermLogger::init(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto).unwrap();
    let args = Args::parse();
    let settings = RehndaSettings::from_file(Path::new(&args.settings_file));
    info!("Rendering at resolution: {}x{}", settings.image_width, settings.image_height());
    let scene = load_scene(&settings);

    let aggregation_config = AggregationConfig {
        samples_per_pixel: settings.num_samples_per_thread(),
        max_sample_depth: settings.max_depth,
    };
    info!("Rendering using {} threads", settings.num_threads());
    let mut render_thread_handles: Vec<JoinHandle<ImageBuffer>> = Vec::new();
    for _i in 1..settings.num_threads() {
        render_thread_handles.push(spawn_render_thread(aggregation_config, scene.clone(), settings.image_width, settings.image_height()));
    }
    let mut main_buffer = ImageBuffer::new(settings.image_width, settings.image_height());

    let render_start = Instant::now();
    info!("Main thread starting rendering");
    sample_pixels(&aggregation_config, &scene, &mut main_buffer, true);
    info!("Main thread done. Main thread took: {:?}", render_start.elapsed());

    render_thread_handles.into_iter().for_each(|x| main_buffer.add_buffer(&x.join().unwrap()));

    let render_duration = render_start.elapsed();
    info!("All threads done. Took {:?}", render_duration);

    let output_file_str = args.output_file.as_ref().or(settings.output_file.as_ref());
    let out_path = output_file_str.map_or(Path::new("out.ppm"), Path::new);
    let image_writer = ImageFileWriter {
        output_file_path: out_path,
    };
    image_writer.write_image_buffer_to_ppm(&main_buffer, settings.num_samples_per_thread() * settings.num_threads()).unwrap();
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
    // TODO FIX DIELECTRIC
    let dielectric_mat = Arc::new(DielectricMaterial {
        refractive_index: 1.5,
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
        material: dielectric_mat,
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