use std::sync::Arc;
use crate::acceleration::bvh::BvhNode;
use crate::core::color::ColorRgbF;
use crate::core::rehnda_math::{Point3f, random_in_range, Vec3Ext};
use crate::hittable::Hittable;
use crate::hittable::sphere::Sphere;
use crate::material::dielectric::DielectricMaterial;
use crate::material::lambertian::LambertianMaterial;
use crate::material::metal::MetalMaterial;
use crate::scene::camera::{Camera, CameraCreateInfo};
use crate::scene::Scene;
use crate::scene::settings::{CameraSettings, RehndaSettings, SceneName};
use crate::texture::solid::SolidTexture;

pub fn load_scene(settings: &RehndaSettings) -> Scene {
    match settings.scene {
        SceneName::RandomSpheres => random_spheres_scene(&settings.camera_settings),
        SceneName::ThreeSpheres => three_spheres_scene(&settings.camera_settings),
        _ => unimplemented!("Unsupported scene name!")
    }
}

fn three_spheres_scene(camera_settings: &CameraSettings) -> Scene {
    let mut objects: Vec<Arc<dyn Hittable>> = Vec::new();

    let ground_material = Arc::new(LambertianMaterial::new_with_solid_color(&ColorRgbF::new(0.8, 0.8, 0.0)));
    let centre_material = Arc::new(LambertianMaterial::new_with_solid_color(&ColorRgbF::new(0.7, 0.3, 0.3)));
    let left_material = Arc::new(DielectricMaterial { refractive_index: 1.5 });
    let right_material = Arc::new(MetalMaterial { albedo: ColorRgbF::new(0.8, 0.6, 0.2), fuzz: 1.0});

    objects.push(Arc::new(Sphere {
        centre: Point3f::new(0.0, -100.5, -1.0),
        radius: 100.0,
        material: ground_material,
    }));

    objects.push(Arc::new(Sphere {
        centre: Point3f::new(0.0, 0.0, -1.0),
        radius: 0.5,
        material: centre_material,
    }));
    objects.push(Arc::new(Sphere {
        centre: Point3f::new(-1.0, 0.0, -1.0),
        radius: 0.5,
        material: left_material.clone(),
    }));
    objects.push(Arc::new(Sphere {
        centre: Point3f::new(-1.0, 0.0, -1.0),
        radius: -0.4,
        material: left_material,
    }));
    objects.push(Arc::new(Sphere {
        centre: Point3f::new(1.0, 0.0, -1.0),
        radius: 0.5,
        material: right_material,
    }));

    let cam_create_info = CameraCreateInfo {
        look_from: Point3f::new(0.0, 0.0, 0.0),
        look_at: Point3f::new(0.0, 0.0, -1.0),
        up: Point3f::new(0.0, 1.0, 0.0),
        vertical_fov_degrees: 100.0,
        aspect_ratio: camera_settings.aspect_ratio(),
        aperture: camera_settings.aperture,
        focus_distance: 1.0,
        time_0: 0.0,
        time_1: 1.0,
    };
    let camera = Camera::new(&cam_create_info);

    Scene {
        world: Arc::new(BvhNode::new(objects.as_slice(), 0, objects.len(), 0.0, 1.0)),
        camera,
    }
}

fn random_spheres_scene(camera_settings: &CameraSettings) -> Scene {
    let mut objects: Vec<Arc<dyn Hittable>> = Vec::new();
    let ground_material = Arc::new(LambertianMaterial {
        texture: Arc::new(SolidTexture { albedo: ColorRgbF::new(0.5, 0.5, 0.5)})
    });
    objects.push(Arc::new(Sphere {
        centre: Point3f::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: ground_material,
    }));
    let num_random_objects = 11;
    for a in -num_random_objects..num_random_objects {
        for b in -num_random_objects..num_random_objects {
            let choose_mat = random_in_range(0.0, 1.0);
            let centre = Point3f::new(a as f32 + 0.9 * random_in_range(0.0, 1.0), 0.2, b as f32 + 0.9 * random_in_range(0.0, 1.0));
            if (centre - Point3f::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo: ColorRgbF = ColorRgbF::random_vec_in_range(0.0, 1.0) * ColorRgbF::random_vec_in_range(0.0, 1.0);
                    let material = Arc::new(LambertianMaterial::new_with_solid_color(&albedo));
                    objects.push(Arc::new(Sphere {
                        centre,
                        radius: 0.2,
                        material,
                    }));
                } else if choose_mat < 0.95 {
                    let albedo: ColorRgbF = ColorRgbF::random_vec_in_range(0.5, 1.0);
                    let fuzz = random_in_range(0.0, 0.5);
                    let material = Arc::new(MetalMaterial {
                        albedo,
                        fuzz,
                    });
                    objects.push(Arc::new(Sphere {
                        centre,
                        radius: 0.2,
                        material,
                    }));
                } else {
                    let material = Arc::new(DielectricMaterial {
                        refractive_index: 1.5,
                    });
                    objects.push(Arc::new(Sphere {
                        centre,
                        radius: 0.2,
                        material,
                    }));
                }
            }
        }
    }

    objects.push(Arc::new(Sphere {
        centre: Point3f::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(DielectricMaterial { refractive_index: 1.5}),
    }));

    objects.push(Arc::new(Sphere {
        centre: Point3f::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(LambertianMaterial::new_with_solid_color(&ColorRgbF::new(0.4, 0.2, 0.1))),
    }));

    objects.push(Arc::new(Sphere {
        centre: Point3f::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(MetalMaterial {
            albedo: ColorRgbF::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        }),
    }));


    let cam_create_info = CameraCreateInfo {
        look_from: Point3f::new(13.0, 2.0, 3.0),
        look_at: Point3f::splat(0.0),
        up: Point3f::new(0.0, 1.0, 0.0),
        vertical_fov_degrees: 20.0,
        aspect_ratio: camera_settings.aspect_ratio(),
        aperture: camera_settings.aperture,
        focus_distance: 10.0,
        time_0: 0.0,
        time_1: 1.0,
    };
    let camera = Camera::new(&cam_create_info);
    Scene {
        world: Arc::new(BvhNode::new(objects.as_slice(), 0, objects.len(), 0.0, 1.0)),
        camera,
    }
}
