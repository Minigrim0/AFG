use std::sync::atomic::{AtomicUsize, Ordering};
use core::fmt;

use bevy::color::palettes::css::PLUM;

/// Transforms the AST of a function into pseudo-asm
use super::{ast::node::{ComparisonType, OperationType}, Node};

static TEMP_VAR_COUNTER: AtomicUsize = AtomicUsize::new(0);

type MaybeInstructions = Result<Vec<PASMInstruction>, String>;

pub enum OperandType {
    Identifier { name: String },
    Literal { value: i32 }
}

pub struct PASMInstruction {
    pub is_label: bool,  // Whether this is just a label or not
    pub opcode: String,  // Will not change until the end
    pub operands: Vec<OperandType>  // Up to two operands
}

impl PASMInstruction {
    pub fn new_label(name: String) -> Self {
        Self {
            is_label: true,
            opcode: name,
            operands: vec![]
        }
    }

    pub fn new(instr: String, operands: Vec<OperandType>) -> Self {
        Self {
            is_label: false,
            opcode: instr,
            operands
        }
    }
}

impl fmt::Display for PASMInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_label {
            write!(f, "{}:", self.opcode)
        } else {
            write!(f, "{}", self.opcode)?;
            for operand in self.operands.iter() {
                match operand {
                    OperandType::Identifier { name } => write!(f, " {}", name)?,
                    OperandType::Literal { value } => write!(f, " #{}", value)?,
                }
            }
            Ok(())
        }
    }
}

pub struct PASMProgram {
    pub instructions: Vec<PASMInstruction>
}

impl fmt::Display  for PASMProgram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for instruction in self.instructions.iter() {
            if instruction.is_label {
                writeln!(f, "{} ", instruction)?;
            } else {
                writeln!(f, "\t{}", instruction)?;
            }
        }
        Ok(())
    }
}

fn create_temp_variable_name<S: AsRef<str>>(pattern: S) -> String {
    let counter = TEMP_VAR_COUNTER.fetch_add(1, Ordering::SeqCst);
    format!("temp_{}_{}", pattern.as_ref(), counter)
}

