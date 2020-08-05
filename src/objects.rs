use rand::{rngs::SmallRng, Rng};

pub use crate::bvh::*;
pub use crate::materials::*;
pub use crate::pdf::*;
pub use crate::ray::*;
pub use crate::transforms::*;
pub use crate::vec3::*;

#[derive(Clone)]
pub struct HitRecord<'a> {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub mat_ptr: &'a dyn Material,
    pub u: f64,
    pub v: f64,
}
pub trait Object: Sync + Send {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, t1: f64, t2: f64) -> Option<Aabb>;
    fn pdf_value(&self, _origin: Vec3, _v: Vec3) -> f64 {
        0.0
    }
    fn random(&self, _origin: Vec3, _rng: &mut SmallRng) -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }
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
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
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
    fn pdf_value(&self, origin: Vec3, v: Vec3) -> f64 {
        let weight = 1.0 / self.objects.len() as f64;
        let mut ans = 0.0;
        for x in self.objects.iter() {
            ans += weight * x.pdf_value(origin, v);
        }
        ans
    }
    fn random(&self, origin: Vec3, rng: &mut SmallRng) -> Vec3 {
        self.objects[rng.gen_range(0, self.objects.len())].random(origin, rng)
    }
}

pub struct Sphere<T: Material> {
    pub center: Vec3,
    pub radius: f64,
    pub material: T,
}
impl<T: Material> Object for Sphere<T> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.ori - self.center;
        let a = ray.dir * ray.dir;
        let half_b = oc * ray.dir;
        let c = oc * oc - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant > 0.0 {
            let ans = (-half_b - discriminant.sqrt()) / a;
            let (texture_u, texture_v) = get_sphere_uv((ray.at(ans) - self.center) / self.radius);
            if ans > t_min && ans < t_max {
                return Some(HitRecord {
                    t: ans,
                    p: ray.at(ans),
                    normal: (ray.at(ans) - self.center) / self.radius,
                    mat_ptr: &self.material,
                    u: texture_u,
                    v: texture_v,
                });
            }
            let ans = (-half_b + discriminant.sqrt()) / a;
            if ans > t_min && ans < t_max {
                return Some(HitRecord {
                    t: ans,
                    p: ray.at(ans),
                    normal: (ray.at(ans) - self.center) / self.radius,
                    mat_ptr: &self.material,
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
    fn pdf_value(&self, origin: Vec3, v: Vec3) -> f64 {
        match self.hit(&Ray::new(origin, v, 0.0), 0.001, f64::MAX) {
            Some(_rec) => {
                let cos_theta_max = (1.0
                    - self.radius * self.radius / (self.center - origin).squared_length())
                .sqrt();
                let solid_angle = 2.0 * std::f64::consts::PI * (1.0 - cos_theta_max);
                1.0 / solid_angle
            }
            None => 0.0,
        }
    }
    fn random(&self, origin: Vec3, rng: &mut SmallRng) -> Vec3 {
        let direction = self.center - origin;
        let distance_squared = direction.squared_length();
        let uvw = ONB::build_from_w(direction);
        uvw.local(random_to_sphere(self.radius, distance_squared, rng))
    }
}

pub struct MovingSphere<T: Material> {
    pub center1: Vec3,
    pub center2: Vec3,
    pub t1: f64,
    pub t2: f64,
    pub radius: f64,
    pub material: T,
}
impl<T: Material> MovingSphere<T> {
    pub fn get_center(&self, t: f64) -> Vec3 {
        self.center1 + (self.center2 - self.center1) * ((t - self.t1) / (self.t2 - self.t1))
    }
}
impl<T: Material> Object for MovingSphere<T> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.ori - self.get_center(ray.time);
        let a = ray.dir * ray.dir;
        let half_b = oc * ray.dir;
        let c = oc * oc - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant > 0.0 {
            let ans = (-half_b - discriminant.sqrt()) / a;
            let (texture_u, texture_v) =
                get_sphere_uv((ray.at(ans) - self.get_center(ray.time)) / self.radius);
            if ans > t_min && ans < t_max {
                return Some(HitRecord {
                    t: ans,
                    p: ray.at(ans),
                    normal: (ray.at(ans) - self.get_center(ray.time)) / self.radius,
                    mat_ptr: &self.material,
                    u: texture_u,
                    v: texture_v,
                });
            }
            let ans = (-half_b + discriminant.sqrt()) / a;
            if ans > t_min && ans < t_max {
                return Some(HitRecord {
                    t: ans,
                    p: ray.at(ans),
                    normal: (ray.at(ans) - self.get_center(ray.time)) / self.radius,
                    mat_ptr: &self.material,
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

pub struct RectXY<T: Material> {
    pub x1: f64,
    pub x2: f64,
    pub y1: f64,
    pub y2: f64,
    pub k: f64,
    pub face: f64,
    pub material: T,
}
impl<T: Material> Object for RectXY<T> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - ray.ori.z) / ray.dir.z;
        if t >= t_min && t <= t_max {
            let x = ray.ori.x + t * ray.dir.x;
            let y = ray.ori.y + t * ray.dir.y;
            if x >= self.x1 && x <= self.x2 && y >= self.y1 && y <= self.y2 {
                return Some(HitRecord {
                    t,
                    p: ray.at(t),
                    normal: Vec3::new(0.0, 0.0, 1.0) * self.face,
                    mat_ptr: &self.material,
                    u: (x - self.x1) / (self.x2 - self.x1),
                    v: (y - self.y1) / (self.y2 - self.y1),
                });
            }
        }
        None
    }
    fn bounding_box(&self, _t1: f64, _t2: f64) -> Option<Aabb> {
        Some(Aabb {
            min: Vec3::new(self.x1, self.y1, self.k - 0.0001),
            max: Vec3::new(self.x2, self.y2, self.k + 0.0001),
        })
    }
    fn pdf_value(&self, origin: Vec3, v: Vec3) -> f64 {
        match self.hit(&Ray::new(origin, v, 0.0), 0.001, f64::MAX) {
            Some(rec) => {
                let area = (self.x2 - self.x1) * (self.y2 - self.y1);
                let distance_squared = rec.t * rec.t * v.squared_length();
                let cosine = (v * rec.normal).abs() / v.length();
                distance_squared / (cosine * area)
            }
            None => 0.0,
        }
    }
    fn random(&self, origin: Vec3, rng: &mut SmallRng) -> Vec3 {
        let random_point = Vec3::new(
            rng.gen_range(self.x1, self.x2),
            rng.gen_range(self.y1, self.y2),
            self.k,
        );
        random_point - origin
    }
}
pub struct RectXZ<T: Material> {
    pub x1: f64,
    pub x2: f64,
    pub z1: f64,
    pub z2: f64,
    pub k: f64,
    pub face: f64,
    pub material: T,
}
impl<T: Material> Object for RectXZ<T> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - ray.ori.y) / ray.dir.y;
        if t >= t_min && t <= t_max {
            let x = ray.ori.x + t * ray.dir.x;
            let z = ray.ori.z + t * ray.dir.z;
            if x >= self.x1 && x <= self.x2 && z >= self.z1 && z <= self.z2 {
                return Some(HitRecord {
                    t,
                    p: ray.at(t),
                    normal: Vec3::new(0.0, 1.0, 0.0) * self.face,
                    mat_ptr: &self.material,
                    u: (x - self.x1) / (self.x2 - self.x1),
                    v: (z - self.z1) / (self.z2 - self.z1),
                });
            }
        }
        None
    }
    fn bounding_box(&self, _t1: f64, _t2: f64) -> Option<Aabb> {
        Some(Aabb {
            min: Vec3::new(self.x1, self.k - 0.0001, self.z1),
            max: Vec3::new(self.x2, self.k + 0.0001, self.z2),
        })
    }
    fn pdf_value(&self, origin: Vec3, v: Vec3) -> f64 {
        match self.hit(&Ray::new(origin, v, 0.0), 0.001, f64::MAX) {
            Some(rec) => {
                let area = (self.x2 - self.x1) * (self.z2 - self.z1);
                let distance_squared = rec.t * rec.t * v.squared_length();
                let cosine = (v * rec.normal).abs() / v.length();
                distance_squared / (cosine * area)
            }
            None => 0.0,
        }
    }
    fn random(&self, origin: Vec3, rng: &mut SmallRng) -> Vec3 {
        let random_point = Vec3::new(
            rng.gen_range(self.x1, self.x2),
            self.k,
            rng.gen_range(self.z1, self.z2),
        );
        random_point - origin
    }
}
pub struct RectYZ<T: Material> {
    pub y1: f64,
    pub y2: f64,
    pub z1: f64,
    pub z2: f64,
    pub k: f64,
    pub face: f64,
    pub material: T,
}
impl<T: Material> Object for RectYZ<T> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - ray.ori.x) / ray.dir.x;
        if t >= t_min && t <= t_max {
            let y = ray.ori.y + t * ray.dir.y;
            let z = ray.ori.z + t * ray.dir.z;
            if y >= self.y1 && y <= self.y2 && z >= self.z1 && z <= self.z2 {
                return Some(HitRecord {
                    t,
                    p: ray.at(t),
                    normal: Vec3::new(1.0, 0.0, 0.0) * self.face,
                    mat_ptr: &self.material,
                    u: (y - self.y1) / (self.y2 - self.y1),
                    v: (z - self.z1) / (self.z2 - self.z1),
                });
            }
        }
        None
    }
    fn bounding_box(&self, _t1: f64, _t2: f64) -> Option<Aabb> {
        Some(Aabb {
            min: Vec3::new(self.k - 0.0001, self.y1, self.z1),
            max: Vec3::new(self.k + 0.0001, self.y2, self.z2),
        })
    }
    fn pdf_value(&self, origin: Vec3, v: Vec3) -> f64 {
        match self.hit(&Ray::new(origin, v, 0.0), 0.001, f64::MAX) {
            Some(rec) => {
                let area = (self.y2 - self.y1) * (self.z2 - self.z1);
                let distance_squared = rec.t * rec.t * v.squared_length();
                let cosine = (v * rec.normal).abs() / v.length();
                distance_squared / (cosine * area)
            }
            None => 0.0,
        }
    }
    fn random(&self, origin: Vec3, rng: &mut SmallRng) -> Vec3 {
        let random_point = Vec3::new(
            self.k,
            rng.gen_range(self.y1, self.y2),
            rng.gen_range(self.z1, self.z2),
        );
        random_point - origin
    }
}

