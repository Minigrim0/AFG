use std::fs;
use std::fmt;

mod errors;
mod machine;
mod parser;
mod enums;

#[cfg(test)]
mod tests;

use parser::parse;
use enums::{OpCodes, OperandType};

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
#[cfg_attr(feature = "bevy", derive(bevy::prelude::Asset, bevy::prelude::TypePath))]
pub struct Program {
    pub original_file: String,
    pub instructions: Vec<Instruction>,
}

impl Program {
    pub fn new(path: String) -> Result<Self, String> {
        let contents = fs::read_to_string(&path).map_err(|e| e.to_string())?;
        let instructions = parse(&contents).map_err(|e| e.to_string())?;

        Ok(Self {
            original_file: path,
            instructions,
        })
    }
}

pub fn get_special_variables() -> Vec<String> {
    vec![
        "$PositionX".to_string(),
        "$PositionY".to_string(), // Read-only Vertical position
        "$Rotation".to_string(),  // Read-only Rotation
        "$Ray0Dist".to_string(),
        "$Ray0Type".to_string(),
        "$Ray1Dist".to_string(),
        "$Ray1Type".to_string(),
        "$Ray2Dist".to_string(),
        "$Ray2Type".to_string(),
        "$Ray3Dist".to_string(),
        "$Ray3Type".to_string(),
        "$Ray4Dist".to_string(),
        "$Ray4Type".to_string(),
        "$Ray5Dist".to_string(),
        "$Ray5Type".to_string(),
        "$Ray6Dist".to_string(),
        "$Ray6Type".to_string(),
        "$VelocityX".to_string(),
        "$VelocityY".to_string(),
        "$Moment".to_string(),
    ]
}

pub mod prelude {
    pub use super::Instruction;
    pub use super::Program;
    pub use super::machine::*;
    pub use super::errors::*;
    pub use super::enums::*;
    pub use super::parser::*;
}
