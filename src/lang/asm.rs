use core::fmt;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Transforms the AST of a function into pseudo-asm
use super::{
    ast::node::{ComparisonType, OperationType},
    Node,
};

static TEMP_VAR_COUNTER: AtomicUsize = AtomicUsize::new(0);

type MaybeInstructions = Result<Vec<PASMInstruction>, String>;

#[derive(Clone, Debug)]
pub enum OperandType {
    Identifier { name: String },
    Literal { value: i32 },
    Stack { register: Box<OperandType>, operation: String, offset: usize }
}

impl OperandType {
    pub fn new_literal(value: i32) -> Self {
        OperandType::Literal { value }
    }

    pub fn new_stack(register: String, offset: i32) -> Self {
        OperandType::Stack {
            register: Box::from(OperandType::Identifier { name: register }),
            operation: if offset < 0 {
                "+".to_string()
            } else {
                "-".to_string()
            },
            offset: offset.abs() as usize
        }
    }

    pub fn is_register(&self) -> bool {
        match self {
            OperandType::Identifier { name } => name.starts_with("'"),
            _ => false
        }
    }

    pub fn is_memory(&self) -> bool {
        match self {
            OperandType::Identifier { name } => name.starts_with("$"),
            _ => false
        }
    }

    pub fn is_frame_variable(&self) -> bool {
        !self.is_register() && !self.is_memory()
    }

    pub fn get_register_name(&self) -> Option<String> {
        match self {
            OperandType::Identifier { name } if name.starts_with("'")  => Some(name.clone()),
            _ => None
        }
    }
}

impl fmt::Display for OperandType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OperandType::Identifier { name } => write!(f, "{}", name),
            OperandType::Literal { value } => write!(f, "#{}", value),
            OperandType::Stack { register, operation, offset } => write!(f, "[{} {} {}]", register, operation, offset)
        }
    }
}

#[derive(Clone)]
/// A Pseudo-assembly instruction.
pub struct PASMInstruction {
    pub is_label: bool,             // Whether this is just a label or not
    pub is_comment: bool,             // Whether this is just a label or not
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
                    OperandType::Literal { value } => write!(f, " #{}", value)?,
                    OperandType::Stack { register, operation, offset } => write!(f, " [{} {} {}]", register, operation, offset)?
                }
            }
            Ok(())
        }
    }
}

pub struct PASMProgram {
    pub functions: HashMap<String, (Vec<String>, Vec<PASMInstruction>)>,
}

pub struct PASMAllocatedProgram {
    pub functions: HashMap<String, Vec<PASMInstruction>>,
}

impl fmt::Display for PASMAllocatedProgram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (_, function) in self.functions.iter() {
            for instruction in function {
                if instruction.is_label {
                    writeln!(f, "{} ", instruction)?;
                } else {
                    writeln!(f, "\t{}", instruction)?;
                }
            }
        }
        Ok(())
    }
}

impl fmt::Display for PASMProgram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (_, function) in self.functions.iter() {
            for instruction in &function.1 {
                if instruction.is_label {
                    writeln!(f, "{} ", instruction)?;
                } else {
                    writeln!(f, "\t{}", instruction)?;
                }
            }
        }
        Ok(())
    }
}

/// Creates a new identifier for a variable with the given pattern
fn create_temp_variable_name<S: AsRef<str>>(pattern: S) -> String {
    let counter = TEMP_VAR_COUNTER.fetch_add(1, Ordering::SeqCst);
    format!("temp_{}_{}", pattern.as_ref(), counter)
}

fn operation_to_asm(operation: &OperationType, lparam: &Box<Node>, rparam: &Box<Node>) -> Result<(String, Vec<PASMInstruction>), String> {
    let temp = create_temp_variable_name("oplpar");
    let mut instructions = vec![];

    let operation = match operation {
        OperationType::Addition => "add",
        OperationType::Substraction => "sub",
        OperationType::Multiplication => "mul",
        OperationType::Division => "div",
        OperationType::Modulo => "mod",
    };

    let lparam_ins = assignment_to_asm(
        &Box::from(Node::Identifier { name: temp.clone() }),
       lparam
    )?;
    instructions.extend(lparam_ins);

    let new_rparam = match &**rparam {
        Node::Identifier { name } if !name.starts_with("$") => OperandType::Identifier { name: name.clone() },
        Node::Identifier { name: _ } => {
            let temp = create_temp_variable_name("oprpar");
            let rparam_ins = assignment_to_asm(
                &Box::from(Node::Identifier { name: temp.clone() }),
               rparam
            )?;
            instructions.extend(rparam_ins);
            OperandType::Identifier { name: temp.clone() }
        },
        Node::Litteral { value } => OperandType::Literal { value: *value },
        _ => {
            return Err(
                "lparam of operation should be either a literal or an identifier"
                    .to_string(),
            )
        }
    };

    instructions.push(PASMInstruction::new(
        operation.to_string(),
        vec![
            OperandType::Identifier { name: temp.clone() },
            new_rparam,
        ],
    ));

    Ok((temp, instructions))
}

