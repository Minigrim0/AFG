use std::fmt;

use crate::lexer::token::TokenLocation;

#[derive(Debug)]
pub enum TokenErrorType {
    Invalid,
    NotALiteral,
    UnexpectedToken,
    UnexpectedEndOfStream,
    ParseError,
    EmptyToken,
    InvalidArithmeticOperator,
    InvalidComparisonOperator,
}

#[derive(Debug)]
pub struct TokenError {
    error_type: TokenErrorType,
    text: String,
    location: Option<TokenLocation>,
}

impl fmt::Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(loc) = &self.location {
            write!(
                f,
                "[Token] {:?}: {} at line {}, column {}",
                self.error_type, self.text, loc.line, loc.column
            )
        } else {
            write!(f, "[Token] {:?}: {}.", self.error_type, self.text)
        }
    }
}

impl TokenError {
    pub fn new<S: AsRef<str>>(
        error_type: TokenErrorType,
        text: S,
        location: Option<TokenLocation>,
    ) -> Self {
        Self {
            error_type,
            text: text.as_ref().to_string(),
            location,
        }
    }
}
