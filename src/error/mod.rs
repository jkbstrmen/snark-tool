use crate::service::io::error::ReadError;
use std::{error, fmt};

#[derive(Debug)]
pub enum Error {
    IoError(ReadError),
    ColourError,
    ConfigError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::IoError(error) => write!(f, "io error: {:?}", error),
            Error::ColourError => write!(f, "Colour error"),
            Error::ConfigError(msg) => write!(f, "Config error: {}", msg),
        };
        Ok(())
    }
}

impl From<ReadError> for Error {
    fn from(err: ReadError) -> Self {
        Error::IoError(err)
    }
}
