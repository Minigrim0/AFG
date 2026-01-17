use std::fmt;

/// A semantic error in the program being compiled
pub enum SemanticError {
    UnknownVariable(String),  // Use of a previously undeclared variable
    InvalidOperation(String), // Invalid operation
    UnknownFunction(String), // Call to an undefined function
    InvalidFunctionCall(String), // Function called with incorrect number of parameters
}

impl fmt::Display for SemanticError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::UnknownVariable(value) => write!(f, "[Semantic] Unknown Variable: {}", value),
            Self::InvalidOperation(value) => write!(f, "[Semantic] Invalid Operation: {}", value),
            Self::UnknownFunction(value) => write!(f, "[Semantic] Unknown Function: {}", value),
            Self::InvalidFunctionCall(value) => write!(f, "[Semantic] Invalid Function Call: {}", value),
        }
    }
}
