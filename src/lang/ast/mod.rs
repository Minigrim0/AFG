use core::fmt;

mod expression;
mod statement;
mod function;
mod identifier;
mod operator;
mod program;

// Difference between statements and expressions:
// - Statements are executed for their side effects (e.g. let x = 2)
// - Expressions are evaluated to produce a value (e.g. 2 + 2 in let x = 2 + 2)
pub trait ASTNodeInfo: fmt::Display + fmt::Debug {}

pub struct ASTNode {
    pub data: Box<dyn ASTNodeInfo>,
    pub children: Vec<ASTNode>,
}

impl ASTNode {
    pub fn new(data: Box<dyn ASTNodeInfo>) -> Self {
        Self {
            data,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: ASTNode) {
        self.children.push(child);
    }
}

pub struct AST {
    pub root: ASTNode,
}

impl AST {
    pub fn new() -> Self {
        Self {
            root: ASTNode {
                kind: ASTNodeKind::Program,
                children: Vec::new(),
            },
        }
    }

    pub fn parse<S: AsRef<str>>(text: S) -> Result<Self, String> {
        let mut ast = Self::new();
        let mut node_stack = vec![&mut ast.root]; // Keep a node stack to push children to the correct parent node
        let mut line_count = 0;

        let mut tokens = text.as_ref().split_whitespace();
        while let Some(token) = tokens.next() {
            match token {
                "function" => {
                    let mut node = ASTNode::new(ASTNodeKind::Function);
                    node.add_child(ASTNode::new(ASTNodeKind::Identifier));
                    node_stack.push(&mut node);
                }
                _ => {
                    println!("Unknown token: {}", token);
                }
            }
        } else {
            println!("No token found on line: {}", line_nbr);
            continue;
        }

        Ok(ast)
    }
}
