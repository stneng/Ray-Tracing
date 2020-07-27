use rand::Rng;
use std::cmp::Ordering;
use std::sync::Arc;

pub use crate::objects::*;
pub use crate::ray::Ray;
pub use crate::vec3::Vec3;

#[derive(Clone, Copy)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}
impl Aabb {
    pub fn surrounding_box(box1: Aabb, box2: Aabb) -> Aabb {
        Aabb {
            min: Vec3::new(
                box1.min.x.min(box2.min.x),
                box1.min.y.min(box2.min.y),
                box1.min.z.min(box2.min.z),
            ),
            max: Vec3::new(
                box1.max.x.max(box2.max.x),
                box1.max.y.max(box2.max.y),
                box1.max.z.max(box2.max.z),
            ),
        }
    }
    pub fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> bool {
        {
            let mut t1 = (self.min.x - ray.ori.x) / ray.dir.x;
            let mut t2 = (self.max.x - ray.ori.x) / ray.dir.x;
            if t1 > t2 {
                std::mem::swap(&mut t1, &mut t2)
            }
            if t_max.min(t2) <= t_min.max(t1) {
                return false;
            }
        }
        {
            let mut t1 = (self.min.y - ray.ori.y) / ray.dir.y;
            let mut t2 = (self.max.y - ray.ori.y) / ray.dir.y;
            if t1 > t2 {
                std::mem::swap(&mut t1, &mut t2)
            }
            if t_max.min(t2) <= t_min.max(t1) {
                return false;
            }
        }
        {
            let mut t1 = (self.min.z - ray.ori.z) / ray.dir.z;
            let mut t2 = (self.max.z - ray.ori.z) / ray.dir.z;
            if t1 > t2 {
                std::mem::swap(&mut t1, &mut t2)
            }
            if t_max.min(t2) <= t_min.max(t1) {
                return false;
            }
        }
        true
    }
}

pub struct BvhNode {
    pub left: Arc<dyn Object>,
    pub right: Arc<dyn Object>,
    pub boxx: Aabb,
}
impl BvhNode {
    pub fn new(
        objects: &mut Vec<Arc<dyn Object>>,
        start: usize,
        end: usize,
        t1: f64,
        t2: f64,
    ) -> BvhNode {
        let left;
        let right;
        let boxx;
        let axis = rand::thread_rng().gen_range(0, 3);
        match axis {
            0 => objects[start..end].sort_by(|a, b| box_x_compare(a, b)),
            1 => objects[start..end].sort_by(|a, b| box_y_compare(a, b)),
            2 => objects[start..end].sort_by(|a, b| box_z_compare(a, b)),
            _ => panic!("axis error"),
        }
        let len = end - start;
        if len == 1 {
            left = objects[start].clone();
            right = objects[start].clone();
        } else if len == 2 {
            left = objects[start].clone();
            right = objects[start + 1].clone();
        } else {
            let mid = start + len / 2;
            left = Arc::new(BvhNode::new(objects, start, mid, t1, t2));
            right = Arc::new(BvhNode::new(objects, mid, end, t1, t2));
        }
        if let Some(box_left) = left.bounding_box(t1, t2) {
            if let Some(box_right) = right.bounding_box(t1, t2) {
                boxx = Aabb::surrounding_box(box_left, box_right);
                return Self { left, right, boxx };
            }
        }
        panic!("No bounding box in BvhNode::new.");
    }
}
impl Object for BvhNode {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.boxx.hit(ray, t_min, t_max) {
            return None;
        }
        if let Some(tmp) = self.left.hit(ray, t_min, t_max) {
            if let Some(tmp) = self.right.hit(ray, t_min, tmp.t) {
                return Some(tmp);
            } else {
                return Some(tmp);
            }
        }
        if let Some(tmp) = self.right.hit(ray, t_min, t_max) {
            return Some(tmp);
        }
        None
    }
    fn bounding_box(&self, _t1: f64, _t2: f64) -> Option<Aabb> {
        Some(self.boxx)
    }
}

fn box_x_compare(a: &Arc<dyn Object>, b: &Arc<dyn Object>) -> Ordering {
    if let Some(box_left) = a.bounding_box(0.0, 0.0) {
        if let Some(box_right) = b.bounding_box(0.0, 0.0) {
            if let Some(cmp) = box_left.min.x.partial_cmp(&box_right.min.x) {
                return cmp;
            }
        }
    }
    panic!("No bounding box in BvhNode::new.");
}
fn box_y_compare(a: &Arc<dyn Object>, b: &Arc<dyn Object>) -> Ordering {
    if let Some(box_left) = a.bounding_box(0.0, 0.0) {
        if let Some(box_right) = b.bounding_box(0.0, 0.0) {
            if let Some(cmp) = box_left.min.y.partial_cmp(&box_right.min.y) {
                return cmp;
            }
        }
    }
    panic!("No bounding box in BvhNode::new.");
}
fn box_z_compare(a: &Arc<dyn Object>, b: &Arc<dyn Object>) -> Ordering {
    if let Some(box_left) = a.bounding_box(0.0, 0.0) {
        if let Some(box_right) = b.bounding_box(0.0, 0.0) {
            if let Some(cmp) = box_left.min.z.partial_cmp(&box_right.min.z) {
                return cmp;
            }
        }
    }
    panic!("No bounding box in BvhNode::new.");
}
