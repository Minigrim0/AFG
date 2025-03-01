use std::fmt;

use super::types::TokenType;

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: Option<String>,
    pub line: usize,
    pub char: usize,
}

impl Token {
    pub fn new(token_type: TokenType, value: Option<String>, line: usize, char: usize) -> Self {
        Self {
            token_type,
            value,
            line,
            char,
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

    pub fn get_literal_value(&self) -> Result<i32, String> {
        if let Some(value) = &self.value {
            value.parse::<i32>().map_err(|e| e.to_string())
        } else {
            Err("Token has no value".to_string())
        }
    }

    pub fn get_value(&self) -> Result<String, String> {
        if let Some(value) = &self.value {
            Ok(value.clone())
        } else {
            Err("Token as no value".to_string())
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
