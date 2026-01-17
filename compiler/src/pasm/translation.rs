use std::sync::atomic::{AtomicUsize, Ordering};

use super::{
    assignment::{imm_to_imm, mem_to_imm},
    MaybeInstructions, OperandType, PASMInstruction,
};
/// Transforms the AST of a function into pseudo-asm
use crate::ast::node::{ComparisonType, Node, NodeKind, OperationType};
use crate::lexer::token::TokenLocation;

static TEMP_VAR_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// Tags instructions that don't have a span with the provided span.
/// This preserves more specific spans from nested nodes while providing
/// a fallback for generated instructions.
fn tag_instructions_with_span(
    instructions: Vec<PASMInstruction>,
    span: &Option<TokenLocation>,
) -> Vec<PASMInstruction> {
    instructions
        .into_iter()
        .map(|mut inst| {
            if inst.span.is_none() {
                inst.span = span.clone();
            }
            inst
        })
        .collect()
}

/// Creates a new identifier for a variable with the given pattern
fn create_temp_variable_name<S: AsRef<str>>(pattern: S) -> String {
    let counter = TEMP_VAR_COUNTER.fetch_add(1, Ordering::SeqCst);
    format!("temp_{}_{}", pattern.as_ref(), counter)
}

/// Loads the given base (for a memory access) into the GPC register for further operations
fn load_base(base: &Box<Node>) -> MaybeInstructions {
    match &base.kind {
        NodeKind::Identifier { name } => Ok(vec![PASMInstruction::new(
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
        &Box::from(Node::new(NodeKind::Register {
            name: "GPA".to_string(),
        })),
        lparam,
    )?);

    let new_rparam = match &rparam.kind {
        NodeKind::MemoryValue { name } => OperandType::Memory { name: name.clone() },
        NodeKind::Identifier { name: _ } => {
            let temp = create_temp_variable_name("oprpar");
            instructions.extend(assignment_to_asm(
                &Box::from(Node::new(NodeKind::new_identifier(temp.clone()))),
                rparam,
            )?);
            OperandType::Identifier { name: temp.clone() }
        }
        NodeKind::Litteral { value } => OperandType::Literal { value: *value },
        NodeKind::MemoryOffset { base, offset } => {
            instructions.extend(load_base(base)?);
            OperandType::MemoryOffset {
                base: Box::from(OperandType::new_register("GPC")),
                offset: match &offset.kind {
                    NodeKind::Register { name } => Box::from(OperandType::new_register(name)),
                    NodeKind::Identifier { name } => Box::from(OperandType::Identifier { name: name.clone() }),
                    NodeKind::Litteral { value } => Box::from(OperandType::new_literal(*value)),
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

    match (&assignant.kind, &assignee.kind) {
        // Id to Id
        (
            NodeKind::Identifier { .. } | NodeKind::Register { .. } | NodeKind::Litteral { .. },
            NodeKind::Identifier { .. } | NodeKind::Register { .. } | NodeKind::Litteral { .. },
        ) => {
            instructions.extend(super::assignment::imm_to_imm(assignant, assignee)?);
        }
        // Mem to Id
        (
            NodeKind::MemoryValue { .. } | NodeKind::MemoryOffset { .. },
            NodeKind::Identifier { .. } | NodeKind::Register { .. } | NodeKind::Litteral { .. },
        ) => {
            instructions.extend(super::assignment::mem_to_imm(assignant, assignee)?);
        }
        // Id to Mem
        (
            NodeKind::Identifier { .. } | NodeKind::Register { .. } | NodeKind::Litteral { .. },
            NodeKind::MemoryValue { .. } | NodeKind::MemoryOffset { .. },
        ) => {
            instructions.extend(super::assignment::imm_to_mem(assignant, assignee)?);
        }
        (
            // Mem to mem
            NodeKind::MemoryValue { .. } | NodeKind::MemoryOffset { .. },
            NodeKind::MemoryValue { .. } | NodeKind::MemoryOffset { .. },
        ) => {
            instructions.extend(super::assignment::mem_to_mem(assignant, assignee)?);
        }
        // Op to Id
        (NodeKind::Operation { .. }, NodeKind::Identifier { .. } | NodeKind::Register { .. }) => {
            instructions.extend(super::assignment::op_to_imm(assignant, assignee)?);
        }
        (
            // Op to Mem
            NodeKind::Operation { .. },
            NodeKind::MemoryValue { .. } | NodeKind::MemoryOffset { .. },
        ) => {
            instructions.extend(super::assignment::op_to_mem(assignant, assignee)?);
        }
        (NodeKind::FunctionCall { .. }, NodeKind::Identifier { .. } | NodeKind::Register { .. }) => {
            instructions.extend(match &assignant.kind {
                NodeKind::FunctionCall {
                    function_name,
                    parameters,
                } => function_to_asm(&function_name, &parameters)?,
                _ => {
                    return Err("Invalid assignant in function to immediate assignment".to_string());
                }
            });

            instructions.extend(super::assignment::imm_to_imm(
                &Box::from(Node::new(NodeKind::Register {
                    name: "FRV".to_string(),
                })),
                assignee,
            )?);
        }
        (NodeKind::FunctionCall { .. }, NodeKind::MemoryOffset { .. } | NodeKind::MemoryValue { .. }) => {
            instructions.extend(match &assignant.kind {
                NodeKind::FunctionCall {
                    function_name,
                    parameters,
                } => function_to_asm(&function_name, &parameters)?,
                _ => {
                    return Err("Invalid assignant in function to memory assignment".to_string());
                }
            });

            instructions.extend(super::assignment::imm_to_mem(
                &Box::from(Node::new(NodeKind::Register {
                    name: "FRV".to_string(),
                })),
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

    let lparam_op = match &lparam.kind {
        NodeKind::Register { name } => OperandType::new_register(name),
        NodeKind::Identifier { .. } | NodeKind::Litteral { .. } => {
            instructions.extend(imm_to_imm(
                lparam,
                &Box::from(Node::new(NodeKind::Register {
                    name: "GPA".to_string(),
                })),
            )?);
            OperandType::new_register("GPA")
        }
        NodeKind::MemoryOffset { .. } | NodeKind::MemoryValue { .. } => {
            instructions.extend(mem_to_imm(
                lparam,
                &Box::from(Node::new(NodeKind::Register {
                    name: "GPA".to_string(),
                })),
            )?);
            OperandType::new_register("GPA")
        }
        // Upgrade: add operation here ?
        _ => return Err("Invalid lparam for comparison".to_string()),
    };

    let rparam_op = match &rparam.kind {
        NodeKind::Register { name } => OperandType::new_register(name),
        NodeKind::Identifier { .. } | NodeKind::Litteral { .. } => {
            instructions.extend(imm_to_imm(
                rparam,
                &Box::from(Node::new(NodeKind::Register {
                    name: "GPB".to_string(),
                })),
            )?);
            OperandType::new_register("GPB")
        }
        NodeKind::MemoryOffset { .. } | NodeKind::MemoryValue { .. } => {
            instructions.extend(mem_to_imm(
                rparam,
                &Box::from(Node::new(NodeKind::Register {
                    name: "GPB".to_string(),
                })),
            )?);
            OperandType::new_register("GPB")
        }
        _ => return Err("Invalid rparam for comparison".to_string()),
    };

    match *comparison {
        ComparisonType::EQ => {
            instructions.extend(vec![
                PASMInstruction::new("cmp".to_string(), vec![lparam_op, rparam_op]),
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
                PASMInstruction::new("cmp".to_string(), vec![lparam_op, rparam_op]),
                PASMInstruction::new(
                    "jz".to_string(),
                    vec![OperandType::Identifier {
                        name: jmp_to.clone(),
                    }],
                ), // If zero jump to next block's label
            ])
        }
        ComparisonType::GE => {
            instructions.extend(vec![
                PASMInstruction::new("cmp".to_string(), vec![lparam_op, rparam_op]),
                PASMInstruction::new(
                    "jn".to_string(),
                    vec![OperandType::Identifier {
                        name: jmp_to.clone(),
                    }],
                ), // lparam < rparam => Jump to next block
            ])
        }
        ComparisonType::GT => {
            instructions.extend(vec![
                PASMInstruction::new("cmp".to_string(), vec![lparam_op.clone(), rparam_op.clone()]),
                PASMInstruction::new(
                    "jn".to_string(),
                    vec![OperandType::Identifier {
                        name: jmp_to.clone(),
                    }],
                ), // lpram < rparam => Jump to next block
                PASMInstruction::new("cmp".to_string(), vec![lparam_op, rparam_op]),
                PASMInstruction::new(
                    "jz".to_string(),
                    vec![OperandType::Identifier {
                        name: jmp_to.clone(),
                    }],
                ), // lparam = rparam => jump to next block
            ])
        }
        ComparisonType::LT => {
            instructions.extend(vec![
                PASMInstruction::new("cmp".to_string(), vec![lparam_op.clone(), rparam_op.clone()]),
                PASMInstruction::new(
                    "jp".to_string(),
                    vec![OperandType::Identifier {
                        name: jmp_to.clone(),
                    }],
                ), // lparam > rparam => jump to next block
                PASMInstruction::new("cmp".to_string(), vec![rparam_op, lparam_op]),
                PASMInstruction::new(
                    "jz".to_string(),
                    vec![OperandType::Identifier {
                        name: jmp_to.clone(),
                    }],
                ), // lparam = rparam => Jump to next block
            ])
        }
        ComparisonType::LE => {
            instructions.extend(vec![
                PASMInstruction::new("cmp".to_string(), vec![lparam_op, rparam_op]),
                PASMInstruction::new(
                    "jp".to_string(),
                    vec![OperandType::Identifier {
                        name: jmp_to.clone(),
                    }],
                ), // If lparam > rparam => Next label
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

    match &condition.kind {
        NodeKind::Comparison {
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
        NodeKind::Identifier { name } => instructions.extend(vec![
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
        NodeKind::Litteral { value } => {
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
        match &node.kind {
            NodeKind::Identifier { name } => instructions.push(
                PASMInstruction::new(
                    "push".to_string(),
                    vec![OperandType::Identifier { name: name.clone() }]
                )
            ),
            NodeKind::Litteral { value } => instructions.push(
                PASMInstruction::new(
                    "push".to_string(),
                    vec![OperandType::Literal { value: *value }]
                )
            ),
            NodeKind::Operation { lparam, rparam, operation } => {
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
            OperandType::new_register("TSP"),
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
    match &value.kind {
        NodeKind::MemoryValue { name } => {
            instructions.push(PASMInstruction::new(
                "load".to_string(),
                vec![
                    OperandType::new_register("FRV"),
                    OperandType::Identifier { name: name.clone() },
                ],
            ));
        }
        NodeKind::Identifier { name } => {
            instructions.push(PASMInstruction::new(
                "mov".to_string(),
                vec![
                    OperandType::new_register("FRV"),
                    OperandType::Identifier { name: name.clone() },
                ],
            ));
        }
        NodeKind::Litteral { value } => {
            instructions.push(PASMInstruction::new(
                "mov".to_string(),
                vec![
                    OperandType::new_register("FRV"),
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
            OperandType::new_register("TSP"),
            OperandType::new_register("SBP"),
        ],
    ));

    // Restore base pointer
    instructions.push(PASMInstruction::new(
        "pop".to_string(),
        vec![OperandType::new_register("SBP")],
    ));

    // Actual return instruction
    instructions.push(PASMInstruction::new("ret".to_string(), vec![]));
    Ok(instructions)
}

/// Produces a print instruction from the AST nodes
fn print_to_asm(node: &Box<Node>) -> MaybeInstructions {
    let (operand, mut instructions) = match &node.kind {
        NodeKind::Identifier { .. } | NodeKind::Litteral { .. } => {
            (super::assignment::ensure_immediate(node)?, vec![])
        }
        NodeKind::MemoryOffset { .. } | NodeKind::MemoryValue { .. } => {
            super::assignment::ensure_memory(node)?
        }
        _ => return Err("Invalid value to print".to_string()),
    };

    instructions.push(PASMInstruction::new("print".to_string(), vec![operand]));
    Ok(instructions)
}

/// Converts an instruction from its AST node representation to pseudo assembly
/// Return either a list of `PASMInstruction` if there is no error in the AST node or
/// an error containing a string explaining the error.
///
/// Generated instructions are tagged with the source node's span for error reporting.
pub fn inst_to_pasm(node: &Box<Node>) -> MaybeInstructions {
    let instructions = match &node.kind {
        NodeKind::Assignment { lparam, rparam } => assignment_to_asm(lparam, rparam)?,
        NodeKind::IfCondition { condition, content } => if_to_asm(condition, content, None)?,
        NodeKind::Loop { content } => loop_to_asm(content)?,
        NodeKind::WhileLoop { condition, content } => while_to_asm(condition, content)?,
        NodeKind::Print { value } => print_to_asm(value)?,
        NodeKind::FunctionCall {
            function_name,
            parameters,
        } => function_to_asm(function_name, parameters)?,
        NodeKind::Return { value } => ret_to_asm(value)?,
        _ => return Err("Not implemented".to_string()),
    };

    // Tag generated instructions with the source node's location
    Ok(tag_instructions_with_span(instructions, &node.span))
}