#[allow(clippy::type_complexity)]
pub struct Cuboid<T: Material + Clone> {
    pub box_min: Vec3,
    pub box_max: Vec3,
    pub sides: (
        RectXY<T>,
        RectXY<T>,
        RectXZ<T>,
        RectXZ<T>,
        RectYZ<T>,
        RectYZ<T>,
    ),
}
impl<T: Material + Clone> Cuboid<T> {
    pub fn new(box_min: Vec3, box_max: Vec3, material: T) -> Self {
        Self {
            box_min,
            box_max,
            sides: (
                RectXY {
                    x1: box_min.x,
                    x2: box_max.x,
                    y1: box_min.y,
                    y2: box_max.y,
                    k: box_min.z,
                    face: -1.0,
                    material: material.clone(),
                },
                RectXY {
                    x1: box_min.x,
                    x2: box_max.x,
                    y1: box_min.y,
                    y2: box_max.y,
                    k: box_max.z,
                    face: 1.0,
                    material: material.clone(),
                },
                RectXZ {
                    x1: box_min.x,
                    x2: box_max.x,
                    z1: box_min.z,
                    z2: box_max.z,
                    k: box_min.y,
                    face: -1.0,
                    material: material.clone(),
                },
                RectXZ {
                    x1: box_min.x,
                    x2: box_max.x,
                    z1: box_min.z,
                    z2: box_max.z,
                    k: box_max.y,
                    face: 1.0,
                    material: material.clone(),
                },
                RectYZ {
                    y1: box_min.y,
                    y2: box_max.y,
                    z1: box_min.z,
                    z2: box_max.z,
                    k: box_min.x,
                    face: -1.0,
                    material: material.clone(),
                },
                RectYZ {
                    y1: box_min.y,
                    y2: box_max.y,
                    z1: box_min.z,
                    z2: box_max.z,
                    k: box_max.x,
                    face: 1.0,
                    material,
                },
            ),
        }
    }
}
impl<T: Material + Clone> Object for Cuboid<T> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut ans: Option<HitRecord> = None;
        let mut closest = t_max;
        if let Some(tmp) = self.sides.0.hit(ray, t_min, closest) {
            ans = Some(tmp.clone());
            closest = tmp.t;
        }
        if let Some(tmp) = self.sides.1.hit(ray, t_min, closest) {
            ans = Some(tmp.clone());
            closest = tmp.t;
        }
        if let Some(tmp) = self.sides.2.hit(ray, t_min, closest) {
            ans = Some(tmp.clone());
            closest = tmp.t;
        }
        if let Some(tmp) = self.sides.3.hit(ray, t_min, closest) {
            ans = Some(tmp.clone());
            closest = tmp.t;
        }
        if let Some(tmp) = self.sides.4.hit(ray, t_min, closest) {
            ans = Some(tmp.clone());
            closest = tmp.t;
        }
        if let Some(tmp) = self.sides.5.hit(ray, t_min, closest) {
            ans = Some(tmp.clone());
        }
        ans
    }
    fn bounding_box(&self, _t1: f64, _t2: f64) -> Option<Aabb> {
        Some(Aabb {
            min: self.box_min,
            max: self.box_max,
        })
    }
    fn pdf_value(&self, origin: Vec3, v: Vec3) -> f64 {
        let weight = 1.0 / 6.0;
        weight * self.sides.0.pdf_value(origin, v)
            + weight * self.sides.1.pdf_value(origin, v)
            + weight * self.sides.2.pdf_value(origin, v)
            + weight * self.sides.3.pdf_value(origin, v)
            + weight * self.sides.4.pdf_value(origin, v)
            + weight * self.sides.5.pdf_value(origin, v)
    }
    fn random(&self, origin: Vec3, rng: &mut SmallRng) -> Vec3 {
        match rng.gen_range(0, 6) {
            0 => self.sides.0.random(origin, rng),
            1 => self.sides.1.random(origin, rng),
            2 => self.sides.2.random(origin, rng),
            3 => self.sides.3.random(origin, rng),
            4 => self.sides.4.random(origin, rng),
            5 => self.sides.5.random(origin, rng),
            _ => Vec3::new(1.0, 0.0, 0.0),
        }
    }
}

