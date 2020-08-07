use proc_macro2::TokenStream;
use quote::quote;
use rand::{rngs::SmallRng, Rng, SeedableRng};

use crate::vec3::*;
struct Object {
    pub bounding_box_min: Vec3,
    pub code: TokenStream,
}

fn bvh_build(objects: &mut Vec<Object>) -> TokenStream {
    let axis = rand::thread_rng().gen_range(0, 3);
    match axis {
        0 => objects.sort_by(|a, b| {
            a.bounding_box_min
                .x
                .partial_cmp(&b.bounding_box_min.x)
                .unwrap()
        }),
        1 => objects.sort_by(|a, b| {
            a.bounding_box_min
                .y
                .partial_cmp(&b.bounding_box_min.y)
                .unwrap()
        }),
        2 => objects.sort_by(|a, b| {
            a.bounding_box_min
                .z
                .partial_cmp(&b.bounding_box_min.z)
                .unwrap()
        }),
        _ => panic!("axis error"),
    }
    let len = objects.len();
    if len == 1 {
        let tmp = objects.remove(0);
        let code = tmp.code;
        quote! {
            #code
        }
    } else {
        let mut objects2 = objects.split_off(objects.len() / 2);
        let left = bvh_build(objects);
        let right = bvh_build(&mut objects2);
        quote! {
            Box::new(BvhNodeStatic::new(#left, #right, 0.0, 1.0))
        }
    }
}
pub fn random_scene_static(switch: bool) -> proc_macro::TokenStream {
    if !switch {
        return proc_macro::TokenStream::from(quote! {
            fn random_scene_static_bvh() -> Box<dyn Object> {
                Box::new(ObjectList { objects: vec![] })
            }
        });
    }
    let mut rng = SmallRng::from_entropy();
    let mut objects = vec![];
    for a in -11..11 {
        for b in -11..11 {
            let center = Vec3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );
            let radius = 0.2;
            let bounding_box_min = center - Vec3::new(radius, radius, radius);
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let rd = rng.gen::<f64>();
                let (x, y, z) = (center.x, center.y, center.z);
                if rd < 0.8 {
                    objects.push(Object {
                        bounding_box_min,
                        code: quote! {
                            Box::new(MovingSphere {
                                center1: Vec3::new(#x, #y, #z),
                                center2: Vec3::new(#x, #y, #z) + Vec3::new(0.0, rng.gen_range(0.0, 0.5), 0.0),
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
                            })
                        },
                    });
                } else if rd < 0.95 {
                    objects.push(Object {
                        bounding_box_min,
                        code: quote! {
                            Box::new(Sphere {
                                center: Vec3::new(#x, #y, #z),
                                radius: 0.2,
                                material: Metal {
                                    albedo: Vec3::random(0.0, 1.0, &mut rng),
                                    fuzz: rng.gen_range(0.0, 0.5),
                                },
                            })
                        },
                    });
                } else {
                    objects.push(Object {
                        bounding_box_min,
                        code: quote! {
                            Box::new(Sphere {
                                center: Vec3::new(#x, #y, #z),
                                radius: 0.2,
                                material: Dielectric { ref_idx: 1.5 },
                            })
                        },
                    });
                }
            }
        }
    }
    let bvh_code = bvh_build(&mut objects);
    proc_macro::TokenStream::from(quote! {
        fn random_scene_static_bvh() -> Box<dyn Object> {
            let mut rng = SmallRng::from_entropy();
            #bvh_code
        }
    })
}
pub fn random_scene_light_static(switch: bool) -> proc_macro::TokenStream {
    if !switch {
        return proc_macro::TokenStream::from(quote! {
            fn random_scene_light_static_bvh() -> Box<dyn Object> {
                Box::new(ObjectList { objects: vec![] })
            }
        });
    }
    let mut rng = SmallRng::from_entropy();
    let mut objects = vec![];
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
            let bounding_box_min = center - Vec3::new(radius, radius, radius);
            if (center - Vec3::new(0.0, radius, 0.0)).length() > 1.3 {
                let rd = rng.gen::<f64>();
                let (x, y, z) = (center.x, center.y, center.z);
                if rd < 0.2 {
                    objects.push(Object {
                        bounding_box_min,
                        code: quote! {
                            Box::new(Sphere {
                                center: Vec3::new(#x, #y, #z),
                                radius: #radius,
                                material: Lambertian {
                                    albedo: SolidColor {
                                        color: Vec3::random(0.1, 0.9, &mut rng),
                                    },
                                },
                            })
                        },
                    });
                } else if rd < 0.4 {
                    objects.push(Object {
                        bounding_box_min,
                        code: quote! {
                            Box::new(Sphere {
                                center: Vec3::new(#x, #y, #z),
                                radius: #radius,
                                material: Metal {
                                    albedo: Vec3::random(0.0, 1.0, &mut rng),
                                    fuzz: rng.gen_range(0.0, 0.5),
                                },
                            })
                        },
                    });
                } else if rd < 0.6 {
                    objects.push(Object {
                        bounding_box_min,
                        code: quote! {
                            Box::new(Sphere {
                                center: Vec3::new(#x, #y, #z),
                                radius: #radius,
                                material: Dielectric {
                                    ref_idx: rng.gen_range(1.5, 2.0),
                                },
                            })
                        },
                    });
                } else if rd < 0.8 {
                    objects.push(Object {
                        bounding_box_min,
                        code: quote! {
                            Box::new(Sphere {
                                center: Vec3::new(#x, #y, #z),
                                radius: #radius * 0.9,
                                material: DiffuseLight {
                                    emit: SolidColor {
                                        color: Vec3::random(0.1, 0.9, &mut rng),
                                    },
                                },
                            })
                        },
                    });
                    objects.push(Object {
                        bounding_box_min,
                        code: quote! {
                            Box::new(Sphere {
                                center: Vec3::new(#x, #y, #z),
                                radius: #radius,
                                material: Dielectric { ref_idx: 1.5 },
                            })
                        },
                    });
                } else {
                    objects.push(Object {
                        bounding_box_min,
                        code: quote! {
                            Box::new(Sphere {
                                center: Vec3::new(#x, #y, #z),
                                radius: #radius * 0.5,
                                material: DiffuseLight {
                                    emit: SolidColor {
                                        color: Vec3::random(0.1, 0.9, &mut rng),
                                    },
                                },
                            })
                        },
                    });
                    objects.push(Object {
                        bounding_box_min,
                        code: quote! {
                            Box::new(Sphere {
                                center: Vec3::new(#x, #y, #z),
                                radius: #radius,
                                material: FrostedDielectric {
                                    ref_idx: 1.5,
                                    fuzz: rng.gen_range(0.3, 0.5),
                                },
                            })
                        },
                    });
                }
            }
        }
    }
    let bvh_code = bvh_build(&mut objects);
    proc_macro::TokenStream::from(quote! {
        fn random_scene_light_static_bvh() -> Box<dyn Object> {
            let mut rng = SmallRng::from_entropy();
            #bvh_code
        }
    })
}
pub fn final_scene_static(switch: bool) -> proc_macro::TokenStream {
    if !switch {
        return proc_macro::TokenStream::from(quote! {
            fn final_scene_static_bvh() -> (Box<dyn Object>, Box<dyn Object>) {
                (Box::new(ObjectList { objects: vec![] }), Box::new(ObjectList { objects: vec![] }))
            }
        });
    }
    let mut rng = SmallRng::from_entropy();
    let mut objects1 = vec![];
    for i in 0..20 {
        for j in 0..20 {
            let w = 100.0;
            let x1 = -1000.0 + i as f64 * w;
            let y1 = 0.0;
            let z1 = -1000.0 + j as f64 * w;
            let x2 = x1 + w;
            let y2 = rng.gen_range(1.0, 101.0);
            let z2 = z1 + w;
            let bounding_box_min = Vec3::new(x1, y1, z1);
            objects1.push(Object {
                bounding_box_min,
                code: quote! {
                    Box::new(Cuboid::new(
                        Vec3::new(#x1, #y1, #z1),
                        Vec3::new(#x2, #y2, #z2),
                        Lambertian {
                            albedo: SolidColor {
                                color: Vec3::new(0.48, 0.83, 0.53),
                            },
                        },
                    ))
                },
            });
        }
    }
    let bvh_code1 = bvh_build(&mut objects1);
    let mut objects2 = vec![];
    for _ in 0..1000 {
        let center = Vec3::random(0.0, 165.0, &mut rng);
        let radius = 10.0;
        let bounding_box_min = center - Vec3::new(radius, radius, radius);
        let (x, y, z) = (center.x, center.y, center.z);
        objects2.push(Object {
            bounding_box_min,
            code: quote! {
                Box::new(Sphere {
                    center: Vec3::new(#x, #y, #z),
                    radius: #radius,
                    material: Lambertian {
                        albedo: SolidColor {
                            color: Vec3::new(0.73, 0.73, 0.73),
                        },
                    },
                })
            },
        });
    }
    let bvh_code2 = bvh_build(&mut objects2);
    proc_macro::TokenStream::from(quote! {
        fn final_scene_static_bvh() -> (Box<dyn Object>, Box<dyn Object>) {
            let mut rng = SmallRng::from_entropy();
            (
                #bvh_code1,
                Box::new(Translate::new(
                    RotateY::new(*#bvh_code2, 15.0),
                    Vec3::new(-100.0, 270.0, 395.0),
                ))
            )
        }
    })
}
