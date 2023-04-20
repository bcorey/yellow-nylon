#![allow(non_snake_case)]
use dioxus::html::data;
use dioxus::prelude::*;
use crate::components::form_utils::*;
use crate::components::ImageForm;
use crate::database_ops::*;

#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ContentType {
    Post,
    Project,
    Note
}

use std::fmt;
impl fmt::Display for ContentType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ContentType::Post => write!(f, "Post"),
            ContentType::Project => write!(f, "Project"),
            ContentType::Note => write!(f, "Note"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ContentForm {
    pub title: String,
    pub tagline: String,
    pub tags: String,
    pub content: String,
    pub is_pinned: bool,
    pub content_type: String,
    pub date: String,
    pub content_entry_id: String,
}

impl ContentForm {
    fn new() -> Self {
        ContentForm {
            title: String::new(),
            tagline: String::new(),
            tags: String::new(),
            content: String::new(),
            is_pinned: false,
            content_type: String::new(),
            date: String::new(),
            content_entry_id: String::new(),
        }
    }

    fn is_ok(&self) -> bool {
        self.title.len() > 0 && 
        self.tagline.len() > 0 && 
        self.content.len() > 0 &&
        self.tags.len() > 0 &&
        self.date.len() > 0 &&
        self.content_entry_id.len() > 0
    }
}

#[inline_props]
pub fn ContentForm(cx: Scope, database_path: String) -> Element {
    let content_form = use_ref(cx, || ContentForm::new());

    let form_state = use_state(cx, || FormState::Closed);
    cx.render(rsx!{
        match **form_state {
            FormState::Closed => rsx!{
                button {
                    onclick: move |_| {
                        form_state.set(FormState::Active);
                    },
                    "Add Content",
                }
            },
            FormState::Active => rsx!{
                ImageForm{
                    database_path: database_path.clone(),
                }
                ContentFormActive {
                    content_form: content_form,
                    form_state: form_state,
                }
            },
            FormState::Submitted => rsx!{
                rsx!{
                    ContentFormSubmitted {
                        content_form: content_form,
                        form_state: form_state,
                        database_path: database_path.clone(),
                    }
                }
            }
        }
        
    })
}

#[inline_props]
fn ContentFormActive<'a>(cx: Scope, content_form: &'a UseRef<ContentForm>, form_state: &'a UseState<FormState>) -> Element {
    
    cx.render(rsx!{
        div {
            class: "content-form",
            label {
                r#for: "title",
                "Title"
            }
            input {
                r#type: "text",
                name: "title",
                oninput: move |evt| content_form.with_mut(|form| form.title = evt.value.clone()),
            }
            label {
                r#for: "tagline",
                "Tagline"
            }
            input {
                r#type: "text",
                name: "tagline",
                oninput: move |evt| content_form.with_mut(|form| form.tagline = evt.value.clone()),
            }
            label {
                r#for: "tags",
                "Tags",
            }
            input {
                r#type: "text",
                oninput: move |evt| content_form.with_mut(|form| form.tags = evt.value.clone()),
            }
            label {
                r#for: "content_entry_id",
                "entry ID",
            }
            input {
                r#type: "text",
                oninput: move |evt| content_form.with_mut(|form| form.content_entry_id = evt.value.clone()),
            }
            label {
                r#for: "content_type",
                "Content Type",
            }
            select {
                name: "content_type",
                oninput: move |evt| content_form.with_mut(|form| form.content_type = evt.value.clone()),
                option {
                    ContentType::Post.to_string()
                }
                option {
                    ContentType::Note.to_string()
                }
                option {
                    ContentType::Project.to_string()
                }
            }
            label {
                r#for: "date",
                "Date"
            }
            input {
                r#type: "date",
                name: "date",
                oninput: move |evt| content_form.with_mut(|form| form.date = evt.value.clone()),
            }
            label {
                r#for: "is_pinned",
                "Pinned"
            }
            input {
                r#type: "checkbox",
                name: "is_pinned",
                oninput: move |evt| content_form.with_mut(|form| form.is_pinned = checkbox_value(evt.value.clone()).unwrap() ),
            }
            label {
                r#for: "content",
                "Body Content"
            }
            textarea {
                name: "content",
                oninput: move |evt| content_form.with_mut(|form| form.content = evt.value.clone()),
            }

            button {
                onclick: move |_| {
                    if content_form.with_mut(|form| form.is_ok()) {
                        form_state.set(FormState::Submitted);
                    }
                },
                "Submit",
            }
        }
    })
}

#[inline_props]
fn ContentFormSubmitted<'a>(cx: Scope, content_form: &'a UseRef<ContentForm>, form_state: &'a UseState<FormState>, database_path: String) -> Element {
    let future = use_future(cx, (), |(),| {
        let row = ContentRow::new(content_form.read().clone());
        content_table_entry(database_path.clone(), row)
    });

    cx.render(rsx!{
        match future.value() {
            Some(response) => rsx!{
                format!("Submitted. {:?}", response),
                button {
                    onclick: move |_| {
                        content_form.set(ContentForm::new());
                        form_state.set(FormState::Closed);
                    },
                    "Ok"
                }
            },
            None => rsx!{"Submitting request"},
        }
        
    })
}