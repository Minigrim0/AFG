use crate::ast::node::{Node, NodeKind};

use super::error::SemanticError;

/// Checks that the left-parameter of an assignment is a valid lparam, that is, it is not a litteral
pub fn is_valid_assignment_lparam(node: &Box<Node>) -> Result<(), SemanticError> {
    match &node.kind {
        NodeKind::Litteral { value } => Err(SemanticError::InvalidOperation(format!(
            "{} is not a valid lparam for an assignment",
            value
        ))),
        _ => Ok(()),
    }
}
