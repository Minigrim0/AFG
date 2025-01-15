use super::ASTNodeInfo;
use std::fmt;

#[derive(Debug)]
// Function declaration (e.g. fn foo() {})
pub struct Function {
    identifier: String,
    parameters: Vec<String>,
}

impl Function {
    pub fn new(identifier: String, parameters: Vec<String>) -> Self {
        Self {
            identifier,
            parameters,
        }
    }
}

impl ASTNodeInfo for Function {}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Function '{}'", self.identifier)
    }
}
