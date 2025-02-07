use std::sync::atomic::{AtomicUsize, Ordering};

use super::{
    assignment::{imm_to_imm, mem_to_imm},
    MaybeInstructions, OperandType, PASMInstruction,
};
/// Transforms the AST of a function into pseudo-asm
use crate::ast::node::{ComparisonType, Node, OperationType};

static TEMP_VAR_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// Creates a new identifier for a variable with the given pattern
fn create_temp_variable_name<S: AsRef<str>>(pattern: S) -> String {
    let counter = TEMP_VAR_COUNTER.fetch_add(1, Ordering::SeqCst);
    format!("temp_{}_{}", pattern.as_ref(), counter)
}

/// Loads the given base (for a memory access) into the GPC register for further operations
fn load_base(base: &Box<Node>) -> MaybeInstructions {
    match &**base {
        Node::Identifier { name } => Ok(vec![PASMInstruction::new(
            "mov".to_string(),
            vec![
                OperandType::Register {
                    name: "GPC".to_string(),
                },
                OperandType::Identifier {
                    name: name.to_string(),
                },
            ],
        )]),
        _ => return Err("base should be an identifier".to_string()),
    }
}

fn operation_to_asm(
    operation: &OperationType,
    lparam: &Box<Node>,
    rparam: &Box<Node>,
) -> Result<(Box<OperandType>, Vec<PASMInstruction>), String> {
    let mut instructions = vec![];

    let operation = match operation {
        OperationType::Addition => "add",
        OperationType::Substraction => "sub",
        OperationType::Multiplication => "mul",
        OperationType::Division => "div",
        OperationType::Modulo => "mod",
    };

    instructions.extend(assignment_to_asm(
        &Box::from(Node::Register {
            name: "GPA".to_string(),
        }),
        lparam,
    )?);

    let new_rparam = match &**rparam {
        Node::Identifier { name } if !name.starts_with("$") => {
            OperandType::Identifier { name: name.clone() }
        }
        Node::Identifier { name: _ } => {
            let temp = create_temp_variable_name("oprpar");
            instructions.extend(assignment_to_asm(
                &Box::from(Node::Identifier { name: temp.clone() }),
                rparam,
            )?);
            OperandType::Identifier { name: temp.clone() }
        }
        Node::Litteral { value } => OperandType::Literal { value: *value },
        Node::MemoryOffset { base, offset } => {
            instructions.extend(load_base(base)?);
            OperandType::MemoryOffset {
                base: Box::from(OperandType::new_register("GPC")),
                offset: match &**offset {
                    Node::Register { name } => Box::from(OperandType::new_register(name)),
                    Node::Identifier { name } => Box::from(OperandType::Identifier { name: name.clone() }),
                    Node::Litteral { value } => Box::from(OperandType::new_literal(*value)),
                    _ => return Err("(OpToAsm) Invalid memory offset. Memory offset should be either a literal, identifier or register.".to_string())
                },
            }
        }
        _ => {
            return Err(
                "lparam of operation should be either a literal or an identifier".to_string(),
            )
        }
    };

    instructions.push(PASMInstruction::new(
        operation.to_string(),
        vec![OperandType::new_register("GPA"), new_rparam],
    ));

    Ok((Box::from(OperandType::new_register("GPA")), instructions))
}

