use crate::procedure::error::{Error, ProcedureError};
use crate::service::chromatic_properties::error::ChromaticPropertiesError;

impl From<ChromaticPropertiesError> for Error {
    fn from(error: ChromaticPropertiesError) -> Self {
        let message = format!("{}", error);
        Error::ProcedureError(ProcedureError { message })
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
