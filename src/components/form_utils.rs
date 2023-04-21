use std::fmt;
use dioxus::prelude::*;

pub fn use_database_path(cx: &ScopeState) -> UseSharedState<Option<String>> {
    use_shared_state::<Option<String>>(cx).expect("Could not read database_path").clone()
}

pub fn use_dist_path(cx: &ScopeState) -> UseSharedState<Option<String>> {
    use_shared_state::<Option<String>>(cx).expect("Could not read dist path").clone()
}


#[derive(Copy, Clone)]
pub enum FormState {
    Closed,
    Active,
    Submitted,
}

#[derive(Copy, Clone, PartialEq)]
pub enum FormMode {
    Edit,
    Create,
    Delete,
}

#[derive(Debug, Clone)]
pub struct CheckboxError;

impl fmt::Display for CheckboxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid string to bool")
    }
}

pub fn checkbox_value(value: String) -> Result<bool, CheckboxError> {
    match value.as_str() {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(CheckboxError)
    }
}