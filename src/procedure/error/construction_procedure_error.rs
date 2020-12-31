use crate::procedure::error::{Error, ProcedureError};
use crate::service::constructions::error::ConstructionError;

impl From<ConstructionError> for Error {
    fn from(error: ConstructionError) -> Self {
        let message = format!("{}", error);
        Error::ProcedureError(ProcedureError { message })
    }
}

impl From<Error> for ConstructionError {
    fn from(error: Error) -> Self {
        let message = format!("{}", error);
        ConstructionError::new(message)
    }
}
