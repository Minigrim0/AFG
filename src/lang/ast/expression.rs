use super::ASTNodeInfo;
use std::fmt;

#[derive(Debug)]
// An expression returns a value. This can be
pub struct Expression {}

impl ASTNodeInfo for Expression {}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Expression")
    }
}
