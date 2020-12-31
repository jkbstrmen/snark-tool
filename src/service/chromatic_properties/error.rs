use std::fmt;

#[derive(Debug)]
pub struct ChromaticPropertiesError {
    pub message: String,
}

impl ChromaticPropertiesError {
    pub fn new(message: impl AsRef<str>) -> Self {
        ChromaticPropertiesError {
            message: String::from(message.as_ref()),
        }
    }
}

impl fmt::Display for ChromaticPropertiesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "chromatic properties error: {}", self.message)?;
        Ok(())
    }
}
