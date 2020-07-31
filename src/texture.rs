use image::RgbImage;
use rand::seq::SliceRandom;
use std::sync::Arc;

pub use crate::objects::*;
pub use crate::ray::Ray;
pub use crate::vec3::*;

pub trait Texture: Sync + Send {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3;
}

pub struct SolidColor {
    pub color: Vec3,
}
impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: Vec3) -> Vec3 {
        self.color
    }
}

pub struct CheckerTextureUV {
    pub odd: Arc<dyn Texture>,
    pub even: Arc<dyn Texture>,
}
impl Texture for CheckerTextureUV {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        if (u / 0.01) as i32 % 2 == (v / 0.01) as i32 % 2 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

pub struct CheckerTexture {
    pub odd: Arc<dyn Texture>,
    pub even: Arc<dyn Texture>,
}
impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

pub struct NoiseTexture {
    pub noise: Perlin,
    pub scale: f64,
}
impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: Vec3) -> Vec3 {
        Vec3::ones() * 0.5 * (1.0 + (self.scale * p.z + 10.0 * self.noise.turb(p, 7)).sin())
    }
}

pub struct ImageTexture {
    pub img: RgbImage,
}
impl ImageTexture {
    pub fn new(filename: &str) -> Self {
        Self {
            img: image::open(filename)
                .expect("ImageTexture:failed to open file")
                .to_rgb(),
        }
    }
}
impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: Vec3) -> Vec3 {
        let u = num::clamp(u, 0.0, 1.0);
        let v = num::clamp(v, 0.0, 1.0);
        let x = num::clamp(
            (u * self.img.width() as f64) as u32,
            0,
            self.img.width() - 1,
        );
        let y = num::clamp(
            (v * self.img.height() as f64) as u32,
            0,
            self.img.height() - 1,
        );
        let color_scale = 1.0 / 255.0;
        let pixel = self.img.get_pixel(x, self.img.height() - 1 - y);
        Vec3::new(
            pixel[0] as f64 * color_scale,
            pixel[1] as f64 * color_scale,
            pixel[2] as f64 * color_scale,
        )
    }
}

pub struct Perlin {
    pub perm_x: Vec<i32>,
    pub perm_y: Vec<i32>,
    pub perm_z: Vec<i32>,
    pub ranvec: Vec<Vec3>,
}
impl Perlin {
    pub fn new() -> Self {
        Self {
            perm_x: Self::perlin_generate_perm(),
            perm_y: Self::perlin_generate_perm(),
            perm_z: Self::perlin_generate_perm(),
            ranvec: Self::perlin_generate_vec(),
        }
    }
    pub fn perlin_generate_perm() -> Vec<i32> {
        let mut ans = vec![];
        for i in 0..256 {
            ans.push(i);
        }
        ans.shuffle(&mut rand::thread_rng());
        ans
    }
    pub fn perlin_generate_vec() -> Vec<Vec3> {
        let mut ans = vec![];
        for _ in 0..256 {
            ans.push(Vec3::random(-1.0, 1.0).unit());
        }
        ans
    }
    #[allow(clippy::many_single_char_names)]
    pub fn noise(&self, p: Vec3) -> f64 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();
        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let c = self.ranvec[(self.perm_x[((i + di) & 255) as usize]
                        ^ self.perm_y[((j + dj) & 255) as usize]
                        ^ self.perm_z[((k + dk) & 255) as usize])
                        as usize];
                    accum += (di as f64 * uu + (1.0 - di as f64) * (1.0 - uu))
                        * (dj as f64 * vv + (1.0 - dj as f64) * (1.0 - vv))
                        * (dk as f64 * ww + (1.0 - dk as f64) * (1.0 - ww))
                        * (c * Vec3::new(u - di as f64, v - dj as f64, w - dk as f64));
                }
            }
        }
        accum
    }
    pub fn turb(&self, p: Vec3, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = p;
        let mut weight = 1.0;
        for _ in 0..depth {
            accum += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }
        accum.abs()
    }
}
