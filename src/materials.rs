use rand::{rngs::SmallRng, Rng};

pub use crate::objects::*;
pub use crate::texture::*;

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)) * ((1.0 - ref_idx) / (1.0 + ref_idx));
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

pub trait Material: Sync + Send {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, rng: &mut SmallRng) -> Option<(Vec3, Ray)>;
    fn emitted(&self, _r_in: &Ray, _rec: &HitRecord, _u: f64, _v: f64, _p: Vec3) -> Vec3 {
        Vec3::zero()
    }
}

#[derive(Clone)]
pub struct Lambertian<T: Texture> {
    pub albedo: T,
}
impl<T: Texture> Material for Lambertian<T> {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, rng: &mut SmallRng) -> Option<(Vec3, Ray)> {
        let target = rec.p + rec.normal + random_unit_vector(rng);
        Some((
            self.albedo.value(rec.u, rec.v, rec.p),
            Ray::new(rec.p, target - rec.p, r_in.time),
        ))
    }
}

#[derive(Clone)]
pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}
impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, rng: &mut SmallRng) -> Option<(Vec3, Ray)> {
        let reflected = reflect(r_in.dir.unit(), rec.normal);
        let scattered = Ray::new(
            rec.p,
            reflected + random_in_unit_sphere(rng) * self.fuzz,
            r_in.time,
        );
        if scattered.dir * rec.normal > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct Dielectric {
    pub ref_idx: f64,
}
impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, rng: &mut SmallRng) -> Option<(Vec3, Ray)> {
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
        if etai_over_etat * sin_theta <= 1.0 && rng.gen::<f64>() > schlick(cos_theta, self.ref_idx)
        {
            let refracted = refract(r_in.dir.unit(), real_normal, etai_over_etat);
            return Some((Vec3::ones(), Ray::new(rec.p, refracted, r_in.time)));
        }
        Some((
            Vec3::ones(),
            Ray::new(rec.p, reflect(r_in.dir.unit(), rec.normal), r_in.time),
        ))
    }
}

#[derive(Clone)]
pub struct FrostedDielectric {
    pub ref_idx: f64,
    pub fuzz: f64,
}
impl Material for FrostedDielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, rng: &mut SmallRng) -> Option<(Vec3, Ray)> {
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
        if etai_over_etat * sin_theta <= 1.0 && rng.gen::<f64>() > schlick(cos_theta, self.ref_idx)
        {
            let refracted = refract(r_in.dir.unit(), real_normal, etai_over_etat);
            return Some((
                Vec3::ones(),
                Ray::new(
                    rec.p,
                    refracted + random_in_unit_sphere(rng) * self.fuzz,
                    r_in.time,
                ),
            ));
        }
        Some((
            Vec3::ones(),
            Ray::new(rec.p, reflect(r_in.dir.unit(), rec.normal), r_in.time),
        ))
    }
}

#[derive(Clone)]
pub struct DiffuseLight<T: Texture> {
    pub emit: T,
}
impl<T: Texture> Material for DiffuseLight<T> {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord, _rng: &mut SmallRng) -> Option<(Vec3, Ray)> {
        None
    }
    fn emitted(&self, r_in: &Ray, rec: &HitRecord, u: f64, v: f64, p: Vec3) -> Vec3 {
        if r_in.dir * rec.normal < 0.0 {
            self.emit.value(u, v, p)
        } else {
            Vec3::zero()
        }
    }
}
