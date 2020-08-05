use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use std::sync::mpsc;
use threadpool::ThreadPool;

pub use crate::camera::*;
pub use crate::objects::*;
pub use crate::scenes::*;

fn ray_color<T: Object>(
    ray: &Ray,
    world: &ObjectList,
    background: Vec3,
    lights: Option<&T>,
    depth: i32,
    rng: &mut SmallRng,
) -> Vec3 {
    if depth <= 0 {
        return Vec3::zero();
    }
    if let Some(rec) = world.hit(ray, 0.001, f64::MAX) {
        let emitted = rec.mat_ptr.emitted(ray, &rec, rec.u, rec.v, rec.p);
        match rec.mat_ptr.scatter(ray, &rec, rng) {
            Some(ScatterRecord::Diffuse { attenuation, pdf }) => {
                let (scattered, pdf_value) = if let Some(lights) = lights {
                    let lights_pdf = ObjectPDF::new(lights, rec.p);
                    let p = MixturePDF::new(&lights_pdf, &pdf);
                    let scattered = Ray::new(rec.p, p.generate(rng), ray.time);
                    let pdf_value = p.value(scattered.dir);
                    (scattered, pdf_value)
                } else {
                    let p = pdf;
                    let scattered = Ray::new(rec.p, p.generate(rng), ray.time);
                    let pdf_value = p.value(scattered.dir);
                    (scattered, pdf_value)
                };
                return emitted
                    + Vec3::elemul(
                        attenuation,
                        ray_color(&scattered, world, background, lights, depth - 1, rng),
                    ) * rec.mat_ptr.scattering_pdf(ray, &rec, &scattered)
                        / pdf_value;
            }
            Some(ScatterRecord::Specular {
                attenuation,
                specular_ray,
            }) => {
                return Vec3::elemul(
                    attenuation,
                    ray_color(&specular_ray, world, background, lights, depth - 1, rng),
                );
            }
            None => {
                return emitted;
            }
        }
    }
    background
}
pub struct ThreadResult {
    pub x: u32,
    pub color: Vec<[u8; 3]>,
}
pub fn run_ray_tracing() {
    println!("ray tracing");

    let is_ci = match std::env::var("CI") {
        Ok(x) => x == "true",
        Err(_) => false,
    };
    let (image_width, image_height, samples_per_pixel, thread_num) = if is_ci {
        (1600, 1600, 1024, 2)
    } else {
        (600, 600, 64, 16)
    };

    let (world, background, cam, lights) = cornell_box(image_width as f64 / image_height as f64);

    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);
    let pbar = ProgressBar::new(image_width as u64);

    let n_jobs = thread_num * 16;
    let (tx, rx) = mpsc::channel();
    let pool = ThreadPool::new(thread_num as usize);
    for i in 0..n_jobs {
        let start_x = image_width * i / n_jobs;
        let end_x = image_width * (i + 1) / n_jobs;
        let tx = tx.clone();
        let world = world.clone();
        let cam = cam.clone();
        let lights = lights.clone();
        let mut rng = SmallRng::from_entropy();
        pool.execute(move || {
            for x in start_x..end_x {
                let mut ans = ThreadResult { x, color: vec![] };
                for y in 0..image_height {
                    let mut color = Vec3::new(0.0, 0.0, 0.0);
                    for _ in 0..samples_per_pixel {
                        let u = (x as f64 + rng.gen::<f64>()) / (image_width as f64 - 1.0);
                        let v = (y as f64 + rng.gen::<f64>()) / (image_height as f64 - 1.0);
                        let ray = cam.get_ray(u, v, &mut rng);
                        color += ray_color::<ObjectList>(
                            &ray,
                            &world,
                            background,
                            Some(&lights),
                            50,
                            &mut rng,
                        );
                    }
                    color /= samples_per_pixel as f64;
                    ans.color.push([
                        (num::clamp(color.x.sqrt(), 0.0, 0.99999) * 256.0) as u8,
                        (num::clamp(color.y.sqrt(), 0.0, 0.99999) * 256.0) as u8,
                        (num::clamp(color.z.sqrt(), 0.0, 0.99999) * 256.0) as u8,
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
