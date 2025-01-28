mod operand_type;
mod instruction;
mod program;
mod translation;

type MaybeInstructions = Result<Vec<PASMInstruction>, String>;

pub use program::{PASMProgram, PASMAllocatedProgram};
pub use instruction::PASMInstruction;
pub use operand_type::OperandType;
