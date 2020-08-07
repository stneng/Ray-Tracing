extern crate proc_macro;

mod scenes_gen;
#[allow(dead_code)]
mod vec3;

#[proc_macro]
pub fn random_scene_static_impl(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    scenes_gen::random_scene_static(item)
}
#[proc_macro]
pub fn random_scene_light_static_impl(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    scenes_gen::random_scene_light_static(item)
}