use std::collections::HashMap;
use std::fmt;
use crate::lang::token::TokenType;
use crate::lang::TokenStream;

pub mod node;
mod function;

use function::Function;
use crate::lang::ast::node::Node;

#[derive(Debug)]
pub struct AST {
    pub functions: HashMap<String, Function>
}

impl AST {
    pub fn parse(tokens: &mut TokenStream) -> Result<Self, String> {
        let mut program = HashMap::new();

        while let Some(token) = tokens.next() {
            match &token.token_type {
                TokenType::KEYWORD if token.value == Some("fn".to_string()) => {
                    let function = Function::parse(tokens)?;
                    program.insert(function.name.clone(), function);
                },
                TokenType::ENDL => continue,
                token_type => return Err(format!("Unexpected token {:?} {:?}", token_type, token.value))
            }
        }

        Ok(Self {
            functions: program
        })
    }

    fn print_block<'a, T>(block: T, f: &mut fmt::Formatter<'_>, level: i32) -> fmt::Result
    where T: IntoIterator<Item = &'a Box<Node>>
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
            match &**inst {
                Node::Identifier { name } => writeln!(f, "{}ID {}", prefix, name)?,
                Node::Litteral { value } => writeln!(f, "{}LIT {}", prefix, value)?,
                Node::Assignment { lparam, rparam } => {
                    writeln!(f, "{}Assignment", prefix)?;
                    Self::print_block(vec![lparam], f, level+1)?;
                    Self::print_block(vec![rparam], f, level+1)?;
                },
                Node::Operation { lparam, rparam, operation } => {
                    writeln!(f, "{}Operation {:?}", prefix, operation)?;
                    Self::print_block(vec![lparam], f, level + 1)?;
                    Self::print_block(vec![rparam], f, level + 1)?;
                },
                Node::Comparison { lparam, rparam, comparison } => {
                    writeln!(f, "{}Comparison {:?}", prefix, comparison)?;
                    Self::print_block(vec![lparam], f, level + 1)?;
                    Self::print_block(vec![rparam], f, level + 1)?;
                },
                Node::WhileLoop { condition, content } => {
                    writeln!(f, "{}While", prefix)?;
                    Self::print_block(vec![condition], f, level + 1)?;
                    writeln!(f, "{}Do", prefix)?;
                    Self::print_block(content, f, level + 1)?;
                },
                Node::Loop { content } => {
                    writeln!(f, "{}Loop", prefix)?;
                    Self::print_block(content, f, level + 1)?;
                },
                Node::IfCondition { condition, content } => {
                    writeln!(f, "{}If", prefix)?;
                    Self::print_block(vec![condition], f, level + 1)?;
                    writeln!(f, "{}Do", prefix)?;
                    Self::print_block(content, f, level + 1)?;
                }
                Node::FunctionCall { function_name, parameters } => {
                    writeln!(f, "{}Call {}", prefix, function_name)?;
                    Self::print_block(parameters, f, level + 1)?;
                }
                Node::Return => {
                    writeln!(f, "{}Return", prefix)?;
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
