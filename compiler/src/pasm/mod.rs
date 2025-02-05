mod assignment;
mod instruction;
mod operand_type;
mod program;
mod translation;

type MaybeInstructions = Result<Vec<PASMInstruction>, String>;

pub use instruction::PASMInstruction;
pub use operand_type::OperandType;
pub use program::{PASMAllocatedProgram, PASMProgram};

#[cfg(test)]
mod tests;
