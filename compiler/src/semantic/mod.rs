/// Semantic module
/// Used to validate the semantics of an AST
use super::ast::AST;
use crate::ast::node::{CodeBlock, Node};

mod error;
mod utils;
mod validity;

pub use error::SemanticError;
pub use utils::*;

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
/// ```rust
/// use afgcompiler::ast::AST;
/// use afgcompiler::prelude::analyze;
///
/// let ast = AST::new(); // Generate an AST
/// match analyze(&ast) {
///     Ok(()) => println!("AST is semantically valid"),
///     Err(e) => println!("Semantic error: {}", e),
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
