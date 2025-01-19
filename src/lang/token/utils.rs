use std::collections::VecDeque;
use super::token::{Token, TokenType};

pub fn ensure_next_token(stream: &mut VecDeque<Token>, expected_ttype: TokenType, expected_value: Option<String>) -> Result<(), String> {
    match stream.pop_front() {
        Some(t) => match (t.token_type, t.value) {
            (ttype, value) if ttype == expected_ttype && value == expected_value => Ok(()),
            (ttype, value) if ttype == expected_ttype => Err(format!("Expected next token to have value {:?} but found {:?}", expected_value, value)),
            (ttype, value) if value == expected_value => Err(format!("Expected next token to have type {:?} but found {:?}", expected_ttype, ttype)),
            (ttype, value) => Err(format!("Expected next token to have type {:?} and value {:?} but found {:?}, {:?}", expected_ttype, expected_value, ttype, value)),
        },
        None => Err("Expected a token but found nothing".to_string()),
    }
}
