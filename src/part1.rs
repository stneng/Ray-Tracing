use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;

pub use crate::objects::*;
pub use crate::ray::Ray;
pub use crate::vec3::Vec3;

fn ray_color(ray: Ray, world: &ObjectList) -> Vec3 {
    if let Some(tmp) = world.hit(ray, 0.0, 233333333333.0) {
        return (tmp.normal + Vec3::new(1.0, 1.0, 1.0)) * 0.5;
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
    }));
    world.add(Box::new(Sphere {
        center: Vec3::new(0.0, -100.5, -1.0),
        radius: 100.0,
    }));

    let viewport_height = 2.0;
    let viewport_width = viewport_height * 16.0 / 9.0;
    let focal_length = 1.0;
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
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
            let color = ray_color(r, &world);
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
