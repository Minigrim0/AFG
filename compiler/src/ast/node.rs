use std::{fmt, iter::Peekable};

use crate::token::{ensure_next_token, Token, TokenType};

use super::super::token::get_until;

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

impl fmt::Display for ComparisonType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr = match self {
            ComparisonType::GT => "GT",
            ComparisonType::GE => "GE",
            ComparisonType::EQ => "EQ",
            ComparisonType::LE => "LE",
            ComparisonType::LT => "LT",
            ComparisonType::DIFF => "DIFF",
        };
        write!(f, "{}", repr)
    }
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

impl fmt::Display for OperationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr = match self {
            OperationType::Addition => "Addition",
            OperationType::Substraction => "Substraction",
            OperationType::Multiplication => "Multiplication",
            OperationType::Division => "Division",
            OperationType::Modulo => "Modulo",
        };
        write!(f, "{}", repr)
    }
}

pub type CodeBlock = Vec<Box<Node>>;

#[derive(Debug)]
pub enum Node {
    Identifier {
        name: String,
    },
    Register {
        name: String,
    },
    MemoryValue {
        // a[0], a[b], ...
        base: Box<Node>,
        offset: usize,
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
        value: Box<Node>,
    },
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Identifier { name } => write!(f, "ID {}", name),
            Node::Litteral { value } => write!(f, "LIT {}", value),
            Node::Register { name } => write!(f, "REG {}", name),
            Node::MemoryValue { base, offset } => write!(f, "MV\n{}\n{}", base, offset),
            Node::Assignment { lparam, rparam } => write!(f, "Assignment: {} {}", lparam, rparam),
            Node::Comparison {
                lparam,
                rparam,
                comparison,
            } => write!(f, "Comparison {} {} {}", lparam, comparison, rparam),
            Node::IfCondition { condition, content } => write!(
                f,
                "if {}\n{}",
                condition,
                content
                    .iter()
                    .map(|n| format!("{}", n))
                    .collect::<Vec<String>>()
                    .join("\n")
            ),
            Node::WhileLoop { condition, content } => write!(
                f,
                "while {}\n{}",
                condition,
                content
                    .iter()
                    .map(|n| format!("{}", n))
                    .collect::<Vec<String>>()
                    .join("\n")
            ),
            Node::Return { value } => write!(f, "ret {}", value),
            Node::Print { value } => write!(f, "Print {}", value),
            Node::Operation {
                lparam,
                rparam,
                operation,
            } => write!(f, "Op {} {} {}", lparam, operation, rparam),
            Node::FunctionCall {
                function_name,
                parameters,
            } => write!(
                f,
                "fn {} {}",
                function_name,
                parameters
                    .iter()
                    .map(|n| format!("{}", n))
                    .collect::<Vec<String>>()
                    .join("\n")
            ),
            Node::Loop { content } => write!(
                f,
                "Loop\n{}",
                content
                    .iter()
                    .map(|n| format!("{}", n))
                    .collect::<Vec<String>>()
                    .join("\n")
            ),
        }
    }
}

impl Default for Node {
    fn default() -> Self {
        Node::Litteral { value: 0 }
    }
}

