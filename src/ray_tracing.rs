use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use rand::Rng;
use std::sync::Arc;

pub use crate::bvh::*;
pub use crate::camera::Camera;
pub use crate::objects::*;
pub use crate::ray::Ray;
pub use crate::vec3::Vec3;

fn ray_color(ray: Ray, world: &ObjectList, depth: i32) -> Vec3 {
    if let Some(rec) = world.hit(ray, 0.001, 233333333333.0) {
        if depth < 50 {
            if let Some((attenuation, scattered)) = rec.mat_ptr.scatter(ray, &rec) {
                return Vec3::elemul(attenuation, ray_color(scattered, world, depth + 1));
            }
        }
        return Vec3::zero();
    }
    let t = (ray.dir.unit().y + 1.0) / 2.0;
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}
fn random_scene() -> ObjectList {
    let mut world = ObjectList { objects: vec![] };
    // ground
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
    world
}
pub fn run_ray_tracing() {
    println!("ray tracing");

    let world = random_scene();

    let cam = Camera::new(
        Vec3::new(13.0, 2.0, 3.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        16.0 / 9.0,
        0.1,
        10.0,
        0.0,
        1.0,
    );

    let image_width = 1600;
    let image_height = 900;
    let samples_per_pixel = 256;
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);
    let pbar = ProgressBar::new(image_width as u64);
    for x in 0..image_width {
        for y in 0..image_height {
            let pixel = img.get_pixel_mut(x, image_height - 1 - y);
            let mut color = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (x as f64 + rand::random::<f64>()) / (image_width as f64 - 1.0);
                let v = (y as f64 + rand::random::<f64>()) / (image_height as f64 - 1.0);
                let ray = cam.get_ray(u, v);
                color += ray_color(ray, &world, 0);
            }
            color /= samples_per_pixel as f64;
            *pixel = image::Rgb([
                (color.x.sqrt() * 255.99999) as u8,
                (color.y.sqrt() * 255.99999) as u8,
                (color.z.sqrt() * 255.99999) as u8,
            ])
        }
        pbar.inc(1);
    }

    img.save("output/ray_tracing.png").unwrap();
    pbar.finish();
}
