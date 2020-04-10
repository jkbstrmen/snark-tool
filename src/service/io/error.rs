use serde::export::Formatter;
use std::num::ParseIntError;
use std::{fmt, io, num};

#[derive(Debug)]
pub struct ReadError {
    pub message: String,
}

impl fmt::Display for ReadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "read error: {}", self.message)?;
        Ok(())
    }
}

impl From<num::ParseIntError> for ReadError {
    fn from(error: ParseIntError) -> Self {
        let message = format!("nested parse int error: {}", error);
        ReadError { message }
    }
}

#[derive(Debug)]
pub struct WriteError {
    pub message: String,
}

impl From<io::Error> for WriteError {
    fn from(error: io::Error) -> Self {
        let message = format!("nested io error: {}", error);
        WriteError { message }
    }
}

impl From<ReadError> for WriteError {
    fn from(error: ReadError) -> Self {
        let message = format!("nested read error: {}", error);
        WriteError { message }
    }
}
