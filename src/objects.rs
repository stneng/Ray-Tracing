use std::sync::Arc;

pub use crate::bvh::*;
pub use crate::materials::*;
pub use crate::ray::Ray;
pub use crate::vec3::Vec3;

#[derive(Clone)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub mat_ptr: Arc<dyn Material>,
    pub u: f64,
    pub v: f64,
}
pub trait Object: Sync + Send {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, t1: f64, t2: f64) -> Option<Aabb>;
}
pub struct ObjectList {
    pub objects: Vec<Arc<dyn Object>>,
}
impl ObjectList {
    pub fn add(&mut self, object: Arc<dyn Object>) {
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
    fn bounding_box(&self, t1: f64, t2: f64) -> Option<Aabb> {
        if self.objects.is_empty() {
            return None;
        }
        let mut ans;
        if let Some(tmp) = self.objects[0].bounding_box(t1, t2) {
            ans = tmp;
        } else {
            return None;
        }
        for i in 1..self.objects.len() {
            if let Some(tmp) = self.objects[i].bounding_box(t1, t2) {
                ans = Aabb::surrounding_box(ans, tmp);
            } else {
                return None;
            }
        }
        Some(ans)
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
            let (texture_u, texture_v) = get_sphere_uv((ray.at(ans) - self.center) / self.radius);
            if ans > t_min && ans < t_max {
                return Some(HitRecord {
                    t: ans,
                    p: ray.at(ans),
                    normal: (ray.at(ans) - self.center) / self.radius,
                    mat_ptr: self.material.clone(),
                    u: texture_u,
                    v: texture_v,
                });
            }
            ans = (-b + discriminant.sqrt()) / (2.0 * a);
            if ans > t_min && ans < t_max {
                return Some(HitRecord {
                    t: ans,
                    p: ray.at(ans),
                    normal: (ray.at(ans) - self.center) / self.radius,
                    mat_ptr: self.material.clone(),
                    u: texture_u,
                    v: texture_v,
                });
            }
        }
        None
    }
    fn bounding_box(&self, _t1: f64, _t2: f64) -> Option<Aabb> {
        Some(Aabb {
            min: self.center - Vec3::new(self.radius, self.radius, self.radius),
            max: self.center + Vec3::new(self.radius, self.radius, self.radius),
        })
    }
}

pub struct MovingSphere {
    pub center1: Vec3,
    pub center2: Vec3,
    pub t1: f64,
    pub t2: f64,
    pub radius: f64,
    pub material: Arc<dyn Material>,
}
impl MovingSphere {
    pub fn get_center(&self, t: f64) -> Vec3 {
        self.center1 + (self.center2 - self.center1) * ((t - self.t1) / (self.t2 - self.t1))
    }
}
impl Object for MovingSphere {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.ori - self.get_center(ray.time);
        let a = ray.dir * ray.dir;
        let b = oc * ray.dir * 2.0;
        let c = oc * oc - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant > 0.0 {
            let mut ans = (-b - discriminant.sqrt()) / (2.0 * a);
            let (texture_u, texture_v) =
                get_sphere_uv((ray.at(ans) - self.get_center(ray.time)) / self.radius);
            if ans > t_min && ans < t_max {
                return Some(HitRecord {
                    t: ans,
                    p: ray.at(ans),
                    normal: (ray.at(ans) - self.get_center(ray.time)) / self.radius,
                    mat_ptr: self.material.clone(),
                    u: texture_u,
                    v: texture_v,
                });
            }
            ans = (-b + discriminant.sqrt()) / (2.0 * a);
            if ans > t_min && ans < t_max {
                return Some(HitRecord {
                    t: ans,
                    p: ray.at(ans),
                    normal: (ray.at(ans) - self.get_center(ray.time)) / self.radius,
                    mat_ptr: self.material.clone(),
                    u: texture_u,
                    v: texture_v,
                });
            }
        }
        None
    }
    fn bounding_box(&self, t1: f64, t2: f64) -> Option<Aabb> {
        Some(Aabb::surrounding_box(
            Aabb {
                min: self.get_center(t1) - Vec3::new(self.radius, self.radius, self.radius),
                max: self.get_center(t1) + Vec3::new(self.radius, self.radius, self.radius),
            },
            Aabb {
                min: self.get_center(t2) - Vec3::new(self.radius, self.radius, self.radius),
                max: self.get_center(t2) + Vec3::new(self.radius, self.radius, self.radius),
            },
        ))
    }
}

fn get_sphere_uv(p: Vec3) -> (f64, f64) {
    let phi = p.z.atan2(p.x);
    let theta = p.y.asin();
    (
        1.0 - (phi + std::f64::consts::PI) / (2.0 * std::f64::consts::PI),
        (theta + std::f64::consts::PI / 2.0) / std::f64::consts::PI,
    )
}
