use core::fmt;
use std::{cell::RefCell, rc::Rc};
use std::collections::{HashSet, VecDeque};

use super::token::{Token, TokenType, TokenStream};

#[derive(Debug, Default)]
pub enum ComparisonType {
    GT,
    GE,
    #[default]
    EQ,
    LE,
    LT,
    DIFF,
}

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
    Loop,
    IfCondition,
    FunctionCall,
    Return,
}

#[derive(Debug, Default)]
/// Example:
/// ```csai
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
    arglist: Vec<Token>,  // When calling a function
    pub descendents: (Option<Rc<RefCell<ASTBlockNode>>>, Option<Rc<RefCell<ASTBlockNode>>>, Vec<ASTBlockNode>),  // First two elements are operands to the block (e.g. two parts of an assignment A + B => (A, B))
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

    pub fn new_comp_operator(operator: String) -> Result<Self, String> {
        Ok(Self {
            node_type: NodeType::Operation,
            value: match operator.as_str() {
                ">" => ComparisonType::GT as i32,
                ">=" => ComparisonType::GE as i32,
                "==" => ComparisonType::EQ as i32,
                "!=" => ComparisonType::DIFF as i32,
                "<=" => ComparisonType::LE as i32,
                "<" => ComparisonType::LT as i32,
                op => return Err(format!("Unknown operator {}, expected one of >, >=, ==, !=, <=, <", op))
            },
            ..Default::default()
        })
    }

    pub fn new_return() -> Self {
        ASTBlockNode {
            node_type: NodeType::Return,
            ..Default::default()
        }
    }

    pub fn new_loop() -> Self {
        ASTBlockNode {
            node_type: NodeType::Loop,
            ..Default::default()
        }
    }

    pub fn new_function_call(func_id: String, parameters: Vec<Token>) -> Self {
        ASTBlockNode {
            node_type: NodeType::FunctionCall,
            name: func_id,
            arglist: parameters,
            ..Default::default()
        }
    }

    /// Parses a tri-comparison expression (e.g. $a < 10)
    pub fn parse_tricomp(tokens: &mut VecDeque<Token>) -> Result<Self, String> {
        match tokens.len() {
            3 => {},
            other => return Err(format!("Expected three tokens to match a tri-comp but found {}", other))
        };

        let first_token = match tokens.pop_front() {
            Some(av) => match av.value {
                Some(v) => Rc::from(RefCell::from(Self::new_id(v))),
                None => return Err(format!("Expected value for identifier token: {:?}", av))
            },
            None => unreachable!()
        };
        let mut operator = match tokens.pop_front() {
            Some(op) if op.ttype == TokenType::OP => match op.value {
                Some(v) => Self::new_comp_operator(v)?,
                None => return Err("Expected operator to have a value".to_string())
            },
            Some(op) => return Err(format!("Expected Comparison token as second token after if/whille operator but found {:?}", op)),
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

    /// Parses a tri-operation expression (e.g. $a + 10)
    pub fn parse_triop(tokens: &mut VecDeque<Token>) -> Result<Self, String> {
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
            Some(op) if op.ttype == TokenType::OP => match op.value {
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
        let mut assignment_stuff = VecDeque::from(stream.get_until(TokenType::LBRACE));
        assignment_stuff.pop_back();  // Remove Lbrace

        let condition = match assignment_stuff.len() {
            3 => Self::parse_tricomp(&mut assignment_stuff)?,
            other => return Err(format!("Expected 3 tokens after `while` keyword but found {} [{:?}]", other, assignment_stuff.iter().map(|s| &s.ttype)))
        };

        Ok(Self {
            node_type: NodeType::WhileLoop,
            descendents: (
                Some(Rc::from(RefCell::from(condition))), None, vec![]
            ),
            ..Default::default()
        })
    }

    pub fn parse_assignment(stream: &mut TokenStream) -> Result<Self, String> {
        let mut assignment_stuff = VecDeque::from(stream.get_until(TokenType::ENDL));
        assignment_stuff.pop_back();  // Remove endl

        let mut top_node = Self {
            node_type: NodeType::Assignment,
            descendents: (
                None, None, vec![]
            ),
            ..Default::default()
        };

        let assigned_to = match assignment_stuff.pop_front() {
            Some(t) if t.ttype == TokenType::ID  => t,
            Some(t) => return Err(format!("Unexpected token {:?} after set keyword", t)),
            None => return Err("Expected identifier after `set` keyword".to_string())
        };

        let lparam = match assigned_to.value {
            Some(v) => Rc::from(RefCell::from(Self::new_id(v))),
            None => return Err("Missing lparam value for assignment operation".to_string())
        };

        super::utils::ensure_next_token(&mut assignment_stuff, TokenType::OP, Some("=".to_string()))?;

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

    pub fn parse_if(stream: &mut TokenStream) -> Result<Self, String> {
        let mut assignment_stuff = VecDeque::from(stream.get_until(TokenType::LBRACE));
        assignment_stuff.pop_back();  // Remove Lbrace

        let condition = match assignment_stuff.len() {
            3 => Self::parse_tricomp(&mut assignment_stuff)?,
            other => return Err(format!("Expected 3 tokens after `while` keyword bu found {}", other))
        };

        Ok(Self {
            node_type: NodeType::IfCondition,
            descendents: (
                Some(Rc::from(RefCell::from(condition))), None, vec![]
            ),
            ..Default::default()
        })
    }

    /// Returns all the variables of the node
    pub fn get_variables(&self) -> HashSet<String> {
        let mut new_set = HashSet::new();
        match self.node_type {
            NodeType::Assignment => {
                new_set.insert(self.name);
            }
        }

        new_set
    }
}

impl fmt::Display for ASTBlockNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.node_type {
            NodeType::Identifier => write!(f, "{{type: \"Identifier\", value: {}}},", self.value),
            NodeType::Litteral => write!(f, "{{type: \"Litteral\", }}"),
            NodeType::Assignment => write!(f, "{{type: \"Assignment\", }}"),
            NodeType::Operation => write!(f, "{{type: \"Operation\", }}"),
            NodeType::Comparison => write!(f, "{{type: \"Comparison\", }}"),
            NodeType::WhileLoop => write!(f, "{{type: \"WhileLoop\", }}"),
            NodeType::Loop => write!(f, "{{type: \"Loop\", }}"),
            NodeType::IfCondition => write!(f, "{{type: \"IfCondition\", }}"),
            NodeType::FunctionCall => write!(f, "{{type: \"FunctionCall\", }}"),
            NodeType::Return => write!(f, "{{type: \"Return\"}}"),
        }
    }
}
