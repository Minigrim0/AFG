use std::collections::VecDeque;

use crate::lang::token::{ensure_next_token, Token, TokenStream, TokenType};

#[derive(Debug, Default)]
pub enum ComparisonType {
    GT,
    GE,
    #[default]
    EQ,
    LE,
    LT,
    DIFF,
}

#[derive(Debug, Default)]
pub enum OperationType {
    #[default]
    Addition,
    Substraction,
    Multiplication,
    Division,
    Modulo,
}

pub type CodeBlock = Vec<Box<Node>>;

#[derive(Debug)]
pub enum Node {
    Identifier {
        name: String,
    },
    Litteral {
        value: i32,
    },
    Assignment {
        lparam: Box<Node>,
        rparam: Box<Node>,
    },
    Operation {
        lparam: Box<Node>,
        rparam: Box<Node>,
        operation: OperationType,
    },
    Print {
        value: Box<Node>,
    },
    Comparison {
        lparam: Box<Node>,
        rparam: Box<Node>,
        comparison: ComparisonType,
    },
    WhileLoop {
        condition: Box<Node>, // Should be a comparison
        content: CodeBlock,
    },
    Loop {
        content: CodeBlock,
    },
    IfCondition {
        condition: Box<Node>, // Should be a Comparison
        content: CodeBlock,
    },
    FunctionCall {
        function_name: String,
        parameters: CodeBlock, // A list of identifiers or literals
    },
    Return {
        value: Option<String>,
    },
}

impl Default for Node {
    fn default() -> Self {
        Node::Litteral { value: 0 }
    }
}

fn new_id_or_litteral(token: Token) -> Result<Node, String> {
    if token.token_type != TokenType::ID {
        return Err(format!(
            "Unexpected token type {:?} expected and ID or a litteral",
            token.token_type
        ));
    }

    if token.is_literal() {
        Ok(Node::Litteral {
            value: match token.value {
                Some(v) => match v.parse::<i32>() {
                    Ok(v) => v,
                    Err(_) => unreachable!(),
                },
                None => return Err("Token should be ID or Litteral but has no value !".to_string()),
            },
        })
    } else {
        Ok(Node::Identifier {
            name: match token.value {
                Some(v) => v,
                None => return Err("Token should have a value".to_string()),
            },
        })
    }
}

fn new_operator(operator: String, lparam: Box<Node>, rparam: Box<Node>) -> Result<Node, String> {
    Ok(Node::Operation {
        rparam,
        lparam,
        operation: match operator.as_str() {
            "+" => OperationType::Addition,
            "-" => OperationType::Substraction,
            "*" => OperationType::Multiplication,
            "/" => OperationType::Division,
            "%" => OperationType::Modulo,
            op => {
                return Err(format!(
                    "Unknown operator {}, expected one of +, -, *, /, %",
                    op
                ))
            }
        },
    })
}

fn new_comp_operator(
    operator: String,
    lparam: Box<Node>,
    rparam: Box<Node>,
) -> Result<Node, String> {
    Ok(Node::Comparison {
        lparam,
        rparam,
        comparison: match operator.as_str() {
            ">" => ComparisonType::GT,
            ">=" => ComparisonType::GE,
            "==" => ComparisonType::EQ,
            "!=" => ComparisonType::DIFF,
            "<=" => ComparisonType::LE,
            "<" => ComparisonType::LT,
            op => {
                return Err(format!(
                    "Unknown comparison operator {}, expected one of >, >=, ==, !=, <=, <",
                    op
                ))
            }
        },
    })
}

fn new_return(value: Option<String>) -> Node {
    Node::Return { value }
}

fn new_loop(stream: &mut TokenStream) -> Result<Node, String> {
    Ok(Node::Loop {
        content: parse_block(stream)?,
    })
}

fn new_function_call(func_id: String, params: Vec<Token>) -> Result<Node, String> {
    let mut parameters = vec![];
    for parameter in params.into_iter() {
        let param = Box::from(new_id_or_litteral(parameter)?);
        parameters.push(param);
    }

    Ok(Node::FunctionCall {
        function_name: func_id,
        parameters,
    })
}

