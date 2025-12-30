use std::error;
use std::fmt;

#[derive(Debug)]
pub struct APIError {
    pub name: String,
    pub message: String,
}

impl APIError {
    pub fn new(name: &str, err: impl error::Error) -> Self {
        Self {
            name: name.to_string(),
            message: err.to_string(),
        }
    }
    pub fn new_msg(name: &str, message: &str) -> Self {
        Self {
            name: name.to_string(),
            message: message.to_string(),
        }
    }
}

impl fmt::Display for APIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.message)
    }
}

impl error::Error for APIError {}
