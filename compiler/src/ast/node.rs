use std::fmt;

use crate::lexer::token::TokenLocation;

#[derive(Debug, Default, Clone, PartialEq)]
pub enum ComparisonType {
    GT,
    GE,
    #[default]
    EQ,
    LE,
    LT,
    DIFF,
}

impl fmt::Display for ComparisonType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr = match self {
            ComparisonType::GT => "GT",
            ComparisonType::GE => "GE",
            ComparisonType::EQ => "EQ",
            ComparisonType::LE => "LE",
            ComparisonType::LT => "LT",
            ComparisonType::DIFF => "DIFF",
        };
        write!(f, "{}", repr)
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum OperationType {
    #[default]
    Addition,
    Substraction,
    Multiplication,
    Division,
    Modulo,
}

impl fmt::Display for OperationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr = match self {
            OperationType::Addition => "Addition",
            OperationType::Substraction => "Substraction",
            OperationType::Multiplication => "Multiplication",
            OperationType::Division => "Division",
            OperationType::Modulo => "Modulo",
        };
        write!(f, "{}", repr)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub kind: NodeKind,
    pub span: Option<TokenLocation>,
}

impl Node {
    pub fn new(kind: NodeKind) -> Self {
        Self { kind, span: None }
    }

    pub fn with_span(kind: NodeKind, span: TokenLocation) -> Self {
        Self {
            kind,
            span: Some(span),
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

pub type CodeBlock = Vec<Box<Node>>;

#[derive(Debug, Clone, PartialEq)]
pub enum NodeKind {
    Identifier {
        name: String,
    },
    Register {
        name: String,
    },
    MemoryOffset {
        // a[0], a[b], ...
        base: Box<Node>,
        offset: Box<Node>, // Literal, Identifier or Register
    },
    MemoryValue {
        // $Velocity
        name: String,
    },
    Litteral {
        value: i32,
    },
    Assignment {
        lparam: Box<Node>,
        rparam: Box<Node>,
    },
    Operation {
        lparam: Box<Node>,
        rparam: Box<Node>,
        operation: OperationType,
    },
    Print {
        value: Box<Node>,
    },
    Comparison {
        lparam: Box<Node>,
        rparam: Box<Node>,
        comparison: ComparisonType,
    },
    WhileLoop {
        condition: Box<Node>, // Should be a comparison
        content: CodeBlock,
    },
    Loop {
        content: CodeBlock,
    },
    IfCondition {
        condition: Box<Node>, // Should be a Comparison
        content: CodeBlock,
    },
    FunctionCall {
        function_name: String,
        parameters: CodeBlock, // A list of identifiers or literals
    },
    Return {
        value: Box<Node>,
    },
}

impl NodeKind {
    pub fn new_identifier(name: String) -> Self {
        if name.starts_with('$') {
            Self::MemoryValue {
                name: name[1..].to_string(),
            }
        } else {
            Self::Identifier { name }
        }
    }

    pub fn new_mem_offset(base: Node, offset: Node) -> Self {
        Self::MemoryOffset {
            base: Box::new(base),
            offset: Box::new(offset),
        }
    }

    /// Creates a new function call node kind
    pub fn new_fun_call(fun_name: String, parameters: CodeBlock) -> Self {
        Self::FunctionCall {
            function_name: fun_name,
            parameters,
        }
    }
}

impl fmt::Display for NodeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NodeKind::Identifier { name } => write!(f, "ID {}", name),
            NodeKind::MemoryValue { name } => write!(f, "MEM {}", name),
            NodeKind::Litteral { value } => write!(f, "LIT {}", value),
            NodeKind::Register { name } => write!(f, "REG {}", name),
            NodeKind::MemoryOffset { base, offset } => write!(f, "MOF\n{}\n{}", base, offset),
            NodeKind::Assignment { lparam, rparam } => {
                write!(f, "Assignment: {} {}", lparam, rparam)
            }
            NodeKind::Comparison {
                lparam,
                rparam,
                comparison,
            } => write!(f, "Comparison {} {} {}", lparam, comparison, rparam),
            NodeKind::IfCondition { condition, content } => write!(
                f,
                "if {}\n{}",
                condition,
                content
                    .iter()
                    .map(|n| format!("{}", n))
                    .collect::<Vec<String>>()
                    .join("\n")
            ),
            NodeKind::WhileLoop { condition, content } => write!(
                f,
                "while {}\n{}",
                condition,
                content
                    .iter()
                    .map(|n| format!("{}", n))
                    .collect::<Vec<String>>()
                    .join("\n")
            ),
            NodeKind::Return { value } => write!(f, "ret {}", value),
            NodeKind::Print { value } => write!(f, "Print {}", value),
            NodeKind::Operation {
                lparam,
                rparam,
                operation,
            } => write!(f, "Op {} {} {}", lparam, operation, rparam),
            NodeKind::FunctionCall {
                function_name,
                parameters,
            } => write!(
                f,
                "fn {} {}",
                function_name,
                parameters
                    .iter()
                    .map(|n| format!("{}", n))
                    .collect::<Vec<String>>()
                    .join("\n")
            ),
            NodeKind::Loop { content } => write!(
                f,
                "Loop\n{}",
                content
                    .iter()
                    .map(|n| format!("{}", n))
                    .collect::<Vec<String>>()
                    .join("\n")
            ),
        }
    }
}

impl Default for NodeKind {
    fn default() -> Self {
        NodeKind::Litteral { value: 0 }
    }
}
