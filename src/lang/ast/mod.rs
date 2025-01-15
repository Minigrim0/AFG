use core::fmt;

use std::cell::RefCell;
use std::rc::Rc;

mod expression;
mod function;
mod identifier;
mod operator;
mod program;
mod statement;

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
            root: Rc::from(RefCell::from(ASTNode {
                data: Box::from(program::Program {}),
                children: Vec::new(),
            })),
        }
    }

    /// Parses a full program from a string
    pub fn parse_program<S: AsRef<str>>(text: S) -> Result<Self, String> {
        let text = text.as_ref().to_string();
        let mut result: Vec<&str> = Vec::new();
        let mut last = 0;
        for (index, matched) in
            text.match_indices(|c| c == ' ' || c == '(' || c == ')' || c == '{' || c == '}')
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

    /// Parses function parameters from a string
    fn parse_fn_args<T>(tokens: &mut T) -> Result<Vec<String>, String>
    where
        T: Iterator,
        T::Item: AsRef<str>,
    {
        let mut args = Vec::new();

        while let Some(arg) = tokens.next() {
            match arg.as_ref() {
                "(" => continue,
                " " => continue,
                ")" => return Ok(args),
                arg => args.push(arg.to_string().replace(',', "")),
            }
        }
        Err("Unclosed Parenthese for function arguments".to_string())
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
                    if let Ok(mut cn) = current_node.try_borrow_mut() {
                        let function_name = tokens
                            .next()
                            .ok_or("No function name found")?
                            .as_ref()
                            .to_string();
                        let function_args = Self::parse_fn_args(tokens)?;
                        let function_node = cn.add_child(ASTNode::new(Box::from(
                            function::Function::new(function_name, function_args),
                        )));
                        Self::parse(tokens, function_node.clone())?;
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
