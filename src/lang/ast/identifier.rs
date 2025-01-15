use super::ASTNodeInfo;
use std::fmt;

#[derive(Debug)]
// Identifier (e.g. variable name, function name)
pub struct Identifier {
}

impl ASTNodeInfo for Identifier {
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Identifier")
    }
}