fn assignment_to_asm(assignee: &Box<Node>, assignant: &Box<Node>) -> MaybeInstructions {
    let mut instructions = vec![];

    match (&**assignant, &**assignee) {
        // Id to Id
        (
            Node::Identifier { .. } | Node::Register { .. } | Node::Litteral { .. },
            Node::Identifier { .. } | Node::Register { .. } | Node::Litteral { .. },
        ) => {
            instructions.extend(super::assignment::imm_to_imm(assignant, assignee)?);
        }
        // Mem to Id
        (
            Node::MemoryValue { .. } | Node::MemoryOffset { .. },
            Node::Identifier { .. } | Node::Register { .. } | Node::Litteral { .. },
        ) => {
            instructions.extend(super::assignment::mem_to_imm(assignant, assignee)?);
        }
        // Id to Mem
        (
            Node::Identifier { .. } | Node::Register { .. } | Node::Litteral { .. },
            Node::MemoryValue { .. } | Node::MemoryOffset { .. },
        ) => {
            instructions.extend(super::assignment::imm_to_mem(assignant, assignee)?);
        }
        (
            // Mem to mem
            Node::MemoryValue { .. } | Node::MemoryOffset { .. },
            Node::MemoryValue { .. } | Node::MemoryOffset { .. },
        ) => {
            instructions.extend(super::assignment::mem_to_mem(assignant, assignee)?);
        }
        // Op to Id
        (Node::Operation { .. }, Node::Identifier { .. } | Node::Register { .. }) => {
            instructions.extend(super::assignment::op_to_imm(assignant, assignee)?);
        }
        (
            // Op to Mem
            Node::Operation { .. },
            Node::MemoryValue { .. } | Node::MemoryOffset { .. },
        ) => {
            instructions.extend(super::assignment::op_to_mem(assignant, assignee)?);
        }
        (Node::FunctionCall { .. }, Node::Identifier { .. } | Node::Register { .. }) => {
            instructions.extend(match &**assignant {
                Node::FunctionCall {
                    function_name,
                    parameters,
                } => function_to_asm(&function_name, &parameters)?,
                node => {
                    return Err("Invalid parameter from".to_string());
                }
            });

            instructions.extend(super::assignment::imm_to_imm(
                &Box::from(Node::Register {
                    name: "FRV".to_string(),
                }),
                assignee,
            )?);
        }
        (Node::FunctionCall { .. }, Node::MemoryOffset { .. } | Node::MemoryValue { .. }) => {
            instructions.extend(match &**assignant {
                Node::FunctionCall {
                    function_name,
                    parameters,
                } => function_to_asm(&function_name, &parameters)?,
                node => {
                    return Err("Invalid parameter from".to_string());
                }
            });

            instructions.extend(super::assignment::imm_to_mem(
                &Box::from(Node::Register {
                    name: "FRV".to_string(),
                }),
                assignee,
            )?);
        }
        _ => {
            println!("Unhandled case: {:?} to {:?}", assignant, assignee);
            return Err("Not implemented".to_string());
        }
    }

    Ok(instructions)
}

fn comparison_to_asm(
    lparam: &Box<Node>,
    rparam: &Box<Node>,
    comparison: &ComparisonType,
    jmp_to: String,
) -> MaybeInstructions {
    let mut instructions = vec![];

    let lparam = match &**lparam {
        Node::Register { name } => OperandType::new_register(name),
        Node::Identifier { .. } | Node::Litteral { .. } => {
            instructions.extend(imm_to_imm(
                lparam,
                &Box::from(Node::Register {
                    name: "GPA".to_string(),
                }),
            )?);
            OperandType::new_register("GPA")
        }
        Node::MemoryOffset { .. } | Node::MemoryValue { .. } => {
            instructions.extend(mem_to_imm(
                lparam,
                &Box::from(Node::Register {
                    name: "GPA".to_string(),
                }),
            )?);
            OperandType::new_register("GPA")
        }
        // Upgrade: add operation here ?
        _ => return Err("Invalid lparam for comparison".to_string()),
    };

    let rparam = match &**rparam {
        Node::Register { name } => OperandType::new_register(name),
        Node::Identifier { .. } | Node::Litteral { .. } => {
            instructions.extend(imm_to_imm(
                rparam,
                &Box::from(Node::Register {
                    name: "GPB".to_string(),
                }),
            )?);
            OperandType::new_register("GPB")
        }
        Node::MemoryOffset { .. } | Node::MemoryValue { .. } => {
            instructions.extend(mem_to_imm(
                rparam,
                &Box::from(Node::Register {
                    name: "GPB".to_string(),
                }),
            )?);
            OperandType::new_register("GPB")
        }
        _ => return Err("Invalid rparam for comparison".to_string()),
    };

    match *comparison {
        ComparisonType::EQ => {
            instructions.extend(vec![
                PASMInstruction::new("cmp".to_string(), vec![lparam, rparam]),
                PASMInstruction::new(
                    "jnz".to_string(),
                    vec![OperandType::Identifier {
                        name: jmp_to.clone(),
                    }],
                ), // If not zero jump to next block's label
            ])
        }
        ComparisonType::DIFF => {
            instructions.extend(vec![
                PASMInstruction::new("cmp".to_string(), vec![lparam, rparam]),
                PASMInstruction::new(
                    "jz".to_string(),
                    vec![OperandType::Identifier {
                        name: jmp_to.clone(),
                    }],
                ), // If not zero jump to next block's label
            ])
        }
        ComparisonType::GE => {
            instructions.extend(vec![
                PASMInstruction::new("cmp".to_string(), vec![lparam, rparam]),
                PASMInstruction::new(
                    "jn".to_string(),
                    vec![OperandType::Identifier {
                        name: jmp_to.clone(),
                    }],
                ), // If not zero jump to next block's label
            ])
        }
        ComparisonType::GT => {
            // Invert operation to only require one jump !
            instructions.extend(vec![
                PASMInstruction::new("cmp".to_string(), vec![rparam, lparam]),
                PASMInstruction::new(
                    "jp".to_string(),
                    vec![OperandType::Identifier {
                        name: jmp_to.clone(),
                    }],
                ), // If not zero jump to next block's label
            ])
        }
        ComparisonType::LT => {
            // Invert operation to only require one jump !
            instructions.extend(vec![
                PASMInstruction::new("cmp".to_string(), vec![rparam, lparam]),
                PASMInstruction::new(
                    "jn".to_string(),
                    vec![OperandType::Identifier {
                        name: jmp_to.clone(),
                    }],
                ), // If not zero jump to next block's label
            ])
        }
        ComparisonType::LE => {
            instructions.extend(vec![
                PASMInstruction::new("cmp".to_string(), vec![lparam, rparam]),
                PASMInstruction::new(
                    "jp".to_string(),
                    vec![OperandType::Identifier {
                        name: jmp_to.clone(),
                    }],
                ), // If not zero jump to next block's label
            ])
        }
    }

    Ok(instructions)
}

