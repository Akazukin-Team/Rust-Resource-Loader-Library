use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum ResourceError {
    NotFound { identifier: String },
    Fetch { identifier: String, cause: String },
    UnsupportedOperation(String),
}

impl std::error::Error for ResourceError {}

impl Display for ResourceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            ResourceError::NotFound { identifier } => {
                write!(
                    f,
                    "The requested resource was not found; Identifier: {}",
                    identifier
                )
            }
            ResourceError::Fetch { identifier, cause } => {
                write!(
                    f,
                    "Failed to fetch the resource; Identifier: {}; Cause: {}",
                    identifier, cause
                )
            }
            ResourceError::UnsupportedOperation(msg) => {
                write!(f, "Unsupported operation: {}", msg)
            }
        }
    }
}
