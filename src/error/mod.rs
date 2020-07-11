use crate::service::io::error::{ReadError, WriteError};
use std::fmt;

#[derive(Debug)]
pub enum Error {
    ReadError(ReadError),
    WriteError(WriteError),
    ProcedureError(ProcedureError),
    ConfigError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ReadError(error) => write!(f, "read error: {:?}", error)?,
            Error::WriteError(error) => write!(f, "write error: {:?}", error)?,
            Error::ConfigError(msg) => write!(f, "config error: {}", msg)?,
            Error::ProcedureError(error) => write!(f, "procedure error: {}", error)?,
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

impl From<ProcedureError> for Error {
    fn from(error: ProcedureError) -> Self {
        Error::ProcedureError(error)
    }
}

#[derive(Debug)]
pub struct ProcedureError {
    pub message: String,
}

impl fmt::Display for ProcedureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "procedure error: {}", self.message)?;
        Ok(())
    }
}
