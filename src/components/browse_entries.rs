#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::database_ops::search_rows;
use crate::database_ops::ContentRow;
use crate::components::*;

#[inline_props]
pub fn EntryViewer(cx: Scope) -> Element {
    let database_path = use_database_path(cx).read().clone().unwrap();
    let form_state = use_state(cx, || FormState::Closed);
    let edit_candidate: &'a UseState<Option<ContentRow>> = use_state(cx, || None);

    let search_term = use_state(cx, || String::new());
    let search_dep = search_term.current().to_string();
    let future = use_future(cx, (&search_dep, form_state,), |(search_dep,form_state,)| 
        search_rows(database_path.clone(), search_dep)
    );
    cx.render(rsx!{
        
        match *form_state.current() {
            FormState::Active | FormState::Submitted => rsx!{
                ContentForm {
                    form_mode: FormMode::Edit,
                    form_state: form_state,
                    content_form: ContentForm::from_row(edit_candidate.as_ref().unwrap().clone()),
                }
            },
            FormState::Closed => rsx!{
                ContentFormWrapper {}
                input {
                    r#type: "text",
                    oninput: move |evt| {
                        search_term.set(evt.value.clone())
                    },
                }
                if let Some(response) = future.value() {
                    rsx!{
                        table {
                            response.iter().map(|content_row| rsx!{
                                ContentTableRow {
                                    row: content_row.clone(),
                                    edit_candidate: edit_candidate,
                                    form_state: form_state,
                                }
                            })
                        }                
                    }
                }
            }
        }        
    })
}

#[inline_props]
fn ContentTableRow<'a>(cx: Scope, row: ContentRow, edit_candidate: &'a UseState<Option<ContentRow>>, form_state: &'a UseState<FormState>) -> Element {
    
    cx.render(rsx!{
        tr {
            onclick: move |_| {
                edit_candidate.set(Some(row.clone()));
                form_state.set(FormState::Active);
            },
            td {
                "{row.content_entry_id.clone().unwrap()}"
            }
            td {
                "{row.title.clone().unwrap()}"
            }
            td {
                "{row.tagline.clone().unwrap()}",
            }
            td {
                "{row.tags.clone().unwrap()}"
            }
            td {
                "{row.date.clone().unwrap()}"
            }
            td {
                "{row.content.clone().unwrap()}"
            }
        }
        
    })
}