use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;

pub use crate::bvh::*;
pub use crate::camera::Camera;
pub use crate::objects::*;
pub use crate::ray::Ray;
pub use crate::scenes::*;
pub use crate::vec3::Vec3;

fn ray_color(ray: Ray, world: &ObjectList, background: Vec3, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3::zero();
    }
    if let Some(rec) = world.hit(ray, 0.001, 233333333333.0) {
        let emitted = rec.mat_ptr.emitted(rec.u, rec.v, rec.p);
        if let Some((attenuation, scattered)) = rec.mat_ptr.scatter(ray, &rec) {
            return emitted
                + Vec3::elemul(
                    attenuation,
                    ray_color(scattered, world, background, depth - 1),
                );
        }
        return emitted;
    }
    background
}

pub fn run_ray_tracing() {
    println!("ray tracing");

    let (world, background, cam) = random_scene();

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
                color += ray_color(ray, &world, background, 50);
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
