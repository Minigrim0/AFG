use std::collections::VecDeque;

use super::token::{Token, TokenType, TokenStream};

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
    Modulo
}

pub type CodeBlock = Vec<Box<Node>>;

#[derive(Debug, Default)]
pub enum Node {
    Identifier {
        name: String
    },
    Litteral {
        value: i32
    },
    Assignment {
        lparam: Box<Node>,
        rparam: Box<Node>
    },
    Operation {
        lparam: Box<Node>,
        rparam: Box<Node>,
        operation: OperationType
    },
    Comparison {
        lparam: Box<Node>,
        rparam: Box<Node>,
        comparison: ComparisonType
    },
    WhileLoop {
        condition: Box<Node>, // Should be a comparison
        content: CodeBlock
    },
    Loop {
        content: CodeBlock
    },
    IfCondition {
        condition: Box<Node>, // Should be a Comparison
        content: CodeBlock
    },
    FunctionCall {
        function_name: String,
        parameters: CodeBlock,  // A list of identifiers or literals
    },
    #[default]
    Return,
}

fn new_id_or_litteral(token: Token) -> Result<Node, String> {
    if token.ttype != TokenType::ID {
        return Err(format!("Unexpected token type {:?} expected and ID or a litteral", token.ttype));
    }

    if token.is_litteral() {
        Ok(Node::Litteral {
            value: match token.value {
                Some(v) => match v.parse::<i32>() {
                    Ok(v) => v,
                    Err(_) => unreachable!()
                },
                None => return Err("Token should be ID or Litteral but has no value !".to_string())
            }
        })
    } else {
        Ok(Node::Identifier { name: match token.value {
                Some(v) => v,
                None => return Err("Token should have a value".to_string())
            }
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
            op => return Err(format!("Unknown operator {}, expected one of +, -, *, /, %", op))
        }
    })
}

fn new_comp_operator(operator: String, lparam: Box<Node>, rparam: Box<Node>) -> Result<Node, String> {
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
            op => return Err(format!("Unknown comparison operator {}, expected one of >, >=, ==, !=, <=, <", op))
        }
    })
}

fn new_return() -> Node {
    Node::Return
}

