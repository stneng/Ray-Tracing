use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use std::sync::mpsc;
use std::thread;

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
pub struct ThreadResult {
    pub x: u32,
    pub color: Vec<[u8; 3]>,
}
pub fn run_ray_tracing() {
    println!("ray tracing");

    let (world, background, cam) = random_scene_light();

    let is_ci = match std::env::var("CI") {
        Ok(x) => x == "true",
        Err(_) => false,
    };
    let (image_width, image_height, samples_per_pixel, thread_num) = if is_ci {
        (1600, 900, 256, 2)
    } else {
        (400, 225, 64, 16)
    };

    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);
    let pbar = ProgressBar::new(image_width as u64);

    let x_per_thread = image_width / thread_num;
    let (tx, rx) = mpsc::channel();
    for i in 0..thread_num {
        let start_x = i * x_per_thread;
        let end_x = std::cmp::min((i + 1) * x_per_thread, image_width);
        let tx = tx.clone();
        let world = world.clone();
        let cam = cam.clone();
        thread::spawn(move || {
            for x in start_x..end_x {
                let mut ans = ThreadResult { x, color: vec![] };
                for y in 0..image_height {
                    let mut color = Vec3::new(0.0, 0.0, 0.0);
                    for _ in 0..samples_per_pixel {
                        let u = (x as f64 + rand::random::<f64>()) / (image_width as f64 - 1.0);
                        let v = (y as f64 + rand::random::<f64>()) / (image_height as f64 - 1.0);
                        let ray = cam.get_ray(u, v);
                        color += ray_color(ray, &world, background, 50);
                    }
                    color /= samples_per_pixel as f64;
                    ans.color.push([
                        (color.x.sqrt() * 255.99999) as u8,
                        (color.y.sqrt() * 255.99999) as u8,
                        (color.z.sqrt() * 255.99999) as u8,
                    ])
                }
                tx.send(ans).expect("failed to send result");
            }
        });
    }
    std::mem::drop(tx);
    for received in rx {
        let x = received.x;
        for y in 0..image_height {
            let pixel = img.get_pixel_mut(x, image_height - 1 - y);
            *pixel = image::Rgb(received.color[y as usize]);
        }
        pbar.inc(1);
    }

    img.save("output/ray_tracing.png").unwrap();
    pbar.finish();
}