fn assignment_to_asm(assignee: &Box<Node>, assignant: &Box<Node>) -> MaybeInstructions {
    let mut memory_operation = false;  // If assignee is memory location, use load/store otherwise mov/movi

    let assignee = match &**assignee {
        Node::Identifier { name } => {
            if name.starts_with("$") {
                memory_operation = true;
            }
            name
        },
        _ => return Err("assignee should be an identifier".to_string())
    }.to_string();

    match &**assignant {
        Node::Operation { lparam, rparam, operation } => {  // Need to perform the operation to assign
            // Need to create temporary variable
            let temp = create_temp_variable_name("arp");
            let mut instructions = vec![];

            match &**lparam {
                Node::Identifier { name } => {
                    instructions.push(PASMInstruction::new("mov".to_string(), vec![OperandType::Identifier { name: temp.clone() }, OperandType::Identifier { name: name.clone() }]))
                },
                Node::Litteral { value } => {
                    instructions.push(PASMInstruction::new("movi".to_string(), vec![OperandType::Identifier { name: temp.clone() }, OperandType::Literal { value: *value }]))
                },
                _ => return Err("lparam of operation should be either a literal or an identifier".to_string())
            }

            let operation = match operation {
                OperationType::Addition => "add",
                OperationType::Substraction => "sub",
                OperationType::Multiplication => "mul",
                OperationType::Division => "div",
                OperationType::Modulo => "mod",
            };

            match &**rparam {
                Node::Identifier { name } => {
                    instructions.push(PASMInstruction::new(format!("{}", operation), vec![OperandType::Identifier { name: temp.clone() }, OperandType::Identifier { name: name.clone() }]))
                },
                Node::Litteral { value } => {
                    instructions.push(PASMInstruction::new(format!("{}i", operation), vec![OperandType::Identifier { name: temp.clone() }, OperandType::Literal { value: *value }]))
                },
                _ => return Err("lparam of operation should be either a literal or an identifier".to_string())
            };

            instructions.push(PASMInstruction::new("mov".to_string(), vec![OperandType::Identifier { name: assignee }, OperandType::Identifier { name: temp.clone() }]));
            Ok(instructions)
        },
        Node::Litteral { value } => {
            if memory_operation {
                Ok(vec![PASMInstruction::new("storei".to_string(), vec![OperandType::Identifier { name: assignee }, OperandType::Literal { value: *value }])])
            } else {
                Ok(vec![PASMInstruction::new("movi".to_string(), vec![OperandType::Identifier { name: assignee }, OperandType::Literal { value: *value }])])
            }
        },
        Node::Identifier { name } => {
            if name.starts_with("$") {
                if memory_operation {
                    let temp = create_temp_variable_name("memory_operation");
                    Ok(vec![
                        PASMInstruction::new("loadi".to_string(), vec![OperandType::Identifier { name: temp.clone() }, OperandType::Identifier { name: name.to_string() } ]),
                        PASMInstruction::new("store".to_string(), vec![OperandType::Identifier { name: assignee }, OperandType::Identifier { name: temp.clone() } ])
                    ])
                } else {
                    Ok(vec![PASMInstruction::new("store".to_string(), vec![OperandType::Identifier { name: assignee }, OperandType::Identifier { name: name.to_string() } ])])
                }
            } else {
                if memory_operation {
                    Ok(vec![PASMInstruction::new("store".to_string(), vec![OperandType::Identifier { name: assignee }, OperandType::Identifier { name: name.to_string() } ])])
                } else {
                    Ok(vec![PASMInstruction::new("mov".to_string(), vec![OperandType::Identifier { name: assignee }, OperandType::Identifier { name: name.to_string() } ])])
                }
            }
        },
        Node::FunctionCall { function_name, parameters } => {
            let mut instructions = function_to_asm(function_name, parameters)?;
            if memory_operation {
                let temp = create_temp_variable_name("function_return");
                instructions.extend(vec![
                    PASMInstruction::new("mov".to_string(), vec![OperandType::Identifier { name: temp.clone() }, OperandType::Identifier { name: "'FRP".to_string() } ]),
                    PASMInstruction::new("store".to_string(), vec![OperandType::Identifier { name: assignee }, OperandType::Identifier { name: temp.to_string() } ])
                ]);
            } else {
                // Move call result to assignee
                instructions.push(
                    PASMInstruction::new("mov".to_string(), vec![OperandType::Identifier { name: assignee }, OperandType::Identifier { name: "'FRP".to_string() } ])
                );
            }
            Ok(instructions)
        }
        _ => Err("rparam of an assignment should be either an operation, a literal or an identifier".to_string())
    }
}

