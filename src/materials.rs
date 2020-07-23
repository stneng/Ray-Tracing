pub use crate::objects::*;
pub use crate::ray::Ray;
pub use crate::vec3::Vec3;
use rand::Rng;

fn random_in_unit_sphere() -> Vec3 {
    loop {
        let t = Vec3::new(
            rand::random::<f64>(),
            rand::random::<f64>(),
            rand::random::<f64>(),
        ) * 2.0
            - Vec3::ones();
        if t.squared_length() <= 1.0 {
            return t;
        }
    }
}
fn random_unit_vector() -> Vec3 {
    let a = rand::thread_rng().gen_range(0.0, 2.0 * std::f64::consts::PI);
    let z = rand::thread_rng().gen_range(-1.0, 1.0);
    let r = ((1.0 - z * z) as f64).sqrt();
    Vec3::new(r * a.cos(), r * a.sin(), z)
}
fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n * 2.0 * (v * n)
}
fn refract(v: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = (-v * n).min(1.0);
    let r_out_perp = (v + n * cos_theta) * etai_over_etat;
    let r_out_parallel = -n * (1.0 - r_out_perp.squared_length()).abs().sqrt();
    r_out_perp + r_out_parallel
}
fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)) * ((1.0 - ref_idx) / (1.0 + ref_idx));
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

pub trait Material: Sync + Send {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> Option<(Vec3, Ray)>;
}

pub struct Lambertian {
    pub albedo: Vec3,
}
impl Material for Lambertian {
    fn scatter(&self, _r_in: Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let target = rec.p + rec.normal + random_unit_vector();
        Some((self.albedo, Ray::new(rec.p, target - rec.p)))
    }
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}
impl Material for Metal {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = reflect(r_in.dir.unit(), rec.normal);
        let scattered = Ray::new(rec.p, reflected + random_in_unit_sphere() * self.fuzz);
        if scattered.dir * rec.normal > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    pub ref_idx: f64,
}
impl Material for Dielectric {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let etai_over_etat;
        let real_normal;
        if r_in.dir * rec.normal > 0.0 {
            etai_over_etat = self.ref_idx;
            real_normal = -rec.normal;
        } else {
            etai_over_etat = 1.0 / self.ref_idx;
            real_normal = rec.normal;
        }
        let cos_theta = (-r_in.dir.unit() * real_normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        if etai_over_etat * sin_theta <= 1.0
            && rand::random::<f64>() > schlick(cos_theta, self.ref_idx)
        {
            let refracted = refract(r_in.dir.unit(), real_normal, etai_over_etat);
            return Some((Vec3::ones(), Ray::new(rec.p, refracted)));
        }
        Some((
            Vec3::ones(),
            Ray::new(rec.p, reflect(r_in.dir.unit(), rec.normal)),
        ))
    }
}
