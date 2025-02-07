use std::fmt;

#[derive(Clone, Debug)]
pub enum OperandType {
    Identifier {
        name: String,
    },
    Literal {
        value: i32,
    },
    MemoryOffset {
        base: Box<OperandType>,
        offset: Box<OperandType>, // Identifier, Literal or Register
    },
    Memory {
        name: String,
    },
    Register {
        name: String,
    },
    Stack {
        register: Box<OperandType>,
        operation: String,
        offset: usize,
    },
}

impl OperandType {
    pub fn new_literal(value: i32) -> Self {
        OperandType::Literal { value }
    }

    pub fn new_stack<S: AsRef<str>>(register: S, offset: i32) -> Self {
        OperandType::Stack {
            register: Box::from(OperandType::Register {
                name: register.as_ref().to_string(),
            }),
            operation: if offset < 0 {
                "+".to_string()
            } else {
                "-".to_string()
            },
            offset: offset.abs() as usize,
        }
    }

    pub fn new_register<S: AsRef<str>>(name: S) -> Self {
        Self::Register {
            name: name.as_ref().to_string(),
        }
    }

    pub fn is_register(&self) -> bool {
        match self {
            OperandType::Identifier { name } => name.starts_with("'"),
            _ => false,
        }
    }

    pub fn is_memory(&self) -> bool {
        match self {
            OperandType::Identifier { name } => name.starts_with("$"),
            _ => false,
        }
    }

    pub fn is_frame_variable(&self) -> bool {
        !self.is_register() && !self.is_memory()
    }

    pub fn get_frame_variable(&self) -> Option<String> {
        match self {
            OperandType::Identifier { name } if self.is_frame_variable() => Some(name.clone()),
            _ => None,
        }
    }

    pub fn get_register_name(&self) -> Option<String> {
        match self {
            OperandType::Identifier { name } if name.starts_with("'") => Some(name.clone()),
            _ => None,
        }
    }
}

impl fmt::Display for OperandType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OperandType::Identifier { name } => write!(f, "@{}", name),
            OperandType::Memory { name } => write!(f, "${}", name),
            OperandType::Register { name } => write!(f, "'{}", name),
            OperandType::Literal { value } => write!(f, "#{}", value),
            OperandType::Stack {
                register,
                operation,
                offset,
            } => write!(f, "[{} {} {}]", register, operation, offset),
            OperandType::MemoryOffset { base, offset } => write!(f, "[{} + {}]", base, offset),
        }
    }
}
