use std::fmt;

#[derive(Copy, Clone)]
pub enum FormState {
    Closed,
    Active,
    Submitted,
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