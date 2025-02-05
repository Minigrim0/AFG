use super::{MaybeInstructions, OperandType, PASMInstruction};
use crate::ast::node::Node;
use crate::ast::node::OperationType;

/// Ensure the operand is either an Identifier, a Register or an Literal
fn ensure_immediate(node: &Box<Node>) -> Result<OperandType, String> {
    match &**node {
        Node::Identifier { name } => Ok(OperandType::Identifier { name: name.clone() }),
        Node::Register { name } => Ok(OperandType::Register { name: name.clone() }),
        Node::Litteral { value } => Ok(OperandType::Literal { value: *value }),
        _ => Err("Operand should be either a Register, Identifier or Literal".to_string()),
    }
}

/// Ensures the operand is either an Identifier or a Literal
fn ensure_loadable_immediate(node: &Box<Node>) -> Result<OperandType, String> {
    match &**node {
        Node::Identifier { name } => Ok(OperandType::Identifier { name: name.clone() }),
        Node::Register { name } => Ok(OperandType::Register { name: name.clone() }),
        _ => Err("Operand should be either a Register, Identifier or Literal".to_string()),
    }
}

/// Ensure the operand is a memory location (Offset or direct value)
/// The result is a tuple with the new operand to use further in the
fn ensure_memory(node: &Box<Node>) -> Result<(OperandType, Vec<PASMInstruction>), String> {
    match &**node {
        Node::MemoryValue { name } => Ok((OperandType::Memory { name: name.clone() }, vec![])),
        Node::MemoryOffset { base, offset } => Ok((
            OperandType::MemoryOffset {
                base: Box::from(OperandType::new_register("GPC")),
                offset: *offset,
            },
            vec![PASMInstruction::new(
                "mov".to_string(),
                vec![
                    OperandType::Register {
                        name: "GPC".to_string(),
                    },
                    match &**base {
                        Node::Identifier { name } => OperandType::Identifier { name: name.clone() },
                        _ => {
                            return Err("Base for memory offset should be an identifier".to_string())
                        }
                    },
                ],
            )],
        )),
        _ => Err("Operand should be either a Memory address or a Memory Offset".to_string()),
    }
}

/// Loads a value into the given register
fn load_to_register<S: AsRef<str>>(
    register: S,
    from: &Box<Node>,
) -> Result<(OperandType, Vec<PASMInstruction>), String> {
    let destination = Box::from(Node::Register {
        name: register.as_ref().to_string(),
    });
    match &**from {
        // If the operand is already in a register
        Node::Register { name } => Ok((OperandType::new_register(name), vec![])),
        Node::Litteral { .. } | Node::Identifier { .. } => {
            Ok((OperandType::new_register(register), imm_to_imm(from, &destination)?))
        },
        Node::MemoryValue { .. } | Node::MemoryOffset { .. } => {
            Ok((OperandType::new_register(register), mem_to_imm(from, &destination)?))
        },
        _ => Err(
            "Operand for loading into a register must be either a Literal, Register, Identifier, Memory Address or Offseted memory address"
                .to_string()
        )
    }
}

/// Performs an assignment from register to register, literal to register or stack to register
pub fn imm_to_imm(from: &Box<Node>, to: &Box<Node>) -> MaybeInstructions {
    let to = ensure_immediate(to)?;
    let from = ensure_immediate(from)?;

    Ok(vec![PASMInstruction::new(
        "mov".to_string(),
        vec![to, from],
    )])
}

/// Performs an assignment from memory to immediate
/// The immediate must either be a register or an identifier for a stack offset
pub fn mem_to_imm(from: &Box<Node>, to: &Box<Node>) -> MaybeInstructions {
    let to = ensure_loadable_immediate(to)?;
    let (from, mut instructions) = ensure_memory(from)?;

    instructions.push(PASMInstruction::new("load".to_string(), vec![to, from]));

    Ok(instructions)
}

/// Performs an assignment from register to memory
pub fn imm_to_mem(from: &Box<Node>, to: &Box<Node>) -> MaybeInstructions {
    let (to, mut instructions) = ensure_memory(to)?;
    let from = ensure_immediate(from)?;

    instructions.push(PASMInstruction::new("store".to_string(), vec![to, from]));

    Ok(instructions)
}

/// Performs an assignment from memory to memory (going through registers)
pub fn mem_to_mem(from: &Box<Node>, to: &Box<Node>) -> MaybeInstructions {
    let (to, to_instructions) = ensure_memory(to)?;
    let (from, mut instructions) = ensure_memory(from)?;

    instructions.push(PASMInstruction::new(
        "load".to_string(),
        vec![OperandType::new_register("GPB"), from],
    ));
    instructions.extend(to_instructions);
    instructions.push(PASMInstruction::new(
        "store".to_string(),
        vec![to, OperandType::new_register("GPB")],
    ));

    Ok(instructions)
}

/// Performs an assignment from memory to memory (going through registers)
pub fn op_to_imm(from: &Box<Node>, to: &Box<Node>) -> MaybeInstructions {
    match &**from {
        Node::Operation {
            lparam,
            rparam,
            operation,
        } => {
            // Load operands into registers
            let (op1_register, mut instructions) = load_to_register("GPA", lparam)?;
            let (op2_register, op2_instructions) = load_to_register("GPB", rparam)?;
            instructions.extend(op2_instructions);
            let operation = match operation {
                OperationType::Addition => "add",
                OperationType::Substraction => "sub",
                OperationType::Multiplication => "mul",
                OperationType::Division => "div",
                OperationType::Modulo => "mod",
            };
            // Perform the operation
            instructions.push(PASMInstruction::new(
                operation.to_string(),
                vec![op1_register.clone(), op2_register],
            ));
            instructions.extend(imm_to_imm(
                &Box::from(Node::Register {
                    name: if let OperandType::Register { name } = op1_register {
                        name
                    } else {
                        unreachable!()
                    },
                }),
                to,
            )?);
            Ok(instructions)
        }
        n => Err(format!(
            "Can't use `op_to_inst` on a non-operation node {:?}",
            n
        )),
    }
}

/// Performs an assignment from memory to memory (going through registers)
pub fn op_to_mem(from: &Box<Node>, to: &Box<Node>) -> MaybeInstructions {
    match &**from {
        Node::Operation {
            lparam,
            rparam,
            operation,
        } => {
            // Load operands into registers
            let (op1_register, mut instructions) = load_to_register("GPA", lparam)?;
            let (op2_register, op2_instructions) = load_to_register("GPB", rparam)?;
            instructions.extend(op2_instructions);
            let operation = match operation {
                OperationType::Addition => "add",
                OperationType::Substraction => "sub",
                OperationType::Multiplication => "mul",
                OperationType::Division => "div",
                OperationType::Modulo => "mod",
            };
            // Perform the operation
            instructions.push(PASMInstruction::new(
                operation.to_string(),
                vec![op1_register.clone(), op2_register],
            ));
            instructions.extend(imm_to_mem(
                &Box::from(Node::Register {
                    name: if let OperandType::Register { name } = op1_register {
                        name
                    } else {
                        unreachable!()
                    },
                }),
                to,
            )?);
            Ok(instructions)
        }
        n => Err(format!(
            "Can't use `op_to_inst` on a non-operation node {:?}",
            n
        )),
    }
}
