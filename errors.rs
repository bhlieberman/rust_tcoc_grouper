use thiserror::Error;
use std::fmt;


#[derive(Error, Debug)]
pub struct FoundationError {
    cause: String,
    backtrace: String,
    enable_suppression: bool,
    traceback: bool,
}

impl std::fmt::Display for FoundationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}