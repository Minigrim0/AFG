use std::collections::HashMap;

use super::asm::PASMInstruction;

fn allocate_memory(
    allocation_map: &mut HashMap<String, usize>,
    current_level: usize,
    variable: String,
) -> (usize, usize) {
    if allocation_map.contains_key(&variable) {
        (*allocation_map.get(&variable).unwrap(), current_level)
    } else {
        allocation_map.insert(variable, current_level + 1);
        (current_level + 1, current_level + 1)
    }
}

/// Updates the PASM program with register allocation.
/// This means that the
/// This is done using the following algorithm.
pub fn allocate(function: &Vec<PASMInstruction>) -> Result<Vec<PASMInstruction>, String> {
    // The variable map associates variables in the code to memory locations
    let mut variable_map: HashMap<String, usize> = HashMap::new();
    let mut memory_top_pointer = 0;
    let mut next_instructions: Vec<PASMInstruction> = Vec::new();

    for instruction in function.iter() {
        // If the instruction is a label, we don't need to do anything
        if instruction.is_label {
            next_instructions.push(instruction.clone());
            continue;
        }

        match instruction.opcode.as_str() {
            // I fthe instruction is a jump, we don't need to do anything
            "jmp" | "jeq" | "jne" | "jlt" | "jgt" | "jle" | "jge" => {
                // These instructions are jumps, they don't need to be modified
                next_instructions.push(instruction.clone());
            }
            // If the instruction is a mov, we need to check if the source is a variable
            // and if the destination is a variable
            "mov" => {
                // If the source is a variable, load it into 'GPA
                if let Some(super::asm::OperandType::Identifier {
                    name: source_variable,
                }) = instruction.operands.get(1)
                {
                    let mut variable_location = 0;
                    (variable_location, memory_top_pointer) = allocate_memory(
                        &mut variable_map,
                        memory_top_pointer,
                        source_variable.clone(),
                    );

                    next_instructions.push(PASMInstruction {
                        is_label: false,
                        opcode: "load".to_string(),
                        operands: vec![
                            super::asm::OperandType::Identifier {
                                name: "'GPA".to_string(),
                            },
                            super::asm::OperandType::Literal {
                                value: variable_location as i32,
                            },
                        ],
                    });
                } else if let Some(super::asm::OperandType::Literal {
                    value: source_value,
                }) = instruction.operands.get(1)
                {
                    next_instructions.push(PASMInstruction {
                        is_label: false,
                        opcode: "mov".to_string(),
                        operands: vec![
                            super::asm::OperandType::Identifier {
                                name: "'GPA".to_string(),
                            },
                            super::asm::OperandType::Literal {
                                value: *source_value,
                            },
                        ],
                    });
                }

                // Destination is most likely a variable, we need to store the value
                if let Some(super::asm::OperandType::Identifier {
                    name: destination_variable,
                }) = instruction.operands.get(0)
                {
                    let mut variable_location = 0;
                    (variable_location, memory_top_pointer) = allocate_memory(
                        &mut variable_map,
                        memory_top_pointer,
                        destination_variable.clone(),
                    );

                    next_instructions.push(PASMInstruction {
                        is_label: false,
                        opcode: "store".to_string(),
                        operands: vec![
                            super::asm::OperandType::Literal {
                                value: variable_location as i32,
                            },
                            super::asm::OperandType::Identifier {
                                name: "'GPA".to_string(),
                            },
                        ],
                    });
                }
            }
            "cmp" => {
                // If any of the operands is a variable, we need to load them.
                if let Some(super::asm::OperandType::Identifier {
                    name: first_var_name,
                }) = instruction.operands.get(0)
                {
                    let mut variable_location = 0;
                    (variable_location, memory_top_pointer) = allocate_memory(
                        &mut variable_map,
                        memory_top_pointer,
                        first_var_name.clone(),
                    );

                    next_instructions.push(PASMInstruction {
                        is_label: false,
                        opcode: "load".to_string(),
                        operands: vec![
                            super::asm::OperandType::Identifier {
                                name: "'GPA".to_string(),
                            },
                            super::asm::OperandType::Literal {
                                value: variable_location as i32,
                            },
                        ],
                    });
                }

                if let Some(super::asm::OperandType::Identifier {
                    name: second_var_name,
                }) = instruction.operands.get(1)
                {
                    let mut variable_location = 0;
                    (variable_location, memory_top_pointer) = allocate_memory(
                        &mut variable_map,
                        memory_top_pointer,
                        second_var_name.clone(),
                    );

                    next_instructions.push(PASMInstruction {
                        is_label: false,
                        opcode: "load".to_string(),
                        operands: vec![
                            super::asm::OperandType::Identifier {
                                name: "'GPB".to_string(),
                            },
                            super::asm::OperandType::Literal {
                                value: variable_location as i32,
                            },
                        ],
                    });
                }

                next_instructions.push(PASMInstruction {
                    operands: vec![
                        super::asm::OperandType::Identifier {
                            name: "'GPA".to_string(),
                        },
                        super::asm::OperandType::Identifier {
                            name: "'GPB".to_string(),
                        },
                    ],
                    ..instruction.clone()
                });
            }
            "pop" => {
                next_instructions.push(PASMInstruction {
                    operands: vec![super::asm::OperandType::Identifier {
                        name: "'GPA".to_string(),
                    }],
                    ..instruction.clone()
                });

                if let Some(super::asm::OperandType::Identifier {
                    name: variable_name,
                }) = instruction.operands.get(0)
                {
                    let mut variable_location = 0;
                    (memory_top_pointer, variable_location) = allocate_memory(
                        &mut variable_map,
                        memory_top_pointer,
                        variable_name.clone(),
                    );

                    next_instructions.push(PASMInstruction {
                        is_label: false,
                        opcode: "store".to_string(),
                        operands: vec![
                            super::asm::OperandType::Literal {
                                value: variable_location as i32,
                            },
                            super::asm::OperandType::Identifier {
                                name: "'GPA".to_string(),
                            },
                        ],
                    });
                }
            }
            _ => {
                next_instructions.push(instruction.clone());
            }
        }
    }

    Ok(next_instructions)
}
