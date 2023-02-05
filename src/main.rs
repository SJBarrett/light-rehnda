use std::path::Path;
use std::thread;
use std::thread::JoinHandle;
use std::time::Instant;

use clap::Parser;
use log::info;
use simplelog::*;

use crate::aggregator::{AggregationConfig, sample_pixels};
use crate::hittable::Hittable;
use crate::image::image_buffer::ImageBuffer;
use crate::image::image_writer::ImageFileWriter;
use crate::scene::Scene;
use crate::scene::scene_builder::load_scene;
use crate::scene::settings::RehndaSettings;

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
    // image_writer.write_image_buffer_to_ppm(&main_buffer, settings.num_samples_per_thread() * settings.num_threads()).unwrap();
    image_writer.write_to_file(&main_buffer, settings.num_samples_per_thread() * settings.num_threads()).unwrap();
    info!("Done!");
}

fn spawn_render_thread(aggregation_config: AggregationConfig, scene: Scene, image_width: usize, image_height: usize) -> JoinHandle<ImageBuffer> {
    thread::spawn(move || {
        let mut img_buffer = ImageBuffer::new(image_width, image_height);
        sample_pixels(&aggregation_config, &scene, &mut img_buffer, false);
        img_buffer
    })
}
