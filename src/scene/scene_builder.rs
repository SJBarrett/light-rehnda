use std::iter::once;
use std::path::Path;
use std::slice;
use std::sync::Arc;
use crate::acceleration::bvh::BvhNode;
use crate::core::color::ColorRgbF;
use crate::core::rehnda_math::{Point3f, random_in_range, Vec3Ext, Vec3f};
use crate::hittable::box_hittable::BoxHittable;
use crate::hittable::constant_medium::ConstantMedium;
use crate::hittable::Hittable;
use crate::hittable::rotate_y::RotateY;
use crate::hittable::sphere::Sphere;
use crate::hittable::translate::Translate;
use crate::hittable::xy_rect::XyRect;
use crate::hittable::xz_rect::XzRect;
use crate::hittable::yz_rect::YzRect;
use crate::material::dielectric::DielectricMaterial;
use crate::material::diffuse_light::DiffuseLight;
use crate::material::lambertian::LambertianMaterial;
use crate::material::metal::MetalMaterial;
use crate::scene::camera::{Camera, CameraCreateInfo};
use crate::scene::Scene;
use crate::scene::settings::{CameraSettings, RehndaSettings, SceneName};
use crate::texture::checker::CheckerTexture;
use crate::texture::image::ImageTexture;
use crate::texture::noise::NoiseTexture;
use crate::texture::solid::SolidTexture;

const DEFAULT_BACKGROUND: ColorRgbF = ColorRgbF::new(0.7, 0.8, 1.0);

pub fn load_scene(settings: &RehndaSettings) -> Scene {
    match settings.scene {
        SceneName::RandomSpheres => random_spheres_scene(&settings.camera_settings),
        SceneName::ThreeSpheres => three_spheres_scene(&settings.camera_settings),
        SceneName::Globe => globe_scene(&settings.camera_settings),
        SceneName::LightsDemo => lights_demo_scene(&settings.camera_settings),
        SceneName::CornellBox => cornell_box(&settings.camera_settings),
        SceneName::CornellSmoke => cornell_smoke(&settings.camera_settings),
        SceneName::CornellFeatureDemo => cornell_feature_demo(&settings.camera_settings),
        _ => unimplemented!("Unsupported scene name!")
    }
}


fn cornell_feature_demo(camera_settings: &CameraSettings) -> Scene {
    let mut objects: Vec<Arc<dyn Hittable>> = Vec::new();

    let checker_texture = Arc::new(CheckerTexture::new(0.1, Arc::new(SolidTexture::new(0.2, 0.3, 0.1)), Arc::new(SolidTexture::new(0.9, 0.9, 0.9))));
    let ground_material = Arc::new(LambertianMaterial::new(checker_texture));
    let red = Arc::new(LambertianMaterial::new_with_solid_color(&ColorRgbF::new(0.65, 0.05, 0.05)));
    let white = Arc::new(LambertianMaterial::new_with_solid_color(&ColorRgbF::new(0.73, 0.73, 0.73)));
    let green = Arc::new(LambertianMaterial::new_with_solid_color(&ColorRgbF::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new_solid_light(&ColorRgbF::new(15.0, 15.0, 15.0)));

    // walls
    objects.push(Arc::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    objects.push(Arc::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    objects.push(Arc::new(XzRect::new(213.0, 343.0, 227.0, 332.0, 554.0, light)));
    // floor
    objects.push(Arc::new(XzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, ground_material)));
    objects.push(Arc::new(XzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())));
    objects.push(Arc::new(XyRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())));

    // left box
    let box_1 = {
        let metal_mat = Arc::new(MetalMaterial { albedo: ColorRgbF::new(1.0, 1.0, 1.0), fuzz: 0.01 });
        let mut box_t: Arc<dyn Hittable> = Arc::new(BoxHittable::new(&Point3f::new(0.0, 0.0, 0.0), &Point3f::new(165.0, 330.0, 165.0), metal_mat));
        box_t = Arc::new(RotateY::new(box_t, 15.0));
        box_t = Arc::new(Translate::new(box_t, &Vec3f::new(265.0, 0.0, 295.0)));
        box_t
    };
    objects.push(box_1);

    // right box
    let box_2 = {
        let mut box_t: Arc<dyn Hittable> = Arc::new(BoxHittable::new(&Point3f::new(0.0, 0.0, 0.0), &Point3f::new(165.0, 165.0, 165.0), white.clone()));
        box_t = Arc::new(RotateY::new(box_t, -18.0));
        box_t = Arc::new(Translate::new(box_t, &Vec3f::new(130.0, 0.0, 65.0)));
        box_t
    };
    objects.push(box_2);

    // glass sphere on right box
    let sphere_mat = Arc::new(DielectricMaterial { refractive_index: 1.5 });
    let glass_sphere = Arc::new(Sphere {
        centre: Point3f::new(212.5, 200.0, 147.5),
        radius: 35.0,
        material:sphere_mat,
    });
    objects.push(glass_sphere);

    // globe
    let earth_texture = Arc::new(ImageTexture::new_from_image_file(Path::new("resources/earthmap.jpg")));
    let earth_surface = Arc::new(LambertianMaterial::new(earth_texture));
    let globe: Arc<dyn Hittable> = Arc::new(Sphere {
        centre: Point3f::new(250.0, 70.0, 250.0),
        radius: 70.0,
        material: earth_surface,
    });
    objects.push(globe);

    // fog floor
    // let mut floor: Arc<dyn Hittable> = Arc::new(BoxHittable::new(&Point3f::new(0.0, 1.0, 0.0), &Point3f::new(555.0, 50.0, 555.0), white.clone()));
    // floor = Arc::new(RotateY::new(floor, 45.0));
    // objects.push(Arc::new(ConstantMedium::new_with_color(floor, 0.05, &ColorRgbF::new(0.8, 0.8, 0.8))));

    let cam_create_info = CameraCreateInfo {
        look_from: Point3f::new(278.0, 278.0, -800.0),
        look_at: Point3f::new(278.0, 278.0, 0.0),
        up: Point3f::new(0.0, 1.0, 0.0),
        vertical_fov_degrees: 40.0,
        aspect_ratio: camera_settings.aspect_ratio(),
        aperture: camera_settings.aperture,
        focus_distance: 10.0,
        time_0: 0.0,
        time_1: 1.0,
    };
    let camera = Camera::new(&cam_create_info);
    Scene {
        world: Arc::new(BvhNode::new(objects.as_slice(), 0.0, 1.0)),
        camera,
        background: ColorRgbF::ZERO,
    }
}

