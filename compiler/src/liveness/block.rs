use core::fmt;
use petgraph::graph::DiGraph;
use std::collections::{HashMap, HashSet};

use crate::pasm::{OperandType, PASMInstruction};

#[derive(Debug, Clone, Copy)]
pub enum BlockTags {
    LoopStart = 1,   // For labels that indicate the start of a loop.
    LoopEnd = 2,     // Block that ends with a jump to a previous label
    InsideLoop = 4,  // All blocks between a LoopStart and LoopEnd. These block might be re-labeled in a later stage
    IfStart = 8,     // A block that conditionnaly jumps to a later label
    IfEnd = 16,      // A block that starts with an `if_exit` label (can be detected from the previous tag's jump)
    InsideIf = 32,   // A block executed only on condition that the if start block that precedes it evaluated to true
}

impl BlockTags {
    pub fn iterator() -> impl Iterator<Item = BlockTags> {
        [BlockTags::LoopStart, BlockTags::LoopEnd, BlockTags::InsideLoop, BlockTags::IfStart, BlockTags::IfEnd, BlockTags::InsideIf].iter().copied()
    }
}

/// A basic block of code with no jumps
#[derive(Clone)]
pub struct Block {
    index: usize, // The index of the block in the function
    instructions: Vec<PASMInstruction>,
    undead_out_sets: Vec<Vec<String>>,
    tags: u8
}

impl fmt::Debug for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} - {}\n{}",
            self.index,
            BlockTags::iterator().filter_map(
                |tag| if self.tags & tag as u8 != 0 {
                    Some(format!("{:?}", tag))
                } else {
                    None
                }
            ).collect::<Vec<String>>().join(", "),
            self.instructions
                .iter()
                .zip(&self.undead_out_sets)
                .map(|(i, s)| format!("{:50} ({})", i, s.join(", ")))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

/// Extracts all the labels from a function with their indexes in the form (index, label)
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
            undead_out_sets: vec![],
            tags: 0
        }
    }

    pub fn dead_analysis(&mut self) {
        self.undead_out_sets.push(vec![]);
        let mut live_set = HashSet::new();
        for instruction in self.instructions.iter().rev() {
            let (live, dead) = instruction.get_live_and_dead();

            for live_item in live {
                live_set.insert(live_item);
            }

            for dead_item in dead {
                live_set.remove(&dead_item);
            }

            self.undead_out_sets.insert(0, live_set.iter().map(|s| s.clone()).collect());
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

    /// Builds a map that maps line numbers to their corresponding block
    fn build_block_map(function: &Vec<PASMInstruction>) -> Result<HashMap<usize, Self>, String> {
        let mut blocks = HashMap::new();
        // Initial block
        let mut current_block = 0;

        for (idx, instruction) in function.iter().enumerate() {
            if idx == 0 && !instruction.is_label {
                return Err("First instruction in a function must be a label".to_string());
            }

            if instruction.is_label {
                // Labels are at the beginning of a block
                let new_block = idx;
                blocks.insert(new_block, Self::new(idx));
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
                        let new_block = idx + 1;
                        blocks.insert(new_block, Self::new(idx + 1));
                        current_block = new_block;
                    }
                    _ => { /* Do nothing */ }
                }
            }
        }

        Ok(blocks)
    }

    /// Transforms the map of blocks into a graph that link together blocks into an execution graph.
    fn into_graph(blocks: &HashMap<usize, Self>, function: &Vec<PASMInstruction>) -> Result<DiGraph<Block, ()>, String> {
        let mut g: DiGraph<Block, ()> = DiGraph::new();
        let mut block_indexes = HashMap::new();
        let labels = extract_labels(function);

        // Add the blocks to the graph
        for (block_name, block) in blocks.iter() {
            let idx = g.add_node(block.clone());
            block_indexes.insert(block_name, idx);
        }

        // Link the blocks together
        for (block_key, block_idx) in &block_indexes {
            let block = blocks.get(*block_key).unwrap();
            let (forced_jump, jump_label) = block.get_final_jump()?;

            if let Some(jump_label) = jump_label {
                let jump_block_name = labels.iter().find(|(_, l)| l == &jump_label).unwrap().0;
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
                let jump_block_name = block.index + block.instructions.len();
                if let Some(jump_block_idx) = block_indexes.get(&jump_block_name) {
                    g.add_edge(*block_idx, *jump_block_idx, ());
                } else {
                    println!("Unable to find block {} in graph", jump_block_name);
                }
            }
        }

        let mut if_exits: Vec<usize> = vec![];
        let mut loop_starts: Vec<usize> = vec![];

        // Find if conditions and exit blocks, loop end blocks
        g = g.map(
            |_, block| {
                let mut block = block.clone();
                if let Ok((forced, Some(label))) = block.get_final_jump() {
                    if let Some((jump_index, _)) = labels.iter().find(|e| e.1 == label) {
                        if !forced && *jump_index > block.index {  // Indicates an if condition block
                            block.tags |= BlockTags::IfStart as u8;
                            if_exits.push(*jump_index);
                        }

                        else if *jump_index < block.index {
                            block.tags |= BlockTags::LoopEnd as u8;
                            loop_starts.push(*jump_index);
                        }
                    }
                }

                if let Some(index) = if_exits.iter().find(|index| **index == block.index) {
                    // Unwrap should be safe, checked right before, single threaded
                    if_exits.remove(if_exits.iter().position(|e| e == index).unwrap());
                    block.tags |= BlockTags::IfEnd as u8;
                }

                block
            },
            |_, e| *e,
        );

        // Find loop starts from previously discovered ends
        g = g.map(
            |_, block| {
                let mut block = block.clone();
                if let Some(index) = loop_starts.iter().find(|index| **index == block.index) {
                    loop_starts.remove(loop_starts.iter().position(|e| e == index).unwrap());
                    block.tags |= BlockTags::LoopStart as u8;
                }

                block
            },
            |_, e| *e,
        );

        Ok(g)
    }

    pub fn from_function(function: &Vec<PASMInstruction>) -> Result<DiGraph<Block, ()>, String> {
        let blocks = Self::build_block_map(function)?;
        let block_graph = Self::into_graph(&blocks, function)?;

        Ok(block_graph.map(|_, n| {
            let mut n = n.clone();
            n.dead_analysis();
            n
        }, |_, e| *e))
    }
}
