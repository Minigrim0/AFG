/// Semantic module
/// Used to validate the semantics of an AST
use std::collections::HashMap;

use super::ast::AST;
use crate::ast::node::{CodeBlock, Node};

mod error;
mod utils;
mod validity;

pub use error::SemanticError;
pub use utils::*;

/// Analyzes a block of code for semantic errors
fn analyze_block(block: &CodeBlock, mut scope: Vec<String>, functions: &HashMap<String, usize>) -> Result<(), SemanticError> {
    for inst in block.iter() {
        match &**inst {
            Node::WhileLoop { content, .. } => {
                analyze_block(content, scope.clone(), functions)?;
            }
            Node::IfCondition { content, .. } => {
                analyze_block(content, scope.clone(), functions)?;
            }
            Node::Loop { content, .. } => {
                analyze_block(content, scope.clone(), functions)?;
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

        match &**inst {
            Node::FunctionCall { function_name, parameters }=> {
                if !functions.contains_key(function_name) {
                    return Err(SemanticError::UnknownFunction(format!(
                        "Function {} is not defined",
                        function_name
                    )));
                }
                let expected_arity = functions[function_name];
                if parameters.len() != expected_arity {
                    return Err(SemanticError::InvalidFunctionCall(format!(
                        "Function {} expects {} parameters, but got {}",
                        function_name,
                        expected_arity,
                        parameters.len()
                    )));
                }
            },
            _ => {}
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
    // Collect function arities for later checks
    let function_arities = ast
        .functions
        .iter()
        .map(|(name, func)| (name.clone(), func.parameters.len()))
        .collect::<HashMap<String, usize>>();

    for (_, func) in &ast.functions {
        let mut in_scope = machine::prelude::get_special_variables();
        in_scope.extend(func.parameters.clone());

        analyze_block(&func.content, in_scope, &function_arities)?;
    }

    Ok(())
}