fn comparison_to_asm(lparam: &Box<Node>, rparam: &Box<Node>, comparison: &ComparisonType, jmp_to: String) -> MaybeInstructions {
    let mut instructions = vec![];

    let lparam_name = match &**lparam {
        Node::Identifier { name } => name.to_string(),
        Node::Litteral { value } => {
            let temp = create_temp_variable_name("clp");
            instructions.push(
                PASMInstruction::new("movi".to_string(), vec![OperandType::Identifier { name: temp.clone() }, OperandType::Literal { value: *value }])
            );
            temp
        },
        _ => return Err("Invalid lparam for comparison".to_string())
    };

    let rparam_name = match &**rparam {
        Node::Identifier { name } => name.to_string(),
        Node::Litteral { value } => {
            let temp = create_temp_variable_name("crp");
            instructions.push(
                PASMInstruction::new("movi".to_string(), vec![OperandType::Identifier { name: temp.clone() }, OperandType::Literal { value: *value }])
            );
            temp
        },
        _ => return Err("Invalid rparam for comparison".to_string())
    };

    match *comparison {
        ComparisonType::EQ => {
            instructions.extend(vec![
                PASMInstruction::new("cmp".to_string(), vec![OperandType::Identifier { name: lparam_name.clone() }, OperandType::Identifier { name: rparam_name.clone() }]),
                PASMInstruction::new("jnz".to_string(), vec![OperandType::Identifier { name: jmp_to.clone() }]),  // If not zero jump to next block's label
            ])
        },
        ComparisonType::DIFF => {
            instructions.extend(vec![
                PASMInstruction::new("cmp".to_string(), vec![OperandType::Identifier { name: lparam_name.clone() }, OperandType::Identifier { name: rparam_name.clone() }]),
                PASMInstruction::new("jz".to_string(), vec![OperandType::Identifier { name: jmp_to.clone() }]),  // If not zero jump to next block's label
            ])
        },
        ComparisonType::GE => {
            instructions.extend(vec![
                PASMInstruction::new("cmp".to_string(), vec![OperandType::Identifier { name: lparam_name.clone() }, OperandType::Identifier { name: rparam_name.clone() }]),
                PASMInstruction::new("jn".to_string(), vec![OperandType::Identifier { name: jmp_to.clone() }]),  // If not zero jump to next block's label
            ])
        },
        ComparisonType::GT => {
            instructions.extend(vec![
                PASMInstruction::new("cmp".to_string(), vec![OperandType::Identifier { name: lparam_name.clone() }, OperandType::Identifier { name: rparam_name.clone() }]),
                PASMInstruction::new("jn".to_string(), vec![OperandType::Identifier { name: jmp_to.clone() }]),  // If not zero jump to next block's label
                PASMInstruction::new("jz".to_string(), vec![OperandType::Identifier { name: jmp_to.clone() }]),  // If not zero jump to next block's label
            ])
        },
        ComparisonType::LT => {
            instructions.extend(vec![
                PASMInstruction::new("cmp".to_string(), vec![OperandType::Identifier { name: lparam_name.clone() }, OperandType::Identifier { name: rparam_name.clone() }]),
                PASMInstruction::new("jz".to_string(), vec![OperandType::Identifier { name: jmp_to.clone() }]),  // If not zero jump to next block's label
                PASMInstruction::new("jp".to_string(), vec![OperandType::Identifier { name: jmp_to.clone() }]),  // If not zero jump to next block's label
            ])
        },
        ComparisonType::LE => {
            instructions.extend(vec![
                PASMInstruction::new("cmp".to_string(), vec![OperandType::Identifier { name: lparam_name.clone() }, OperandType::Identifier { name: rparam_name.clone() }]),
                PASMInstruction::new("jp".to_string(), vec![OperandType::Identifier { name: jmp_to.clone() }]),  // If not zero jump to next block's label
            ])
        },
    }

    Ok(instructions)
}

