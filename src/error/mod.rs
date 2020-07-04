use crate::service::io::error::{ReadError, WriteError};
use std::fmt;

#[derive(Debug)]
pub enum Error {
    ReadError(ReadError),
    WriteError(WriteError),
    // ColourError,
    ConfigError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ReadError(error) => write!(f, "read error: {:?}", error)?,
            Error::WriteError(error) => write!(f, "write error: {:?}", error)?,
            // Error::ColourError => write!(f, "Colour error")?,
            Error::ConfigError(msg) => write!(f, "Config error: {}", msg)?,
        };
        Ok(())
    }
}

impl From<ReadError> for Error {
    fn from(err: ReadError) -> Self {
        Error::ReadError(err)
    }
}

impl From<WriteError> for Error {
    fn from(error: WriteError) -> Self {
        Error::WriteError(error)
    }
}
