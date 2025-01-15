use super::ASTNodeInfo;
use std::fmt;

#[derive(Debug)]
pub struct Expression {} // Expression (e.g. binary operation, function call)

impl ASTNodeInfo for Expression {}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Expression")
    }
}
