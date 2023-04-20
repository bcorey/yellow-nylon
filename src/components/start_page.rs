#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::file_selector::*;
use crate::components::*;

#[inline_props]
pub fn StartPage(cx: Scope) -> Element {
    let selected_path: &UseState<Option<String>> = use_state(cx, || None);
    let image_form: &UseState<Option<ImageForm>> = use_state(cx, || None);
    
    cx.render(rsx!{
        h1 {"hi.vis"}
        button {
            onclick:move |_| selected_path.set(choose_file(FileType::Database)),
            "Choose database"
        }

        if let Some(path) = selected_path.as_ref() {
            rsx!{
                ContentForm {
                    database_path: path.clone(),
                }
            }
        }
    })
}
