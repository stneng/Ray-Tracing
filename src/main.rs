mod camera;
//mod image_test;
mod bvh;
mod materials;
mod objects;
mod ray;
mod ray_tracing;
#[allow(dead_code)]
mod scenes;
mod texture;
mod transforms;
#[allow(clippy::float_cmp)]
mod vec3;
fn main() {
    //image_test::run_test();
    //image_test::run_rgb();
    //image_test::run_julia_set();
    ray_tracing::run_ray_tracing();
}