fn assignment_to_asm(assignee: &Box<Node>, assignant: &Box<Node>) -> MaybeInstructions {
    let mut memory_operation = false; // If assignee is memory location, use load/store otherwise mov/movi

    let assignee = match &**assignee {
        Node::Identifier { name } => {
            if name.starts_with("$") {
                memory_operation = true;
            }
            name
        }
        _ => return Err("assignee should be an identifier".to_string()),
    }
    .to_string();

    match &**assignant {
        Node::Operation {
            lparam,
            rparam,
            operation,
        } => {
            // Need to perform the operation to assign
            // Need to create temporary variable
            let (temp, mut instructions) = operation_to_asm(operation, lparam, rparam)?;

            instructions.push(PASMInstruction::new(
                "mov".to_string(),
                vec![
                    OperandType::Identifier { name: assignee },
                    OperandType::Identifier { name: temp.clone() },
                ],
            ));
            Ok(instructions)
        }
        Node::Litteral { value } => {
            if memory_operation {
                Ok(vec![PASMInstruction::new(
                    "store".to_string(),
                    vec![
                        OperandType::Identifier { name: assignee },
                        OperandType::Literal { value: *value },
                    ],
                )])
            } else {
                Ok(vec![PASMInstruction::new(
                    "mov".to_string(),
                    vec![
                        OperandType::Identifier { name: assignee },
                        OperandType::Literal { value: *value },
                    ],
                )])
            }
        }
        Node::Identifier { name } => {
            if name.starts_with("$") {
                if memory_operation {
                    let temp = create_temp_variable_name("mem_op");
                    Ok(vec![
                        PASMInstruction::new(
                            "load".to_string(),
                            vec![
                                OperandType::Identifier { name: temp.clone() },
                                OperandType::Identifier {
                                    name: name.to_string(),
                                },
                            ],
                        ),
                        PASMInstruction::new(
                            "store".to_string(),
                            vec![
                                OperandType::Identifier { name: assignee },
                                OperandType::Identifier { name: temp.clone() },
                            ],
                        ),
                    ])
                } else {
                    Ok(vec![PASMInstruction::new(
                        "load".to_string(),
                        vec![
                            OperandType::Identifier { name: assignee },
                            OperandType::Identifier {
                                name: name.to_string(),
                            },
                        ],
                    )])
                }
            } else {
                if memory_operation {
                    Ok(vec![PASMInstruction::new(
                        "store".to_string(),
                        vec![
                            OperandType::Identifier { name: assignee },
                            OperandType::Identifier {
                                name: name.to_string(),
                            },
                        ],
                    )])
                } else {
                    Ok(vec![PASMInstruction::new(
                        "mov".to_string(),
                        vec![
                            OperandType::Identifier { name: assignee },
                            OperandType::Identifier {
                                name: name.to_string(),
                            },
                        ],
                    )])
                }
            }
        }
        Node::FunctionCall {
            function_name,
            parameters,
        } => {
            let mut instructions = function_to_asm(function_name, parameters)?;
            if memory_operation {
                let temp = create_temp_variable_name("function_return");
                instructions.extend(vec![
                    PASMInstruction::new(
                        "mov".to_string(),
                        vec![
                            OperandType::Identifier { name: temp.clone() },
                            OperandType::Identifier {
                                name: "'FRV".to_string(),
                            },
                        ],
                    ),
                    PASMInstruction::new(
                        "store".to_string(),
                        vec![
                            OperandType::Identifier { name: assignee },
                            OperandType::Identifier {
                                name: temp.to_string(),
                            },
                        ],
                    ),
                ]);
            } else {
                // Move call result to assignee
                instructions.push(PASMInstruction::new(
                    "mov".to_string(),
                    vec![
                        OperandType::Identifier { name: assignee },
                        OperandType::Identifier {
                            name: "'FRV".to_string(),
                        },
                    ],
                ));
            }
            Ok(instructions)
        }
        _ => Err(
            "rparam of an assignment should be either an operation, a literal or an identifier"
                .to_string(),
        ),
    }
}

