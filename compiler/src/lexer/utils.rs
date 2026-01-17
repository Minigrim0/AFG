use std::fmt::{self, Display};

use nom_locate::LocatedSpan;

use crate::lexer::token::TokenLocation;

use super::token::Token;

pub type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug)]
pub struct LexerError {
    pub message: String,
    pub location: TokenLocation,
}

impl Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Lexer Error at line {}, column {}: {}",
            self.location.line, self.location.column, self.message
        )
    }
}

pub struct LexResult<'a> {
    pub tokens: Vec<Token<'a>>,
    pub errors: Vec<LexerError>,
}

impl<'a> LexResult<'a> {
    pub fn is_ok(&self) -> bool {
        self.errors.is_empty()
    }
}