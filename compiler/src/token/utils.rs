use std::fmt;
use std::iter::Peekable;

use crate::error::{TokenError, TokenErrorType};

use super::{kind::TokenType, Token};

pub fn ensure_next_token<T: Iterator<Item = Token>>(
    stream: &mut Peekable<T>,
    expected_ttype: TokenType,
    expected_value: Option<String>,
) -> Result<(), TokenError> {
    match stream.next() {
        Some(t) => match (t.token_type, t.value) {
            (ttype, value) if ttype == expected_ttype && value == expected_value => Ok(()),
            (ttype, value) if ttype == expected_ttype => Err(TokenError::new(
                TokenErrorType::UnexpectedToken,
                format!(
                    "Expected next token to have value {:?} but found {:?}",
                    expected_value, value
                ),
                Some(t.meta),
            )),
            (ttype, value) if value == expected_value => Err(TokenError::new(
                TokenErrorType::UnexpectedToken,
                format!(
                    "Expected next token to have type {:?} but found {:?}",
                    expected_ttype, ttype
                ),
                Some(t.meta),
            )),
            (ttype, value) => Err(TokenError::new(
                TokenErrorType::UnexpectedToken,
                format!(
                    "Expected next token to have type {:?} and value {:?} but found {:?}, {:?}",
                    expected_ttype, expected_value, ttype, value
                ),
                Some(t.meta),
            )),
        },
        None => Err(TokenError::new(
            TokenErrorType::UnexpectedEndOfStream,
            "Expected a token but found nothing".to_string(),
            None,
        )),
    }
}

/// Consumes the stream until a token of the given type is found,
/// returning an peekable iterator over the collected tokens
/// e.g.
/// ```rust
/// use std::iter::Peekable;
///
/// use afgcompiler::token::{Token, TokenType, get_until};
///
/// let mut stream : Peekable<_> = vec![
///     Token::new(TokenType::ID, Some("test".to_string()), 0, 0),
///     Token::new(TokenType::OP, Some("=".to_string()), 0, 0),
///     Token::new(TokenType::ID, Some("12".to_string()), 0, 0),
///     Token::new(TokenType::ENDL, None, 0, 0),
/// ].into_iter().peekable();
/// let stuff = get_until(&mut stream, TokenType::ENDL, true);
/// assert_eq!(stuff.last().and_then(|t| Some(t.token_type)), Some(TokenType::ENDL));
/// ```
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