fn cornell_box(camera_settings: &CameraSettings) -> Scene {
    let mut objects: Vec<Arc<dyn Hittable>> = Vec::new();

    let red = Arc::new(LambertianMaterial::new_with_solid_color(&ColorRgbF::new(0.65, 0.05, 0.05)));
    let white = Arc::new(LambertianMaterial::new_with_solid_color(&ColorRgbF::new(0.73, 0.73, 0.73)));
    let green = Arc::new(LambertianMaterial::new_with_solid_color(&ColorRgbF::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new_solid_light(&ColorRgbF::new(15.0, 15.0, 15.0)));

    objects.push(Arc::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    objects.push(Arc::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    objects.push(Arc::new(XzRect::new(213.0, 343.0, 227.0, 332.0, 554.0, light)));
    objects.push(Arc::new(XzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, white.clone())));
    objects.push(Arc::new(XzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())));
    objects.push(Arc::new(XyRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())));

    let box_1 = {
        let mut box_t: Arc<dyn Hittable> = Arc::new(BoxHittable::new(&Point3f::new(0.0, 0.0, 0.0), &Point3f::new(165.0, 330.0, 165.0), white.clone()));
        box_t = Arc::new(RotateY::new(box_t, 15.0));
        box_t = Arc::new(Translate::new(box_t, &Vec3f::new(265.0, 0.0, 295.0)));
        box_t
    };
    objects.push(box_1);

    let box_2 = {
        let mut box_t: Arc<dyn Hittable> = Arc::new(BoxHittable::new(&Point3f::new(0.0, 0.0, 0.0), &Point3f::new(165.0, 165.0, 165.0), white.clone()));
        box_t = Arc::new(RotateY::new(box_t, -18.0));
        box_t = Arc::new(Translate::new(box_t, &Vec3f::new(130.0, 0.0, 65.0)));
        box_t
    };
    objects.push(box_2);

    let cam_create_info = CameraCreateInfo {
        look_from: Point3f::new(278.0, 278.0, -800.0),
        look_at: Point3f::new(278.0, 278.0, 0.0),
        up: Point3f::new(0.0, 1.0, 0.0),
        vertical_fov_degrees: 40.0,
        aspect_ratio: camera_settings.aspect_ratio(),
        aperture: camera_settings.aperture,
        focus_distance: 10.0,
        time_0: 0.0,
        time_1: 1.0,
    };
    let camera = Camera::new(&cam_create_info);
    Scene {
        world: Arc::new(BvhNode::new(objects.as_slice(), 0.0, 1.0)),
        camera,
        background: ColorRgbF::ZERO,
    }
}


