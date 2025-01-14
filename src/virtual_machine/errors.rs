use std::fmt;
use thiserror::Error;

#[derive(fmt::Debug, Error)]
pub struct ParsingError {
    line: u32,
    msg: String
}

impl ParsingError {
    pub fn new(line: u32, msg: String) -> Self {
        ParsingError {
            line,
            msg
        }
    }
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParseError: Error on line {}: {}", self.line, self.msg)
    }
}
