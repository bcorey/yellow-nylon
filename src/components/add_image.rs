#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::database_ops::*;
use crate::file_selector::*;
use crate::components::form_utils::*;


#[derive(Clone, PartialEq, Debug)]
pub struct ImageForm {
    pub image_path: String,
    pub image_caption: String,
    pub is_content_thumbnail: bool,
    pub is_pinned: bool,
    pub content_entry_id: String,
}



impl ImageForm {
    fn new() -> Self {
        ImageForm { 
            image_path: String::new(), 
            image_caption: String::new(), 
            is_content_thumbnail: false, 
            is_pinned: false, 
            content_entry_id: String::new() 
        }
    }

    fn checkbox_value(value: String) -> Result<bool, CheckboxError> {
        match value.as_str() {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => Err(CheckboxError)
        }
    }

    fn is_ok(&self) -> bool {
        self.image_path.len() > 0 && self.image_caption.len() >0 && self.content_entry_id.len() > 0
    }
}


#[inline_props]
pub fn ImageForm<'a>(cx: Scope<'a>, form_state: &'a UseState<FormState>, content_entry_id: String) -> Element {
    let image_form: &UseRef<ImageForm> = use_ref(cx, || ImageForm::new());
    let image_path: &UseState<Option<String>> = use_state(cx, || None);

    cx.render(rsx!{
        match *form_state.current() {
            FormState::Closed => rsx!{
                ImageFormClosed {
                    image_form: image_form,
                    image_path: image_path,
                    form_state: form_state,
                }
            },
            FormState::Active => rsx!{
                ImageFormActive {
                    image_form: image_form,
                    form_state: form_state,
                    content_entry_id: content_entry_id.clone()
                }
            },
            FormState::Submitted => rsx!{
                ImageFormSubmitted {
                    image_form: image_form,
                    form_state: form_state,
                }                
            }
        }        
    })
}

#[inline_props]
fn ImageFormClosed<'a>(
    cx: Scope<'a>, 
    image_path: &'a UseState<Option<String>>, 
    image_form: &'a UseRef<ImageForm>, 
    form_state: &'a UseState<FormState>
) -> Element {
    cx.render(rsx!{
        button {
            onclick: move |_| {
                let path_select = choose_file(FileType::Image);
                if let Some(path) = path_select {
                    image_path.set(Some(path.clone()));
                    image_form.with_mut(|form| form.image_path = path.clone());
                    form_state.set(FormState::Active);
                }
            },
            "Add Image"
        }
    })
}

#[inline_props]
fn ImageFormActive<'a>(cx: Scope, image_form: &'a UseRef<ImageForm>, form_state: &'a UseState<FormState>, content_entry_id: String) -> Element {

    cx.render(rsx!{
        div {
            class: "image-form",
            rsx!{
                label {
                    r#for: "image_caption",
                    "Image Caption",
                }
                input {
                    r#type: "text",
                    name: "image_caption",
                    oninput: move |evt| image_form.with_mut(|form| form.image_caption = evt.value.clone()),
                }
                label {
                    r#for: "is_content_thumbnail",
                    "Make this a thumbnail"
                }
                input {
                    r#type: "checkbox",
                    name: "is_content_thumbnail",
                    oninput: move |evt| image_form.with_mut(|form| 
                        form.is_content_thumbnail = ImageForm::checkbox_value(evt.value.clone()).unwrap()),
                }
                label {
                    r#for: "is_pinned",
                    "Pin this image",                    
                }
                input {
                    r#type: "checkbox",
                    name: "is_pinned",
                    oninput: move |evt| image_form.with_mut(|form| 
                        form.is_pinned = ImageForm::checkbox_value(evt.value.clone()).unwrap()),
                }
                button {
                    onclick: move |_| {
                        image_form.with_mut(|form| form.content_entry_id = content_entry_id.clone());

                        if image_form.with_mut(|form| form.is_ok()) {
                            form_state.set(FormState::Submitted);
                        }
                    },
                    "Submit"
                }
                button {
                    onclick: move |_| {
                        form_state.set(FormState::Closed);
                        image_form.set(ImageForm::new());
                    },
                    "Cancel",
                }
            }
            
        }
    })
}

#[inline_props]
fn ImageFormSubmitted<'a>(cx: Scope, image_form: &'a UseRef<ImageForm>, form_state: &'a UseState<FormState>) -> Element {
    let database_path = use_database_path(cx).read().clone().unwrap();
    
    let future = use_future(cx, (), |()| {
        to_owned![database_path];
        let row = ImageRow::new(image_form.read().clone());
        async move {
            row.add_image_row(database_path.clone()).await
        }
    });
    
    cx.render(rsx!{
        match future.value() {
            Some(result) => rsx!{
                format!("Submitted. {:?} rows affected.", result)
                button {
                    onclick: move |_| {
                        image_form.set(ImageForm::new());
                        form_state.set(FormState::Closed);
                    },
                    "ok"
                }
            },
            None => rsx!{"Executing operation"}
        }
    })
}