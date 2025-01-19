use crate::lang::ast::node::{CodeBlock, Node};

/// Semantic module
/// Used to validate the semantics of an AST
pub enum SemanticError {
    UnknownVariable(String),  // Use of a previously undeclared variable
    InvalidOperation(String)  // Invalid operation
}


/// Returns the newly created/assigned variables
/// This goes through the assignment nodes and returns their lparam
pub fn get_new_variables(node: &Box<Node>) -> Vec<&String> {
    match &**node {
        Node::Identifier { name } => vec![name],
        Node::Assignment { lparam, .. } => {
            get_new_variables(lparam)
        },
        _ => vec![]
    }
}

pub fn get_used_variables(node: &Box<Node>) -> Vec<&String> {
    match &**node {
        Node::Identifier { name } => vec![name],
        Node::Assignment { rparam, .. } => {
            get_used_variables(rparam)
        },
        Node::Operation { lparam,  rparam , .. } => {
            let mut vars = get_used_variables(lparam);
            vars.extend(get_used_variables(rparam));
            vars
        },
        Node::Comparison { lparam, rparam, .. } => {
            let mut vars = get_used_variables(lparam);
            vars.extend(get_used_variables(rparam));
            vars
        },
        Node::WhileLoop { condition, .. } => {
            get_used_variables(condition)
        },
        Node::IfCondition { condition, .. } => {
            get_used_variables(condition)
        }
        _ => vec![]
    }
}

fn analyze_block(block: &CodeBlock, mut scope: Vec<String>) -> Result<(), SemanticError> {
    for inst in block.iter() {
        match &**inst {
            Node::WhileLoop { content, .. } => {
                analyze_block(content, scope.clone())?;
            },
            Node::IfCondition { content, .. } => {
                analyze_block(content, scope.clone())?;
            },
            Node::Loop { content, .. } => {
                analyze_block(content, scope.clone())?;
            },
            _ => {}
        }

        let used_vars = get_used_variables(inst);
        for var in used_vars.iter()  {
            if !scope.contains(var) {
                return Err(SemanticError::UnknownVariable( format!("{} is not in scope", var) ));
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
/// ```ignore
/// let ast = build_sample_ast(); // Generates an AST
/// match analyze(&ast) {
///     Ok(()) => println!("AST is semantically valid"),
///     Err(e) => println!("Semantic error: {:?}", e),
/// }
/// ```
pub fn analyze(ast: &super::AST) -> Result<(), SemanticError> {
    for (func_name, func) in &ast.functions {
        println!("Analyzing: {}", func_name);
        let in_scope = crate::virtual_machine::get_special_variables();

        analyze_block(&func.content, in_scope)?;
    }

    Ok(())
}