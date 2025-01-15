use super::ASTNodeInfo;
use std::fmt;

#[derive(Debug)]
// Literal (e.g. number, string)
struct Literal {}

impl ASTNodeInfo for Literal {}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Literal")
    }
}
