use crate::ast::node::{CodeBlock, Node};
use super::ast::AST;

/// Semantic module
/// Used to validate the semantics of an AST
pub enum SemanticError {
    UnknownVariable(String),  // Use of a previously undeclared variable
    InvalidOperation(String), // Invalid operation
}

/// Checks that the left-parameter of an assignment is a valid lparam, that is, it is not a litteral
fn is_valid_assignment_lparam(node: &Box<Node>) -> Result<(), SemanticError> {
    match &**node {
        Node::Litteral { value } => Err(SemanticError::InvalidOperation(format!(
            "{} is not a valid lparam for an assignment",
            value
        ))),
        _ => Ok(()),
    }
}

/// Returns all the variables declared by this node
/// This function is used to check what variables are in the scope
fn get_new_variables(node: &Box<Node>) -> Vec<&String> {
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
        },
        Node::WhileLoop { condition, .. } => {
            get_used_variables(condition)
        },
        Node::IfCondition { condition, .. } => {
            get_used_variables(condition)
        },
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

/// Analyzes a block of code for semantic errors
fn analyze_block(block: &CodeBlock, mut scope: Vec<String>) -> Result<(), SemanticError> {
    for inst in block.iter() {
        match &**inst {
            Node::WhileLoop { content, .. } => {
                analyze_block(content, scope.clone())?;
            }
            Node::IfCondition { content, .. } => {
                analyze_block(content, scope.clone())?;
            }
            Node::Loop { content, .. } => {
                analyze_block(content, scope.clone())?;
            }
            _ => {}
        }

        let used_vars = get_used_variables(inst)?;
        for var in used_vars.iter() {
            if !scope.contains(var) {
                return Err(SemanticError::UnknownVariable(format!(
                    "{} is not in scope",
                    var
                )));
            }
        }

        let new_vars = get_new_variables(inst);
        scope.extend(new_vars.into_iter().map(|v| v.clone()));
    }

    Ok(())
}

/// Analyzes the given Abstract Syntax Tree (AST) for semantic errors.
///
/// This function iterates through all functions within the AST and validates them
/// using semantic rules. Specifically, it checks for issues like the use of
/// undeclared variables or invalid scopes during function execution.
///
/// # Arguments
/// * `ast` - A reference to the AST object which contains functions and their corresponding content.
///
/// # Returns
/// * `Ok(())` - If the AST is successfully validated without any semantic errors.
/// * `Err(SemanticError)` - If a semantic error is encountered, such as an undeclared variable.
///
/// # Errors
/// * `SemanticError::UnknownVariable` - Returned if a variable is used without being declared in the current scope.
/// * `SemanticError::InvalidOperation` - Returned if the AST contains operations that are not semantically valid.
///
/// # Example
/// ```rust,ignore
/// let ast = build_sample_ast(); // Generates an AST
/// match analyze(&ast) {
///     Ok(()) => println!("AST is semantically valid"),
///     Err(e) => println!("Semantic error: {:?}", e),
/// }
/// ```
pub fn analyze(ast: &AST) -> Result<(), SemanticError> {
    for (_, func) in &ast.functions {
        let mut in_scope = machine::get_special_variables();
        in_scope.extend(func.parameters.clone());

        analyze_block(&func.content, in_scope)?;
    }

    Ok(())
}
