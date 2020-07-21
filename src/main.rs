#[allow(clippy::float_cmp)]
mod vec3;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use num::complex::Complex;

pub use vec3::Vec3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray {
    pub ori: Vec3,
    pub dir: Vec3,
}
impl Ray {
    pub fn new(ori: Vec3, dir: Vec3) -> Self {
        Self { ori, dir }
    }
    pub fn at(self, t: f64) -> Vec3 {
        self.ori + self.dir * t
    }
}

fn run_test() {
    let x = Vec3::new(1.0, 1.0, 1.0);
    println!("{:?}", x);

    let mut img: RgbImage = ImageBuffer::new(1024, 512);
    let pbar = ProgressBar::new(1024);

    for x in 0..1024 {
        for y in 0..512 {
            let pixel = img.get_pixel_mut(x, y);
            let color = (x / 4) as u8;
            *pixel = image::Rgb([color, color, color]);
        }
        pbar.inc(1);
    }

    img.save("output/test.png").unwrap();
    pbar.finish();
}
fn run_rgb() {
    println!("rgb");

    let mut img: RgbImage = ImageBuffer::new(256, 256);
    let pbar = ProgressBar::new(256);

    for x in 0..256 {
        for y in 0..256 {
            let pixel = img.get_pixel_mut(x, y);
            *pixel = image::Rgb([x as u8, y as u8, 63]);
        }
        pbar.inc(1);
    }

    img.save("output/rgb.png").unwrap();
    pbar.finish();
}
fn run_julia_set() {
    println!("julia set");

    let mut img: RgbImage = ImageBuffer::new(4000, 4000);
    let pbar = ProgressBar::new(4000);

    for x in 0..4000 {
        for y in 0..4000 {
            let pixel = img.get_pixel_mut(x, y);
            let mut z = Complex::new((x as f64 - 2000.0) / 1000.0, (y as f64 - 2000.0) / 1000.0);
            let c = Complex::new(-0.70176, -0.3842);
            let mut i = 0;
            while i < 70 && z.norm_sqr() < 2.0 * 2.0 {
                z = z * z + c;
                i += 1;
            }
            let t = i as f64 / 70.0;
            let color = Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t;
            *pixel = image::Rgb([
                (color.x * 255.99999) as u8,
                (color.y * 255.99999) as u8,
                (color.z * 255.99999) as u8,
            ]);
        }
        pbar.inc(1);
    }

    img.save("output/julia_set.png").unwrap();
    pbar.finish();
}
fn ray_color(r: Ray) -> Vec3 {
    let t = (r.dir.unit().y + 1.0) / 2.0;
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}
fn run_part1() {
    println!("part1");

    let viewport_height = 2.0;
    let viewport_width = viewport_height / 16.0 * 9.0;
    let focal_length = 1.0;
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_height, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_width, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    let mut img: RgbImage = ImageBuffer::new(400, 225);
    let pbar = ProgressBar::new(400);

    for x in 0..400 {
        for y in 0..225 {
            let pixel = img.get_pixel_mut(x, 224 - y);
            let r = Ray::new(
                origin,
                lower_left_corner + horizontal * (x as f64 / 399.0) + vertical * (y as f64 / 224.0)
                    - origin,
            );
            let color = ray_color(r);
            *pixel = image::Rgb([
                (color.x * 255.99999) as u8,
                (color.y * 255.99999) as u8,
                (color.z * 255.99999) as u8,
            ])
        }
        pbar.inc(1);
    }

    img.save("output/part1.png").unwrap();
    pbar.finish();
}
fn main() {
    run_test();
    run_rgb();
    run_julia_set();
    run_part1();
}
