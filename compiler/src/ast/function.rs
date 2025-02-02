use std::iter::Peekable;

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
    pub fn parse<T: Iterator<Item = Token>>(stream: &mut Peekable<T>) -> Result<Self, String> {
        if let Some(function_name) = stream.next() {
            if stream.next().and_then(|t| Some(t.token_type)) != Some(TokenType::LPAREN) {
                return Err("Unexpected token after function name, expected (".to_string());
            }

            let args = get_until(stream, TokenType::RPAREN, false);

            if stream.next().and_then(|t| Some(t.token_type)) != Some(TokenType::LBRACE) {
                return Err("Unexpected token after function name, expected {".to_string());
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
            Err("Missing function name".to_string())
        }
    }
}
