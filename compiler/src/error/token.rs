use std::fmt;

use crate::token::TokenMetaData;

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
    metadata: Option<TokenMetaData>,
}

impl fmt::Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(md) = self.metadata {
            write!(
                f,
                "[Token] {:?}: {} at line {}, char {}",
                self.error_type, self.text, md.line, md.char
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
        metadata: Option<TokenMetaData>,
    ) -> Self {
        Self {
            error_type,
            text: text.as_ref().to_string(),
            metadata,
        }
    }
}