fn comparison_to_asm(
    lparam: &Box<Node>,
    rparam: &Box<Node>,
    comparison: &ComparisonType,
    jmp_to: String,
) -> MaybeInstructions {
    let mut instructions = vec![];

    let lparam_name = match &**lparam {
        Node::Identifier { name } => name.to_string(),
        Node::Litteral { value } => {
            let temp = create_temp_variable_name("clp");
            instructions.push(PASMInstruction::new(
                "mov".to_string(),
                vec![
                    OperandType::Identifier { name: temp.clone() },
                    OperandType::Literal { value: *value },
                ],
            ));
            temp
        }
        _ => return Err("Invalid lparam for comparison".to_string()),
    };

    let rparam_name = match &**rparam {
        Node::Identifier { name } => name.to_string(),
        Node::Litteral { value } => {
            let temp = create_temp_variable_name("crp");
            instructions.push(PASMInstruction::new(
                "mov".to_string(),
                vec![
                    OperandType::Identifier { name: temp.clone() },
                    OperandType::Literal { value: *value },
                ],
            ));
            temp
        }
        _ => return Err("Invalid rparam for comparison".to_string()),
    };

    match *comparison {
        ComparisonType::EQ => {
            instructions.extend(vec![
                PASMInstruction::new(
                    "cmp".to_string(),
                    vec![
                        OperandType::Identifier {
                            name: lparam_name.clone(),
                        },
                        OperandType::Identifier {
                            name: rparam_name.clone(),
                        },
                    ],
                ),
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
                PASMInstruction::new(
                    "cmp".to_string(),
                    vec![
                        OperandType::Identifier {
                            name: lparam_name.clone(),
                        },
                        OperandType::Identifier {
                            name: rparam_name.clone(),
                        },
                    ],
                ),
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
                PASMInstruction::new(
                    "cmp".to_string(),
                    vec![
                        OperandType::Identifier {
                            name: lparam_name.clone(),
                        },
                        OperandType::Identifier {
                            name: rparam_name.clone(),
                        },
                    ],
                ),
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
                PASMInstruction::new(
                    "cmp".to_string(),
                    vec![
                        OperandType::Identifier {
                            name: rparam_name.clone(),
                        },
                        OperandType::Identifier {
                            name: lparam_name.clone(),
                        },
                    ],
                ),
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
                PASMInstruction::new(
                    "cmp".to_string(),
                    vec![
                        OperandType::Identifier {
                            name: rparam_name.clone(),
                        },
                        OperandType::Identifier {
                            name: lparam_name.clone(),
                        },
                    ],
                ),
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
                PASMInstruction::new(
                    "cmp".to_string(),
                    vec![
                        OperandType::Identifier {
                            name: lparam_name.clone(),
                        },
                        OperandType::Identifier {
                            name: rparam_name.clone(),
                        },
                    ],
                ),
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
                        vec![OperandType::Identifier { name: temp }]
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
    instructions.push(
        PASMInstruction::new(
            "sub".to_string(),
            vec![
                OperandType::Identifier { name: "'TSP".to_string() },
                OperandType::Literal { value: parameters.len() as i32 }
            ]
        )
    );

    Ok(instructions)
}

/// Produces the instructions needed for a function return.
/// 1. Puts the return value into the 'FRV register
/// 2. Restores the stack pointer to its original value
/// 3. Restores the base pointer to its original value
/// 4. actual ret instruction
fn ret_to_asm(value: &Option<String>) -> MaybeInstructions {
    let mut instructions = vec![];

    // Return value goes in FRV
    if let Some(v) = value {
        instructions.extend(assignment_to_asm(
            &Box::from(Node::Identifier {
                name: "'FRV".to_string(),
            }),
            &Box::from(Node::Identifier { name: v.clone() }),
        )?);
    }

    // Restore stack pointer
    instructions.push(
        PASMInstruction::new(
            "mov".to_string(),
            vec![
                OperandType::Identifier { name: "'TSP".to_string() },
                OperandType::Identifier { name: "'SBP".to_string() }
            ]
        )
    );

    // Restore base pointer
    instructions.push(
        PASMInstruction::new(
            "pop".to_string(),
            vec![
                OperandType::Identifier { name: "'SBP".to_string() }
            ]
        )
    );

    // Actual return instruction
    instructions.push(PASMInstruction::new("ret".to_string(), vec![]));
    Ok(instructions)
}

/// Produces a print instruction from the AST nodes
fn print_to_asm(node: &Box<Node>) -> MaybeInstructions {
    match &**node {
        Node::Identifier { name } => Ok(vec![PASMInstruction::new(
            "print".to_string(),
            vec![OperandType::Identifier { name: name.clone() }],
        )]),
        Node::Litteral { value } => Ok(vec![PASMInstruction::new(
            "print".to_string(),
            vec![OperandType::Literal { value: *value }],
        )]),
        _ => Err("Invalid value to print".to_string()),
    }
}

fn inst_to_pasm(node: &Box<Node>) -> MaybeInstructions {
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

impl PASMProgram {
    pub fn parse(ast: super::AST) -> Result<Self, String> {
        let mut functions = HashMap::new();

        for (function_name, fun) in ast.functions {
            let mut instructions = vec![PASMInstruction::new_label(format!(
                "function_{}_label",
                function_name
            ))];

            // First, push SBP
            instructions.push(
                PASMInstruction::new(
                    "push".to_string(),
                    vec![OperandType::Identifier { name: "'SBP".to_string() }]
                )
            );

            // Make stack pointer the base pointer
            instructions.push(
                PASMInstruction::new(
                    "mov".to_string(),
                    vec![
                        OperandType::Identifier { name: "'SBP".to_string() },
                        OperandType::Identifier { name: "'TSP".to_string() }
                    ]
                )
            );

            for inst in fun.content {
                instructions.extend(inst_to_pasm(&inst)?);
            }

            // Finally pop SBP
            instructions.push(PASMInstruction::new(
                "pop".to_string(),
                vec![OperandType::Identifier { name: "'SBP".to_string() }]
            ));

            functions.insert(function_name, (fun.parameters, instructions));
        }

        Ok(PASMProgram { functions })
    }
}
