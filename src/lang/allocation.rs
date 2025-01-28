use std::collections::HashMap;

use bevy::color::palettes::css::PERU;

use super::asm::{OperandType, PASMInstruction};

/// Updates the allocation map if the queried variable is not yet alllocated.
/// If the returned offset is negative, it means the variable is a parameter of the function
fn allocate_memory(
    allocation_map: &mut HashMap<String, i32>,
    current_level: usize,
    variable: String,
) -> (i32, usize) {
    if allocation_map.contains_key(&variable) {
        (*allocation_map.get(&variable).unwrap(), current_level)
    } else {
        allocation_map.insert(variable, current_level as i32);
        (current_level as i32, current_level + 1)
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
    variable_map: &mut HashMap<String, i32>,
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
            // Is it the first time we see this variable ?
            if memory_pointer != new_pointer {
                println!("Error, unknown variable used as rparam");
            }
            memory_pointer = new_pointer;

            instructions.push(PASMInstruction::new(
                "mov".to_string(),
                vec![
                    OperandType::Identifier {
                        name: register.as_ref().to_string(),
                    },
                    OperandType::Stack {
                        register: Box::from(OperandType::Identifier {
                            name: "'SBP".to_string()
                        }),
                        operation: if variable_location < 0 {
                            "+".to_string()
                        } else {
                            "-".to_string()
                        },
                        offset: variable_location.abs() as usize
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
    variable_map: &mut HashMap<String, i32>,
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
            let first_time = memory_pointer != new_pointer;
            memory_pointer = new_pointer;

            if first_time {
                instructions.push(PASMInstruction::new(
                    "push".to_string(),
                    vec![
                        OperandType::Identifier {
                            name: register.as_ref().to_string(),
                        }
                    ]));
            } else {
                instructions.push(PASMInstruction::new(
                    "mov".to_string(),
                    vec![
                        OperandType::Stack {
                            register: Box::from(OperandType::Identifier { name: "'SBP".to_string() }),
                            operation: if variable_location < 0 {
                                    "+".to_string()
                                } else {
                                    "-".to_string()
                            },
                            offset: variable_location.abs() as usize
                        },
                        OperandType::Identifier {
                            name: register.as_ref().to_string(),
                        },
                    ],
                ));
            }
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
    function: &(Vec<String>, Vec<PASMInstruction>),
) -> Result<Vec<PASMInstruction>, String> {
    // The variable map associates variables in the code to memory locations
    let mut variable_map: HashMap<String, i32> = HashMap::new();
    let mut next_instructions: Vec<PASMInstruction> = Vec::new();
    let mut stack_offset_pointer = 1;  // 0 Is reserved for 'SBP already

    for (index, parameter) in function.0.iter().enumerate() {
        variable_map.insert(parameter.clone(), - (index as i32 + 2));
    }

    for instruction in function.1.iter() {
        // If the instruction is a label, we don't need to do anything
        if instruction.is_label {
            next_instructions.push(instruction.clone());
            continue;
        }

        // next_instructions.push(PASMInstruction::new_comment(format!("OG: {}", instruction)));
        match instruction.opcode.as_str() {
            // If the instruction is a mov, we need to check if the source is a variable
            // and if the destination is a variable
            "mov" => {
                // If operand2_location has a value, its the offset of this variable in the stack.
                // If not, the operands is a literal or a register (meaning cimply copy it)
                let operand2_location = if let OperandType::Identifier { name } = &instruction.operands[1] {
                    if name.starts_with("$") || name.starts_with("'") {
                        None
                    } else {
                        let (variable_location, new_pointer) =
                            allocate_memory(&mut variable_map, stack_offset_pointer, name.clone());
                        // Is this a new variable ?
                        if stack_offset_pointer != new_pointer {
                            return Err(format!("Unknown variable used as rparam for mov instruction: `{}`", instruction));
                        }
                        Some(variable_location)
                    }
                } else {
                    None
                };

                match &instruction.operands[0] {
                    OperandType::Identifier { name } if instruction.operands[0].is_frame_variable() => {
                        // moving into a known variable
                        if variable_map.contains_key(name) {
                            let operand1_location = variable_map[name];
                            next_instructions.push(PASMInstruction::new(
                                "mov".to_string(),
                                vec![
                                    OperandType::new_stack("'SBP".to_string(), operand1_location),
                                    if let Some(operand2_location) = operand2_location {
                                        OperandType::new_stack("'SBP".to_string(), operand2_location)
                                    } else {
                                       instruction.operands[1].clone()
                                    }
                                ]
                            ))
                        } else {  // Pushing to a new variable
                            let (_, new_pointer) =
                                allocate_memory(&mut variable_map, stack_offset_pointer, name.clone());
                            stack_offset_pointer = new_pointer;
                            next_instructions.push(PASMInstruction::new(
                                "push".to_string(),
                                vec![
                                    if let Some(operand2_location) = operand2_location {
                                        OperandType::new_stack("'SBP".to_string(), operand2_location)
                                    } else {
                                       instruction.operands[1].clone()
                                    }
                                ]
                            ))
                        }
                    }
                    // moving to a register
                    _ => {
                        next_instructions.push(PASMInstruction::new(
                            "mov".to_string(),
                            vec![
                                instruction.operands[0].clone(),
                                if let Some(operand2_location) = operand2_location {
                                    OperandType::new_stack("'SBP".to_string(), operand2_location)
                                } else {
                                   instruction.operands[1].clone()
                                }
                            ]
                        ))
                    }
                };
            }
            "load" => {
                // If the source is a variable, we need to load it into a register
                let (new_instructions, new_pointer) = load_variable(
                    instruction.operands.get(1),
                    "'GPA",
                    &mut variable_map,
                    stack_offset_pointer,
                );
                stack_offset_pointer = new_pointer;
                next_instructions.extend(new_instructions);

                // If the destination is a variable, we need to store the result
                let (new_instructions, new_pointer) = store_variable(
                    instruction.operands.get(0),
                    "'GPA",
                    &mut variable_map,
                    stack_offset_pointer,
                );
                stack_offset_pointer = new_pointer;
                next_instructions.extend(new_instructions);
            }
            "store" => {
                let operand2_location = if let OperandType::Identifier { name } = &instruction.operands[1] {
                    if name.starts_with("$") || name.starts_with("'") {
                        None
                    } else {
                        let (variable_location, new_pointer) =
                            allocate_memory(&mut variable_map, stack_offset_pointer, name.clone());
                        // Is this a new variable ?
                        if stack_offset_pointer != new_pointer {
                            return Err(format!("Unknown variable used as rparam for mov instruction: `{}`", instruction));
                        }
                        Some(variable_location)
                    }
                } else {
                    None
                };

                next_instructions.push(PASMInstruction::new(
                    "store".to_string(),
                    vec![
                        instruction.operands[0].clone(),
                        if let Some(operand2_location) = operand2_location {
                            OperandType::new_stack("'SBP".to_string(), operand2_location)
                        } else {
                           instruction.operands[1].clone()  // Either a register or an immediate value
                        }
                    ]
                ))
            }
            "add" | "sub" | "mul" | "div" | "mod" => {
                // Load first operand into GPA
                let (new_instructions, new_pointer) = load_variable(
                    instruction.operands.get(0),
                    "'GPA",
                    &mut variable_map,
                    stack_offset_pointer,
                );
                stack_offset_pointer = new_pointer;
                next_instructions.extend(new_instructions);

                // Load second operand into GPB
                let (new_instructions, new_pointer) = load_variable(
                    instruction.operands.get(1),
                    "'GPB",
                    &mut variable_map,
                    stack_offset_pointer,
                );
                stack_offset_pointer = new_pointer;
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
                    instruction.operands.get(0),
                    "'GPA",
                    &mut variable_map,
                    stack_offset_pointer,
                );
                stack_offset_pointer = new_pointer;
                next_instructions.extend(new_instructions);
            }
            "cmp" => {
                // load first operand into GPA
                let (new_instructions, new_pointer) = load_variable(
                    instruction.operands.get(0),
                    "'GPA",
                    &mut variable_map,
                    stack_offset_pointer,
                );
                stack_offset_pointer = new_pointer;
                next_instructions.extend(new_instructions);

                // Load second operand into GPB
                let (new_instructions, new_pointer) = load_variable(
                    instruction.operands.get(1),
                    "'GPB",
                    &mut variable_map,
                    stack_offset_pointer,
                );
                stack_offset_pointer = new_pointer;
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
            "push" | "print" => {
                // Load the variable into GPA
                if let Some(operand) = instruction.operands.get(0) {
                    if operand.is_register() {
                        next_instructions.push(instruction.clone());
                    } else {
                        let (new_instructions, new_pointer) = load_variable(
                            instruction.operands.get(0),
                            "'GPA",
                            &mut variable_map,
                            stack_offset_pointer,
                        );
                        stack_offset_pointer = new_pointer;
                        next_instructions.extend(new_instructions);

                        // Push the variable
                        next_instructions.push(PASMInstruction::new(
                            instruction.opcode.clone(),
                            vec![super::asm::OperandType::Identifier {
                                name: "'GPA".to_string(),
                            }],
                        ));
                    }
                }
            }
            // Other instructions don't need to be modified
            _ => {
                next_instructions.push(instruction.clone());
            }
        }
    }

    Ok(next_instructions)
}
