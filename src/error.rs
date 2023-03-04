use std::fmt::Error;

pub enum HectaError {
    FileIO,
    InvalidOperation(String),
    InvalidCursorPos(String),
    StandardError(Error),
}

impl From<Error> for HectaError {
    fn from(e: Error) -> Self {
        HectaError::StandardError(e)
    }
}
