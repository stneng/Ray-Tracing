use rand::Rng;
use std::cmp::Ordering;

pub use crate::objects::*;

#[derive(Clone)]
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
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
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

pub struct Bvh {
    pub root: Box<dyn Object>,
}
impl Bvh {
    pub fn new(objects: &mut Vec<Box<dyn Object>>, t1: f64, t2: f64) -> Self {
        Self {
            root: BvhNode::build(objects, t1, t2),
        }
    }
}
impl Object for Bvh {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.root.hit(ray, t_min, t_max)
    }
    fn bounding_box(&self, t1: f64, t2: f64) -> Option<Aabb> {
        self.root.bounding_box(t1, t2)
    }
}
pub struct BvhNode {
    pub left: Box<dyn Object>,
    pub right: Box<dyn Object>,
    pub boxx: Aabb,
}
impl BvhNode {
    pub fn build(objects: &mut Vec<Box<dyn Object>>, t1: f64, t2: f64) -> Box<dyn Object> {
        let left;
        let right;
        let boxx;
        let axis = rand::thread_rng().gen_range(0, 3);
        match axis {
            0 => objects.sort_by(|a, b| box_x_compare(&**a, &**b)),
            1 => objects.sort_by(|a, b| box_y_compare(&**a, &**b)),
            2 => objects.sort_by(|a, b| box_z_compare(&**a, &**b)),
            _ => panic!("axis error"),
        }
        let len = objects.len();
        if len == 1 {
            return objects.remove(0);
        } else {
            let mut objects2 = objects.split_off(objects.len() / 2);
            left = BvhNode::build(objects, t1, t2);
            right = BvhNode::build(&mut objects2, t1, t2);
        }
        if let Some(box_left) = left.bounding_box(t1, t2) {
            if let Some(box_right) = right.bounding_box(t1, t2) {
                boxx = Aabb::surrounding_box(box_left, box_right);
                return Box::new(Self { left, right, boxx });
            }
        }
        panic!("No bounding box in BvhNode::build.");
    }
}
impl Object for BvhNode {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
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
        Some(self.boxx.clone())
    }
}

pub struct BvhNodeStatic<T1: Object, T2: Object> {
    pub left: Box<T1>,
    pub right: Box<T2>,
    pub boxx: Aabb,
}
impl<T1: Object, T2: Object> BvhNodeStatic<T1, T2> {
    pub fn new(left: Box<T1>, right: Box<T2>, t1: f64, t2: f64) -> Self {
        if let Some(box_left) = left.bounding_box(t1, t2) {
            if let Some(box_right) = right.bounding_box(t1, t2) {
                let boxx = Aabb::surrounding_box(box_left, box_right);
                return Self { left, right, boxx };
            }
        }
        panic!("No bounding box in BvhNodeStatic::new.");
    }
}
impl<T1: Object, T2: Object> Object for BvhNodeStatic<T1, T2> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
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
        Some(self.boxx.clone())
    }
}

fn box_x_compare(a: &dyn Object, b: &dyn Object) -> Ordering {
    if let Some(box_left) = a.bounding_box(0.0, 0.0) {
        if let Some(box_right) = b.bounding_box(0.0, 0.0) {
            if let Some(cmp) = box_left.min.x.partial_cmp(&box_right.min.x) {
                return cmp;
            }
        }
    }
    panic!("No bounding box in BvhNode::build.");
}
fn box_y_compare(a: &dyn Object, b: &dyn Object) -> Ordering {
    if let Some(box_left) = a.bounding_box(0.0, 0.0) {
        if let Some(box_right) = b.bounding_box(0.0, 0.0) {
            if let Some(cmp) = box_left.min.y.partial_cmp(&box_right.min.y) {
                return cmp;
            }
        }
    }
    panic!("No bounding box in BvhNode::build.");
}
fn box_z_compare(a: &dyn Object, b: &dyn Object) -> Ordering {
    if let Some(box_left) = a.bounding_box(0.0, 0.0) {
        if let Some(box_right) = b.bounding_box(0.0, 0.0) {
            if let Some(cmp) = box_left.min.z.partial_cmp(&box_right.min.z) {
                return cmp;
            }
        }
    }
    panic!("No bounding box in BvhNode::build.");
}