/// Parses a tri-comparison expression (e.g. $a < 10)
fn parse_tricomp(tokens: &mut VecDeque<Token>) -> Result<Node, String> {
    match tokens.len() {
        3 => {}
        other => {
            return Err(format!(
                "Expected three tokens to match a tri-comp but found {}",
                other
            ))
        }
    };

    let lparam = tokens
        .pop_front()
        .and_then(|rp| Some(new_id_or_litteral(rp)))
        .unwrap_or(Err(
            "Unable to find the rparameter for the comparison operation".to_string(),
        ))?;

    let operator = tokens
        .pop_front()
        .and_then(|op| Some(Ok(op.value.unwrap_or("MISSING_TOKEN".to_string()))))
        .unwrap_or(Err("Unable to find the comparison operator".to_string()))?;

    let rparam = tokens
        .pop_front()
        .and_then(|lp| Some(new_id_or_litteral(lp)))
        .unwrap_or(Err(
            "Unable to find the rparameter for the comparison operation".to_string(),
        ))?;

    new_comp_operator(operator, Box::from(lparam), Box::from(rparam))
}

/// Parses a tri-operation expression (e.g. $a + 10)
fn parse_triop(tokens: &mut VecDeque<Token>) -> Result<Node, String> {
    match tokens.len() {
        3 => {}
        other => {
            return Err(format!(
                "Expected three tokens to match a tri-ops but found {}",
                other
            ))
        }
    };

    let lparam = tokens
        .pop_front()
        .and_then(|rp| Some(new_id_or_litteral(rp)))
        .unwrap_or(Err(
            "Unable to find the rparameter for the comparison operation".to_string(),
        ))?;

    let operator = tokens
        .pop_front()
        .and_then(|op| Some(Ok(op.value.unwrap_or("MISSING_TOKEN".to_string()))))
        .unwrap_or(Err("Unable to find the comparison operator".to_string()))?;

    let rparam = tokens
        .pop_front()
        .and_then(|lp| Some(new_id_or_litteral(lp)))
        .unwrap_or(Err(
            "Unable to find the rparameter for the comparison operation".to_string(),
        ))?;

    new_operator(operator, Box::from(lparam), Box::from(rparam))
}

fn parse_while(stream: &mut TokenStream) -> Result<Node, String> {
    let mut assignment_stuff = VecDeque::from(stream.get_until(TokenType::LBRACE));
    assignment_stuff.pop_back(); // Remove Lbrace

    let condition = match assignment_stuff.len() {
        3 => parse_tricomp(&mut assignment_stuff)?,
        other => {
            return Err(format!(
                "Expected 3 tokens after `while` keyword but found {} [{:?}]",
                other,
                assignment_stuff.iter().map(|s| &s.token_type)
            ))
        }
    };

    Ok(Node::WhileLoop {
        condition: Box::from(condition),
        content: parse_block(stream)?,
    })
}

fn parse_assignment(stream: &mut TokenStream) -> Result<Node, String> {
    let mut assignment_stuff = VecDeque::from(stream.get_until(TokenType::ENDL));
    assignment_stuff.pop_back(); // Remove endl

    let lparam = match assignment_stuff.pop_front() {
        Some(token) => match new_id_or_litteral(token)? {
            Node::Identifier { name } => Node::Identifier { name },
            _ => {
                return Err("Lparam of assignment must be an identifier not a litteral".to_string())
            }
        },
        None => return Err("Expected identifier after `set` keyword".to_string()),
    };

    ensure_next_token(&mut assignment_stuff, TokenType::OP, Some("=".to_string()))?;

    let rparam = if assignment_stuff.iter().any(|t: &Token| match t.token_type {
        TokenType::LPAREN | TokenType::RPAREN => true,
        _ => false,
    }) {
        parse_function(&mut TokenStream {
            tokens: assignment_stuff,
        })?
    } else {
        match assignment_stuff.len() {
            1 => match assignment_stuff.pop_front() {
                Some(token) => new_id_or_litteral(token)?,
                None => return Err("Expected token after assignment operation".to_string()),
            },
            3 => parse_triop(&mut assignment_stuff)?,
            t => {
                return Err(format!(
                    "Expected one or three tokens after assignment operator but found {}",
                    t
                ))
            }
        }
    };

    Ok(Node::Assignment {
        lparam: Box::from(lparam),
        rparam: Box::from(rparam),
    })
}

