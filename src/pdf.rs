use rand::{rngs::SmallRng, Rng};

pub use crate::objects::*;

pub struct ONB {
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
}
impl ONB {
    pub fn build_from_w(w: Vec3) -> Self {
        let w = w.unit();
        let a = if w.x.abs() > 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };
        let v = Vec3::cross(w, a).unit();
        let u = Vec3::cross(w, v).unit();
        Self { u, v, w }
    }
    pub fn local(&self, a: Vec3) -> Vec3 {
        self.u * a.x + self.v * a.y + self.w * a.z
    }
}

pub trait PDF: Sync + Send {
    fn value(&self, direction: Vec3) -> f64;
    fn generate(&self, rng: &mut SmallRng) -> Vec3;
}

pub struct CosinePDF {
    pub uvw: ONB,
}
impl CosinePDF {
    pub fn new(w: Vec3) -> Self {
        Self {
            uvw: ONB::build_from_w(w),
        }
    }
}
impl PDF for CosinePDF {
    fn value(&self, direction: Vec3) -> f64 {
        let cosine = direction.unit() * self.uvw.w;
        if cosine > 0.0 {
            cosine / std::f64::consts::PI
        } else {
            0.0
        }
    }
    fn generate(&self, rng: &mut SmallRng) -> Vec3 {
        self.uvw.local(random_cosine_direction(rng))
    }
}

pub struct ObjectPDF<'a, T: Object> {
    pub object: &'a T,
    pub origin: Vec3,
}
impl<'a, T: Object> ObjectPDF<'a, T> {
    pub fn new(object: &'a T, origin: Vec3) -> Self {
        Self { object, origin }
    }
}
impl<'a, T: Object> PDF for ObjectPDF<'a, T> {
    fn value(&self, direction: Vec3) -> f64 {
        self.object.pdf_value(self.origin, direction)
    }
    fn generate(&self, rng: &mut SmallRng) -> Vec3 {
        self.object.random(self.origin, rng)
    }
}

pub struct MixturePDF<'a, T1: PDF, T2: PDF> {
    pub p1: &'a T1,
    pub p2: &'a T2,
}
impl<'a, T1: PDF, T2: PDF> MixturePDF<'a, T1, T2> {
    pub fn new(p1: &'a T1, p2: &'a T2) -> Self {
        Self { p1, p2 }
    }
}
impl<'a, T1: PDF, T2: PDF> PDF for MixturePDF<'a, T1, T2> {
    fn value(&self, direction: Vec3) -> f64 {
        0.5 * self.p1.value(direction) + 0.5 * self.p2.value(direction)
    }
    fn generate(&self, rng: &mut SmallRng) -> Vec3 {
        if rng.gen::<f64>() < 0.5 {
            self.p1.generate(rng)
        } else {
            self.p2.generate(rng)
        }
    }
}