fn cornell_smoke(camera_settings: &CameraSettings) -> Scene {
    let mut objects: Vec<Arc<dyn Hittable>> = Vec::new();

    let red = Arc::new(LambertianMaterial::new_with_solid_color(&ColorRgbF::new(0.65, 0.05, 0.05)));
    let white = Arc::new(LambertianMaterial::new_with_solid_color(&ColorRgbF::new(0.73, 0.73, 0.73)));
    let green = Arc::new(LambertianMaterial::new_with_solid_color(&ColorRgbF::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new_solid_light(&ColorRgbF::new(15.0, 15.0, 15.0)));

    objects.push(Arc::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    objects.push(Arc::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    objects.push(Arc::new(XzRect::new(213.0, 343.0, 227.0, 332.0, 554.0, light)));
    objects.push(Arc::new(XzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, white.clone())));
    objects.push(Arc::new(XzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())));
    objects.push(Arc::new(XyRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())));

    let box_1 = {
        let mut box_t: Arc<dyn Hittable> = Arc::new(BoxHittable::new(&Point3f::new(0.0, 0.0, 0.0), &Point3f::new(165.0, 330.0, 165.0), white.clone()));
        box_t = Arc::new(RotateY::new(box_t, 15.0));
        box_t = Arc::new(Translate::new(box_t, &Vec3f::new(265.0, 0.0, 295.0)));
        box_t
    };
    objects.push(Arc::new(ConstantMedium::new_with_color(box_1, 0.01, &ColorRgbF::new(0.0, 0.0, 0.0))));

    let box_2 = {
        let mut box_t: Arc<dyn Hittable> = Arc::new(BoxHittable::new(&Point3f::new(0.0, 0.0, 0.0), &Point3f::new(165.0, 165.0, 165.0), white.clone()));
        box_t = Arc::new(RotateY::new(box_t, -18.0));
        box_t = Arc::new(Translate::new(box_t, &Vec3f::new(130.0, 0.0, 65.0)));
        box_t
    };
    objects.push(Arc::new(ConstantMedium::new_with_color(box_2, 0.01, &ColorRgbF::new(1.0, 1.0, 1.0))));

    let cam_create_info = CameraCreateInfo {
        look_from: Point3f::new(278.0, 278.0, -800.0),
        look_at: Point3f::new(278.0, 278.0, 0.0),
        up: Point3f::new(0.0, 1.0, 0.0),
        vertical_fov_degrees: 40.0,
        aspect_ratio: camera_settings.aspect_ratio(),
        aperture: camera_settings.aperture,
        focus_distance: 10.0,
        time_0: 0.0,
        time_1: 1.0,
    };
    let camera = Camera::new(&cam_create_info);
    Scene {
        world: Arc::new(BvhNode::new(objects.as_slice(), 0.0, 1.0)),
        camera,
        background: ColorRgbF::ZERO,
    }
}

fn lights_demo_scene(camera_settings: &CameraSettings) -> Scene {
    let mut objects: Vec<Arc<dyn Hittable>> = Vec::new();
    let perlin_texture = Arc::new(NoiseTexture { scale: 4.0 });
    objects.push(Arc::new(Sphere {
        centre: Point3f::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Arc::new(LambertianMaterial::new(perlin_texture.clone())),
    }));
    objects.push(Arc::new(Sphere {
        centre: Point3f::new(0.0, 2.0, 0.0),
        radius: 2.0,
        material: Arc::new(LambertianMaterial::new(perlin_texture)),
    }));

    let diff_light = Arc::new(DiffuseLight::new_solid_light(&ColorRgbF::new(4.0, 4.0, 4.0)));
    objects.push(Arc::new(XyRect::new(3.0, 5.0, 1.0, 3.0, -2.0, diff_light)));

    let cam_create_info = CameraCreateInfo {
        look_from: Point3f::new(26.0, 3.0, 6.0),
        look_at: Point3f::new(0.0, 2.0, 0.0),
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
        world: Arc::new(BvhNode::new(objects.as_slice(), 0.0, 1.0)),
        camera,
        background: ColorRgbF::ZERO,
    }
}

fn globe_scene(camera_settings: &CameraSettings) -> Scene {
    let earth_texture = Arc::new(ImageTexture::new_from_image_file(Path::new("resources/earthmap.jpg")));
    let earth_surface = Arc::new(LambertianMaterial::new(earth_texture));
    let globe: Arc<dyn Hittable> = Arc::new(Sphere {
        centre: Point3f::ZERO,
        radius: 2.0,
        material: earth_surface,
    });

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
        world: Arc::new(BvhNode::new(slice::from_ref(&globe), 0.0, 1.0)),
        camera,
        background: DEFAULT_BACKGROUND,
    }
}

fn three_spheres_scene(camera_settings: &CameraSettings) -> Scene {
    let mut objects: Vec<Arc<dyn Hittable>> = Vec::new();

    let checker_texture = Arc::new(CheckerTexture::new(10.0, Arc::new(SolidTexture::new(0.2, 0.3, 0.1)), Arc::new(SolidTexture::new(0.9, 0.9, 0.9))));
    let ground_material = Arc::new(LambertianMaterial::new(checker_texture));
    let centre_material = Arc::new(LambertianMaterial::new_with_solid_color(&ColorRgbF::new(0.7, 0.3, 0.3)));
    let left_material = Arc::new(DielectricMaterial { refractive_index: 1.5 });
    let right_material = Arc::new(MetalMaterial { albedo: ColorRgbF::new(0.8, 0.6, 0.2), fuzz: 0.7});

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
        world: Arc::new(BvhNode::new(objects.as_slice(), 0.0, 1.0)),
        camera,
        background: DEFAULT_BACKGROUND,
    }
}

fn random_spheres_scene(camera_settings: &CameraSettings) -> Scene {
    let mut objects: Vec<Arc<dyn Hittable>> = Vec::new();
    let ground_material = Arc::new(LambertianMaterial {
        texture: Arc::new(NoiseTexture { scale: 4.0 })
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
        world: Arc::new(BvhNode::new(objects.as_slice(), 0.0, 1.0)),
        camera,
        background: DEFAULT_BACKGROUND,
    }
}
