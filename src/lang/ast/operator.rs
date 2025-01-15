use super::ASTNodeInfo;
use std::fmt;

// Operator (e.g. +, -, *, /)
#[derive(Debug)]
pub struct Operator {}

impl ASTNodeInfo for Operator {}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Operator")
    }
}
