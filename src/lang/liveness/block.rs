use crate::lang::asm::OperandType;

use super::super::asm::PASMInstruction;
use core::fmt;
use petgraph::graph::DiGraph;
use std::{collections::HashMap, f32::consts::PI};

use petgraph::dot::{Config, Dot};
use std::fs;

/// A basic block of code with no jumps
#[derive(Clone)]
pub struct Block {
    index: usize, // The index of the block in the function
    instructions: Vec<PASMInstruction>,
}

impl fmt::Debug for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({}) {}",
            self.index,
            self.instructions
                .iter()
                .map(|i| format!("{:?}", i))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Block {} {}",
            self.index,
            self.instructions
                .iter()
                .map(|i| format!("{}", i))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

/// Extracts all the labels from a function with their indexes
fn extract_labels(function: &Vec<PASMInstruction>) -> Vec<(usize, String)> {
    function
        .iter()
        .enumerate()
        .filter_map(|(idx, inst)| {
            if inst.is_label {
                Some((idx, inst.opcode.clone()))
            } else {
                None
            }
        })
        .collect()
}

impl Block {
    pub fn new(index: usize) -> Self {
        Self {
            index,
            instructions: Vec::new(),
        }
    }

    /// Wether this block has a final (forced) jump and the label that it jumps to (if any)
    /// If the block does not have a final jump, the result will be (false, None)
    /// If the block has a conditional jump at the end, the result will be (false, Some(label))
    /// If the block has an unconditional jump at the end, the result will be (true, Some(label))
    fn get_final_jump(&self) -> Result<(bool, Option<String>), String> {
        let last_instruction = match self.instructions.last() {
            Some(inst) => inst,
            None => return Ok((false, None)),
        };

        match last_instruction.opcode.as_str() {
            "jmp" => match last_instruction.operands.first() {
                Some(OperandType::Identifier { name }) => Ok((true, Some(name.clone()))),
                _ => Err(format!(
                    "No label found for jump instruction in block {}",
                    self.index
                )),
            },
            "jp" | "jn" | "jz" | "jnz" => match last_instruction.operands.first() {
                Some(OperandType::Identifier { name }) => Ok((false, Some(name.clone()))),
                _ => Err(format!(
                    "No label found for conditional jump instruction in block {}",
                    self.index
                )),
            },
            _ => Ok((false, None)),
        }
    }

    pub fn from_function(function: &Vec<PASMInstruction>) -> Result<Self, String> {
        let mut blocks = HashMap::new();
        // Initial block
        let mut current_block = String::new();

        for (idx, instruction) in function.iter().enumerate() {
            if idx == 0 && !instruction.is_label {
                return Err("First instruction in a function must be a label".to_string());
            }

            if instruction.is_label {
                // Labels are at the beginning of a block
                let new_block = format!("block_{}", idx);
                blocks.insert(new_block.clone(), Self::new(idx));
                current_block = new_block;
                if let Some(block) = blocks.get_mut(&current_block) {
                    block.instructions.push(instruction.clone());
                }
            } else {
                if let Some(block) = blocks.get_mut(&current_block) {
                    block.instructions.push(instruction.clone());
                }
                match instruction.opcode.as_str() {
                    "jmp" | "jp" | "jn" | "jz" | "jnz" => {
                        // Jumps are in the previous block
                        let new_block = format!("block_{}", idx + 1);
                        blocks.insert(new_block.clone(), Self::new(idx + 1));
                        current_block = new_block;
                    }
                    _ => { /* Do nothing */ }
                }
            }
        }

        let mut g: DiGraph<Block, ()> = DiGraph::new();
        let mut block_indexes = HashMap::new();
        for (block_name, block) in blocks.iter() {
            let idx = g.add_node(block.clone());
            block_indexes.insert(block_name, idx);
        }

        let labels = extract_labels(function);

        for (block_key, block_idx) in &block_indexes {
            let block = blocks.get(*block_key).unwrap();
            let (forced_jump, jump_label) = block.get_final_jump()?;

            if let Some(jump_label) = jump_label {
                let jump_block_name = format!(
                    "block_{}",
                    &labels.iter().find(|(_, l)| l == &jump_label).unwrap().0
                );
                if let Some(jump_block_idx) = block_indexes.get(&jump_block_name) {
                    g.add_edge(*block_idx, *jump_block_idx, ());
                } else {
                    println!(
                        "Unable to link block {} to block {}",
                        block_key, jump_block_name
                    );
                }
            }

            if !forced_jump {
                let jump_block_name = format!("block_{}", block.index + block.instructions.len());
                if let Some(jump_block_idx) = block_indexes.get(&jump_block_name) {
                    g.add_edge(*block_idx, *jump_block_idx, ());
                } else {
                    println!("Unable to find block {} in graph", jump_block_name);
                }
            }
        }

        let ast_output = "graph.viz";
        if let Err(e) = fs::write(
            &ast_output,
            format!("{:#?}", Dot::with_config(&g, &[Config::EdgeNoLabel])),
        ) {
            println!("Error writing graph: {}", e);
        }

        // for (block_name, block) in blocks.iter() {
        //     println!("Block: {}", block_name);
        //     for instruction in block.instructions.iter() {
        //         println!("\t\t{}", instruction);
        //     }
        // }

        Ok(Self::new(0))
    }
}
