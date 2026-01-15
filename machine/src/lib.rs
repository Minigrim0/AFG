use std::fmt;
use std::fs;

mod enums;
mod errors;
mod machine;
mod parser;
mod variables;

#[cfg(feature = "bevy")]
mod components;

#[cfg(test)]
mod tests;

use enums::{OpCodes, OperandType};
use parser::parse;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Instruction {
    pub opcode: OpCodes,
    pub operand_1: OperandType,
    pub operand_2: OperandType,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {} {}", self.opcode, self.operand_1, self.operand_2)
    }
}

#[derive(Debug)]
#[cfg_attr(
    feature = "bevy",
    derive(bevy::prelude::Asset, bevy::prelude::TypePath)
)]
pub struct Program {
    pub original_file: String,
    pub instructions: Vec<Instruction>,
    pub textual_instructions: String,
}

impl Program {
    pub fn new(path: String) -> Result<Self, String> {
        let contents = fs::read_to_string(&path).map_err(|e| e.to_string())?;
        let instructions = parse(&contents).map_err(|e| e.to_string())?;

        Ok(Self {
            original_file: path,
            instructions,
            textual_instructions: contents,
        })
    }
}

pub mod prelude {
    pub use super::enums::*;
    pub use super::errors::*;
    pub use super::machine::*;
    pub use super::parser::*;
    pub use super::variables::*;
    pub use super::Instruction;
    pub use super::Program;

    #[cfg(feature = "bevy")]
    pub use super::components::*;
}
