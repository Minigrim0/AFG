use std::fmt;

mod lexer;
mod types;
mod utils;

pub use lexer::lex;
pub use types::TokenType;
pub use utils::{ensure_next_token, get_until};

use crate::error::{TokenError, TokenErrorType};

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Copy)]
/// The metadata of a token, its line and starting character numbers
pub struct TokenMetaData {
    pub line: usize,
    pub char: usize,
}

#[derive(Debug, Clone)]
/// A token extracted from the raw string
pub struct Token {
    pub token_type: TokenType,
    pub value: Option<String>,
    pub meta: TokenMetaData,
}

impl Token {
    pub fn new(token_type: TokenType, value: Option<String>, line: usize, char: usize) -> Self {
        Self {
            token_type,
            value,
            meta: TokenMetaData { line, char },
        }
    }

    pub fn is_literal(&self) -> bool {
        if self.token_type != TokenType::ID {
            return false;
        }
        self.value.is_some()
            && self
                .value
                .as_ref()
                .and_then(|v| Some(v.parse::<i32>().is_ok()))
                == Some(true)
    }

    pub fn is(&self, of_type: TokenType) -> bool {
        self.token_type == of_type
    }

    pub fn get_literal_value(&self) -> Result<i32, TokenError> {
        if let Some(value) = &self.value {
            value.parse::<i32>().map_err(|e| {
                TokenError::new(
                    TokenErrorType::ParseError,
                    format!("Unable to parse token value: {e}"),
                    Some(self.meta),
                )
            })
        } else {
            Err(TokenError::new(
                TokenErrorType::NotALiteral,
                "Token is not a literal",
                Some(self.meta),
            ))
        }
    }

    pub fn get_value(&self) -> Result<String, TokenError> {
        if let Some(value) = &self.value {
            Ok(value.clone())
        } else {
            Err(TokenError::new(
                TokenErrorType::EmptyToken,
                "Token as no value",
                Some(self.meta),
            ))
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.token_type)?;
        if let Some(value) = self.value.clone() {
            write!(f, " = {}", value)?
        }
        Ok(())
    }
}