///  If exit label is Some, this function will not add an exit label !
fn if_to_asm(
    condition: &Box<Node>,
    content: &Vec<Box<Node>>,
    exit_label: Option<String>,
) -> MaybeInstructions {
    let mut instructions = vec![];
    let next_block_label = match &exit_label {
        Some(v) => v.clone(),
        None => create_temp_variable_name("if_exit"),
    };

    match &**condition {
        Node::Comparison {
            lparam,
            rparam,
            comparison,
        } => {
            instructions.extend(comparison_to_asm(
                lparam,
                rparam,
                comparison,
                next_block_label.clone(),
            )?);
        }
        Node::Identifier { name } => instructions.extend(vec![
            PASMInstruction::new(
                "cmp".to_string(),
                vec![
                    OperandType::Identifier { name: name.clone() },
                    OperandType::Literal { value: 0 },
                ],
            ),
            PASMInstruction::new(
                "jz".to_string(),
                vec![OperandType::Identifier {
                    name: next_block_label.clone(),
                }],
            ),
        ]),
        Node::Litteral { value } => {
            let temp_condition = create_temp_variable_name("cp");
            instructions.extend(vec![
                PASMInstruction::new(
                    "mov".to_string(),
                    vec![
                        OperandType::Identifier {
                            name: temp_condition.clone(),
                        },
                        OperandType::Literal { value: *value },
                    ],
                ),
                PASMInstruction::new(
                    "cmp".to_string(),
                    vec![
                        OperandType::Identifier {
                            name: temp_condition.clone(),
                        },
                        OperandType::Literal { value: 0 },
                    ],
                ),
                PASMInstruction::new(
                    "jz".to_string(),
                    vec![OperandType::Identifier {
                        name: next_block_label.clone(),
                    }],
                ),
            ])
        }
        _ => return Err("Unexpected ast node for if condition".to_string()),
    }

    for node in content.iter() {
        instructions.extend(inst_to_pasm(node)?)
    }

    if !exit_label.is_some() {
        instructions.push(PASMInstruction::new_label(next_block_label.clone()));
    }

    Ok(instructions)
}

fn while_to_asm(condition: &Box<Node>, content: &Vec<Box<Node>>) -> MaybeInstructions {
    let before_label = create_temp_variable_name("while_condition");
    let after_label = create_temp_variable_name("while_exit");
    let mut instructions = vec![PASMInstruction::new_label(before_label.clone())];

    instructions.extend(if_to_asm(condition, content, Some(after_label.clone()))?);
    instructions.extend(vec![
        PASMInstruction::new(
            "jmp".to_string(),
            vec![OperandType::Identifier {
                name: before_label.clone(),
            }],
        ),
        PASMInstruction::new_label(after_label.clone()),
    ]);

    Ok(instructions)
}

fn loop_to_asm(content: &Vec<Box<Node>>) -> MaybeInstructions {
    let label = create_temp_variable_name("loop_label");
    let mut instructions = vec![PASMInstruction::new_label(label.to_string())];

    for node in content {
        instructions.extend(inst_to_pasm(node)?)
    }
    instructions.push(PASMInstruction::new(
        "jmp".to_string(),
        vec![OperandType::Identifier {
            name: label.clone(),
        }],
    ));

    Ok(instructions)
}

