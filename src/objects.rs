use std::sync::Arc;

pub use crate::materials::*;
pub use crate::ray::Ray;
pub use crate::vec3::Vec3;

#[derive(Clone)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub mat_ptr: Arc<dyn Material>,
}
pub trait Object {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
pub struct ObjectList {
    pub objects: Vec<Box<dyn Object>>,
}
impl ObjectList {
    pub fn add(&mut self, object: Box<dyn Object>) {
        self.objects.push(object);
    }
}
impl Object for ObjectList {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut ans: Option<HitRecord> = None;
        let mut closest = t_max;
        for x in self.objects.iter() {
            if let Some(tmp) = x.hit(ray, t_min, closest) {
                ans = Some(tmp.clone());
                closest = tmp.t;
            }
        }
        ans
    }
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Arc<dyn Material>,
}
impl Object for Sphere {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.ori - self.center;
        let a = ray.dir * ray.dir;
        let b = oc * ray.dir * 2.0;
        let c = oc * oc - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant > 0.0 {
            let mut ans = (-b - discriminant.sqrt()) / (2.0 * a);
            if ans > t_min && ans < t_max {
                return Some(HitRecord {
                    t: ans,
                    p: ray.at(ans),
                    normal: (ray.at(ans) - self.center) / self.radius,
                    mat_ptr: self.material.clone(),
                });
            }
            ans = (-b + discriminant.sqrt()) / (2.0 * a);
            if ans > t_min && ans < t_max {
                return Some(HitRecord {
                    t: ans,
                    p: ray.at(ans),
                    normal: (ray.at(ans) - self.center) / self.radius,
                    mat_ptr: self.material.clone(),
                });
            }
        }
        None
    }
}
