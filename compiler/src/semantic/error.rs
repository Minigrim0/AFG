use std::fmt;

/// A semantic error in the program being compiled
pub enum SemanticError {
    UnknownVariable(String),  // Use of a previously undeclared variable
    InvalidOperation(String), // Invalid operation
}

impl fmt::Display for SemanticError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::UnknownVariable(value) => write!(f, "Unknown Variable: {}", value),
            Self::InvalidOperation(value) => write!(f, "Invalid Operation: {}", value),
        }
    }
}
