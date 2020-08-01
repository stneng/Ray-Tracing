pub use crate::vec3::Vec3;

pub struct Ray {
    pub ori: Vec3,
    pub dir: Vec3,
    pub time: f32,
}
impl Ray {
    pub fn new(ori: Vec3, dir: Vec3, time: f32) -> Self {
        Self { ori, dir, time }
    }
    pub fn at(&self, t: f32) -> Vec3 {
        self.ori + self.dir * t
    }
}