fn new_loop(stream: &mut TokenStream) -> Result<Node, String> {
    Ok(Node::Loop {
        content: parse_block(stream)?
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
        3 => {},
        other => return Err(format!("Expected three tokens to match a tri-comp but found {}", other))
    };

    let lparam = tokens
        .pop_front()
        .and_then(|rp| Some(new_id_or_litteral(rp)))
        .unwrap_or(Err("Unable to find the rparameter for the comparison operation".to_string()))?;

    let operator = tokens
        .pop_front()
        .and_then(|op| Some(Ok(op.value.unwrap_or("MISSING_TOKEN".to_string()))))
        .unwrap_or(Err("Unable to find the comparison operator".to_string()))?;

    let rparam = tokens
        .pop_front()
        .and_then(|lp| Some(new_id_or_litteral(lp)))
        .unwrap_or(Err("Unable to find the rparameter for the comparison operation".to_string()))?;

    new_comp_operator(operator, Box::from(lparam), Box::from(rparam))
}

/// Parses a tri-operation expression (e.g. $a + 10)
fn parse_triop(tokens: &mut VecDeque<Token>) -> Result<Node, String> {
    match tokens.len() {
        3 => {},
        other => return Err(format!("Expected three tokens to match a tri-ops but found {}", other))
    };

    let lparam = tokens
        .pop_front()
        .and_then(|rp| Some(new_id_or_litteral(rp)))
        .unwrap_or(Err("Unable to find the rparameter for the comparison operation".to_string()))?;

    let operator = tokens
        .pop_front()
        .and_then(|op| Some(Ok(op.value.unwrap_or("MISSING_TOKEN".to_string()))))
        .unwrap_or(Err("Unable to find the comparison operator".to_string()))?;

    let rparam = tokens
        .pop_front()
        .and_then(|lp| Some(new_id_or_litteral(lp)))
        .unwrap_or(Err("Unable to find the rparameter for the comparison operation".to_string()))?;

    new_operator(operator, Box::from(lparam), Box::from(rparam))
}

fn parse_while(stream: &mut TokenStream) -> Result<Node, String> {
    let mut assignment_stuff = VecDeque::from(stream.get_until(TokenType::LBRACE));
    assignment_stuff.pop_back();  // Remove Lbrace

    let condition = match assignment_stuff.len() {
        3 => parse_tricomp(&mut assignment_stuff)?,
        other => return Err(format!("Expected 3 tokens after `while` keyword but found {} [{:?}]", other, assignment_stuff.iter().map(|s| &s.ttype)))
    };

    Ok(Node::WhileLoop {
        condition: Box::from(condition),
        content: parse_block(stream)?
    })
}

fn parse_assignment(stream: &mut TokenStream) -> Result<Node, String> {
    let mut assignment_stuff = VecDeque::from(stream.get_until(TokenType::ENDL));
    assignment_stuff.pop_back();  // Remove endl

    let lparam = match assignment_stuff.pop_front() {
        Some(token) => match new_id_or_litteral(token)? {
            Node::Identifier { name } => Node::Identifier { name },
            _ => return Err("Lparam of assignment must be an identifier not a litteral".to_string())
        },
        None => return Err("Expected identifier after `set` keyword".to_string())
    };

    super::utils::ensure_next_token(&mut assignment_stuff, TokenType::OP, Some("=".to_string()))?;

    let rparam = match assignment_stuff.len() {
        1 => {
            match assignment_stuff.pop_front() {
                Some(token) => new_id_or_litteral(token)?,
                None => return Err("Expected token after assignment operation".to_string())
            }
        },
        3 => {
            parse_triop(&mut assignment_stuff)?
        },
        t => return Err(format!("Expected one or three tokens after assignment operator but found {}", t))
    };

    Ok(Node::Assignment {
        lparam: Box::from(lparam),
        rparam: Box::from(rparam)
    })
}

fn parse_if(stream: &mut TokenStream) -> Result<Node, String> {
    let mut assignment_stuff = VecDeque::from(stream.get_until(TokenType::LBRACE));
    assignment_stuff.pop_back();  // Remove Lbrace

    let condition = match assignment_stuff.len() {
        3 => parse_tricomp(&mut assignment_stuff)?,
        other => return Err(format!("Expected 3 tokens after `while` keyword bu found {}", other))
    };

    Ok(Node::IfCondition {
        condition: Box::from(condition),
        content: parse_block(stream)?
    })
}

/// Parses a block of code (within a function)
pub fn parse_block(stream: &mut TokenStream) -> Result<CodeBlock, String> {
    let mut block_tree = vec![];
    while let Some(token) = stream.next() {
        match token.ttype {
            TokenType::KEYWORD if token.value == Some("set".to_string()) => {
                let assignment = parse_assignment(stream)?;
                block_tree.push(Box::from(assignment));
            },
            TokenType::KEYWORD if token.value == Some("while".to_string()) => {
                let while_block = parse_while(stream)?;
                block_tree.push(Box::from(while_block));
            }
            TokenType::KEYWORD if token.value == Some("if".to_string()) => {
                let if_block = parse_if(stream)?;
                block_tree.push(Box::from(if_block));
            }
            TokenType::KEYWORD if token.value == Some("return".to_string()) => {
                block_tree.push(Box::from(new_return()))
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

                    let function_call = new_function_call(func_id, args)?;
                    block_tree.push(Box::from(function_call));
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

                let loop_block = new_loop(stream)?;
                block_tree.push(Box::from(loop_block));
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

/// Inner block printing
fn __print_block(block: CodeBlock, level: i32) {
    let mut prefix = String::new();
    for _ in 0..level {
        prefix.push_str(" |  ");
    }

    for inst in block.into_iter() {
        match *inst {
            Node::Identifier { name } => println!("{}ID {}", prefix, name),
            Node::Litteral { value } => println!("{}LIT {}", prefix, value),
            Node::Assignment { lparam, rparam } => {
                println!("{}Assignment", prefix);
                __print_block(vec![lparam], level+1);
                __print_block(vec![rparam], level+1);
            },
            Node::Operation { lparam, rparam, operation } => {
                println!("{}Operation {:?}", prefix, operation);
                __print_block(vec![lparam], level+1);
                __print_block(vec![rparam], level+1);
            },
            Node::Comparison { lparam, rparam, comparison } => {
                println!("{}Comparison {:?}", prefix, comparison);
                __print_block(vec![lparam], level+1);
                __print_block(vec![rparam], level+1);
            },
            Node::WhileLoop { condition, content } => {
                println!("{}While", prefix);
                __print_block(vec![condition], level+1);
                println!("{}Do", prefix);
                __print_block(content, level+1);
            },
            Node::Loop { content } => {
                println!("{}Loop", prefix);
                __print_block(content, level+1);
            },
            Node::IfCondition { condition, content } => {
                println!("{}If", prefix);
                __print_block(vec![condition], level+1);
                println!("{}Do", prefix);
                __print_block(content, level+1);
            }
            Node::FunctionCall { function_name, parameters } => {
                println!("{}Call {}", prefix, function_name);
                __print_block(parameters, level+1);
            }
            Node::Return => {
                println!("{}Return", prefix);
            }
        }
    }
}

pub fn print_block(block: CodeBlock) {
    __print_block(block, 0);
}
