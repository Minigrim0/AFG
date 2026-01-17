use crate::ast::node::{Node, NodeKind};
use crate::lexer::token::TokenLocation;

use super::validity::is_valid_assignment_lparam;
use super::error::SemanticError;

pub fn show_span_location(node_span: &Option<TokenLocation>) -> String {
    match node_span {
        Some(span) => format!(" at line {} column {}", span.line, span.column),
        None => "".to_string(),
    }
}

/// Returns all the variables declared by this node
/// This function is used to check what variables are in the scope
pub fn get_new_variables(node: &Box<Node>) -> Vec<&String> {
    match &node.kind {
        NodeKind::Identifier { name } => vec![name],
        NodeKind::Assignment { lparam, .. } => get_new_variables(lparam),
        _ => vec![],
    }
}

// Returns all the variables used by this node and its children
// This function is used to check if a variable is used before being declared
pub fn get_used_variables(node: &Box<Node>) -> Result<Vec<&String>, SemanticError> {
    match &node.kind {
        NodeKind::Identifier { name } => Ok(vec![name]),
        NodeKind::Assignment { rparam, lparam } => {
            is_valid_assignment_lparam(lparam)?;
            get_used_variables(rparam)
        }
        NodeKind::Operation { lparam, rparam, .. } => {
            let mut vars = get_used_variables(lparam)?;
            vars.extend(get_used_variables(rparam)?);
            Ok(vars)
        }
        NodeKind::Comparison { lparam, rparam, .. } => {
            let mut vars = get_used_variables(lparam)?;
            vars.extend(get_used_variables(rparam)?);
            Ok(vars)
        }
        NodeKind::WhileLoop { condition, .. } => get_used_variables(condition),
        NodeKind::IfCondition { condition, .. } => get_used_variables(condition),
        NodeKind::FunctionCall { parameters, .. } => {
            let mut vars = vec![];
            for param in parameters.iter() {
                vars.extend(get_used_variables(param)?);
            }
            Ok(vars)
        }
        _ => Ok(vec![]),
    }
}
