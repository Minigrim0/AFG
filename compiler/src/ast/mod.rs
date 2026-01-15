use std::collections::HashMap;
use std::fmt;

use crate::error::TokenError;
use crate::lexer::{parse_source, token::Token};

mod function;
pub mod node;
mod parser;

use function::Function;
pub use node::{Node, NodeKind};
pub use parser::Parser;

#[derive(Debug)]
pub struct AST {
    pub functions: HashMap<String, Function>,
}

impl AST {
    /// Parse source code into an AST
    pub fn parse(source: &str) -> Result<Self, TokenError> {
        let lex_result = parse_source(source);

        // TODO: Handle lexer errors properly
        // For now, we'll ignore them and try to parse anyway

        let mut parser = Parser::new(lex_result.tokens);
        parser.parse_program()
    }

    /// Parse from an existing token stream
    pub fn parse_tokens(tokens: Vec<Token>) -> Result<Self, TokenError> {
        let mut parser = Parser::new(tokens);
        parser.parse_program()
    }

    pub fn new() -> Self {
        Self {
            functions: HashMap::from([("main".to_string(), Function::new("main".to_string()))]),
        }
    }

    fn print_block<'a, T>(block: T, f: &mut fmt::Formatter<'_>, level: i32) -> fmt::Result
    where
        T: IntoIterator<Item = &'a Box<Node>>,
    {
        let mut prefix = String::new();
        for a in 0..level {
            if a == level - 1 {
                prefix.push_str(" |--");
            } else {
                prefix.push_str(" |  ");
            }
        }

        for inst in block.into_iter() {
            match &inst.kind {
                NodeKind::Identifier { name } => writeln!(f, "{}ID {}", prefix, name)?,
                NodeKind::MemoryValue { name } => writeln!(f, "{}MEM {}", prefix, name)?,
                NodeKind::Litteral { value } => writeln!(f, "{}LIT {}", prefix, value)?,
                NodeKind::Register { name } => writeln!(f, "{}REG {}", prefix, name)?,
                NodeKind::MemoryOffset { base, offset } => {
                    writeln!(f, "{}MOF", prefix)?;
                    Self::print_block(vec![base], f, level + 1)?;
                    Self::print_block(vec![offset], f, level + 1)?;
                }
                NodeKind::Assignment { lparam, rparam } => {
                    writeln!(f, "{}Assignment", prefix)?;
                    Self::print_block(vec![lparam], f, level + 1)?;
                    Self::print_block(vec![rparam], f, level + 1)?;
                }
                NodeKind::Operation {
                    lparam,
                    rparam,
                    operation,
                } => {
                    writeln!(f, "{}Operation {:?}", prefix, operation)?;
                    Self::print_block(vec![lparam], f, level + 1)?;
                    Self::print_block(vec![rparam], f, level + 1)?;
                }
                NodeKind::Print { value } => {
                    writeln!(f, "{}Print", prefix)?;
                    Self::print_block(vec![value], f, level + 1)?;
                }
                NodeKind::Comparison {
                    lparam,
                    rparam,
                    comparison,
                } => {
                    writeln!(f, "{}Comparison {:?}", prefix, comparison)?;
                    Self::print_block(vec![lparam], f, level + 1)?;
                    Self::print_block(vec![rparam], f, level + 1)?;
                }
                NodeKind::WhileLoop { condition, content } => {
                    writeln!(f, "{}While", prefix)?;
                    Self::print_block(vec![condition], f, level + 1)?;
                    writeln!(f, "{}Do", prefix)?;
                    Self::print_block(content, f, level + 1)?;
                }
                NodeKind::Loop { content } => {
                    writeln!(f, "{}Loop", prefix)?;
                    Self::print_block(content, f, level + 1)?;
                }
                NodeKind::IfCondition { condition, content } => {
                    writeln!(f, "{}If", prefix)?;
                    Self::print_block(vec![condition], f, level + 1)?;
                    writeln!(f, "{}Do", prefix)?;
                    Self::print_block(content, f, level + 1)?;
                }
                NodeKind::FunctionCall {
                    function_name,
                    parameters,
                } => {
                    writeln!(f, "{}Call {}", prefix, function_name)?;
                    Self::print_block(parameters, f, level + 1)?;
                }
                NodeKind::Return { value } => {
                    writeln!(f, "{}Return", prefix)?;
                    Self::print_block(vec![value], f, level + 1)?;
                }
            }
        }

        Ok(())
    }
}

impl fmt::Display for AST {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (name, function) in &self.functions {
            writeln!(f, "Function: {}", name)?;
            Self::print_block(function.content.iter(), f, 0)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;
