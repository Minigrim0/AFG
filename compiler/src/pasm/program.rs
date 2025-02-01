use std::collections::{HashMap, HashSet};
use std::fmt;

use super::translation::inst_to_pasm;
use super::{OperandType, PASMInstruction};

use crate::ast::AST;

fn get_frame_variables(function: &Vec<PASMInstruction>) -> Vec<String> {
    let mut frame_variables = HashSet::new();

    for instruction in function.iter() {
        for operand in instruction.operands.iter() {
            if let Some(name) = operand.get_frame_variable() {
                frame_variables.insert(name.clone());
            }
        }
    }

    frame_variables.into_iter().collect()
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

impl PASMProgram {
    pub fn parse(ast: AST) -> Result<Self, String> {
        let mut functions = HashMap::new();

        for (function_name, fun) in ast.functions {
            let mut instructions = vec![PASMInstruction::new_label(format!(
                "function_{}_label",
                function_name
            ))];

            // First, push SBP
            if function_name != "main" {
                instructions.push(PASMInstruction::new(
                    "push".to_string(),
                    vec![OperandType::Identifier {
                        name: "'SBP".to_string(),
                    }],
                ));
            }
            // Make stack pointer the base pointer
            instructions.push(PASMInstruction::new(
                "mov".to_string(),
                vec![
                    OperandType::Identifier {
                        name: "'SBP".to_string(),
                    },
                    OperandType::Identifier {
                        name: "'TSP".to_string(),
                    },
                ],
            ));

            let mut inner_instructions = vec![];
            for inst in fun.content {
                inner_instructions.extend(inst_to_pasm(&inst)?);
            }

            // Allocate stack
            let frame_variables = get_frame_variables(&inner_instructions);
            let stack_size = frame_variables
                .into_iter()
                .filter(|variable| !fun.parameters.iter().position(|v| v == variable).is_some())
                .collect::<Vec<String>>()
                .len();

            instructions.push(PASMInstruction::new(
                "sub".to_string(),
                vec![
                    OperandType::Identifier {
                        name: "'TSP".to_string(),
                    },
                    OperandType::Literal {
                        value: stack_size as i32,
                    },
                ],
            ));

            // Restoring the stack pointer & base pointer and moving the return value to the FRV register
            // is handled by the return instruction translation unit
            instructions.extend(inner_instructions);

            if function_name == "main" {
                instructions.push(PASMInstruction::new("halt".to_string(), vec![]));
            }

            functions.insert(function_name, (fun.parameters, instructions));
        }

        Ok(PASMProgram { functions })
    }
}
