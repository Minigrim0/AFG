use crate::prelude::*;
use crate::token::TokenType;

use super::node::{CodeBlock, parse_block};

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<String>,
    pub content: CodeBlock
}

impl Function {
    pub fn parse(stream: &mut TokenStream) -> Result<Self, String> {
        if let Some(function_name) = stream.next() {
            if stream.next().and_then(|t| Some(t.token_type)) != Some(TokenType::LPAREN) {
                return Err("Unexpected token after function name, expected (".to_string());
            }

            let mut args = stream.get_until(TokenType::RPAREN);
            args.pop();  // Pop the RPAREN

            if stream.next().and_then(|t| Some(t.token_type)) != Some(TokenType::LBRACE) {
                return Err("Unexpected token after function name, expected {".to_string());
            }

            Ok(Self {
                name: match function_name.value {
                    Some(v) => v,
                    None => unreachable!()
                },
                parameters: args.into_iter().map(|arg| arg.value.unwrap().replace(",", "")).collect::<Vec<String>>(),
                content: parse_block(stream)?
            })
        } else {
            Err("Missing function name".to_string())
        }
    }
}
