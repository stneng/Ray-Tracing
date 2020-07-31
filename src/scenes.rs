use rand::Rng;
use std::sync::Arc;

pub use crate::bvh::*;
pub use crate::camera::Camera;
pub use crate::objects::*;
pub use crate::ray::Ray;
pub use crate::vec3::Vec3;

pub fn random_scene() -> (ObjectList, Vec3, Camera) {
    let mut world = ObjectList { objects: vec![] };
    world.add(Arc::new(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Arc::new(Lambertian {
            albedo: Arc::new(CheckerTexture {
                odd: Arc::new(SolidColor {
                    color: Vec3::new(0.2, 0.3, 0.1),
                }),
                even: Arc::new(SolidColor {
                    color: Vec3::new(0.9, 0.9, 0.9),
                }),
            }),
        }),
    }));
    let mut box1 = ObjectList { objects: vec![] };
    for a in -11..11 {
        for b in -11..11 {
            let center = Vec3::new(
                a as f64 + 0.9 * rand::random::<f64>(),
                0.2,
                b as f64 + 0.9 * rand::random::<f64>(),
            );
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let rd = rand::random::<f64>();
                if rd < 0.8 {
                    box1.add(Arc::new(MovingSphere {
                        center1: center,
                        center2: center
                            + Vec3::new(0.0, rand::thread_rng().gen_range(0.0, 0.5), 0.0),
                        t1: 0.0,
                        t2: 1.0,
                        radius: 0.2,
                        material: Arc::new(Lambertian {
                            albedo: Arc::new(SolidColor {
                                color: Vec3::elemul(Vec3::random(0.0, 1.0), Vec3::random(0.0, 1.0)),
                            }),
                        }),
                    }));
                } else if rd < 0.95 {
                    box1.add(Arc::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Arc::new(Metal {
                            albedo: Vec3::random(0.0, 1.0),
                            fuzz: rand::thread_rng().gen_range(0.0, 0.5),
                        }),
                    }));
                } else {
                    box1.add(Arc::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Arc::new(Dielectric { ref_idx: 1.5 }),
                    }));
                }
            }
        }
    }
    let len = box1.objects.len();
    world.add(Arc::new(BvhNode::new(&mut box1.objects, 0, len, 0.0, 1.0)));
    world.add(Arc::new(Sphere {
        center: Vec3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Lambertian {
            albedo: Arc::new(SolidColor {
                color: Vec3::new(0.4, 0.2, 0.1),
            }),
        }),
    }));
    world.add(Arc::new(Sphere {
        center: Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Metal {
            albedo: Vec3::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        }),
    }));
    world.add(Arc::new(Sphere {
        center: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Dielectric { ref_idx: 1.5 }),
    }));
    (
        world,
        Vec3::new(0.7, 0.8, 1.0),
        Camera::new(
            Vec3::new(13.0, 2.0, 3.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            20.0,
            16.0 / 9.0,
            0.1,
            10.0,
            0.0,
            1.0,
        ),
    )
}
pub fn random_scene_light() -> (ObjectList, Vec3, Camera) {
    let mut world = ObjectList { objects: vec![] };
    world.add(Arc::new(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Arc::new(Lambertian {
            albedo: Arc::new(CheckerTexture {
                odd: Arc::new(SolidColor {
                    color: Vec3::new(0.2, 0.3, 0.1),
                }),
                even: Arc::new(SolidColor {
                    color: Vec3::new(0.9, 0.9, 0.9),
                }),
            }),
        }),
    }));
    let mut box1 = ObjectList { objects: vec![] };
    for a in -11..11 {
        for b in -11..11 {
            let radius = rand::thread_rng().gen_range(0.08, 0.25);
            let center = Vec3::new(
                (a as f64 + 0.9 * rand::random::<f64>()) / 2.0,
                radius,
                (b as f64 + 0.9 * rand::random::<f64>()) / 2.0,
            );

            if (center - Vec3::new(-0.2, radius, -0.2)).length() > 1.3 + radius {
                let rd = rand::random::<f64>();
                if rd < 0.2 {
                    box1.add(Arc::new(Sphere {
                        center,
                        radius,
                        material: Arc::new(Lambertian {
                            albedo: Arc::new(SolidColor {
                                color: Vec3::random(0.1, 0.9),
                            }),
                        }),
                    }));
                } else if rd < 0.4 {
                    box1.add(Arc::new(Sphere {
                        center,
                        radius,
                        material: Arc::new(Metal {
                            albedo: Vec3::random(0.0, 1.0),
                            fuzz: rand::thread_rng().gen_range(0.0, 0.5),
                        }),
                    }));
                } else if rd < 0.6 {
                    box1.add(Arc::new(Sphere {
                        center,
                        radius,
                        material: Arc::new(Dielectric {
                            ref_idx: rand::thread_rng().gen_range(1.5, 2.0),
                        }),
                    }));
                } else if rd < 0.8 {
                    box1.add(Arc::new(Sphere {
                        center,
                        radius: radius * 0.9,
                        material: Arc::new(DiffuseLight {
                            emit: Arc::new(SolidColor {
                                color: Vec3::random(0.1, 0.9),
                            }),
                        }),
                    }));
                    box1.add(Arc::new(Sphere {
                        center,
                        radius,
                        material: Arc::new(Dielectric { ref_idx: 1.5 }),
                    }));
                } else {
                    box1.add(Arc::new(Sphere {
                        center,
                        radius: radius * 0.5,
                        material: Arc::new(DiffuseLight {
                            emit: Arc::new(SolidColor {
                                color: Vec3::random(0.1, 0.9),
                            }),
                        }),
                    }));
                    box1.add(Arc::new(Sphere {
                        center,
                        radius,
                        material: Arc::new(FrostedDielectric {
                            ref_idx: 1.5,
                            fuzz: rand::thread_rng().gen_range(0.3, 0.5),
                        }),
                    }));
                }
            }
        }
    }
    let len = box1.objects.len();
    world.add(Arc::new(BvhNode::new(&mut box1.objects, 0, len, 0.0, 1.0)));
    world.add(Arc::new(Sphere {
        center: Vec3::new(0.0, 0.8, 0.0),
        radius: 0.8,
        material: Arc::new(DiffuseLight {
            emit: Arc::new(CheckerTexture {
                odd: Arc::new(SolidColor {
                    color: Vec3::new(1.0, 0.6, 0.4),
                }),
                even: Arc::new(SolidColor {
                    color: Vec3::new(1.0, 0.75, 0.3),
                }),
            }),
        }),
    }));
    world.add(Arc::new(Sphere {
        center: Vec3::new(1.3, 0.5, 0.0),
        radius: 0.5,
        material: Arc::new(Dielectric { ref_idx: 1.5 }),
    }));
    world.add(Arc::new(Sphere {
        center: Vec3::new(-1.3, 0.5, 0.0),
        radius: 0.5,
        material: Arc::new(Metal {
            albedo: Vec3::new(0.8, 0.6, 0.3),
            fuzz: 0.0,
        }),
    }));
    (
        world,
        Vec3::zero(),
        Camera::new(
            Vec3::new(6.0, 3.0, 6.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            40.0,
            16.0 / 9.0,
            0.1,
            9.0,
            0.0,
            1.0,
        ),
    )
}