fn function_to_asm(function_name: &String, parameters: &Vec<Box<Node>>) -> MaybeInstructions {
    let mut instructions = vec![];

    // Push parameters in reverse order
    for node in parameters.iter().rev() {
        match &**node {
            Node::Identifier { name } => instructions.push(
                PASMInstruction::new(
                    "push".to_string(),
                    vec![OperandType::Identifier { name: name.clone() }]
                )
            ),
            Node::Litteral { value } => instructions.push(
                PASMInstruction::new(
                    "push".to_string(),
                    vec![OperandType::Literal { value: *value }]
                )
            ),
            Node::Operation { lparam, rparam, operation } => {
                let (temp, operation_instructions) = operation_to_asm(operation, lparam, rparam)?;
                instructions.extend(operation_instructions);
                instructions.push(
                    PASMInstruction::new(
                        "push".to_string(),
                        vec![(&*temp).clone()]
                    )
                )
            }
            _ => {
                return Err(
                    "Invalid value in function call, only identifiers, literals and operations are allowed"
                        .to_string(),
                )
            }
        }
    }

    // Call the actual function, the return address will be pushed by the VM
    instructions.push(PASMInstruction::new(
        "call".to_string(),
        vec![OperandType::Identifier {
            name: format!("function_{}_label", function_name),
        }],
    ));

    // Restore the stack pointer
    instructions.push(PASMInstruction::new(
        "add".to_string(),
        vec![
            OperandType::Identifier {
                name: "'TSP".to_string(),
            },
            OperandType::Literal {
                value: parameters.len() as i32,
            },
        ],
    ));

    Ok(instructions)
}

/// Produces the instructions needed for a function return.
/// 1. Puts the return value into the 'FRV register
/// 2. Restores the stack pointer to its original value
/// 3. Restores the base pointer to its original value
/// 4. actual ret instruction
fn ret_to_asm(value: &Box<Node>) -> MaybeInstructions {
    let mut instructions = vec![];

    // Return value goes in FRV
    match &**value {
        Node::Identifier { name } => {
            if name.starts_with("$") {
                instructions.push(PASMInstruction::new(
                    "load".to_string(),
                    vec![
                        OperandType::Identifier {
                            name: "'FRV".to_string(),
                        },
                        OperandType::Identifier { name: name.clone() },
                    ],
                ));
            } else {
                instructions.push(PASMInstruction::new(
                    "mov".to_string(),
                    vec![
                        OperandType::Identifier {
                            name: "'FRV".to_string(),
                        },
                        OperandType::Identifier { name: name.clone() },
                    ],
                ));
            }
        }
        Node::Litteral { value } => {
            instructions.push(PASMInstruction::new(
                "mov".to_string(),
                vec![
                    OperandType::Identifier {
                        name: "'FRV".to_string(),
                    },
                    OperandType::Literal { value: *value },
                ],
            ));
        }
        _ => {
            return Err("Invalid return value".to_string());
        }
    }

    // Restore stack pointer
    instructions.push(PASMInstruction::new(
        "mov".to_string(),
        vec![
            OperandType::Identifier {
                name: "'TSP".to_string(),
            },
            OperandType::Identifier {
                name: "'SBP".to_string(),
            },
        ],
    ));

    // Restore base pointer
    instructions.push(PASMInstruction::new(
        "pop".to_string(),
        vec![OperandType::Identifier {
            name: "'SBP".to_string(),
        }],
    ));

    // Actual return instruction
    instructions.push(PASMInstruction::new("ret".to_string(), vec![]));
    Ok(instructions)
}

/// Produces a print instruction from the AST nodes
fn print_to_asm(node: &Box<Node>) -> MaybeInstructions {
    let (operand, mut instructions) = match &**node {
        Node::Identifier { .. } | Node::Litteral { .. } => {
            (super::assignment::ensure_immediate(node)?, vec![])
        }
        Node::MemoryOffset { .. } => super::assignment::ensure_memory(node)?,
        _ => return Err("Invalid value to print".to_string()),
    };

    instructions.push(PASMInstruction::new("print".to_string(), vec![operand]));
    Ok(instructions)
}

/// Converts an instruction from its AST node representation to pseudo assembly
/// Return either a list of `PASMInstruction` if there is no error in the AST node or
/// an error containing a string explaining the error
pub fn inst_to_pasm(node: &Box<Node>) -> MaybeInstructions {
    match &**node {
        Node::Assignment { lparam, rparam } => Ok(assignment_to_asm(lparam, rparam)?),
        Node::IfCondition { condition, content } => Ok(if_to_asm(condition, content, None)?),
        Node::Loop { content } => Ok(loop_to_asm(content)?),
        Node::WhileLoop { condition, content } => Ok(while_to_asm(condition, content)?),
        Node::Print { value } => Ok(print_to_asm(value)?),
        Node::FunctionCall {
            function_name,
            parameters,
        } => Ok(function_to_asm(function_name, parameters)?),
        Node::Return { value } => Ok(ret_to_asm(value)?),
        _ => Err("Not implemented".to_string()),
    }
}
