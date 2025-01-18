use std::collections::HashMap;
use super::token::{Token, TokenStream, TokenType};
use super::node::ASTBlockNode;

/// Parses a block of code (within a function)
fn parse_block(stream: &mut TokenStream) -> Result<Vec<ASTBlockNode>, String> {
    let mut block_tree = vec![];
    while let Some(token) = stream.next() {
        match token.ttype {
            TokenType::KEYWORD if token.value == Some("set".to_string()) => {
                let assignment = ASTBlockNode::parse_assignment(stream)?;
                block_tree.push(assignment);
            },
            TokenType::KEYWORD if token.value == Some("while".to_string()) => {
                let mut while_block = ASTBlockNode::parse_while(stream)?;
                let while_content = parse_block(stream)?;
                while_block.descendents.2 = while_content;
                block_tree.push(while_block);
            }
            TokenType::KEYWORD if token.value == Some("if".to_string()) => {
                let mut if_block = ASTBlockNode::parse_if(stream)?;
                let if_content = parse_block(stream)?;
                if_block.descendents.2 = if_content;
                block_tree.push(if_block);
            }
            TokenType::KEYWORD if token.value == Some("return".to_string()) => {
                block_tree.push(ASTBlockNode::new_return())
            }
            TokenType::KEYWORD if token.value == Some("call".to_string()) => {
                if let Some(function_name) = stream.next() {
                    let func_id = match function_name.value {
                        Some(v) => v,
                        None => unreachable!()
                    };

                    if stream.next().and_then(|t| Some(t.ttype)) != Some(TokenType::LPAREN) {
                        return Err("Unexpected token after function name, expected (".to_string());
                    }

                    let mut args = stream.get_until(TokenType::RPAREN);
                    args.pop();  // Pop the RPAREN

                    let function_call = ASTBlockNode::new_function_call(func_id, args);
                    block_tree.push(function_call);
                } else {
                    return Err("Missing function name after call keyword".to_string())
                }
            }
            TokenType::KEYWORD if token.value == Some("loop".to_string()) => {
                if let Some(t) = stream.next() {
                    if t.ttype != TokenType::LBRACE {
                        return Err(format!("Unexpected token {:?} after loop keyword, expected LBRACE", t.ttype))
                    }
                } else {
                    return Err("Expected LBRACE after loop keyword".to_string());
                }

                let mut loop_block = ASTBlockNode::new_loop();
                let loop_content = parse_block(stream)?;
                loop_block.descendents.2 = loop_content;
                block_tree.push(loop_block);
            }
            TokenType::COMMENT => {
                // Skip the whole comment
                stream.get_until(TokenType::ENDL);
                continue;
            }
            TokenType::ENDL => {
                continue
            }
            TokenType::RBRACE => {
                break;
            },
            t => println!("Unexpected or unhandled token: {:?} {:?}", t, token.value)
        }
    }

    Ok(block_tree)
}

/// Parses a function (function are building blocks of the language)
fn parse_function(stream: &mut TokenStream) -> Result<(Token, Vec<Token>, Vec<ASTBlockNode>), String> {
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
