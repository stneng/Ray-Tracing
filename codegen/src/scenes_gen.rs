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
pub fn random_scene_static(_item: proc_macro::TokenStream) -> proc_macro::TokenStream {
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
