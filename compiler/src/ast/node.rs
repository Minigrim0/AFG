use std::fmt;

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

pub type CodeBlock = Vec<Box<Node>>;

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
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

impl Node {
    pub fn new_identifier(name: String) -> Self {
        if name.starts_with('$') {
            Self::MemoryValue {
                name: name[1..].to_string(),
            }
        } else {
            Self::Identifier { name }
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Identifier { name } => write!(f, "ID {}", name),
            Node::MemoryValue { name } => write!(f, "MEM {}", name),
            Node::Litteral { value } => write!(f, "LIT {}", value),
            Node::Register { name } => write!(f, "REG {}", name),
            Node::MemoryOffset { base, offset } => write!(f, "MOF\n{}\n{}", base, offset),
            Node::Assignment { lparam, rparam } => write!(f, "Assignment: {} {}", lparam, rparam),
            Node::Comparison {
                lparam,
                rparam,
                comparison,
            } => write!(f, "Comparison {} {} {}", lparam, comparison, rparam),
            Node::IfCondition { condition, content } => write!(
                f,
                "if {}\n{}",
                condition,
                content
                    .iter()
                    .map(|n| format!("{}", n))
                    .collect::<Vec<String>>()
                    .join("\n")
            ),
            Node::WhileLoop { condition, content } => write!(
                f,
                "while {}\n{}",
                condition,
                content
                    .iter()
                    .map(|n| format!("{}", n))
                    .collect::<Vec<String>>()
                    .join("\n")
            ),
            Node::Return { value } => write!(f, "ret {}", value),
            Node::Print { value } => write!(f, "Print {}", value),
            Node::Operation {
                lparam,
                rparam,
                operation,
            } => write!(f, "Op {} {} {}", lparam, operation, rparam),
            Node::FunctionCall {
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
            Node::Loop { content } => write!(
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

impl Default for Node {
    fn default() -> Self {
        Node::Litteral { value: 0 }
    }
}
