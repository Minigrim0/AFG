use super::token::{Token, TokenType};
use std::fmt;
use std::iter::Peekable;

pub fn ensure_next_token<T: Iterator<Item = Token>>(
    stream: &mut Peekable<T>,
    expected_ttype: TokenType,
    expected_value: Option<String>,
) -> Result<(), String> {
    match stream.next() {
        Some(t) => match (t.token_type, t.value) {
            (ttype, value) if ttype == expected_ttype && value == expected_value => Ok(()),
            (ttype, value) if ttype == expected_ttype => Err(format!(
                "Expected next token to have value {:?} but found {:?}",
                expected_value, value
            )),
            (ttype, value) if value == expected_value => Err(format!(
                "Expected next token to have type {:?} but found {:?}",
                expected_ttype, ttype
            )),
            (ttype, value) => Err(format!(
                "Expected next token to have type {:?} and value {:?} but found {:?}, {:?}",
                expected_ttype, expected_value, ttype, value
            )),
        },
        None => Err("Expected a token but found nothing".to_string()),
    }
}

pub fn get_until<T: Iterator<Item = Token>>(
    stream: &mut T,
    token_type: TokenType,
    include_last: bool,
) -> Peekable<impl Iterator<Item = Token> + ExactSizeIterator + fmt::Debug> {
    let mut tokens = vec![];
    while let Some(token) = stream.next() {
        let is_end_token = token.token_type == token_type;
        if !is_end_token || (is_end_token && include_last) {
            tokens.push(token);
        }
        if is_end_token {
            break;
        }
    }
    tokens.into_iter().peekable()
}