pub struct ConstantMedium<T1: Object, T2: Material> {
    pub boundary: T1,
    pub phase_function: T2,
    pub density: f64,
}
impl<T1: Object, T2: Texture> ConstantMedium<T1, Isotropic<T2>> {
    pub fn new(boundary: T1, texture: T2, density: f64) -> Self {
        Self {
            boundary,
            phase_function: Isotropic { albedo: texture },
            density,
        }
    }
}
impl<T1: Object, T2: Material> Object for ConstantMedium<T1, T2> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if let Some(mut rec1) = self.boundary.hit(ray, f64::MIN, f64::MAX) {
            if let Some(mut rec2) = self.boundary.hit(ray, rec1.t + 0.0001, f64::MAX) {
                if rec1.t < t_min {
                    rec1.t = t_min;
                }
                if rec2.t > t_max {
                    rec2.t = t_max;
                }
                if rec1.t < rec2.t {
                    let distance_inside_boundary = (rec2.t - rec1.t) * ray.dir.length();
                    let hit_distance = -(1.0 / self.density) * rand::random::<f64>().ln();
                    if hit_distance <= distance_inside_boundary {
                        let t = rec1.t + hit_distance / ray.dir.length();
                        return Some(HitRecord {
                            t,
                            p: ray.at(t),
                            normal: Vec3::new(1.0, 0.0, 0.0),
                            mat_ptr: &self.phase_function,
                            u: 0.0,
                            v: 0.0,
                        });
                    }
                }
            }
        }
        None
    }
    fn bounding_box(&self, t1: f64, t2: f64) -> Option<Aabb> {
        self.boundary.bounding_box(t1, t2)
    }
}
