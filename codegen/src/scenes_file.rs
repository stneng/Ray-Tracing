use proc_macro2::TokenStream;
use quote::quote;
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;

fn get_vec3(vec3: &Value) -> TokenStream {
    if let Some(x) = vec3["x"].as_f64() {
        if let Some(y) = vec3["y"].as_f64() {
            if let Some(z) = vec3["z"].as_f64() {
                return quote! {
                    Vec3::new(#x, #y, #z)
                };
            }
        }
    }
    quote! { compile_error! { "Vec3 error" } }
}
fn build_object(object: &Value) -> TokenStream {
    match object["type"].as_str() {
        Some("ObjectList") => {
            if let Some(items) = object["items"].as_array() {
                let mut tokens = vec![];
                for x in items {
                    let token = build_object(x);
                    tokens.push(quote! {
                        Box::new(#token),
                    });
                }
                let token: TokenStream = tokens.into_iter().collect();
                quote! {
                    ObjectList { objects: vec![#token] }
                }
            } else {
                quote! { compile_error! { "ObjectList no items" } }
            }
        }
        Some("Sphere") => {
            if let Some(radius) = object["radius"].as_f64() {
                let center = get_vec3(&object["center"]);
                let material = build_material(&object["material"]);
                return quote! {
                    Sphere {
                        center: #center,
                        radius: #radius,
                        material: #material,
                    }
                };
            }
            quote! { compile_error! { "Sphere error" } }
        }
        Some("BVHNode") => {
            let left = build_object(&object["left"]);
            let right = build_object(&object["right"]);
            quote! {
                BvhNodeStatic::new(Box::new(#left), Box::new(#right), 0.0, 1.0)
            }
        }
        _ => {
            quote! { compile_error! { "Object type error" } }
        }
    }
}
fn build_material(material: &Value) -> TokenStream {
    match material["type"].as_str() {
        Some("Lambertian") => {
            let albedo = build_texture(&material["albedo"]);
            quote! {
                Lambertian { albedo: #albedo }
            }
        }
        Some("Metal") => {
            if let Some(fuzz) = material["fuzz"].as_f64() {
                let albedo = get_vec3(&material["albedo"]);
                return quote! {
                    Metal {
                        albedo: #albedo,
                        fuzz: #fuzz,
                    }
                };
            }
            quote! { compile_error! { "Metal error" } }
        }
        Some("Dielectric") => {
            if let Some(ref_idx) = material["ref_idx"].as_f64() {
                return quote! {
                    Dielectric { ref_idx: #ref_idx }
                };
            }
            quote! { compile_error! { "Dielectric error" } }
        }
        Some("DiffuseLight") => {
            let emit = build_texture(&material["emit"]);
            quote! {
                DiffuseLight { emit: #emit }
            }
        }
        _ => {
            quote! { compile_error! { "Material type error" } }
        }
    }
}
fn build_texture(texture: &Value) -> TokenStream {
    match texture["type"].as_str() {
        Some("SolidColor") => {
            let color = get_vec3(&texture["color"]);
            quote! {
                SolidColor { color: #color }
            }
        }
        Some("CheckerTexture") => {
            let odd = build_texture(&texture["odd"]);
            let even = build_texture(&texture["even"]);
            quote! {
                CheckerTexture {
                    odd: #odd,
                    even: #even,
                }
            }
        }
        _ => {
            quote! { compile_error! { "Texture type error" } }
        }
    }
}
fn get_camera(cam: &Value) -> TokenStream {
    let lookfrom = get_vec3(&cam["lookfrom"]);
    let lookat = get_vec3(&cam["lookat"]);
    let vup = get_vec3(&cam["vup"]);
    if let Some(vfov) = cam["vfov"].as_f64() {
        if let Some(aspect_ratio) = cam["aspect_ratio"].as_f64() {
            if let Some(aperture) = cam["aperture"].as_f64() {
                if let Some(focus_dist) = cam["focus_dist"].as_f64() {
                    return quote! {
                        Camera::new(
                            #lookfrom,
                            #lookat,
                            #vup,
                            #vfov,
                            #aspect_ratio,
                            #aperture,
                            #focus_dist,
                            0.0,
                            1.0,
                        )
                    };
                }
            }
        }
    }
    quote! { compile_error! { "Camera error" } }
}
fn from_file() -> (TokenStream, TokenStream) {
    let file = File::open("codegen/data/scene_200_no_bvh.json").unwrap();
    let reader = BufReader::new(file);
    let data: Value = serde_json::from_reader(reader).unwrap();
    (build_object(&data["objects"]), get_camera(&data["camera"]))
}

pub fn scene_from_file(switch: bool) -> proc_macro::TokenStream {
    if !switch {
        return proc_macro::TokenStream::from(
            quote! { pub fn scene_from_file(_aspect_ratio: f64){} },
        );
    }
    let (world, cam) = from_file();
    proc_macro::TokenStream::from(quote! {
        pub fn scene_from_file(_aspect_ratio: f64) -> (Arc<ObjectList>, Vec3, Arc<Camera>, Arc<Option<ObjectList>>) {
            (
                Arc::new(#world),
                Vec3::zero(),
                Arc::new(#cam),
                Arc::new(None),
            )
        }
    })
}
