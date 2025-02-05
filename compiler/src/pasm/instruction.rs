use std::fmt;

use super::OperandType;

#[derive(Clone)]
/// A Pseudo-assembly instruction.
pub struct PASMInstruction {
    pub is_label: bool,             // Whether this is just a label or not
    pub is_comment: bool,           // Whether this is just a label or not
    pub opcode: String,             // Will not change until the end
    pub operands: Vec<OperandType>, // Up to two operands
}

impl PASMInstruction {
    pub fn new_label(name: String) -> Self {
        Self {
            is_label: true,
            is_comment: false,
            opcode: name,
            operands: vec![],
        }
    }

    pub fn new_comment(comment: String) -> Self {
        Self {
            is_label: false,
            is_comment: true,
            opcode: comment,
            operands: vec![],
        }
    }

    pub fn new(instr: String, operands: Vec<OperandType>) -> Self {
        Self {
            is_label: false,
            is_comment: false,
            opcode: instr,
            operands,
        }
    }

    pub fn get_live_and_dead(&self) -> (Vec<String>, Vec<String>) {
        let mut operand_0 = if let Some(OperandType::Identifier { name }) = self.operands.get(0) {
            if !name.starts_with("$") && !name.starts_with("'") {
                vec![name.clone()]
            } else {
                vec![]
            }
        } else {
            vec![]
        };

        let operand_1 = if let Some(OperandType::Identifier { name }) = self.operands.get(1) {
            if !name.starts_with("$") && !name.starts_with("'") {
                vec![name.clone()]
            } else {
                vec![]
            }
        } else {
            vec![]
        };

        match self.opcode.as_str() {
            "load" | "pop" | "mov" => (operand_1, operand_0),
            "add" | "sub" | "mul" | "div" | "mod" | "cmp" | "store" | "push" => {
                operand_0.extend(operand_1);
                (operand_0, vec![])
            }
            _ => (vec![], vec![]),
        }
    }

    /// If this instruction is a jump, returns the label to jump to
    pub fn jump_to(&self) -> Option<String> {
        if self.opcode.starts_with("j") || self.opcode == "call" {
            if let Some(OperandType::Identifier { name }) = self.operands.get(0) {
                return Some(name.clone());
            }
        }
        None
    }
}

impl fmt::Debug for PASMInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format!("{}", self).fmt(f)
    }
}

impl fmt::Display for PASMInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_label {
            write!(f, "{}:", self.opcode)
        } else if self.is_comment {
            write!(f, "; {}", self.opcode)
        } else {
            write!(f, "{}", self.opcode)?;
            for operand in self.operands.iter() {
                match operand {
                    OperandType::Identifier { name } => {
                        if name.starts_with("$") || name.starts_with("'") {
                            write!(f, " {}", name)?
                        } else {
                            write!(f, " @{}", name)?
                        }
                    }
                    OperandType::Register { name } => write!(f, " '{}", name)?,
                    OperandType::Memory { name } => write!(f, " ${}", name)?,
                    OperandType::Literal { value } => write!(f, " #{}", value)?,
                    OperandType::Stack {
                        register,
                        operation,
                        offset,
                    } => write!(f, " [{} {} {}]", register, operation, offset)?,
                    OperandType::MemoryOffset { base, offset } => {
                        write!(f, " [{} + {}]", base, offset)?
                    }
                }
            }
            Ok(())
        }
    }
}
