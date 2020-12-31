use std::fmt;

#[derive(Debug)]
pub struct ConstructionError {
    pub message: String,
}

impl ConstructionError {
    pub fn new(message: impl AsRef<str>) -> Self {
        ConstructionError {
            message: String::from(message.as_ref()),
        }
    }
}

impl fmt::Display for ConstructionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "construction error: {}", self.message)?;
        Ok(())
    }
}
