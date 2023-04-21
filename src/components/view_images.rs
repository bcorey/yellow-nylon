#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::components::*;
use crate::database_ops::*;

#[inline_props]
pub fn ImageViewer(cx: Scope, content_entry_id: String) -> Element {
    let image_form_state = use_state(cx, || FormState::Closed);
    let database_path = use_database_path(cx).read().clone().unwrap();

    let future = use_future(cx, (content_entry_id, image_form_state,), |(content_entry_id,_image_form_state,)| {
        ImageRow::get_content_images(database_path.clone(), content_entry_id)
    });

    cx.render(rsx!{
        ImageForm {
            form_state: image_form_state,
            content_entry_id: content_entry_id.clone(),
        }
        match future.value() {
            Some(response) => rsx!{
                response.iter().map(|row| rsx!{
                    ImageTableRow {
                        row: row.clone(),
                    }
                })
            },
            None => rsx!{"loading"}
        }
    })
}

use base64::{engine::general_purpose, Engine as _};

#[inline_props]
fn ImageTableRow(cx: Scope, row: ImageRow) -> Element {
    let src = use_state(cx, || {
        let encoding = general_purpose::STANDARD.encode(row.image_original.clone().unwrap());
        format!("data:image/jpg;base64, {}", encoding)
    });

    let row_state = use_state(cx, || FormState::Active);
    
    cx.render(rsx!{
        match *row_state.current() {
            FormState::Active => rsx!{
                img {
                    style: "position: relative; display: flex; width: 4rem; height: 4rem; object-fit: cover;",
                    src: "{src}",
                }
                div {
                    "{row.image_name.clone().unwrap()}",
                }
                div {
                    "{row.image_caption.clone().unwrap()}",
                }
                button {
                    onclick: move |_| row_state.set(FormState::Submitted),
                    "Delete",
                }
            },
            FormState::Submitted => rsx!{
                ImageTableRowDelete {
                    row_state: row_state,
                    row: row.clone(),
                }
            },
            FormState::Closed => rsx!{""}
        }
        
    })
}

#[inline_props]
fn ImageTableRowDelete<'a>(cx: Scope<'a>, row_state: &'a UseState<FormState>, row: ImageRow) -> Element {
    let database_path = use_database_path(cx).read().clone().unwrap();
    let future = use_future(cx, (), |_| {
        to_owned![row, database_path];
        async move {
            row.delete_image_row(database_path).await
        }
    });
    cx.render(rsx!{
        match future.value() {
            Some(response) => rsx!{
                format!("{:?}", response),
                button {
                    onclick: move |_| row_state.set(FormState::Closed),
                    "Ok"
                }
            },
            None => rsx!{"loading"}
        }
    })
}
