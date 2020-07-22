use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use num::complex::Complex;

pub use crate::vec3::Vec3;

pub fn run_test() {
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
pub fn run_rgb() {
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
pub fn run_julia_set() {
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
