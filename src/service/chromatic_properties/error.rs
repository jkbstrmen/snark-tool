use std::fmt;
use crate::error::{ProcedureError, Error};

#[derive(Debug)]
pub struct ChromaticPropertiesError {
    pub message: String,
}

impl ChromaticPropertiesError {
    pub fn new(message: impl AsRef<str>) -> Self {
        ChromaticPropertiesError { message: String::from(message.as_ref()) }
    }
}

impl fmt::Display for ChromaticPropertiesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "chromatic properties procedure error: {}", self.message)?;
        Ok(())
    }
}

impl From<ChromaticPropertiesError> for Error {
    fn from(error: ChromaticPropertiesError) -> Self {
        let message = format!("{}", error);
        Error::ProcedureError(ProcedureError{ message })
    }
}

impl From<Error> for ChromaticPropertiesError {
    fn from(error: Error) -> Self {
        let message = format!("{}", error);
        ChromaticPropertiesError::new(message)
    }
}

impl From<serde_json::error::Error> for ChromaticPropertiesError {
    fn from(error: serde_json::error::Error) -> Self {
        let message = format!("serde json error: {}", error);
        ChromaticPropertiesError { message }
    }
}

