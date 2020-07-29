use rand::Rng;
use std::sync::Arc;

pub use crate::bvh::*;
pub use crate::camera::Camera;
pub use crate::objects::*;
pub use crate::ray::Ray;
pub use crate::vec3::Vec3;

pub fn random_scene() -> (ObjectList, Vec3, Camera) {
    let mut world = ObjectList { objects: vec![] };
    world.add(Arc::new(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Arc::new(Lambertian {
            albedo: Arc::new(CheckerTexture {
                odd: Arc::new(SolidColor {
                    color: Vec3::new(0.2, 0.3, 0.1),
                }),
                even: Arc::new(SolidColor {
                    color: Vec3::new(0.9, 0.9, 0.9),
                }),
            }),
        }),
    }));
    let mut box1 = ObjectList { objects: vec![] };
    for a in -11..11 {
        for b in -11..11 {
            let center = Vec3::new(
                a as f64 + 0.9 * rand::random::<f64>(),
                0.2,
                b as f64 + 0.9 * rand::random::<f64>(),
            );
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let rd = rand::random::<f64>();
                if rd < 0.8 {
                    box1.add(Arc::new(MovingSphere {
                        center1: center,
                        center2: center
                            + Vec3::new(0.0, rand::thread_rng().gen_range(0.0, 0.5), 0.0),
                        t1: 0.0,
                        t2: 1.0,
                        radius: 0.2,
                        material: Arc::new(Lambertian {
                            albedo: Arc::new(SolidColor {
                                color: Vec3::elemul(Vec3::random(0.0, 1.0), Vec3::random(0.0, 1.0)),
                            }),
                        }),
                    }));
                } else if rd < 0.95 {
                    box1.add(Arc::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Arc::new(Metal {
                            albedo: Vec3::random(0.0, 1.0),
                            fuzz: rand::thread_rng().gen_range(0.0, 0.5),
                        }),
                    }));
                } else {
                    box1.add(Arc::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Arc::new(Dielectric { ref_idx: 1.5 }),
                    }));
                }
            }
        }
    }
    let len = box1.objects.len();
    world.add(Arc::new(BvhNode::new(&mut box1.objects, 0, len, 0.0, 1.0)));
    world.add(Arc::new(Sphere {
        center: Vec3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Lambertian {
            albedo: Arc::new(SolidColor {
                color: Vec3::new(0.4, 0.2, 0.1),
            }),
        }),
    }));
    world.add(Arc::new(Sphere {
        center: Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Metal {
            albedo: Vec3::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        }),
    }));
    world.add(Arc::new(Sphere {
        center: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Dielectric { ref_idx: 1.5 }),
    }));
    (
        world,
        Vec3::new(0.7, 0.8, 1.0),
        Camera::new(
            Vec3::new(13.0, 2.0, 3.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            20.0,
            16.0 / 9.0,
            0.1,
            10.0,
            0.0,
            1.0,
        ),
    )
}
pub fn random_scene_light() -> (ObjectList, Vec3, Camera) {
    let mut world = ObjectList { objects: vec![] };
    world.add(Arc::new(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Arc::new(Lambertian {
            albedo: Arc::new(CheckerTexture {
                odd: Arc::new(SolidColor {
                    color: Vec3::new(0.2, 0.3, 0.1),
                }),
                even: Arc::new(SolidColor {
                    color: Vec3::new(0.9, 0.9, 0.9),
                }),
            }),
        }),
    }));
    let mut box1 = ObjectList { objects: vec![] };
    for a in -11..11 {
        for b in -11..11 {
            let radius = rand::thread_rng().gen_range(0.08, 0.25);
            let center = Vec3::new(
                (a as f64 + 0.9 * rand::random::<f64>()) / 2.0,
                radius,
                (b as f64 + 0.9 * rand::random::<f64>()) / 2.0,
            );

            if (center - Vec3::new(-0.2, radius, -0.2)).length() > 1.3 + radius {
                box1.add(Arc::new(Sphere {
                    center,
                    radius: radius * 0.5,
                    material: Arc::new(DiffuseLight {
                        emit: Arc::new(SolidColor {
                            color: Vec3::random(0.1, 0.9),
                        }),
                    }),
                }));
                let fuzz = rand::thread_rng().gen_range(0.3, 0.7);
                box1.add(Arc::new(Sphere {
                    center,
                    radius,
                    material: Arc::new(FrostedDielectric { ref_idx: 1.5, fuzz }),
                }));
            }
        }
    }
    let len = box1.objects.len();
    world.add(Arc::new(BvhNode::new(&mut box1.objects, 0, len, 0.0, 1.0)));
    world.add(Arc::new(Sphere {
        center: Vec3::new(0.0, 0.8, 0.0),
        radius: 1.0,
        material: Arc::new(FrostedDielectric { ref_idx: 1.5, fuzz: 0.6 }),
    }));
    world.add(Arc::new(Sphere {
        center: Vec3::new(0.0, 0.8, 0.0),
        radius: 0.8,
        material: Arc::new(DiffuseLight {
            emit: Arc::new(CheckerTexture {
                odd: Arc::new(SolidColor {
                    color: Vec3::new(1.0, 0.6, 0.4),
                }),
                even: Arc::new(SolidColor {
                    color: Vec3::new(1.0, 0.75, 0.3),
                }),
            }),
        }),
    }));
    world.add(Arc::new(Sphere {
        center: Vec3::new(1.3, 0.5, 0.0),
        radius: 0.5,
        material: Arc::new(Dielectric { ref_idx: 1.5 }),
    }));
    world.add(Arc::new(Sphere {
        center: Vec3::new(-1.3, 0.5, 0.0),
        radius: 0.5,
        material: Arc::new(Metal {
            albedo: Vec3::new(0.8, 0.6, 0.3),
            fuzz: 0.0,
        }),
    }));
    (
        world,
        Vec3::zero(),
        Camera::new(
            Vec3::new(6.0, 3.0, 6.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            20.0,
            16.0 / 9.0,
            0.1,
            9.0,
            0.0,
            1.0,
        ),
    )
}
