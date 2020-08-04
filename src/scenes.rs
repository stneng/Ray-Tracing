use rand::{rngs::SmallRng, Rng, SeedableRng};
use std::sync::Arc;

pub use crate::camera::*;
pub use crate::objects::*;

pub fn random_scene(aspect_ratio: f64) -> (Arc<ObjectList>, Vec3, Arc<Camera>) {
    let mut rng = SmallRng::from_entropy();
    let mut world = ObjectList { objects: vec![] };
    world.add(Arc::new(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Lambertian {
            albedo: CheckerTexture {
                odd: SolidColor {
                    color: Vec3::new(0.2, 0.3, 0.1),
                },
                even: SolidColor {
                    color: Vec3::new(0.9, 0.9, 0.9),
                },
            },
        },
    }));
    let mut box1 = ObjectList { objects: vec![] };
    for a in -11..11 {
        for b in -11..11 {
            let center = Vec3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let rd = rng.gen::<f64>();
                if rd < 0.8 {
                    box1.add(Arc::new(MovingSphere {
                        center1: center,
                        center2: center + Vec3::new(0.0, rng.gen_range(0.0, 0.5), 0.0),
                        t1: 0.0,
                        t2: 1.0,
                        radius: 0.2,
                        material: Lambertian {
                            albedo: SolidColor {
                                color: Vec3::elemul(
                                    Vec3::random(0.0, 1.0, &mut rng),
                                    Vec3::random(0.0, 1.0, &mut rng),
                                ),
                            },
                        },
                    }));
                } else if rd < 0.95 {
                    box1.add(Arc::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Metal {
                            albedo: Vec3::random(0.0, 1.0, &mut rng),
                            fuzz: rng.gen_range(0.0, 0.5),
                        },
                    }));
                } else {
                    box1.add(Arc::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Dielectric { ref_idx: 1.5 },
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
        material: Lambertian {
            albedo: SolidColor {
                color: Vec3::new(0.4, 0.2, 0.1),
            },
        },
    }));
    world.add(Arc::new(Sphere {
        center: Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Metal {
            albedo: Vec3::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        },
    }));
    world.add(Arc::new(Sphere {
        center: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Dielectric { ref_idx: 1.5 },
    }));
    (
        Arc::new(world),
        Vec3::new(0.7, 0.8, 1.0),
        Arc::new(Camera::new(
            Vec3::new(13.0, 2.0, 3.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            20.0,
            aspect_ratio,
            0.1,
            10.0,
            0.0,
            1.0,
        )),
    )
}
pub fn random_scene_light(aspect_ratio: f64) -> (Arc<ObjectList>, Vec3, Arc<Camera>) {
    let mut rng = SmallRng::from_entropy();
    let mut world = ObjectList { objects: vec![] };
    world.add(Arc::new(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Lambertian {
            albedo: CheckerTexture {
                odd: SolidColor {
                    color: Vec3::new(0.2, 0.3, 0.1),
                },
                even: SolidColor {
                    color: Vec3::new(0.9, 0.9, 0.9),
                },
            },
        },
    }));
    let mut box1 = ObjectList { objects: vec![] };
    let mut box1_sphere = vec![
        (Vec3::new(0.0, 0.8, 0.0), 0.8),
        (Vec3::new(1.3, 0.5, 0.0), 0.5),
        (Vec3::new(-1.3, 0.5, 0.0), 0.5),
    ];
    for a in -11..11 {
        for b in -11..11 {
            let mut radius = rng.gen_range(0.08, 0.25);
            let mut center = Vec3::new(
                (a as f64 + 0.9 * rng.gen::<f64>()) / 2.0,
                radius,
                (b as f64 + 0.9 * rng.gen::<f64>()) / 2.0,
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
                radius = rng.gen_range(0.08, 0.25);
                center = Vec3::new(
                    (a as f64 + 0.9 * rng.gen::<f64>()) / 2.0,
                    radius,
                    (b as f64 + 0.9 * rng.gen::<f64>()) / 2.0,
                );
            }
            box1_sphere.push((center, radius));
            if (center - Vec3::new(0.0, radius, 0.0)).length() > 1.3 {
                let rd = rng.gen::<f64>();
                if rd < 0.2 {
                    box1.add(Arc::new(Sphere {
                        center,
                        radius,
                        material: Lambertian {
                            albedo: SolidColor {
                                color: Vec3::random(0.1, 0.9, &mut rng),
                            },
                        },
                    }));
                } else if rd < 0.4 {
                    box1.add(Arc::new(Sphere {
                        center,
                        radius,
                        material: Metal {
                            albedo: Vec3::random(0.0, 1.0, &mut rng),
                            fuzz: rng.gen_range(0.0, 0.5),
                        },
                    }));
                } else if rd < 0.6 {
                    box1.add(Arc::new(Sphere {
                        center,
                        radius,
                        material: Dielectric {
                            ref_idx: rng.gen_range(1.5, 2.0),
                        },
                    }));
                } else if rd < 0.8 {
                    box1.add(Arc::new(Sphere {
                        center,
                        radius: radius * 0.9,
                        material: DiffuseLight {
                            emit: SolidColor {
                                color: Vec3::random(0.1, 0.9, &mut rng),
                            },
                        },
                    }));
                    box1.add(Arc::new(Sphere {
                        center,
                        radius,
                        material: Dielectric { ref_idx: 1.5 },
                    }));
                } else {
                    box1.add(Arc::new(Sphere {
                        center,
                        radius: radius * 0.5,
                        material: DiffuseLight {
                            emit: SolidColor {
                                color: Vec3::random(0.1, 0.9, &mut rng),
                            },
                        },
                    }));
                    box1.add(Arc::new(Sphere {
                        center,
                        radius,
                        material: FrostedDielectric {
                            ref_idx: 1.5,
                            fuzz: rng.gen_range(0.3, 0.5),
                        },
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
        material: DiffuseLight {
            emit: CheckerTexture {
                odd: SolidColor {
                    color: Vec3::new(1.0, 0.6, 0.4),
                },
                even: SolidColor {
                    color: Vec3::new(1.0, 0.75, 0.3),
                },
            },
        },
    }));
    world.add(Arc::new(Sphere {
        center: Vec3::new(1.3, 0.5, 0.0),
        radius: 0.5,
        material: Dielectric { ref_idx: 1.5 },
    }));
    world.add(Arc::new(Sphere {
        center: Vec3::new(-1.3, 0.5, 0.0),
        radius: 0.5,
        material: Metal {
            albedo: Vec3::new(0.8, 0.6, 0.3),
            fuzz: 0.0,
        },
    }));
    (
        Arc::new(world),
        Vec3::zero(),
        Arc::new(Camera::new(
            Vec3::new(6.0, 3.0, 6.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            40.0,
            aspect_ratio,
            0.05,
            9.0,
            0.0,
            1.0,
        )),
    )
}
pub fn two_checker_spheres(aspect_ratio: f64) -> (Arc<ObjectList>, Vec3, Arc<Camera>) {
    let mut world = ObjectList { objects: vec![] };
    world.add(Arc::new(Sphere {
        center: Vec3::new(0.0, -10.0, 0.0),
        radius: 10.0,
        material: Lambertian {
            albedo: CheckerTextureUV {
                odd: SolidColor {
                    color: Vec3::new(0.2, 0.3, 0.1),
                },
                even: SolidColor {
                    color: Vec3::new(0.9, 0.9, 0.9),
                },
            },
        },
    }));
    world.add(Arc::new(Sphere {
        center: Vec3::new(0.0, 10.0, 0.0),
        radius: 10.0,
        material: Lambertian {
            albedo: CheckerTexture {
                odd: SolidColor {
                    color: Vec3::new(0.2, 0.3, 0.1),
                },
                even: SolidColor {
                    color: Vec3::new(0.9, 0.9, 0.9),
                },
            },
        },
    }));
    (
        Arc::new(world),
        Vec3::new(0.7, 0.8, 1.0),
        Arc::new(Camera::new(
            Vec3::new(13.0, 2.0, 3.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            20.0,
            aspect_ratio,
            0.0,
            10.0,
            0.0,
            1.0,
        )),
    )
}
pub fn two_perlin_spheres(aspect_ratio: f64) -> (Arc<ObjectList>, Vec3, Arc<Camera>) {
    let mut world = ObjectList { objects: vec![] };
    world.add(Arc::new(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Lambertian {
            albedo: NoiseTexture {
                noise: Perlin::new(),
                scale: 4.0,
            },
        },
    }));
    world.add(Arc::new(Sphere {
        center: Vec3::new(0.0, 2.0, 0.0),
        radius: 2.0,
        material: Lambertian {
            albedo: NoiseTexture {
                noise: Perlin::new(),
                scale: 4.0,
            },
        },
    }));
    (
        Arc::new(world),
        Vec3::new(0.7, 0.8, 1.0),
        Arc::new(Camera::new(
            Vec3::new(13.0, 2.0, 3.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            20.0,
            aspect_ratio,
            0.0,
            10.0,
            0.0,
            1.0,
        )),
    )
}
pub fn earth(aspect_ratio: f64) -> (Arc<ObjectList>, Vec3, Arc<Camera>) {
    let mut world = ObjectList { objects: vec![] };
    world.add(Arc::new(Sphere {
        center: Vec3::new(0.0, 0.0, 0.0),
        radius: 2.0,
        material: Lambertian {
            albedo: ImageTexture::new("images/earthmap.jpg"),
        },
    }));
    (
        Arc::new(world),
        Vec3::new(0.7, 0.8, 1.0),
        Arc::new(Camera::new(
            Vec3::new(13.0, 2.0, 3.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            20.0,
            aspect_ratio,
            0.0,
            10.0,
            0.0,
            1.0,
        )),
    )
}
pub fn simple_light(aspect_ratio: f64) -> (Arc<ObjectList>, Vec3, Arc<Camera>) {
    let mut world = ObjectList { objects: vec![] };
    world.add(Arc::new(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Lambertian {
            albedo: NoiseTexture {
                noise: Perlin::new(),
                scale: 4.0,
            },
        },
    }));
    world.add(Arc::new(Sphere {
        center: Vec3::new(0.0, 2.0, 0.0),
        radius: 2.0,
        material: Lambertian {
            albedo: NoiseTexture {
                noise: Perlin::new(),
                scale: 4.0,
            },
        },
    }));
    world.add(Arc::new(RectXY {
        x1: 3.0,
        x2: 5.0,
        y1: 1.0,
        y2: 3.0,
        k: -2.0,
        face: 1.0,
        material: DiffuseLight {
            emit: SolidColor {
                color: Vec3::new(4.0, 4.0, 4.0),
            },
        },
    }));
    (
        Arc::new(world),
        Vec3::zero(),
        Arc::new(Camera::new(
            Vec3::new(26.0, 3.0, 6.0),
            Vec3::new(0.0, 2.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            20.0,
            aspect_ratio,
            0.0,
            26.7,
            0.0,
            1.0,
        )),
    )
}
pub fn cornell_box(aspect_ratio: f64) -> (Arc<ObjectList>, Vec3, Arc<Camera>) {
    let mut world = ObjectList { objects: vec![] };
    world.add(Arc::new(RectYZ {
        y1: 0.0,
        y2: 555.0,
        z1: 0.0,
        z2: 555.0,
        k: 555.0,
        face: -1.0,
        material: Lambertian {
            albedo: SolidColor {
                color: Vec3::new(0.12, 0.45, 0.15),
            },
        },
    }));
    world.add(Arc::new(RectYZ {
        y1: 0.0,
        y2: 555.0,
        z1: 0.0,
        z2: 555.0,
        k: 0.0,
        face: 1.0,
        material: Lambertian {
            albedo: SolidColor {
                color: Vec3::new(0.65, 0.05, 0.05),
            },
        },
    }));
    world.add(Arc::new(RectXZ {
        x1: 213.0,
        x2: 343.0,
        z1: 227.0,
        z2: 332.0,
        k: 554.0,
        face: -1.0,
        material: DiffuseLight {
            emit: SolidColor {
                color: Vec3::new(15.0, 15.0, 15.0),
            },
        },
    }));
    world.add(Arc::new(RectXZ {
        x1: 0.0,
        x2: 555.0,
        z1: 0.0,
        z2: 555.0,
        k: 0.0,
        face: 1.0,
        material: Lambertian {
            albedo: SolidColor {
                color: Vec3::new(0.73, 0.73, 0.73),
            },
        },
    }));
    world.add(Arc::new(RectXZ {
        x1: 0.0,
        x2: 555.0,
        z1: 0.0,
        z2: 555.0,
        k: 555.0,
        face: -1.0,
        material: Lambertian {
            albedo: SolidColor {
                color: Vec3::new(0.73, 0.73, 0.73),
            },
        },
    }));
    world.add(Arc::new(RectXY {
        x1: 0.0,
        x2: 555.0,
        y1: 0.0,
        y2: 555.0,
        k: 555.0,
        face: -1.0,
        material: Lambertian {
            albedo: SolidColor {
                color: Vec3::new(0.73, 0.73, 0.73),
            },
        },
    }));
    world.add(Arc::new(Translate::new(
        RotateY::new(
            Cuboid::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(165.0, 330.0, 165.0),
                Lambertian {
                    albedo: SolidColor {
                        color: Vec3::new(0.73, 0.73, 0.73),
                    },
                },
            ),
            15.0,
        ),
        Vec3::new(265.0, 0.0, 295.0),
    )));
    world.add(Arc::new(Translate::new(
        RotateY::new(
            Cuboid::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(165.0, 165.0, 165.0),
                Lambertian {
                    albedo: SolidColor {
                        color: Vec3::new(0.73, 0.73, 0.73),
                    },
                },
            ),
            -18.0,
        ),
        Vec3::new(130.0, 0.0, 65.0),
    )));
    (
        Arc::new(world),
        Vec3::zero(),
        Arc::new(Camera::new(
            Vec3::new(278.0, 278.0, -800.0),
            Vec3::new(278.0, 278.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            40.0,
            aspect_ratio,
            0.0,
            28.3,
            0.0,
            1.0,
        )),
    )
}
pub fn cornell_smoke(aspect_ratio: f64) -> (Arc<ObjectList>, Vec3, Arc<Camera>) {
    let mut world = ObjectList { objects: vec![] };
    world.add(Arc::new(RectYZ {
        y1: 0.0,
        y2: 555.0,
        z1: 0.0,
        z2: 555.0,
        k: 555.0,
        face: -1.0,
        material: Lambertian {
            albedo: SolidColor {
                color: Vec3::new(0.12, 0.45, 0.15),
            },
        },
    }));
    world.add(Arc::new(RectYZ {
        y1: 0.0,
        y2: 555.0,
        z1: 0.0,
        z2: 555.0,
        k: 0.0,
        face: 1.0,
        material: Lambertian {
            albedo: SolidColor {
                color: Vec3::new(0.65, 0.05, 0.05),
            },
        },
    }));
    world.add(Arc::new(RectXZ {
        x1: 113.0,
        x2: 443.0,
        z1: 127.0,
        z2: 432.0,
        k: 554.0,
        face: -1.0,
        material: DiffuseLight {
            emit: SolidColor {
                color: Vec3::new(7.0, 7.0, 7.0),
            },
        },
    }));
    world.add(Arc::new(RectXZ {
        x1: 0.0,
        x2: 555.0,
        z1: 0.0,
        z2: 555.0,
        k: 0.0,
        face: 1.0,
        material: Lambertian {
            albedo: SolidColor {
                color: Vec3::new(0.73, 0.73, 0.73),
            },
        },
    }));
    world.add(Arc::new(RectXZ {
        x1: 0.0,
        x2: 555.0,
        z1: 0.0,
        z2: 555.0,
        k: 555.0,
        face: -1.0,
        material: Lambertian {
            albedo: SolidColor {
                color: Vec3::new(0.73, 0.73, 0.73),
            },
        },
    }));
    world.add(Arc::new(RectXY {
        x1: 0.0,
        x2: 555.0,
        y1: 0.0,
        y2: 555.0,
        k: 555.0,
        face: -1.0,
        material: Lambertian {
            albedo: SolidColor {
                color: Vec3::new(0.73, 0.73, 0.73),
            },
        },
    }));
    world.add(Arc::new(ConstantMedium::new(
        Translate::new(
            RotateY::new(
                Cuboid::new(
                    Vec3::new(0.0, 0.0, 0.0),
                    Vec3::new(165.0, 330.0, 165.0),
                    Lambertian {
                        albedo: SolidColor {
                            color: Vec3::new(0.73, 0.73, 0.73),
                        },
                    },
                ),
                15.0,
            ),
            Vec3::new(265.0, 0.0, 295.0),
        ),
        SolidColor {
            color: Vec3::zero(),
        },
        0.01,
    )));
    world.add(Arc::new(ConstantMedium::new(
        Translate::new(
            RotateY::new(
                Cuboid::new(
                    Vec3::new(0.0, 0.0, 0.0),
                    Vec3::new(165.0, 165.0, 165.0),
                    Lambertian {
                        albedo: SolidColor {
                            color: Vec3::new(0.73, 0.73, 0.73),
                        },
                    },
                ),
                -18.0,
            ),
            Vec3::new(130.0, 0.0, 65.0),
        ),
        SolidColor {
            color: Vec3::ones(),
        },
        0.01,
    )));
    (
        Arc::new(world),
        Vec3::zero(),
        Arc::new(Camera::new(
            Vec3::new(278.0, 278.0, -800.0),
            Vec3::new(278.0, 278.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            40.0,
            aspect_ratio,
            0.0,
            28.3,
            0.0,
            1.0,
        )),
    )
}
pub fn final_scene(aspect_ratio: f64) -> (Arc<ObjectList>, Vec3, Arc<Camera>) {
    let mut rng = SmallRng::from_entropy();
    let mut world = ObjectList { objects: vec![] };
    let mut box1 = ObjectList { objects: vec![] };
    for i in 0..20 {
        for j in 0..20 {
            let w = 100.0;
            let x1 = -1000.0 + i as f64 * w;
            let y1 = 0.0;
            let z1 = -1000.0 + j as f64 * w;
            let x2 = x1 + w;
            let y2 = rng.gen_range(1.0, 101.0);
            let z2 = z1 + w;
            box1.add(Arc::new(Cuboid::new(
                Vec3::new(x1, y1, z1),
                Vec3::new(x2, y2, z2),
                Lambertian {
                    albedo: SolidColor {
                        color: Vec3::new(0.48, 0.83, 0.53),
                    },
                },
            )));
        }
    }
    let len = box1.objects.len();
    world.add(Arc::new(BvhNode::new(&mut box1.objects, 0, len, 0.0, 1.0)));
    world.add(Arc::new(RectXZ {
        x1: 123.0,
        x2: 423.0,
        z1: 147.0,
        z2: 412.0,
        k: 554.0,
        face: -1.0,
        material: DiffuseLight {
            emit: SolidColor {
                color: Vec3::new(7.0, 7.0, 7.0),
            },
        },
    }));
    world.add(Arc::new(MovingSphere {
        center1: Vec3::new(400.0, 400.0, 200.0),
        center2: Vec3::new(400.0, 400.0, 200.0) + Vec3::new(30.0, 0.0, 0.0),
        t1: 0.0,
        t2: 1.0,
        radius: 50.0,
        material: Lambertian {
            albedo: SolidColor {
                color: Vec3::new(0.7, 0.3, 0.1),
            },
        },
    }));
    world.add(Arc::new(Sphere {
        center: Vec3::new(260.0, 150.0, 45.0),
        radius: 50.0,
        material: Dielectric { ref_idx: 1.5 },
    }));
    world.add(Arc::new(Sphere {
        center: Vec3::new(0.0, 150.0, 145.0),
        radius: 50.0,
        material: Metal {
            albedo: Vec3::new(0.8, 0.8, 0.9),
            fuzz: 10.0,
        },
    }));
    world.add(Arc::new(Sphere {
        center: Vec3::new(360.0, 150.0, 145.0),
        radius: 70.0,
        material: Dielectric { ref_idx: 1.5 },
    }));
    world.add(Arc::new(ConstantMedium::new(
        Sphere {
            center: Vec3::new(360.0, 150.0, 145.0),
            radius: 70.0,
            material: Dielectric { ref_idx: 1.5 },
        },
        SolidColor {
            color: Vec3::new(0.2, 0.4, 0.9),
        },
        0.2,
    )));
    world.add(Arc::new(ConstantMedium::new(
        Sphere {
            center: Vec3::new(0.0, 0.0, 0.0),
            radius: 5000.0,
            material: Dielectric { ref_idx: 1.5 },
        },
        SolidColor {
            color: Vec3::ones(),
        },
        0.0001,
    )));
    world.add(Arc::new(Sphere {
        center: Vec3::new(400.0, 200.0, 400.0),
        radius: 100.0,
        material: Lambertian {
            albedo: ImageTexture::new("images/earthmap.jpg"),
        },
    }));
    world.add(Arc::new(Sphere {
        center: Vec3::new(220.0, 280.0, 300.0),
        radius: 80.0,
        material: Lambertian {
            albedo: NoiseTexture {
                noise: Perlin::new(),
                scale: 0.1,
            },
        },
    }));
    let mut box2 = ObjectList { objects: vec![] };
    for _ in 0..1000 {
        box2.add(Arc::new(Sphere {
            center: Vec3::random(0.0, 165.0, &mut rng),
            radius: 10.0,
            material: Lambertian {
                albedo: SolidColor {
                    color: Vec3::new(0.73, 0.73, 0.73),
                },
            },
        }))
    }
    let len = box2.objects.len();
    world.add(Arc::new(Translate::new(
        RotateY::new(BvhNode::new(&mut box2.objects, 0, len, 0.0, 1.0), 15.0),
        Vec3::new(-100.0, 270.0, 395.0),
    )));
    (
        Arc::new(world),
        Vec3::zero(),
        Arc::new(Camera::new(
            Vec3::new(478.0, 278.0, -600.0),
            Vec3::new(278.0, 278.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            40.0,
            aspect_ratio,
            0.0,
            28.3,
            0.0,
            1.0,
        )),
    )
}
