use log::error;
use std::{fmt, iter::Peekable};

use crate::{
    error::{TokenError, TokenErrorType},
    token::{ensure_next_token, Token, TokenMetaData, TokenType},
};

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
    MemoryOffset {
        // a[0], a[b], ...
        base: Box<Node>,
        offset: Box<Node>, // Literal, Identifier or Register
    },
    MemoryValue {
        // $Velocity
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
        value: Box<Node>,
    },
}

impl Node {
    pub fn new_identifier(name: String) -> Self {
        if name.starts_with("$") {
            Self::MemoryValue {
                name: name.replace("$", ""),
            }
        } else {
            Self::Identifier { name }
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Identifier { name } => write!(f, "ID {}", name),
            Node::MemoryValue { name } => write!(f, "MEM {}", name),
            Node::Litteral { value } => write!(f, "LIT {}", value),
            Node::Register { name } => write!(f, "REG {}", name),
            Node::MemoryOffset { base, offset } => write!(f, "MOF\n{}\n{}", base, offset),
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

fn new_id_or_litteral<T: Iterator<Item = Token>>(
    tokens: &mut Peekable<T>,
    previous_token_md: TokenMetaData,
) -> Result<Node, TokenError> {
    if tokens.peek().and_then(|t| Some(&t.token_type)) != Some(&TokenType::ID) {
        if let Some(token) = tokens.peek() {
            return Err(TokenError::new(
                TokenErrorType::UnexpectedToken,
                format!(
                    "Unexpected token type {:?} expected an ID or a litteral",
                    token.token_type
                ),
                Some(token.meta),
            ));
        } else {
            return Err(TokenError::new(
                TokenErrorType::UnexpectedEndOfStream,
                "Unexpected end of token stream".to_string(),
                Some(previous_token_md),
            ));
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
                None => {
                    return Err(TokenError::new(
                        TokenErrorType::Invalid,
                        "Token should be ID or Litteral but has no value !".to_string(),
                        Some(token.meta),
                    ))
                }
            },
        })
    } else {
        let base_identifier = match &token.value {
            Some(v) => Node::new_identifier(v.clone()),
            None => {
                return Err(TokenError::new(
                    TokenErrorType::Invalid,
                    "Token should have a value",
                    Some(token.meta),
                ))
            }
        };

        if tokens.peek().and_then(|t| Some(&t.token_type)) == Some(&TokenType::LBRACKET) {
            tokens.next();
            let result = Ok(Node::MemoryOffset {
                base: Box::from(base_identifier),
                offset: match tokens.next().ok_or(TokenError::new(
                    TokenErrorType::UnexpectedEndOfStream,
                    "Missing token for memory offset",
                    None,
                ))? {
                    token if token.is_literal() => Box::from(Node::Litteral {
                        value: token.get_literal_value()?,
                    }),
                    token if token.is(TokenType::ID) => Box::from(Node::Identifier {
                        name: token.get_value()?,
                    }),
                    token => {
                        error!("Invalid token {:?}", token);
                        return Err(TokenError::new(
                            TokenErrorType::Invalid,
                            "Invalid token type for new id or literal",
                            Some(token.meta),
                        ));
                    }
                },
            });
            ensure_next_token(tokens, TokenType::RBRACKET, None)?;
            result
        } else {
            Ok(base_identifier)
        }
    }
}

fn new_operator(
    operator: String,
    lparam: Box<Node>,
    rparam: Box<Node>,
    md: TokenMetaData,
) -> Result<Node, TokenError> {
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
                return Err(TokenError::new(
                    TokenErrorType::InvalidArithmeticOperator,
                    format!("Unknown operator {}, expected one of +, -, *, /, %", op),
                    Some(md),
                ))
            }
        },
    })
}

fn new_comp_operator(
    operator: String,
    lparam: Box<Node>,
    rparam: Box<Node>,
    md: TokenMetaData,
) -> Result<Node, TokenError> {
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
                return Err(TokenError::new(
                    TokenErrorType::InvalidComparisonOperator,
                    format!(
                        "Unknown comparison operator {}, expected one of >, >=, ==, !=, <=, <",
                        op
                    ),
                    Some(md),
                ))
            }
        },
    })
}

