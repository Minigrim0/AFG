use std::collections::HashMap;

use super::asm::{OperandType, PASMInstruction};

fn allocate_memory(
    allocation_map: &mut HashMap<String, usize>,
    current_level: usize,
    variable: String,
) -> (usize, usize) {
    if allocation_map.contains_key(&variable) {
        (*allocation_map.get(&variable).unwrap(), current_level)
    } else {
        allocation_map.insert(variable, current_level);
        (current_level, current_level + 1)
    }
}

/// Loads a variable into a register. If the variable is a special variable, it is simply loaded
/// from memory. If the variable is a normal variable, it is loaded from its allocated place (if any)
/// and given one if it doesn't have one.
/// If the variable is a literal, it is simply moved into the register.
/// Returns the new memory pointer (changed if a new allocation happened) and the instructions to load the variable.
fn load_variable<S: AsRef<str>>(
    variable: Option<&OperandType>,
    register: S,
    variable_map: &mut HashMap<String, usize>,
    mut memory_pointer: usize,
) -> (Vec<PASMInstruction>, usize) {
    let mut instructions = vec![];

    if let Some(OperandType::Identifier {
        name: source_variable,
    }) = variable
    {
        // Special variable, simply load into register
        if source_variable.starts_with("$") {
            instructions.push(PASMInstruction::new(
                "load".to_string(),
                vec![
                    OperandType::Identifier {
                        name: register.as_ref().to_string(),
                    },
                    OperandType::Identifier {
                        name: source_variable.clone(),
                    },
                ],
            ));
        }
        // Normal variable, allocate/find memory location and load into register
        else if !source_variable.starts_with("'") {
            let (variable_location, new_pointer) =
                allocate_memory(variable_map, memory_pointer, source_variable.clone());
            memory_pointer = new_pointer;

            instructions.push(PASMInstruction::new(
                "load".to_string(),
                vec![
                    OperandType::Identifier {
                        name: register.as_ref().to_string(),
                    },
                    OperandType::Literal {
                        value: variable_location as i32,
                    },
                ],
            ));
        }
    }
    // Literal, simply move into register
    else if let Some(OperandType::Literal {
        value: source_value,
    }) = variable
    {
        instructions.push(PASMInstruction::new(
            "mov".to_string(),
            vec![
                OperandType::Identifier {
                    name: register.as_ref().to_string(),
                },
                OperandType::Literal {
                    value: *source_value,
                },
            ],
        ));
    }

    (instructions, memory_pointer)
}

/// Stores the content of a register into a variable. If the variable is a special variable, the register's content
/// is stored into the sepcial variable. If the variable is a normal variable, the register's content is stored into
/// the memory location allocated for the variable (the variable gets a location allocated if needed). If the variable
/// is a register, the register's content is moved into the register.
fn store_variable<S: AsRef<str>>(
    variable: Option<&OperandType>,
    register: S,
    variable_map: &mut HashMap<String, usize>,
    mut memory_pointer: usize,
) -> (Vec<PASMInstruction>, usize) {
    let mut instructions = vec![];

    if let Some(OperandType::Identifier {
        name: destination_variable,
    }) = variable
    {
        if destination_variable.starts_with("$") {
            instructions.push(PASMInstruction::new(
                "store".to_string(),
                vec![
                    OperandType::Identifier {
                        name: destination_variable.clone(),
                    },
                    OperandType::Identifier {
                        name: register.as_ref().to_string(),
                    },
                ],
            ));
        } else if destination_variable.starts_with("'") {
            instructions.push(PASMInstruction::new(
                "mov".to_string(),
                vec![
                    OperandType::Identifier {
                        name: destination_variable.clone(),
                    },
                    OperandType::Identifier {
                        name: register.as_ref().to_string(),
                    },
                ],
            ));
        } else {
            let (variable_location, new_pointer) =
                allocate_memory(variable_map, memory_pointer, destination_variable.clone());
            memory_pointer = new_pointer;

            instructions.push(PASMInstruction::new(
                "store".to_string(),
                vec![
                    OperandType::Literal {
                        value: variable_location as i32,
                    },
                    OperandType::Identifier {
                        name: register.as_ref().to_string(),
                    },
                ],
            ));
        }
    }

    (instructions, memory_pointer)
}

