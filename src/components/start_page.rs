#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::file_selector::*;
use crate::components::*;
use crate::components::form_utils::*;

#[inline_props]
pub fn StartPage(cx: Scope) -> Element {
    let start_page_state = use_state(cx, || FormState::Active);
    use_shared_state_provider::<Option<String>>(cx, || None);
    let db_path = use_database_path(cx);

    cx.render(rsx!{
        h1 {"hi.vis"}
       
        match **start_page_state {
            FormState::Active => rsx!{
                button {
                    onclick:move |_| {
                        *db_path.write() = choose_file(FileType::Database);
                        start_page_state.set(FormState::Closed);
                    },
                    "Choose database"
                }
            },
            _ => rsx!{
                button {
                    onclick: move |_| {
                        *db_path.write() = None;
                        start_page_state.set(FormState::Active);
                    },
                    "Close Database"
                },
                
                DatabaseContainer {
                    
                }
            }
        }
    })
}

#[inline_props]
pub fn DatabaseContainer(cx: Scope) -> Element {
    let compile_form_state = use_state(cx, || FormState::Closed);

    cx.render(rsx!{
        match **compile_form_state {
            FormState::Closed => rsx!{
                button {
                    onclick: move |_| compile_form_state.set(FormState::Active),
                    "Compile Pages"
                },
                EntryViewer {}
            },
            FormState::Active => rsx!{
                button {
                    onclick: move |_| compile_form_state.set(FormState::Closed),
                    "Cancel",
                },
                CompilePagesForm {}
            },
            FormState::Submitted => rsx!{
                ""
            }
        }
    })
}
