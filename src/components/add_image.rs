#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::database_ops::*;
use crate::file_selector::*;
use crate::components::form_utils::*;


#[derive(Clone, PartialEq, Debug)]
pub struct ImageForm {
    pub image_path: Option<String>,
    pub image_caption: Option<String>,
    pub is_content_thumbnail: bool,
    pub is_pinned: bool,
    pub content_entry_id: Option<String>,
}



impl ImageForm {
    fn new() -> Self {
        ImageForm { image_path: None, image_caption: None, is_content_thumbnail: false, is_pinned: false, content_entry_id: None }
    }

    fn checkbox_value(value: String) -> Result<bool, CheckboxError> {
        match value.as_str() {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => Err(CheckboxError)
        }
    }

    fn is_ok(&self) -> bool {
        self.image_path.is_some() && self.image_caption.is_some() && self.content_entry_id.is_some()
    }
}


#[inline_props]
pub fn ImageForm(cx: Scope, database_path: String) -> Element {
    let image_form: &UseRef<ImageForm> = use_ref(cx, || ImageForm::new());
    let image_path: &UseState<Option<String>> = use_state(cx, || None);

    let form_state = use_state(cx, || FormState::Closed);
    cx.render(rsx!{
        match **form_state {
            FormState::Closed => rsx!{
                button {
                    onclick: move |_| {
                        let path = choose_file(FileType::Image);
                        image_path.set(path.clone());
                        image_form.with_mut(|form| form.image_path = path.clone());
                    },
                    "Select Image"
                }
            },
            FormState::Active => rsx!{
                ImageFormActive {
                    image_form: image_form,
                    form_state: form_state,
                }
            },
            FormState::Submitted => rsx!{
                ImageFormSubmitted {
                    image_form: image_form,
                    form_state: form_state,
                    database_path: database_path.clone(),
                }                
            }
        }        
    })
}

#[inline_props]
fn ImageFormActive<'a>(cx: Scope, image_form: &'a UseRef<ImageForm>, form_state: &'a UseState<FormState>) -> Element {

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
                    oninput: move |evt| image_form.with_mut(|form| form.image_caption = Some(evt.value.clone())),
                    
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
                label {
                    r#for: "content_entry_id",
                    "content ID:"
                }
                input {
                    r#type: "text",
                    oninput: move |evt| image_form.with_mut(|form| form.content_entry_id = Some(evt.value.clone())),
                }
                button {
                    onclick: move |_| {
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
fn ImageFormSubmitted<'a>(cx: Scope, image_form: &'a UseRef<ImageForm>, form_state: &'a UseState<FormState>, database_path: String) -> Element {
    let future = use_future(cx, (), |()| {
        let row = ImageRow::new(image_form.read().clone());
        image_table_entry(database_path.clone(), row)
    });
    
    cx.render(rsx!{
        match future.value() {
            Some(result) => rsx!{
                format!("Submitted. {:?} rows affected.", result)
                button {
                    onclick: move |_| {
                        image_form.set(ImageForm::new());
                        form_state.set(FormState::Active);
                    },
                    "ok"
                }
            },
            None => rsx!{"Executing operation"}
        }
    })
}