///  If exit label is Some, this function will not add an exit label !
fn if_to_asm(condition: &Box<Node>, content: &Vec<Box<Node>>, exit_label: Option<String>) -> MaybeInstructions {
    let mut instructions = vec![];
    let next_block_label = match &exit_label {
        Some(v) => v.clone(),
        None => create_temp_variable_name("if_exit")
    };

    match &**condition {
        Node::Comparison { lparam, rparam, comparison } => {
            instructions.extend(comparison_to_asm(lparam, rparam, comparison, next_block_label.clone())?);
        },
        Node::Identifier { name } => {
            instructions.extend(vec![
                PASMInstruction::new("cmp".to_string(), vec![OperandType::Identifier { name: name.clone() }, OperandType::Literal { value: 0 }]),
                PASMInstruction::new("jz".to_string(), vec![OperandType::Identifier { name: next_block_label.clone() }])
            ])
        },
        Node::Litteral { value } => {
            let temp_condition = create_temp_variable_name("cp");
            instructions.extend(vec![
                PASMInstruction::new("movi".to_string(), vec![OperandType::Identifier { name: temp_condition.clone() }, OperandType::Literal { value: *value }]),
                PASMInstruction::new("cmp".to_string(), vec![OperandType::Identifier { name: temp_condition.clone() }, OperandType::Literal { value: 0 }]),
                PASMInstruction::new("jz".to_string(), vec![OperandType::Identifier { name: next_block_label.clone() }])
            ])
        },
        _ => return Err("Unexpected ast node for if condition".to_string())
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
    let mut instructions = vec![
        PASMInstruction::new_label(before_label.clone()),
    ];

    instructions.extend(
        if_to_asm(condition, content, Some(after_label.clone()))?
    );
    instructions.extend(vec![
        PASMInstruction::new("jmp".to_string(), vec![OperandType::Identifier { name: before_label.clone() }]),
        PASMInstruction::new_label(after_label.clone())
    ]);

    Ok(instructions)
}

fn loop_to_asm(content: &Vec<Box<Node>>) -> MaybeInstructions {
    let label = create_temp_variable_name("loop_label");
    let mut instructions = vec![
        PASMInstruction::new_label(label.to_string())
    ];

    for node in content {
        instructions.extend(inst_to_pasm(node)?)
    }
    instructions.push(
        PASMInstruction::new("jmp".to_string(), vec![OperandType::Identifier { name: label.clone() }])
    );

    Ok(instructions)
}

fn function_to_asm(function_name: &String, parameters: &Vec<Box<Node>>) -> MaybeInstructions {
    let mut instructions = vec![];

    for node in parameters.iter().rev() {
        match &**node {
            Node::Identifier { name } =>  instructions.push(
                PASMInstruction::new("push".to_string(), vec![OperandType::Identifier { name: name.clone() }])
            ),
            Node::Litteral { value } => instructions.push(
                PASMInstruction::new("pushi".to_string(), vec![OperandType::Literal { value: *value }])
            ),
            _ => return Err("Invalid value in function call, only identifiers and literals are allowed".to_string())
        }
    }

    instructions.push(
        PASMInstruction::new("call".to_string(), vec![OperandType::Identifier { name: format!("function_{}_label", function_name) }])
    );

    Ok(instructions)
}

fn ret_to_asm(value: &Option<String>) -> MaybeInstructions {
    let mut instructions = vec![];

    if let Some(v) = value {
        instructions.extend(
            assignment_to_asm(
                &Box::from(Node::Identifier { name: "'FRP".to_string() }),
                &Box::from(Node::Identifier { name: v.clone() })
            )?
        );
    }
    instructions.push(
        PASMInstruction::new("ret".to_string(), vec![])
    );
    Ok(instructions)
}

fn inst_to_pasm(node: &Box<Node>) -> MaybeInstructions {
    match &**node {
        Node::Assignment { lparam, rparam } => Ok(assignment_to_asm(lparam, rparam)?),
        Node::IfCondition { condition, content } => Ok(if_to_asm(condition, content, None)?),
        Node::Loop { content } => Ok(loop_to_asm(content)?),
        Node::WhileLoop { condition, content } => Ok(while_to_asm(condition, content)?),
        Node::FunctionCall { function_name, parameters } => Ok(function_to_asm(function_name, parameters)?),
        Node::Return { value } => Ok(ret_to_asm(value)?),
        _ => Err("Not implemented".to_string())
    }
}

impl PASMProgram {
    pub fn parse(ast: super::AST) -> Result<Self, String> {
        let mut instructions = vec![];

        for (function_name, fun) in ast.functions {
            instructions.push(PASMInstruction::new_label(format!("function_{}_label", function_name)));

            for argument in fun.parameters {
                instructions.push(PASMInstruction::new("pop".to_string(), vec![OperandType::Identifier { name: argument }]));
            }

            for inst in fun.content {
                instructions.extend(inst_to_pasm(&inst)?);
            }
        }

        Ok(PASMProgram { instructions })
    }
}
