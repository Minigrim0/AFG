use super::ASTNodeInfo;
use std::fmt;

#[derive(Debug)]
struct Function {} // Function declaration (e.g. fn foo() {})
impl ASTNodeInfo for Function {
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Function")
    }
}
