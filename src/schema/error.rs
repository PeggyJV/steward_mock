use std::{error::Error, fmt};

#[derive(Debug)]
pub enum SchemaError {
    SerializationError(String),
}

impl Error for SchemaError {}

impl fmt::Display for SchemaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SchemaError::SerializationError(msg) => write!(f, "serialization failed: {}", msg),
        }
    }
}
