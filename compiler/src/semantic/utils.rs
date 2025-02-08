use crate::ast::Node;

use super::validity::is_valid_assignment_lparam;
use super::error::SemanticError;

/// Returns all the variables declared by this node
/// This function is used to check what variables are in the scope
pub fn get_new_variables(node: &Box<Node>) -> Vec<&String> {
    match &**node {
        Node::Identifier { name } => vec![name],
        Node::Assignment { lparam, .. } => get_new_variables(lparam),
        _ => vec![],
    }
}

// Returns all the variables used by this node and its children
// This function is used to check if a variable is used before being declared
pub fn get_used_variables(node: &Box<Node>) -> Result<Vec<&String>, SemanticError> {
    match &**node {
        Node::Identifier { name } => Ok(vec![name]),
        Node::Assignment { rparam, lparam } => {
            is_valid_assignment_lparam(lparam)?;
            get_used_variables(rparam)
        }
        Node::Operation { lparam, rparam, .. } => {
            let mut vars = get_used_variables(lparam)?;
            vars.extend(get_used_variables(rparam)?);
            Ok(vars)
        }
        Node::Comparison { lparam, rparam, .. } => {
            let mut vars = get_used_variables(lparam)?;
            vars.extend(get_used_variables(rparam)?);
            Ok(vars)
        }
        Node::WhileLoop { condition, .. } => get_used_variables(condition),
        Node::IfCondition { condition, .. } => get_used_variables(condition),
        Node::FunctionCall { parameters, .. } => {
            let mut vars = vec![];
            for param in parameters.iter() {
                vars.extend(get_used_variables(param)?);
            }
            Ok(vars)
        }
        _ => Ok(vec![]),
    }
}
