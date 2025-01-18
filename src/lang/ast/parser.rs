use std::collections::HashMap;
use super::token::{Token, TokenStream, TokenType};
use super::node::{parse_block, CodeBlock};

/// Parses a function (function are building blocks of the language)
fn parse_function(stream: &mut TokenStream) -> Result<(Token, Vec<Token>, CodeBlock), String> {
    if let Some(function_name) = stream.next() {
        if stream.next().and_then(|t| Some(t.ttype)) != Some(TokenType::LPAREN) {
            return Err("Unexpected token after function name, expected (".to_string());
        }

        let mut args = stream.get_until(TokenType::RPAREN);
        args.pop();  // Pop the RPAREN

        if stream.next().and_then(|t| Some(t.ttype)) != Some(TokenType::LBRACE) {
            return Err("Unexpected token after function name, expected {".to_string());
        }

        Ok((function_name, args, parse_block(stream)?))
    } else {
        Err("Missing function name".to_string())
    }
}

/// Parses a full program from text. A program is expected to be composed only of functions
pub fn parse_program<S: AsRef<str>>(text: S) -> Result<HashMap<String, super::Function>, String> {
    let mut tokens = TokenStream::from_text(text.as_ref().to_string());

    let mut program = HashMap::new();

    while let Some(token) = tokens.next() {
        match &token.ttype {
            TokenType::KEYWORD if token.value == Some("fn".to_string()) => {
                let (func_name, args, content) = parse_function(&mut tokens)?;

                let function = super::Function {
                    name: match func_name.value {
                        Some(v) => v,
                        None => unreachable!()
                    },
                    parameters: args.into_iter().map(|arg| arg.value.unwrap().replace(",", "")).collect::<Vec<String>>(),
                    content
                };
                program.insert(function.name.clone(), function);
            },
            TokenType::ENDL => continue,
            ttype => return Err(format!("Unexpected token {:?} {:?}", ttype, token.value))
        }
    }

    Ok(program)
}