fn parse_function(stream: &mut TokenStream) -> Result<Node, String> {
    if let Some(function_name) = stream.next() {
        let func_id = match function_name.value {
            Some(v) => v,
            None => unreachable!(),
        };

        if stream.next().and_then(|t| Some(t.token_type)) != Some(TokenType::LPAREN) {
            return Err("Unexpected token after function name, expected (".to_string());
        }

        let mut args = stream.get_until(TokenType::RPAREN);
        args.pop(); // Pop the RPAREN

        let function_call = new_function_call(func_id, args)?;
        Ok(function_call)
    } else {
        Err("Missing function name after call keyword".to_string())
    }
}

fn parse_if(stream: &mut TokenStream) -> Result<Node, String> {
    let mut assignment_stuff = VecDeque::from(stream.get_until(TokenType::LBRACE));
    assignment_stuff.pop_back(); // Remove Lbrace

    let condition = match assignment_stuff.len() {
        3 => parse_tricomp(&mut assignment_stuff)?,
        other => {
            return Err(format!(
                "Expected 3 tokens after `while` keyword bu found {}",
                other
            ))
        }
    };

    Ok(Node::IfCondition {
        condition: Box::from(condition),
        content: parse_block(stream)?,
    })
}

fn parse_print(stream: &mut TokenStream) -> Result<Node, String> {
    let mut assignment_stuff = VecDeque::from(stream.get_until(TokenType::ENDL));
    assignment_stuff.pop_back(); // Remove endl

    let value = match assignment_stuff.pop_front() {
        Some(token) => new_id_or_litteral(token)?,
        None => return Err("Expected identifier or litteral after `print` keyword".to_string()),
    };

    Ok(Node::Print {
        value: Box::from(value),
    })
}

/// Parses a block of code (within a function)
pub fn parse_block(stream: &mut TokenStream) -> Result<CodeBlock, String> {
    let mut block_tree = vec![];
    while let Some(token) = stream.next() {
        match token.token_type {
            TokenType::KEYWORD if token.value == Some("set".to_string()) => {
                let assignment = parse_assignment(stream)?;
                block_tree.push(Box::from(assignment));
            }
            TokenType::KEYWORD if token.value == Some("while".to_string()) => {
                let while_block = parse_while(stream)?;
                block_tree.push(Box::from(while_block));
            }
            TokenType::KEYWORD if token.value == Some("if".to_string()) => {
                let if_block = parse_if(stream)?;
                block_tree.push(Box::from(if_block));
            }
            TokenType::KEYWORD if token.value == Some("return".to_string()) => {
                if let Some(token) = stream.next() {
                    match token.token_type {
                        TokenType::ENDL => block_tree.push(Box::from(new_return(None))),
                        TokenType::ID => block_tree.push(Box::from(new_return(token.value))),
                        _ => continue,
                    }
                }
            }
            TokenType::KEYWORD if token.value == Some("call".to_string()) => {
                block_tree.push(Box::from(parse_function(stream)?));
            }
            TokenType::KEYWORD if token.value == Some("loop".to_string()) => {
                if let Some(t) = stream.next() {
                    if t.token_type != TokenType::LBRACE {
                        return Err(format!(
                            "Unexpected token {:?} after loop keyword, expected LBRACE",
                            t.token_type
                        ));
                    }
                } else {
                    return Err("Expected LBRACE after loop keyword".to_string());
                }

                let loop_block = new_loop(stream)?;
                block_tree.push(Box::from(loop_block));
            }
            TokenType::KEYWORD if token.value == Some("print".to_string()) => {
                let print_block = parse_print(stream)?;
                block_tree.push(Box::from(print_block));
            }
            TokenType::COMMENT => {
                // Skip the whole comment
                stream.get_until(TokenType::ENDL);
                continue;
            }
            TokenType::ENDL => continue,
            TokenType::RBRACE => {
                break;
            }
            t => println!("Unexpected or unhandled token: {:?} {:?}", t, token.value),
        }
    }

    Ok(block_tree)
}
