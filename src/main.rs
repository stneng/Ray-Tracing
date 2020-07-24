mod camera;
//mod image_test;
mod materials;
mod objects;
mod part1;
mod ray;
#[allow(clippy::float_cmp)]
mod vec3;
fn main() {
    //image_test::run_test();
    //image_test::run_rgb();
    //image_test::run_julia_set();
    part1::run_part1();
}
