use std::{
    cell::RefCell,
    rc::{Rc, Weak}
};
use std::collections::VecDeque;

use super::TokenStream;

type SafeTable = Rc<RefCell<Vec<String>>>;


#[derive(Debug, Default)]
pub enum OperationType {
    #[default]
    Addition,
    Substraction,
    Multiplication,
    Division,
    Modulo
}


#[derive(Debug, Default)]
pub enum NodeType {
    Identifier,
    #[default]
    Litteral,
    Assignment,
    Operation,
    Comparison,
    WhileLoop,
    FunctionCall,
}

#[derive(Debug, Default)]
/// Example:
/// ```
/// set $Velocity = $Velocity + 500;
/// ```
/// ASTBlockNode { type: Assignment, name: None, value: None, arglist: [], descendents: [
///     ASTBlockNode { type: Identifier, name: "$Velocity", value: None, arglist: [], descendents: [None; 3], local_table: []},
///     ASTBlockNode { type: Operation, name: "", value: OP_ADDITION, arglist: [], descendents: [
///             ASTBlockNode { type: Identifier, name: "$Velocity", value: None arglist: [], descendents: [None; 3], local_table: [] },
///             ASTBlockNode { type: Literal, name: "", value: Some(500), arglist: [], descendents: [None; 3], local_table: [] },
///             None
///     ], local_table: []},
/// ], local_table: []}
pub struct ASTBlockNode {
    node_type: NodeType,
    name: String,
    value: i32,
    arglist: Vec<String>,  // When calling a function
    descendents: (Option<Rc<RefCell<ASTBlockNode>>>, Option<Rc<RefCell<ASTBlockNode>>>, Vec<ASTBlockNode>),  // First two elements are operands to the block (e.g. two parts of an assignment A + B => (A, B))
    local_table: Option<SafeTable>,
}

impl ASTBlockNode {
    pub fn new(node_type: NodeType) -> Self {
        Self {
            node_type,
            ..Default::default()
        }
    }

    pub fn new_id(id: String) -> Self {
        Self {
            node_type: NodeType::Identifier,
            name: id,
            ..Default::default()
        }
    }

    pub fn new_operator(operator: String) -> Result<Self, String> {
        Ok(Self {
            node_type: NodeType::Operation,
            value: match operator.as_str() {
                "+" => OperationType::Addition as i32,
                "-" => OperationType::Substraction as i32,
                "*" => OperationType::Multiplication as i32,
                "/" => OperationType::Division as i32,
                "%" => OperationType::Modulo as i32,
                op => return Err(format!("Unknown operator {}, expected one of +, -, *, /, %", op))
            },
            ..Default::default()
        })
    }

    pub fn ensure_next_token(stream: &mut VecDeque<super::Token>, expected_ttype: super::TokenType, expected_value: Option<String>) -> Result<(), String> {
        match stream.pop_front() {
            Some(t) => match (t.ttype, t.value) {
                (ttype, value) if ttype == expected_ttype && value == expected_value => Ok(()),
                (ttype, value) if ttype == expected_ttype => Err(format!("Expected next token to have value {:?} but found {:?}", expected_value, value)),
                (ttype, value) if value == expected_value => Err(format!("Expected next token to have type {:?} but found {:?}", expected_ttype, ttype)),
                (ttype, value) => Err(format!("Expected next token to have type {:?} and value {:?} but found {:?}, {:?}", expected_ttype, expected_value, ttype, value)),
            },
            None => Err("Expected a token but found nothing".to_string()),
        }
    }

    pub fn parse_triop(tokens: &mut VecDeque<super::Token>) -> Result<Self, String> {
        match tokens.len() {
            3 => {},
            other => return Err(format!("Expected three tokens to match a tri-ops but found {}", other))
        };

        let first_token = match tokens.pop_front() {
            Some(av) => match av.value {
                Some(v) => Rc::from(RefCell::from(Self::new_id(v))),
                None => return Err(format!("Expected value for identifier token: {:?}", av))
            },
            None => unreachable!()
        };
        let mut operator = match tokens.pop_front() {
            Some(op) if op.ttype == super::TokenType::OP => match op.value {
                Some(v) => Self::new_operator(v)?,
                None => return Err("Expected operator to have a value".to_string())
            },
            Some(op) => return Err(format!("Expected OP token as second token after assignment operator but found {:?}", op)),
            None => unreachable!()
        };
        let second_token = match tokens.pop_front() {
            Some(av) => match av.value {
                Some(v) => Rc::from(RefCell::from(Self::new_id(v))),
                None => return Err(format!("Expected value for identifier token: {:?}", av))
            },
            None => return Err("Expected token after assignment operation".to_string())
        };

        operator.descendents.0 = Some(first_token);
        operator.descendents.1 = Some(second_token);

        Ok(operator)
    }

    pub fn parse_while(stream: &mut TokenStream) -> Result<Self, String> {
        let mut assignment_stuff = VecDeque::from(stream.get_until(super::TokenType::LBRACE));
        assignment_stuff.pop_back();  // Remove Lbrace

        match assignment_stuff.len() {
            3 => {},
            other => return Err(format!("Expected 3 tokens after `while` keyword bu found {}", other))
        };


    }

    pub fn parse_assignment(stream: &mut TokenStream) -> Result<Self, String> {
        let mut assignment_stuff = VecDeque::from(stream.get_until(super::TokenType::ENDL));
        assignment_stuff.pop_back();  // Remove endl
        println!("Parsing assignement from: {:?}", assignment_stuff);

        let mut top_node = Self {
            node_type: NodeType::Assignment,
            descendents: (
                None, None, vec![]
            ),
            ..Default::default()
        };

        let assigned_to = match assignment_stuff.pop_front() {
            Some(t) if t.ttype == super::TokenType::ID  => t,
            Some(t) => return Err(format!("Unexpected token {:?} after set keyword", t)),
            None => return Err("Expected identifier after `set` keyword".to_string())
        };

        let lparam = match assigned_to.value {
            Some(v) => Rc::from(RefCell::from(Self::new_id(v))),
            None => return Err("Missing lparam value for assignment operation".to_string())
        };

        Self::ensure_next_token(&mut assignment_stuff, super::TokenType::OP, Some("=".to_string()))?;

        let rparam = match assignment_stuff.len() {
            1 => {
                match assignment_stuff.pop_front() {
                    Some(av) => match av.value {
                        Some(v) => Rc::from(RefCell::from(Self::new_id(v))),
                        None => return Err(format!("Expected value for identifier token: {:?}", av))
                    },
                    None => return Err("Expected token after assignment operation".to_string())
                }
            },
            3 => {
                Rc::from(RefCell::from(Self::parse_triop(&mut assignment_stuff)?))
            },
            t => return Err(format!("Expected one or three tokens after assignment operator but found {}", t))
        };

        top_node.descendents.0 = Some(lparam);
        top_node.descendents.1 = Some(rparam);

        Ok(top_node)
    }
}