pub fn text_scene_light() -> (ObjectList, Vec3, Camera) {
    let mut world = ObjectList { objects: vec![] };
    world.add(Arc::new(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Arc::new(FrostedDielectric {
            ref_idx: 1.5,
            fuzz: 0.1,
        }),
    }));
    world.add(Arc::new(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: -999.5,
        material: Arc::new(FrostedDielectric {
            ref_idx: 1.5,
            fuzz: 0.1,
        }),
    }));
    let mut box1 = ObjectList { objects: vec![] };

    use image::{ImageBuffer, RgbaImage};
    let mut image: RgbaImage = ImageBuffer::new(200, 30);
    render_text(&mut image, "RAYTRACING", "EncodeSansExpanded-Medium.ttf", 24.0, image::Rgba([76, 167, 235, 255]));
    let scale = 20.0;

    for i in 0..image.width() {
        for j in 0..image.height() {
            let pixel = image.get_pixel(i, j);
            if pixel[3] > 0 {
                let center = Vec3::new((i as f64 - 65.0) / scale, (25.0 - j as f64) / scale, 2.0)
                    + random_in_unit_sphere() / scale / 2.0;
                let radius = 0.45 / scale;
                box1.add(Arc::new(Sphere {
                    center,
                    radius,
                    material: Arc::new(Dielectric { ref_idx: 1.5 }),
                }));
                box1.add(Arc::new(Sphere {
                    center,
                    radius: radius * 0.9,
                    material: Arc::new(DiffuseLight {
                        emit: Arc::new(SolidColor {
                            color: Vec3::new(
                                pixel[0] as f64 / 256.0,
                                pixel[1] as f64 / 256.0,
                                pixel[2] as f64 / 256.0,
                            ),
                        }),
                    }),
                }));
            }
        }
    }

    let mut image: RgbaImage = ImageBuffer::new(50, 50);
    render_text(&mut image, "♥", "Arimo-Bold.ttf", 50.0, image::Rgba([239, 130, 127, 255]));
    let scale = 10.0;

    for i in 0..image.width() {
        for j in 0..image.height() {
            let pixel = image.get_pixel(i, j);
            if pixel[3] > 0 {
                let center = Vec3::new((i as f64 - 10.0) / scale, (30.0 - j as f64) / scale, 0.0)
                    + random_in_unit_sphere() / scale / 2.0;
                let radius = 0.45 / scale;
                box1.add(Arc::new(Sphere {
                    center,
                    radius,
                    material: Arc::new(Dielectric { ref_idx: 1.5 }),
                }));
                box1.add(Arc::new(Sphere {
                    center,
                    radius: radius * 0.9,
                    material: Arc::new(DiffuseLight {
                        emit: Arc::new(SolidColor {
                            color: Vec3::new(
                                pixel[0] as f64 / 256.0,
                                pixel[1] as f64 / 256.0,
                                pixel[2] as f64 / 256.0,
                            ),
                        }),
                    }),
                }));
            }
        }
    }

    let len = box1.objects.len();
    world.add(Arc::new(BvhNode::new(&mut box1.objects, 0, len, 0.0, 1.0)));
    world.add(Arc::new(Sphere {
        center: Vec3::new(1.0, -4.0, -1.0),
        radius: 2.0,
        material: Arc::new(DiffuseLight {
            emit: Arc::new(SolidColor {
                color: Vec3::new(1.0, 0.6, 0.4),
            }),
        }),
    }));
    world.add(Arc::new(Sphere {
        center: Vec3::new(-1.0, -4.0, 1.0),
        radius: 2.0,
        material: Arc::new(DiffuseLight {
            emit: Arc::new(SolidColor {
                color: Vec3::new(1.0, 0.6, 0.4),
            }),
        }),
    }));
    world.add(Arc::new(Sphere {
        center: Vec3::new(1.0, -4.0, 1.0),
        radius: 2.0,
        material: Arc::new(DiffuseLight {
            emit: Arc::new(SolidColor {
                color: Vec3::new(1.0, 0.75, 0.3),
            }),
        }),
    }));
    world.add(Arc::new(Sphere {
        center: Vec3::new(-1.0, -4.0, -1.0),
        radius: 2.0,
        material: Arc::new(DiffuseLight {
            emit: Arc::new(SolidColor {
                color: Vec3::new(1.0, 0.75, 0.3),
            }),
        }),
    }));
    world.add(Arc::new(Sphere {
        center: Vec3::new(2.3, 0.5, 0.0),
        radius: 0.5,
        material: Arc::new(Dielectric { ref_idx: 1.5 }),
    }));
    world.add(Arc::new(Sphere {
        center: Vec3::new(0.0, 1.1, -3.0),
        radius: 1.1,
        material: Arc::new(Dielectric { ref_idx: 1.5 }),
    }));
    world.add(Arc::new(Sphere {
        center: Vec3::new(0.0, 1.1, -3.0),
        radius: 1.0,
        material: Arc::new(DiffuseLight {
            emit: Arc::new(SolidColor {
                color: Vec3::ones(),
            }),
        }),
    }));
    world.add(Arc::new(Sphere {
        center: Vec3::new(-2.3, 0.5, 0.0),
        radius: 0.5,
        material: Arc::new(Metal {
            albedo: Vec3::new(0.8, 0.6, 0.3),
            fuzz: 0.0,
        }),
    }));
    (
        world,
        Vec3::zero(),
        Camera::new(
            Vec3::new(2.0, 3.0, 6.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            40.0,
            16.0 / 9.0,
            0.1,
            9.0,
            0.0,
            1.0,
        ),
    )
}

fn is_ci() -> bool {
    option_env!("CI").unwrap_or_default() == "true"
}

fn render_text(image: &mut image::RgbaImage, msg: &str, font: &str, size: f32, color: image::Rgba<u8>) {
    use image::Rgba;
    use rusttype::Font;

    let font_file = if is_ci() {
        font.to_owned()
    } else {
        format!("output/{}", font)
    };
    let font_path = std::env::current_dir().unwrap().join(font_file);
    let data = std::fs::read(&font_path).unwrap();
    let font: Font = Font::try_from_vec(data).unwrap_or_else(|| {
        panic!(format!(
            "error constructing a Font from data at {:?}",
            font_path
        ));
    });

    imageproc::drawing::draw_text_mut(
        image,
        color,
        0,
        0,
        rusttype::Scale::uniform(size),
        &font,
        msg,
    );
}
