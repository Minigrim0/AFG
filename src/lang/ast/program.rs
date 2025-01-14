use super::ASTNodeInfo;
use std::fmt;

// Program (e.g. the root node, the entry point)
#[derive(Debug)]
struct Program {

}

impl ASTNodeInfo for Program {
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Program")
    }
}
