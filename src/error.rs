use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum HectaError {
    IOError(std::io::Error),
    InvalidOperation(String),
    InvalidCursorPos(String),
    // StandardError(std::io::Error),
}

impl Error for HectaError {
    // fn from(e: Error) -> Self {
    //     HectaError::StandardError(e)
    // }
}
impl From<std::io::Error> for HectaError {
    fn from(e: std::io::Error) -> Self {
        HectaError::IOError(e)
    }
}
impl Display for HectaError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
