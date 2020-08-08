extern crate proc_macro;

mod scenes_file;
mod scenes_gen;
#[allow(dead_code)]
mod vec3;

const SWITCH: bool = false;

#[proc_macro]
pub fn random_scene_static_impl(_item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    scenes_gen::random_scene_static(SWITCH)
}
#[proc_macro]
pub fn random_scene_light_static_impl(_item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    scenes_gen::random_scene_light_static(SWITCH)
}
#[proc_macro]
pub fn final_scene_static_impl(_item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    scenes_gen::final_scene_static(SWITCH)
}
#[proc_macro]
pub fn scene_from_file_impl(_item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    scenes_file::scene_from_file(SWITCH)
}