/// Updates the PASM program with register allocation.
/// Allocation is in its most basic form, where each variable is allocated a memory location.
/// If a variable is used in an instruction, it is loaded into a register, and if it is the destination
/// of an instruction, the result is stored in the variable.
/// This is far from optimal but easy to implement.
pub fn allocate(
    function: &Vec<PASMInstruction>,
    mut memory_top_pointer: usize,
) -> Result<(Vec<PASMInstruction>, usize), String> {
    // The variable map associates variables in the code to memory locations
    let mut variable_map: HashMap<String, usize> = HashMap::new();
    let mut next_instructions: Vec<PASMInstruction> = Vec::new();

    for instruction in function.iter() {
        // If the instruction is a label, we don't need to do anything
        if instruction.is_label {
            next_instructions.push(instruction.clone());
            continue;
        }

        match instruction.opcode.as_str() {
            // If the instruction is a mov, we need to check if the source is a variable
            // and if the destination is a variable
            "mov" | "load" => {
                // If the source is a variable, we need to load it into a register
                let (new_instructions, new_pointer) = load_variable(
                    instruction.operands.get(1),
                    "'GPA",
                    &mut variable_map,
                    memory_top_pointer,
                );
                memory_top_pointer = new_pointer;
                next_instructions.extend(new_instructions);

                // If the destination is a variable, we need to store the result
                let (new_instructions, new_pointer) = store_variable(
                    instruction.operands.get(0),
                    "'GPA",
                    &mut variable_map,
                    memory_top_pointer,
                );
                memory_top_pointer = new_pointer;
                next_instructions.extend(new_instructions);
            }
            "add" | "sub" | "mul" | "div" | "mod" => {
                // Load first operand into GPA
                let (new_instructions, new_pointer) = load_variable(
                    instruction.operands.get(0),
                    "'GPA",
                    &mut variable_map,
                    memory_top_pointer,
                );
                memory_top_pointer = new_pointer;
                next_instructions.extend(new_instructions);

                // Load second operand into GPB
                let (new_instructions, new_pointer) = load_variable(
                    instruction.operands.get(1),
                    "'GPB",
                    &mut variable_map,
                    memory_top_pointer,
                );
                memory_top_pointer = new_pointer;
                next_instructions.extend(new_instructions);

                // Perform the operation
                next_instructions.push(PASMInstruction::new(
                    instruction.opcode.clone(),
                    vec![
                        OperandType::Identifier {
                            name: "'GPA".to_string(),
                        },
                        OperandType::Identifier {
                            name: "'GPB".to_string(),
                        },
                    ],
                ));

                // Save the result into the destination variable
                let (new_instructions, new_pointer) = store_variable(
                    instruction.operands.get(2),
                    "'GPA",
                    &mut variable_map,
                    memory_top_pointer,
                );
                memory_top_pointer = new_pointer;
                next_instructions.extend(new_instructions);
            }
            "cmp" => {
                // load first operand into GPA
                let (new_instructions, new_pointer) = load_variable(
                    instruction.operands.get(0),
                    "'GPA",
                    &mut variable_map,
                    memory_top_pointer,
                );
                memory_top_pointer = new_pointer;
                next_instructions.extend(new_instructions);

                // Load second operand into GPB
                let (new_instructions, new_pointer) = load_variable(
                    instruction.operands.get(1),
                    "'GPB",
                    &mut variable_map,
                    memory_top_pointer,
                );
                memory_top_pointer = new_pointer;
                next_instructions.extend(new_instructions);

                // Compare the two operands
                next_instructions.push(PASMInstruction::new(
                    instruction.opcode.clone(),
                    vec![
                        super::asm::OperandType::Identifier {
                            name: "'GPA".to_string(),
                        },
                        super::asm::OperandType::Identifier {
                            name: "'GPB".to_string(),
                        },
                    ],
                ));
            }
            "pop" => {
                next_instructions.push(PASMInstruction::new(
                    instruction.opcode.clone(),
                    vec![super::asm::OperandType::Identifier {
                        name: "'GPA".to_string(),
                    }],
                ));

                let (new_instructions, new_pointer) = store_variable(
                    instruction.operands.get(0),
                    "'GPA",
                    &mut variable_map,
                    memory_top_pointer,
                );
                memory_top_pointer = new_pointer;
                next_instructions.extend(new_instructions);
            }
            "push" => {
                // Load the variable into GPA
                let (new_instructions, new_pointer) = load_variable(
                    instruction.operands.get(0),
                    "'GPA",
                    &mut variable_map,
                    memory_top_pointer,
                );
                memory_top_pointer = new_pointer;
                next_instructions.extend(new_instructions);

                // Push the variable
                next_instructions.push(PASMInstruction::new(
                    instruction.opcode.clone(),
                    vec![super::asm::OperandType::Identifier {
                        name: "'GPA".to_string(),
                    }],
                ));
            }
            // Other instructions don't need to be modified
            _ => {
                next_instructions.push(instruction.clone());
            }
        }
    }

    Ok((next_instructions, memory_top_pointer))
}
