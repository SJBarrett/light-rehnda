use std::fs::File;
use std::io::Read;
use std::num::NonZeroUsize;
use std::path::Path;
use std::thread;
use log::info;
use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct RehndaSettings {
    pub scene: SceneName,
    pub output_file: Option<String>,
    pub max_depth: usize,
    pub image_width: usize,
    pub num_samples: usize,
    pub camera_settings: CameraSettings,
    num_threads: Option<usize>,
}


#[derive(Clone, Deserialize, Debug)]
pub enum SceneName {
    ThreeSpheres,
    RandomSpheres,
    Globe,
}

#[derive(Clone, Deserialize, Debug)]
struct AspectRatioSettings {
    width: f32,
    height: f32,
}

#[derive(Clone, Deserialize, Debug)]
pub struct CameraSettings {
    aspect_ratio: AspectRatioSettings,
    pub aperture: f32,
}

impl CameraSettings {
    pub fn aspect_ratio(&self) -> f32 {
        self.aspect_ratio.width / self.aspect_ratio.height
    }
}

impl RehndaSettings {
    pub fn from_file(path: &Path) -> RehndaSettings {
        let mut file = File::open(path).expect("File not found!");
        let mut data = String::new();
        file.read_to_string(&mut data).expect("Error reading settings file");

        let settings = ron::from_str(&data).unwrap();
        info!("Loaded settings: {:?}", settings);
        settings
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.camera_settings.aspect_ratio()
    }

    pub fn num_threads(&self) -> usize {
        self.num_threads.unwrap_or(thread::available_parallelism().map(NonZeroUsize::get).unwrap_or(1))
    }

    pub fn num_samples_per_thread(&self) -> usize {
        self.num_samples / self.num_threads()
    }

    pub fn image_height(&self) -> usize {
        (self.image_width as f32 / self.aspect_ratio()) as usize
    }
}