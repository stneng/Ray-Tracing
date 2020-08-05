use rand::rngs::SmallRng;

pub use crate::objects::*;

pub struct Translate<T: Object> {
    pub object: T,
    pub offset: Vec3,
}
impl<T: Object> Translate<T> {
    pub fn new(object: T, offset: Vec3) -> Self {
        Self { object, offset }
    }
}
impl<T: Object> Object for Translate<T> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_ray = Ray::new(ray.ori - self.offset, ray.dir, ray.time);
        match self.object.hit(&moved_ray, t_min, t_max) {
            Some(rec) => Some(HitRecord {
                p: rec.p + self.offset,
                ..rec
            }),
            None => None,
        }
    }
    fn bounding_box(&self, t1: f64, t2: f64) -> Option<Aabb> {
        match self.object.bounding_box(t1, t2) {
            Some(rec) => Some(Aabb {
                min: rec.min + self.offset,
                max: rec.max + self.offset,
            }),
            None => None,
        }
    }
    fn pdf_value(&self, origin: Vec3, v: Vec3) -> f64 {
        self.object.pdf_value(origin - self.offset, v)
    }
    fn random(&self, origin: Vec3, rng: &mut SmallRng) -> Vec3 {
        self.object.random(origin - self.offset, rng) + self.offset
    }
}

pub struct RotateY<T: Object> {
    pub object: T,
    pub sin_theta: f64,
    pub cos_theta: f64,
    pub boxx: Option<Aabb>,
}
impl<T: Object> RotateY<T> {
    pub fn new(object: T, angle: f64) -> Self {
        let radians = angle * std::f64::consts::PI / 180.0;
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let boxx = match object.bounding_box(0.0, 1.0) {
            Some(boxx) => {
                let mut min = Vec3::new(f64::MAX, f64::MAX, f64::MAX);
                let mut max = Vec3::new(f64::MIN, f64::MIN, f64::MIN);
                for i in 0..2 {
                    for j in 0..2 {
                        for k in 0..2 {
                            let x = i as f64 * boxx.max.x + (1.0 - i as f64) * boxx.min.x;
                            let y = j as f64 * boxx.max.y + (1.0 - j as f64) * boxx.min.y;
                            let z = k as f64 * boxx.max.z + (1.0 - k as f64) * boxx.min.z;
                            let tester = Vec3::new(
                                cos_theta * x + sin_theta * z,
                                y,
                                -sin_theta * x + cos_theta * z,
                            );
                            min.x = min.x.min(tester.x);
                            min.y = min.y.min(tester.y);
                            min.z = min.z.min(tester.z);
                            max.x = max.x.max(tester.x);
                            max.y = max.y.max(tester.y);
                            max.z = max.z.max(tester.z);
                        }
                    }
                }
                Some(Aabb { min, max })
            }
            None => None,
        };
        Self {
            object,
            sin_theta,
            cos_theta,
            boxx,
        }
    }
}
impl<T: Object> Object for RotateY<T> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let rotated_ray = Ray::new(
            Vec3::new(
                self.cos_theta * ray.ori.x - self.sin_theta * ray.ori.z,
                ray.ori.y,
                self.sin_theta * ray.ori.x + self.cos_theta * ray.ori.z,
            ),
            Vec3::new(
                self.cos_theta * ray.dir.x - self.sin_theta * ray.dir.z,
                ray.dir.y,
                self.sin_theta * ray.dir.x + self.cos_theta * ray.dir.z,
            ),
            ray.time,
        );
        match self.object.hit(&rotated_ray, t_min, t_max) {
            Some(rec) => Some(HitRecord {
                p: Vec3::new(
                    self.cos_theta * rec.p.x + self.sin_theta * rec.p.z,
                    rec.p.y,
                    -self.sin_theta * rec.p.x + self.cos_theta * rec.p.z,
                ),
                normal: Vec3::new(
                    self.cos_theta * rec.normal.x + self.sin_theta * rec.normal.z,
                    rec.normal.y,
                    -self.sin_theta * rec.normal.x + self.cos_theta * rec.normal.z,
                ),
                ..rec
            }),
            None => None,
        }
    }
    fn bounding_box(&self, _t1: f64, _t2: f64) -> Option<Aabb> {
        self.boxx.clone()
    }
    fn pdf_value(&self, origin: Vec3, v: Vec3) -> f64 {
        let rotated_origin = Vec3::new(
            self.cos_theta * origin.x - self.sin_theta * origin.z,
            origin.y,
            self.sin_theta * origin.x + self.cos_theta * origin.z,
        );
        let rotated_v = Vec3::new(
            self.cos_theta * v.x - self.sin_theta * v.z,
            v.y,
            self.sin_theta * v.x + self.cos_theta * v.z,
        );
        self.object.pdf_value(rotated_origin, rotated_v)
    }
    fn random(&self, origin: Vec3, rng: &mut SmallRng) -> Vec3 {
        let rotated_origin = Vec3::new(
            self.cos_theta * origin.x - self.sin_theta * origin.z,
            origin.y,
            self.sin_theta * origin.x + self.cos_theta * origin.z,
        );
        let rec = self.object.random(rotated_origin, rng);
        Vec3::new(
            self.cos_theta * rec.x + self.sin_theta * rec.z,
            rec.y,
            -self.sin_theta * rec.x + self.cos_theta * rec.z,
        )
    }
}
