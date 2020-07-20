#[allow(clippy::float_cmp)]
mod vec3;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;

pub use vec3::Vec3;

fn main() {
    let x = Vec3::new(1.0, 1.0, 1.0);
    println!("{:?}", x);

    let mut img: RgbImage = ImageBuffer::new(256, 256);
    let pbar = ProgressBar::new(256);

    for x in 0..256 {
        for y in 0..256 {
            let pixel = img.get_pixel_mut(x, y);
            *pixel = image::Rgb([x as u8, y as u8, 63]);
        }
        pbar.inc(1);
    }

    img.save("output/test.png").unwrap();
    pbar.finish();
}