fn new_id_or_litteral<T: Iterator<Item = Token>>(tokens: &mut Peekable<T>) -> Result<Node, String> {
    if tokens.peek().and_then(|t| Some(&t.token_type)) != Some(&TokenType::ID) {
        if let Some(token) = tokens.peek() {
            return Err(format!(
                "Unexpected token type {:?} expected an ID or a litteral (line: {} char: {})",
                token.token_type, token.line, token.char
            ));
        } else {
            return Err("Unexpected end of token stream".to_string());
        }
    }

    let token = tokens.next().unwrap();

    if token.is_literal() {
        Ok(Node::Litteral {
            value: match &token.value {
                Some(v) => match v.parse::<i32>() {
                    Ok(v) => v,
                    Err(_) => unreachable!(),
                },
                None => return Err("Token should be ID or Litteral but has no value !".to_string()),
            },
        })
    } else {
        let base_identifier = match &token.value {
            Some(v) => Node::Identifier { name: v.clone() },
            None => return Err("Token should have a value".to_string()),
        };

        if tokens.peek().and_then(|t| Some(&t.token_type)) == Some(&TokenType::LBRACKET) {
            tokens.next();
            let result = Ok(Node::MemoryValue {
                base: Box::from(base_identifier),
                offset: tokens
                    .next()
                    .ok_or("Missing token for memory offset")
                    .and_then(|t| {
                        t.value
                            .ok_or("offset token should have a value")
                            .and_then(|v| {
                                v.parse::<usize>().map_err(|_| "offset should be a number")
                            })
                    })?,
            });
            ensure_next_token(tokens, TokenType::RBRACKET, None)?;
            result
        } else {
            Ok(base_identifier)
        }
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

fn new_return<T: Iterator<Item = Token>>(tokens: &mut Peekable<T>) -> Result<Node, String> {
    if tokens.peek().is_some() {
        Ok(Node::Return {
            value: Box::from(new_id_or_litteral(tokens)?),
        })
    } else {
        Ok(Node::Return {
            value: Box::from(Node::Litteral { value: 0 }),
        })
    }
}

fn new_loop<T: Iterator<Item = Token>>(stream: &mut Peekable<T>) -> Result<Node, String> {
    Ok(Node::Loop {
        content: parse_block(stream)?,
    })
}

fn new_function_call<T: Iterator<Item = Token>>(
    func_id: String,
    mut params: Peekable<T>,
) -> Result<Node, String> {
    let mut parameters = vec![];
    while let Ok(param) = new_id_or_litteral(&mut params) {
        parameters.push(Box::from(param));
    }

    Ok(Node::FunctionCall {
        function_name: func_id,
        parameters,
    })
}

/// Parses a tri-comparison expression (e.g. $a < 10)
/// Tokens is expected to be three items long !
fn parse_tricomp<T>(tokens: &mut Peekable<T>) -> Result<Node, String>
where
    T: Iterator<Item = Token>,
{
    let lparam = new_id_or_litteral(tokens)?;

    let operator = tokens
        .next()
        .and_then(|op| Some(Ok(op.value.unwrap_or("MISSING_TOKEN".to_string()))))
        .unwrap_or(Err("Unable to find the comparison operator".to_string()))?;

    let rparam = new_id_or_litteral(tokens)?;

    new_comp_operator(operator, Box::from(lparam), Box::from(rparam))
}

fn parse_while<T: Iterator<Item = Token>>(stream: &mut Peekable<T>) -> Result<Node, String> {
    let mut loop_condition_token_stream = get_until(stream, TokenType::LBRACE, false);

    let condition = match loop_condition_token_stream.len() {
        3 => parse_tricomp(&mut loop_condition_token_stream)?,
        other => {
            return Err(format!(
                "Expected 3 tokens after `while` keyword but found {} [{:?}]",
                other,
                loop_condition_token_stream
                    .map(|s| format!("{}", &s.token_type))
                    .collect::<Vec<String>>()
                    .join("\n")
            ))
        }
    };

    Ok(Node::WhileLoop {
        condition: Box::from(condition),
        content: parse_block(stream)?,
    })
}

fn parse_assignment<T: Iterator<Item = Token>>(stream: &mut Peekable<T>) -> Result<Node, String> {
    let mut assignment_stream = get_until(stream, TokenType::ENDL, false);

    let lparam = if assignment_stream.peek().is_some() {
        new_id_or_litteral(&mut assignment_stream)?
    } else {
        return Err("Expected identifier after `set` keyword".to_string());
    };

    ensure_next_token(&mut assignment_stream, TokenType::OP, Some("=".to_string()))?;

    let remaining_stream = assignment_stream.collect::<Vec<Token>>();
    let rparam = if remaining_stream
        .iter()
        .any(|t: &Token| matches!(t.token_type, TokenType::LPAREN | TokenType::RPAREN))
    {
        parse_function_call(&mut remaining_stream.into_iter().peekable())?
    } else {
        let mut assignment_iter = remaining_stream.into_iter().peekable();
        let first_assignant = new_id_or_litteral(&mut assignment_iter)?;
        if assignment_iter.peek().is_some() {
            let operator = assignment_iter
                .next()
                .and_then(|op| Some(Ok(op.value.unwrap_or("MISSING_TOKEN".to_string()))))
                .unwrap_or(Err("Unable to find the comparison operator".to_string()))?;

            let second_assignant = new_id_or_litteral(&mut assignment_iter)?;

            new_operator(
                operator,
                Box::from(first_assignant),
                Box::from(second_assignant),
            )?
        } else {
            first_assignant
        }
    };

    Ok(Node::Assignment {
        lparam: Box::from(lparam),
        rparam: Box::from(rparam),
    })
}

fn parse_function_call<T: Iterator<Item = Token>>(
    stream: &mut Peekable<T>,
) -> Result<Node, String> {
    if let Some(function_name) = stream.next() {
        let func_id = match function_name.value {
            Some(v) => v,
            None => unreachable!(),
        };

        if stream.next().and_then(|t| Some(t.token_type)) != Some(TokenType::LPAREN) {
            return Err("Unexpected token after function name, expected (".to_string());
        }

        let function_call =
            new_function_call(func_id, get_until(stream, TokenType::RPAREN, false))?;
        Ok(function_call)
    } else {
        Err("Missing function name after call keyword".to_string())
    }
}

fn parse_if<T: Iterator<Item = Token>>(stream: &mut Peekable<T>) -> Result<Node, String> {
    let mut branching_condition_tokens = get_until(stream, TokenType::LBRACE, false);

    let condition = parse_tricomp(&mut branching_condition_tokens)?;

    Ok(Node::IfCondition {
        condition: Box::from(condition),
        content: parse_block(stream)?,
    })
}

fn parse_print<T: Iterator<Item = Token>>(stream: &mut Peekable<T>) -> Result<Node, String> {
    let mut print_argument = get_until(stream, TokenType::ENDL, false);

    Ok(Node::Print {
        value: Box::from(new_id_or_litteral(&mut print_argument)?),
    })
}

/// Parses a block of code (within a function)
pub fn parse_block<T: Iterator<Item = Token>>(
    stream: &mut Peekable<T>,
) -> Result<CodeBlock, String> {
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
                        TokenType::ENDL => block_tree
                            .push(Box::from(new_return(&mut vec![].into_iter().peekable())?)),
                        TokenType::ID => block_tree.push(Box::from(new_return(stream)?)),
                        _ => continue,
                    }
                }
            }
            TokenType::KEYWORD if token.value == Some("call".to_string()) => {
                block_tree.push(Box::from(parse_function_call(stream)?));
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
                let _ = get_until(stream, TokenType::ENDL, true);
                continue;
            }
            TokenType::ENDL => continue,
            TokenType::RBRACE => {
                break;
            }
            t => {
                return Err(format!(
                    "Unexpected or unhandled token: {:?} {:?}",
                    t, token.value
                ))
            }
        }
    }

    Ok(block_tree)
}
