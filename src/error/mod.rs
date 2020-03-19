use crate::service::io::error::ReadError;
use std::{error, fmt};

#[derive(Debug)]
pub enum Error {
    IoError(ReadError),
    ColourError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::IoError(error) => write!(f, "io error: {:?}", error),

            // MyError::CannotOpenFile(e) => write!(f, "error while opening file: {}", e),
            // ....
            Error::ColourError => write!(f, "Colour error"),
        };
        Ok(())
    }
}

// impl error::Error for Error {
//
// }

impl From<ReadError> for Error {
    fn from(err: ReadError) -> Self {
        Error::IoError(err)
    }
}
