use core::fmt;
use regex::Regex;

use std::cell::RefCell;
use std::rc::Rc;

mod expression;
mod function;
mod identifier;
mod operator;
mod program;
mod assignment;

// mod node;

// use node::ASTNode;

// Difference between statements and expressions:
// - Statements are executed for their side effects (e.g. let x = 2)
// - Expressions are evaluated to produce a value (e.g. 2 + 2 in let x = 2 + 2)
pub trait ASTNodeInfo: fmt::Display + fmt::Debug {}

#[derive(Debug)]
pub struct ASTNode {
    pub data: Box<dyn ASTNodeInfo>,
    pub children: Vec<Rc<RefCell<ASTNode>>>,
}

impl ASTNode {
    pub fn new(data: Box<dyn ASTNodeInfo>) -> Self {
        Self {
            data,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: ASTNode) -> Rc<RefCell<ASTNode>> {
        let child = Rc::new(RefCell::new(child));
        self.children.push(child.clone());
        child
    }
}

#[derive(Debug)]
pub struct AST {
    pub root: Rc<RefCell<ASTNode>>,
}

impl AST {
    pub fn new() -> Self {
        Self {
            root: Rc::from(RefCell::from(ASTNode::new(Box::from(program::Program::new())))),
        }
    }

    /// Parses a full program from a string
    pub fn parse_program<S: AsRef<str>>(text: S) -> Result<Self, String> {
        let text = text.as_ref().to_string();
        let mut result: Vec<&str> = Vec::new();
        let mut last = 0;
        for (index, matched) in
            text.match_indices(|c| c == ' ' || c == '(' || c == ')' || c == '{' || c == '}' || c == '\n' || c == '\n')
        {
            if last != index {
                result.push(&text[last..index]);
            }
            result.push(matched);
            last = index + matched.len();
        }
        if last < text.len() {
            result.push(&text[last..]);
        }
        println!("Tokens: {:?}", result);

        let ast = AST::new();
        Self::parse(
            &mut result.into_iter().filter(|t| *t != " "),
            ast.root.clone(),
        )?;
        Ok(ast)
    }

    /// Parses the current block from a string
    pub fn parse<T>(tokens: &mut T, current_node: Rc<RefCell<ASTNode>>) -> Result<(), String>
    where
        T: Iterator,
        T::Item: AsRef<str>,
    {
        let tokens = tokens.into_iter();

        while let Some(token) = tokens.next() {
            match token.as_ref() {
                "fn" => {
                    let function_name = tokens
                        .next()
                        .ok_or("No function name found")?
                        .as_ref()
                        .to_string();

                    let function_node = function::Function::new(function_name, tokens)?;

                    let opening_brace = tokens.next().ok_or("Missing code block after function definition")?.as_ref().to_string();
                    if opening_brace != "{" {
                        return Err(format!("Unexpected token '{}', expected '{{' after function definition", opening_brace));
                    }

                    let function_node =  if let Ok(mut cn) = current_node.try_borrow_mut() {
                        cn.add_child(ASTNode::new(Box::from(
                            function_node,
                        )))
                    } else {
                        return Err("Unable to borrow current node as mut !".to_string());
                    };

                    Self::parse(tokens, function_node.clone())?;
                },
                "let" => {
                    let variable_name = tokens.next()
                        .and_then(|t| {
                            let var_name = Regex::new(r"[a-zA-Z_]+").unwrap();
                            if t.as_ref() != "=" && var_name.is_match(t.as_ref()) {
                                Some(t)
                            } else {
                                None
                            }
                        }).ok_or("Missing or invalid variable name after let token")?.as_ref().to_string();
                    if tokens.next().ok_or("Missing = operator after variable name in let statement")?.as_ref() != "=" {
                        return Err("Missing = operator after variable name in let statement".to_string())
                    }
                    if let Ok(mut cn) = current_node.try_borrow_mut() {
                        let mut node = assignment::Assignment::new(variable_name);
                        node.parse_expression(tokens)?;

                        let assignment_node = cn.add_child(ASTNode::new(Box::from(
                            node,
                        )));
                        // Avoid blocking next stages
                        std::mem::drop(cn);
                        Self::parse(tokens, assignment_node.clone())?;
                    } else {
                        return Err("Unable to borrow current node as mut !".to_string());
                    }
                }
                unk_tok => {
                    println!("Unknown token: {}", unk_tok);
                }
            }
        }

        Ok(())
    }
}
