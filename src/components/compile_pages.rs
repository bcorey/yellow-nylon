#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::components::path_hooks::use_database_path;

#[inline_props]
pub fn CompilePagesForm(cx: Scope, dist_path: String) -> Element {
    let database_path = use_database_path(cx).read().clone().unwrap();
    let future = use_future(cx, (), |_| {
        crate::compile_static_pages::compile_content(database_path, dist_path.clone())
    });

    cx.render(rsx!{
        match future.value() {
            Some(_response) => rsx!{"compiled at {dist_path}"},
            None => rsx!{"compiling"}
        }
    })
}