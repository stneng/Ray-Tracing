use rand::Rng;
use std::sync::Arc;

pub use crate::bvh::*;
pub use crate::camera::Camera;
pub use crate::objects::*;
pub use crate::ray::Ray;
pub use crate::vec3::Vec3;

pub fn random_scene(aspect_ratio: f64) -> (ObjectList, Vec3, Camera) {
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
            aspect_ratio,
            0.1,
            10.0,
            0.0,
            1.0,
        ),
    )
}
pub fn random_scene_light(aspect_ratio: f64) -> (ObjectList, Vec3, Camera) {
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
    let mut box1_sphere = vec![
        (Vec3::new(0.0, 0.8, 0.0), 0.8),
        (Vec3::new(1.3, 0.5, 0.0), 0.5),
        (Vec3::new(-1.3, 0.5, 0.0), 0.5),
    ];
    for a in -11..11 {
        for b in -11..11 {
            let mut radius = rand::thread_rng().gen_range(0.08, 0.25);
            let mut center = Vec3::new(
                (a as f64 + 0.9 * rand::random::<f64>()) / 2.0,
                radius,
                (b as f64 + 0.9 * rand::random::<f64>()) / 2.0,
            );
            loop {
                let mut done = true;
                for (c, r) in box1_sphere.iter() {
                    if (*c - center).length() < (r + radius) {
                        done = false;
                        break;
                    }
                }
                if done {
                    break;
                }
                radius = rand::thread_rng().gen_range(0.08, 0.25);
                center = Vec3::new(
                    (a as f64 + 0.9 * rand::random::<f64>()) / 2.0,
                    radius,
                    (b as f64 + 0.9 * rand::random::<f64>()) / 2.0,
                );
            }
            box1_sphere.push((center, radius));
            if (center - Vec3::new(0.0, radius, 0.0)).length() > 1.3 {
                let rd = rand::random::<f64>();
                if rd < 0.2 {
                    box1.add(Arc::new(Sphere {
                        center,
                        radius,
                        material: Arc::new(Lambertian {
                            albedo: Arc::new(SolidColor {
                                color: Vec3::random(0.1, 0.9),
                            }),
                        }),
                    }));
                } else if rd < 0.4 {
                    box1.add(Arc::new(Sphere {
                        center,
                        radius,
                        material: Arc::new(Metal {
                            albedo: Vec3::random(0.0, 1.0),
                            fuzz: rand::thread_rng().gen_range(0.0, 0.5),
                        }),
                    }));
                } else if rd < 0.6 {
                    box1.add(Arc::new(Sphere {
                        center,
                        radius,
                        material: Arc::new(Dielectric {
                            ref_idx: rand::thread_rng().gen_range(1.5, 2.0),
                        }),
                    }));
                } else if rd < 0.8 {
                    box1.add(Arc::new(Sphere {
                        center,
                        radius: radius * 0.9,
                        material: Arc::new(DiffuseLight {
                            emit: Arc::new(SolidColor {
                                color: Vec3::random(0.1, 0.9),
                            }),
                        }),
                    }));
                    box1.add(Arc::new(Sphere {
                        center,
                        radius,
                        material: Arc::new(Dielectric { ref_idx: 1.5 }),
                    }));
                } else {
                    box1.add(Arc::new(Sphere {
                        center,
                        radius: radius * 0.5,
                        material: Arc::new(DiffuseLight {
                            emit: Arc::new(SolidColor {
                                color: Vec3::random(0.1, 0.9),
                            }),
                        }),
                    }));
                    box1.add(Arc::new(Sphere {
                        center,
                        radius,
                        material: Arc::new(FrostedDielectric {
                            ref_idx: 1.5,
                            fuzz: rand::thread_rng().gen_range(0.3, 0.5),
                        }),
                    }));
                }
            }
        }
    }
    let len = box1.objects.len();
    world.add(Arc::new(BvhNode::new(&mut box1.objects, 0, len, 0.0, 1.0)));
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
            40.0,
            aspect_ratio,
            0.05,
            9.0,
            0.0,
            1.0,
        ),
    )
}
pub fn two_checker_spheres(aspect_ratio: f64) -> (ObjectList, Vec3, Camera) {
    let mut world = ObjectList { objects: vec![] };
    world.add(Arc::new(Sphere {
        center: Vec3::new(0.0, -10.0, 0.0),
        radius: 10.0,
        material: Arc::new(Lambertian {
            albedo: Arc::new(CheckerTextureUV {
                odd: Arc::new(SolidColor {
                    color: Vec3::new(0.2, 0.3, 0.1),
                }),
                even: Arc::new(SolidColor {
                    color: Vec3::new(0.9, 0.9, 0.9),
                }),
            }),
        }),
    }));
    world.add(Arc::new(Sphere {
        center: Vec3::new(0.0, 10.0, 0.0),
        radius: 10.0,
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
    (
        world,
        Vec3::new(0.7, 0.8, 1.0),
        Camera::new(
            Vec3::new(13.0, 2.0, 3.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            20.0,
            aspect_ratio,
            0.0,
            10.0,
            0.0,
            1.0,
        ),
    )
}
pub fn two_perlin_spheres(aspect_ratio: f64) -> (ObjectList, Vec3, Camera) {
    let mut world = ObjectList { objects: vec![] };
    world.add(Arc::new(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Arc::new(Lambertian {
            albedo: Arc::new(NoiseTexture {
                noise: Perlin::new(),
                scale: 4.0,
            }),
        }),
    }));
    world.add(Arc::new(Sphere {
        center: Vec3::new(0.0, 2.0, 0.0),
        radius: 2.0,
        material: Arc::new(Lambertian {
            albedo: Arc::new(NoiseTexture {
                noise: Perlin::new(),
                scale: 4.0,
            }),
        }),
    }));
    (
        world,
        Vec3::new(0.7, 0.8, 1.0),
        Camera::new(
            Vec3::new(13.0, 2.0, 3.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            20.0,
            aspect_ratio,
            0.0,
            10.0,
            0.0,
            1.0,
        ),
    )
}
pub fn earth(aspect_ratio: f64) -> (ObjectList, Vec3, Camera) {
    let mut world = ObjectList { objects: vec![] };
    world.add(Arc::new(Sphere {
        center: Vec3::new(0.0, 0.0, 0.0),
        radius: 2.0,
        material: Arc::new(Lambertian {
            albedo: Arc::new(ImageTexture::new("images/earthmap.jpg")),
        }),
    }));
    (
        world,
        Vec3::new(0.7, 0.8, 1.0),
        Camera::new(
            Vec3::new(13.0, 2.0, 3.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            20.0,
            aspect_ratio,
            0.0,
            10.0,
            0.0,
            1.0,
        ),
    )
}
pub fn simple_light(aspect_ratio: f64) -> (ObjectList, Vec3, Camera) {
    let mut world = ObjectList { objects: vec![] };
    world.add(Arc::new(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Arc::new(Lambertian {
            albedo: Arc::new(NoiseTexture {
                noise: Perlin::new(),
                scale: 4.0,
            }),
        }),
    }));
    world.add(Arc::new(Sphere {
        center: Vec3::new(0.0, 2.0, 0.0),
        radius: 2.0,
        material: Arc::new(Lambertian {
            albedo: Arc::new(NoiseTexture {
                noise: Perlin::new(),
                scale: 4.0,
            }),
        }),
    }));
    world.add(Arc::new(RectXY {
        x1: 3.0,
        x2: 5.0,
        y1: 1.0,
        y2: 3.0,
        k: -2.0,
        face: 1.0,
        material: Arc::new(DiffuseLight {
            emit: Arc::new(SolidColor {
                color: Vec3::new(4.0, 4.0, 4.0),
            }),
        }),
    }));
    (
        world,
        Vec3::zero(),
        Camera::new(
            Vec3::new(26.0, 3.0, 6.0),
            Vec3::new(0.0, 2.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            20.0,
            aspect_ratio,
            0.0,
            26.7,
            0.0,
            1.0,
        ),
    )
}
pub fn cornell_box(aspect_ratio: f64) -> (ObjectList, Vec3, Camera) {
    let mut world = ObjectList { objects: vec![] };
    world.add(Arc::new(RectYZ {
        y1: 0.0,
        y2: 555.0,
        z1: 0.0,
        z2: 555.0,
        k: 555.0,
        face: -1.0,
        material: Arc::new(Lambertian {
            albedo: Arc::new(SolidColor {
                color: Vec3::new(0.12, 0.45, 0.15),
            }),
        }),
    }));
    world.add(Arc::new(RectYZ {
        y1: 0.0,
        y2: 555.0,
        z1: 0.0,
        z2: 555.0,
        k: 0.0,
        face: 1.0,
        material: Arc::new(Lambertian {
            albedo: Arc::new(SolidColor {
                color: Vec3::new(0.65, 0.05, 0.05),
            }),
        }),
    }));
    world.add(Arc::new(RectXZ {
        x1: 213.0,
        x2: 343.0,
        z1: 227.0,
        z2: 332.0,
        k: 554.0,
        face: -1.0,
        material: Arc::new(DiffuseLight {
            emit: Arc::new(SolidColor {
                color: Vec3::new(15.0, 15.0, 15.0),
            }),
        }),
    }));
    world.add(Arc::new(RectXZ {
        x1: 0.0,
        x2: 555.0,
        z1: 0.0,
        z2: 555.0,
        k: 0.0,
        face: 1.0,
        material: Arc::new(Lambertian {
            albedo: Arc::new(SolidColor {
                color: Vec3::new(0.73, 0.73, 0.73),
            }),
        }),
    }));
    world.add(Arc::new(RectXZ {
        x1: 0.0,
        x2: 555.0,
        z1: 0.0,
        z2: 555.0,
        k: 555.0,
        face: -1.0,
        material: Arc::new(Lambertian {
            albedo: Arc::new(SolidColor {
                color: Vec3::new(0.73, 0.73, 0.73),
            }),
        }),
    }));
    world.add(Arc::new(RectXY {
        x1: 0.0,
        x2: 555.0,
        y1: 0.0,
        y2: 555.0,
        k: 555.0,
        face: -1.0,
        material: Arc::new(Lambertian {
            albedo: Arc::new(SolidColor {
                color: Vec3::new(0.73, 0.73, 0.73),
            }),
        }),
    }));
    world.add(Arc::new(Box::new(
        Vec3::new(130.0, 0.0, 65.0),
        Vec3::new(295.0, 165.0, 230.0),
        Arc::new(Lambertian {
            albedo: Arc::new(SolidColor {
                color: Vec3::new(0.73, 0.73, 0.73),
            }),
        }),
    )));
    world.add(Arc::new(Box::new(
        Vec3::new(265.0, 0.0, 295.0),
        Vec3::new(430.0, 330.0, 460.0),
        Arc::new(Lambertian {
            albedo: Arc::new(SolidColor {
                color: Vec3::new(0.73, 0.73, 0.73),
            }),
        }),
    )));
    (
        world,
        Vec3::zero(),
        Camera::new(
            Vec3::new(278.0, 278.0, -800.0),
            Vec3::new(278.0, 278.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            40.0,
            aspect_ratio,
            0.0,
            28.3,
            0.0,
            1.0,
        ),
    )
}
