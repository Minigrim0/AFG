use std::iter::Peekable;

use crate::error::{TokenError, TokenErrorType};
use crate::token::{Token, TokenType};

use super::super::token::get_until;
use super::node::{parse_block, CodeBlock};

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<String>,
    pub content: CodeBlock,
}

impl Function {
    pub fn new(name: String) -> Self {
        Self {
            name,
            parameters: vec![],
            content: vec![],
        }
    }

    pub fn parse<T: Iterator<Item = Token>>(stream: &mut Peekable<T>) -> Result<Self, TokenError> {
        if let Some(function_name) = stream.next() {
            let fn_meta = function_name.meta;

            if let Some(next_token) = stream.next() {
                if next_token.token_type != TokenType::LPAREN {
                    return Err(TokenError::new(
                        TokenErrorType::UnexpectedToken,
                        "Unexpected token after function name, expected (",
                        Some(next_token.meta)
                    ));
                }
            } else {
                return Err(TokenError::new(
                    TokenErrorType::UnexpectedEndOfStream,
                    "Expected LPAREN after function name",
                    Some(fn_meta)
                ));
            }

            let args = get_until(stream, TokenType::RPAREN, false);

            if let Some(next_token) = stream.next() {
                if next_token.token_type != TokenType::LBRACE {
                    return Err(TokenError::new(
                        TokenErrorType::UnexpectedToken,
                        "Unexpected token after function parameters, expected {",
                        Some(next_token.meta)
                    ));
                }
            } else {
                return Err(TokenError::new(
                    TokenErrorType::UnexpectedEndOfStream,
                    "Expected LBRACE after function parameters",
                    Some(fn_meta)
                ));
            }

            Ok(Self {
                name: match function_name.value {
                    Some(v) => v,
                    None => unreachable!(),
                },
                parameters: args
                    .map(|arg| arg.value.unwrap().replace(",", ""))
                    .collect::<Vec<String>>(),
                content: parse_block(stream)?,
            })
        } else {
            Err(TokenError::new(
                TokenErrorType::UnexpectedEndOfStream,
                "Missing function name",
                None
            ))
        }
    }
}
