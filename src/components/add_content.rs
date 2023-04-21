#![allow(non_snake_case)]
use dioxus::html::data;
use dioxus::prelude::*;
use crate::components::form_utils::*;
use crate::components::*;
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

#[derive(Clone, Debug, PartialEq)]
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

use crate::database_ops::ContentRow;

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

    pub fn from_row(row: ContentRow) -> Self {
        ContentForm {
            title: row.title.unwrap(),
            tagline: row.tagline.unwrap(),
            tags: row.tags.unwrap(),
            content: row.content.unwrap(),
            is_pinned: row.is_pinned.unwrap() != 0,
            content_type: row.content_type.unwrap(),
            date: row.date.unwrap(),
            content_entry_id: row.content_entry_id.unwrap(),
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
pub fn ContentFormWrapper(cx: Scope, database_path: String) -> Element {
    let form_state = use_state(cx, || FormState::Closed);
    cx.render(rsx!{
        ContentForm {
            database_path: database_path.clone(),
            form_state: form_state,
            form_mode: FormMode::Create,
        }
    })
}

#[inline_props]
pub fn ContentForm<'a>(cx: Scope<'a>, database_path: String, form_state: &'a UseState<FormState>, form_mode: FormMode, content_form: Option<ContentForm>) -> Element {
    let content_form = use_ref(cx, || {
        match content_form {
            Some(form) => form.clone(),
            None => ContentForm::new(),
        }
    });

    let form_mode = use_state(cx, || *form_mode);
    cx.render(rsx!{
        match *form_state.current() {
            FormState::Closed => rsx!{
                button {
                    onclick: move |_| {
                        form_state.set(FormState::Active);
                    },
                    "Add Content",
                }
            },
            FormState::Active => rsx!{
                ImageViewer {
                    database_path: database_path.clone(),
                    content_entry_id: content_form.read().content_entry_id.clone(),
                }
                ContentFormActive {
                    content_form: content_form,
                    form_state: form_state,
                    form_mode: form_mode,
                }
            },
            FormState::Submitted => rsx!{
                rsx!{
                    ContentFormSubmitted {
                        content_form: content_form,
                        form_state: form_state,
                        form_mode: *form_mode.current(),
                        database_path: database_path.clone(),
                    }
                }
            }
        }
        
    })
}

#[inline_props]
fn ContentFormActive<'a>(
    cx: Scope, 
    content_form: &'a UseRef<ContentForm>, 
    form_state: &'a UseState<FormState>,
    form_mode: &'a UseState<FormMode>,
) -> Element {
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
                value: "{content_form.with(|form| form.title.clone())}",
            }
            label {
                r#for: "tagline",
                "Tagline"
            }
            input {
                r#type: "text",
                name: "tagline",
                oninput: move |evt| content_form.with_mut(|form| form.tagline = evt.value.clone()),
                value: "{content_form.with(|form| form.tagline.clone())}"
            }
            label {
                r#for: "tags",
                "Tags",
            }
            input {
                r#type: "text",
                name: "tags",
                oninput: move |evt| content_form.with_mut(|form| form.tags = evt.value.clone()),
                value: "{content_form.with(|form| form.tags.clone())}",
            }
            label {
                r#for: "content_type",
                "Content Type",
            }
            select {
                name: "content_type",
                oninput: move |evt| content_form.with_mut(|form| form.content_type = evt.value.clone()),
                value: "{content_form.with(|form| form.content_type.clone())}",
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
                oninput: move |evt| {
                    content_form.with_mut(|form| form.date = evt.value.clone());
                    if *form_mode.current() == FormMode::Create {
                        content_form.with_mut(|form| form.content_entry_id = evt.value.clone());
                    }
                },
                value: "{content_form.with(|form| form.date.clone())}",
            }
            label {
                r#for: "is_pinned",
                "Pinned"
            }
            input {
                r#type: "checkbox",
                name: "is_pinned",
                oninput: move |evt| content_form.with_mut(|form| form.is_pinned = checkbox_value(evt.value.clone()).unwrap() ),
                value: "{content_form.with(|form| form.is_pinned.clone())}"
            }
            label {
                r#for: "content",
                "Body Content"
            }
            textarea {
                name: "content",
                oninput: move |evt| content_form.with_mut(|form| form.content = evt.value.clone()),
                value: "{content_form.with(|form| form.content.clone())}"
            }
            if *form_mode.current() == FormMode::Edit {
                rsx!{
                    button {
                        onclick: move |_| {
                            if content_form.with(|form| form.content_entry_id.len() > 0) {
                                form_mode.set(FormMode::Delete);
                                form_state.set(FormState::Submitted);
                            }
                        },
                        "Delete",
                    }
                }
            }
            button {
                onclick: move |_| {
                    content_form.set(ContentForm::new());
                    form_state.set(FormState::Closed);
                },
                "Cancel",
            }
            button {
                onclick: move |_| {
                    if content_form.with(|form| form.is_ok()) {
                        form_state.set(FormState::Submitted);
                    }
                    
                },
                "Submit",
            }

            
        }
    })
}


#[inline_props]
fn ContentFormSubmitted<'a>(
    cx: Scope, 
    content_form: &'a UseRef<ContentForm>, 
    form_state: &'a UseState<FormState>,
    form_mode: FormMode,
    database_path: String,
) -> Element {
    let future = use_future(cx, (), |_| {
        to_owned![database_path, form_mode];
        let mut row = ContentRow::new(content_form.read().clone());
        async move {
            match form_mode {
                FormMode::Delete => ContentRow::delete_content_row(database_path.clone(), row.content_entry_id.unwrap()).await,
                _ => row.enter_content_row(database_path.clone(), form_mode).await,

            }
        }
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