fn new_return<T: Iterator<Item = Token>>(
    tokens: &mut Peekable<T>,
    p_md: TokenMetaData,
) -> Result<Node, TokenError> {
    if tokens.peek().is_some() {
        Ok(Node::Return {
            value: Box::from(new_id_or_litteral(tokens, p_md)?),
        })
    } else {
        Ok(Node::Return {
            value: Box::from(Node::Litteral { value: 0 }),
        })
    }
}

fn new_loop<T: Iterator<Item = Token>>(stream: &mut Peekable<T>) -> Result<Node, TokenError> {
    Ok(Node::Loop {
        content: parse_block(stream)?,
    })
}

fn new_function_call<T: Iterator<Item = Token>>(
    func_id: String,
    mut params: Peekable<T>,
    p_md: TokenMetaData,
) -> Result<Node, TokenError> {
    let mut parameters = vec![];
    while let Ok(param) = new_id_or_litteral(&mut params, p_md) {
        parameters.push(Box::from(param));
    }

    Ok(Node::FunctionCall {
        function_name: func_id,
        parameters,
    })
}

/// Parses a tri-comparison expression (e.g. $a < 10)
/// Tokens is expected to be three items long !
fn parse_tricomp<T>(tokens: &mut Peekable<T>) -> Result<Node, TokenError>
where
    T: Iterator<Item = Token>,
{
    let initial_meta = tokens.peek().map(|t| t.meta).unwrap_or(TokenMetaData { line: 0, char: 0 });
    let lparam = new_id_or_litteral(tokens, initial_meta)?;

    let operator_token = tokens
        .next()
        .ok_or_else(|| TokenError::new(
            TokenErrorType::UnexpectedEndOfStream,
            "Unable to find the comparison operator",
            Some(initial_meta)
        ))?;
    let operator = operator_token.value.unwrap_or("MISSING_TOKEN".to_string());
    let operator_meta = operator_token.meta;

    let rparam = new_id_or_litteral(tokens, operator_meta)?;

    new_comp_operator(operator, Box::from(lparam), Box::from(rparam), operator_meta)
}

/// Parses a while loop condition
fn parse_while<T: Iterator<Item = Token>>(stream: &mut Peekable<T>) -> Result<Node, TokenError> {
    let mut loop_condition_token_stream = get_until(stream, TokenType::LBRACE, false);

    let condition = parse_tricomp(&mut loop_condition_token_stream)?;

    Ok(Node::WhileLoop {
        condition: Box::from(condition),
        content: parse_block(stream)?,
    })
}

