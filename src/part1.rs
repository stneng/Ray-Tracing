use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use std::sync::Arc;

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
pub fn run_part1() {
    println!("part1");

    let mut world = ObjectList { objects: vec![] };
    world.add(Box::new(Sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
        material: Arc::new(Lambertian {
            albedo: Vec3::new(0.1, 0.2, 0.5),
        }),
    }));
    world.add(Box::new(Sphere {
        center: Vec3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        material: Arc::new(Lambertian {
            albedo: Vec3::new(0.8, 0.8, 0.0),
        }),
    }));
    world.add(Box::new(Sphere {
        center: Vec3::new(-1.0, 0.0, -1.0),
        radius: 0.5,
        material: Arc::new(Dielectric { ref_idx: 1.5 }),
    }));
    world.add(Box::new(Sphere {
        center: Vec3::new(-1.0, 0.0, -1.0),
        radius: -0.45,
        material: Arc::new(Dielectric { ref_idx: 1.5 }),
    }));
    world.add(Box::new(Sphere {
        center: Vec3::new(1.0, 0.0, -1.0),
        radius: 0.5,
        material: Arc::new(Metal {
            albedo: Vec3::new(0.8, 0.6, 0.2),
            fuzz: 0.0,
        }),
    }));

    let cam = Camera::new(
        Vec3::new(3.0, 3.0, 2.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        16.0 / 9.0,
        2.0,
        (Vec3::new(0.0, 0.0, -1.0) - Vec3::new(3.0, 3.0, 2.0)).length(),
    );

    let image_width = 400;
    let image_height = 225;
    let samples_per_pixel = 100;
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

    img.save("output/part1.png").unwrap();
    pbar.finish();
}
