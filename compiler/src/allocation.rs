use std::collections::HashMap;

use super::pasm::{OperandType, PASMInstruction};

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

fn get_operand_location(
    operand: &OperandType,
    variable_map: &mut HashMap<String, i32>,
    stack_offset_pointer: usize,
) -> (Option<i32>, usize) {
    if let OperandType::Identifier { name } = operand {
        let (variable_location, new_pointer) =
            allocate_memory(variable_map, stack_offset_pointer, name.clone());
        // Is this a new variable ?
        (Some(variable_location), new_pointer)
    } else {
        (None, stack_offset_pointer)
    }
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
    let mut stack_offset_pointer = 1; // 0 Is reserved for 'SBP already

    for (index, parameter) in function.0.iter().enumerate() {
        variable_map.insert(parameter.clone(), -(index as i32 + 2));
    }

    for instruction in function.1.iter() {
        // If the instruction is a label, we don't need to do anything
        if instruction.is_label {
            next_instructions.push(instruction.clone());
            continue;
        }

        next_instructions.push(PASMInstruction::new_comment(format!("{}", instruction)));

        // Track where new instructions start so we can tag them with the source span
        let new_insts_start = next_instructions.len();

        // next_instructions.push(PASMInstruction::new_comment(format!("OG: {}", instruction)));
        match instruction.opcode.as_str() {
            // If the instruction is a mov, we need to check if the source is a variable
            // and if the destination is a variable
            "mov" => {
                // If operand2_location has a value, its the offset of this variable in the stack.
                // If not, the operands is a literal or a register (meaning cimply copy it)
                let (operand2_location, new_pointer) = get_operand_location(
                    &instruction.operands[1],
                    &mut variable_map,
                    stack_offset_pointer,
                );
                stack_offset_pointer = new_pointer;

                match &instruction.operands[0] {
                    OperandType::Identifier { name }
                        if instruction.operands[0].is_frame_variable() =>
                    {
                        // moving into a known variable
                        if !variable_map.contains_key(name) {
                            let (_, new_pointer) = allocate_memory(
                                &mut variable_map,
                                stack_offset_pointer,
                                name.clone(),
                            );
                            stack_offset_pointer = new_pointer;
                        }
                        let operand1_location = variable_map[name];
                        next_instructions.push(PASMInstruction::new(
                            "mov".to_string(),
                            vec![
                                OperandType::new_stack("SBP", operand1_location),
                                if let Some(operand2_location) = operand2_location {
                                    OperandType::new_stack("SBP", operand2_location)
                                } else {
                                    instruction.operands[1].clone()
                                },
                            ],
                        ));
                    }
                    // moving to a register
                    _ => next_instructions.push(PASMInstruction::new(
                        "mov".to_string(),
                        vec![
                            instruction.operands[0].clone(),
                            if let Some(operand2_location) = operand2_location {
                                OperandType::new_stack("SBP", operand2_location)
                            } else {
                                instruction.operands[1].clone()
                            },
                        ],
                    )),
                };
            }
            "load" => {
                // If operand1_location has a value, its the offset of this variable in the stack.
                // If not, the operands is a literal or a register (meaning simply copy it)
                let (operand1_location, new_pointer) = get_operand_location(
                    &instruction.operands[0],
                    &mut variable_map,
                    stack_offset_pointer,
                );
                stack_offset_pointer = new_pointer;

                next_instructions.push(PASMInstruction::new(
                    "load".to_string(),
                    vec![
                        if operand1_location.is_some() {
                            // The operand is a variable, load it into a register
                            OperandType::Identifier {
                                name: "'GPA".to_string(),
                            }
                        } else {
                            instruction.operands[0].clone()
                        },
                        instruction.operands[1].clone(),
                    ],
                ));
                if let Some(operand1_location) = operand1_location {
                    // Still work to do, move the result into the variable
                    next_instructions.push(PASMInstruction::new(
                        "mov".to_string(),
                        vec![
                            OperandType::new_stack("SBP", operand1_location),
                            OperandType::Identifier {
                                name: "'GPA".to_string(),
                            },
                        ],
                    ));
                }
            }
            "store" => {
                // If operand2_location has a value, its the offset of this variable in the stack.
                // If not, the operands is a literal or a register (meaning cimply copy it)
                let (operand2_location, new_pointer) = get_operand_location(
                    &instruction.operands[1],
                    &mut variable_map,
                    stack_offset_pointer,
                );
                stack_offset_pointer = new_pointer;

                next_instructions.push(PASMInstruction::new(
                    "store".to_string(),
                    vec![
                        instruction.operands[0].clone(),
                        if let Some(operand2_location) = operand2_location {
                            OperandType::new_stack("SBP", operand2_location)
                        } else {
                            instruction.operands[1].clone() // Either a register or an immediate value
                        },
                    ],
                ))
            }
            "add" | "sub" | "mul" | "div" | "mod" => {
                // If operandX_location has a value, its the offset of this variable in the stack.
                // If not, the operands is a literal or a register (meaning simply copy it)
                let (operand1_maybe_location, new_pointer) = get_operand_location(
                    &instruction.operands[0],
                    &mut variable_map,
                    stack_offset_pointer,
                );
                let (operand2_maybe_location, new_pointer) =
                    get_operand_location(&instruction.operands[1], &mut variable_map, new_pointer);
                stack_offset_pointer = new_pointer;

                let operand1_location = {
                    if let Some(offset1) = operand1_maybe_location {
                        next_instructions.push(PASMInstruction::new(
                            "mov".to_string(),
                            vec![
                                OperandType::new_register("GPA"),
                                OperandType::new_stack("SBP", offset1),
                            ],
                        ));
                        OperandType::new_register("GPA")
                    } else {
                        if instruction.operands[0].is_memory() {
                            next_instructions.push(PASMInstruction::new(
                                "load".to_string(),
                                vec![
                                    OperandType::new_register("GPA"),
                                    instruction.operands[0].clone(),
                                ],
                            ));
                            OperandType::new_register("GPA")
                        } else {
                            instruction.operands[0].clone()
                        }
                    }
                };

                let operand2_location = {
                    if let Some(offset2) = operand2_maybe_location {
                        next_instructions.push(PASMInstruction::new(
                            "mov".to_string(),
                            vec![
                                OperandType::new_register("GPB"),
                                OperandType::new_stack("SBP", offset2),
                            ],
                        ));
                        OperandType::new_register("GPB")
                    } else {
                        if instruction.operands[1].is_memory() {
                            next_instructions.push(PASMInstruction::new(
                                "load".to_string(),
                                vec![
                                    OperandType::new_register("GPB"),
                                    instruction.operands[1].clone(),
                                ],
                            ));
                            OperandType::new_register("GPB")
                        } else {
                            instruction.operands[1].clone()
                        }
                    }
                };

                next_instructions.push(PASMInstruction::new(
                    instruction.opcode.clone(),
                    vec![operand1_location, operand2_location],
                ));

                // Store the result in the destination variable
                if let Some(offset1) = operand1_maybe_location {
                    next_instructions.push(PASMInstruction::new(
                        "mov".to_string(),
                        vec![
                            OperandType::new_stack("SBP", offset1),
                            OperandType::new_register("GPA"),
                        ],
                    ));
                }
            }
            "cmp" => {
                // load first operand into GPA
                let (operand1_location, new_pointer) = get_operand_location(
                    &instruction.operands[0],
                    &mut variable_map,
                    stack_offset_pointer,
                );
                let (operand2_location, new_pointer) =
                    get_operand_location(&instruction.operands[1], &mut variable_map, new_pointer);
                stack_offset_pointer = new_pointer;

                let operand1_location = {
                    if let Some(offset1) = operand1_location {
                        next_instructions.push(PASMInstruction::new(
                            "mov".to_string(),
                            vec![
                                OperandType::new_register("GPA"),
                                OperandType::new_stack("SBP", offset1),
                            ],
                        ));
                        OperandType::new_register("GPA")
                    } else {
                        if instruction.operands[0].is_memory() {
                            next_instructions.push(PASMInstruction::new(
                                "load".to_string(),
                                vec![
                                    OperandType::new_register("GPA"),
                                    instruction.operands[0].clone(),
                                ],
                            ));
                            OperandType::new_register("GPA")
                        } else {
                            instruction.operands[0].clone()
                        }
                    }
                };

                let operand2_location = {
                    if let Some(offset2) = operand2_location {
                        next_instructions.push(PASMInstruction::new(
                            "mov".to_string(),
                            vec![
                                OperandType::new_register("GPB"),
                                OperandType::new_stack("SBP", offset2),
                            ],
                        ));
                        OperandType::new_register("GPB")
                    } else {
                        if instruction.operands[1].is_memory() {
                            next_instructions.push(PASMInstruction::new(
                                "load".to_string(),
                                vec![
                                    OperandType::new_register("GPB"),
                                    instruction.operands[1].clone(),
                                ],
                            ));
                            OperandType::new_register("GPB")
                        } else {
                            instruction.operands[1].clone()
                        }
                    }
                };

                // Compare the two operands
                next_instructions.push(PASMInstruction::new(
                    instruction.opcode.clone(),
                    vec![operand1_location, operand2_location],
                ));
            }
            "push" | "print" => {
                let (operand1_location, new_pointer) = get_operand_location(
                    &instruction.operands[0],
                    &mut variable_map,
                    stack_offset_pointer,
                );
                stack_offset_pointer = new_pointer;

                let operand1_location = {
                    if let Some(offset1) = operand1_location {
                        next_instructions.push(PASMInstruction::new(
                            "mov".to_string(),
                            vec![
                                OperandType::new_register("GPA"),
                                OperandType::new_stack("SBP", offset1),
                            ],
                        ));
                        OperandType::new_register("GPA")
                    } else {
                        if instruction.operands[0].is_memory() {
                            next_instructions.push(PASMInstruction::new(
                                "load".to_string(),
                                vec![
                                    OperandType::new_register("GPA"),
                                    instruction.operands[0].clone(),
                                ],
                            ));
                            OperandType::new_register("GPA")
                        } else {
                            instruction.operands[0].clone()
                        }
                    }
                };

                // Push/Print the variable
                next_instructions.push(PASMInstruction::new(
                    instruction.opcode.clone(),
                    vec![operand1_location],
                ));
            }
            // Other instructions don't need to be modified
            _ => {
                next_instructions.push(instruction.clone());
            }
        }

        // Tag all newly generated instructions with the source instruction's span
        for inst in next_instructions[new_insts_start..].iter_mut() {
            if inst.span.is_none() {
                inst.span = instruction.span.clone();
            }
        }
    }

    Ok(next_instructions)
}