/// Parses an assignment instructions
fn parse_assignment<T: Iterator<Item = Token>>(
    stream: &mut Peekable<T>,
    p_md: TokenMetaData,
) -> Result<Node, TokenError> {
    let mut assignment_stream = get_until(stream, TokenType::ENDL, false);

    let lparam = if assignment_stream.peek().is_some() {
        new_id_or_litteral(&mut assignment_stream, p_md)?
    } else {
        return Err(TokenError::new(
            crate::error::TokenErrorType::UnexpectedEndOfStream,
            "Expected identifier after `set` keyword".to_string(),
            Some(p_md),
        ));
    };

    ensure_next_token(&mut assignment_stream, TokenType::OP, Some("=".to_string()))?;

    let remaining_stream = assignment_stream.collect::<Vec<Token>>();
    let rparam = if remaining_stream
        .iter()
        .any(|t: &Token| matches!(t.token_type, TokenType::LPAREN | TokenType::RPAREN))
    {
        parse_function_call(&mut remaining_stream.into_iter().peekable(), p_md)?
    } else {
        let mut assignment_iter = remaining_stream.into_iter().peekable();
        let first_assignant = new_id_or_litteral(&mut assignment_iter, p_md)?;
        if assignment_iter.peek().is_some() {
            let operator_token = assignment_iter
                .next()
                .ok_or_else(|| TokenError::new(
                    TokenErrorType::UnexpectedEndOfStream,
                    "Unable to find the arithmetic operator",
                    Some(p_md)
                ))?;
            let operator = operator_token.value.unwrap_or("MISSING_TOKEN".to_string());
            let operator_meta = operator_token.meta;

            let second_assignant = new_id_or_litteral(&mut assignment_iter, operator_meta)?;

            new_operator(
                operator,
                Box::from(first_assignant),
                Box::from(second_assignant),
                operator_meta,
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
    p_md: TokenMetaData,
) -> Result<Node, TokenError> {
    if let Some(function_name) = stream.next() {
        let fn_meta = function_name.meta;
        let func_id = match function_name.value {
            Some(v) => v,
            None => unreachable!(),
        };

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

        let function_call =
            new_function_call(func_id, get_until(stream, TokenType::RPAREN, false), fn_meta)?;
        Ok(function_call)
    } else {
        Err(TokenError::new(
            TokenErrorType::UnexpectedEndOfStream,
            "Missing function name after call keyword",
            Some(p_md)
        ))
    }
}

fn parse_if<T: Iterator<Item = Token>>(
    stream: &mut Peekable<T>,
    _p_md: TokenMetaData,
) -> Result<Node, TokenError> {
    let mut branching_condition_tokens = get_until(stream, TokenType::LBRACE, false);

    let condition = parse_tricomp(&mut branching_condition_tokens)?;

    Ok(Node::IfCondition {
        condition: Box::from(condition),
        content: parse_block(stream)?,
    })
}

fn parse_print<T: Iterator<Item = Token>>(
    stream: &mut Peekable<T>,
    p_md: TokenMetaData,
) -> Result<Node, TokenError> {
    let mut print_argument = get_until(stream, TokenType::ENDL, false);

    Ok(Node::Print {
        value: Box::from(new_id_or_litteral(&mut print_argument, p_md)?),
    })
}

/// Parses a block of code (within a function)
pub fn parse_block<T: Iterator<Item = Token>>(
    stream: &mut Peekable<T>,
) -> Result<CodeBlock, TokenError> {
    let mut block_tree = vec![];
    while let Some(token) = stream.next() {
        match token.token_type {
            TokenType::KEYWORD if token.value == Some("set".to_string()) => {
                let assignment = parse_assignment(stream, token.meta)?;
                block_tree.push(Box::from(assignment));
            }
            TokenType::KEYWORD if token.value == Some("while".to_string()) => {
                let while_block = parse_while(stream)?;
                block_tree.push(Box::from(while_block));
            }
            TokenType::KEYWORD if token.value == Some("if".to_string()) => {
                let if_block = parse_if(stream, token.meta)?;
                block_tree.push(Box::from(if_block));
            }
            TokenType::KEYWORD if token.value == Some("return".to_string()) => {
                let return_meta = token.meta;
                if let Some(next_token) = stream.peek() {
                    match next_token.token_type {
                        TokenType::ENDL => block_tree
                            .push(Box::from(new_return(&mut vec![].into_iter().peekable(), return_meta)?)),
                        TokenType::ID => block_tree.push(Box::from(new_return(stream, return_meta)?)),
                        _ => continue,
                    }
                }
            }
            TokenType::KEYWORD if token.value == Some("call".to_string()) => {
                block_tree.push(Box::from(parse_function_call(stream, token.meta)?));
            }
            TokenType::KEYWORD if token.value == Some("loop".to_string()) => {
                if let Some(t) = stream.next() {
                    if t.token_type != TokenType::LBRACE {
                        return Err(TokenError::new(
                            TokenErrorType::UnexpectedToken,
                            format!("Unexpected token {:?} after loop keyword, expected LBRACE", t.token_type),
                            Some(t.meta)
                        ));
                    }
                } else {
                    return Err(TokenError::new(
                        TokenErrorType::UnexpectedEndOfStream,
                        "Expected LBRACE after loop keyword",
                        None
                    ));
                }

                let loop_block = new_loop(stream)?;
                block_tree.push(Box::from(loop_block));
            }
            TokenType::KEYWORD if token.value == Some("print".to_string()) => {
                let print_block = parse_print(stream, token.meta)?;
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
                return Err(TokenError::new(
                    TokenErrorType::UnexpectedToken,
                    format!(
                        "Unexpected or unhandled token: {:?} {:?} (line: {}, char: {})",
                        t, token.value, token.meta.line, token.meta.char
                    ),
                    Some(token.meta)
                ))
            }
        }
    }

    Ok(block_tree)
}
