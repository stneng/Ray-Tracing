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
