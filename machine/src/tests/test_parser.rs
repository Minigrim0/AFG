use std::fs;

use crate::virtual_machine::OperandType;

use super::super::parser::parse;
use super::super::{Instruction, OpCodes, Registers, MemoryMappedProperties};

#[test]
fn test_parser() {
    let text = fs::read_to_string("assets/programs/test.asmfg");
    assert!(text.is_ok(), "Unable to read input file: {}", text.err().unwrap().to_string());
    let instructions = parse(text.unwrap());

    let expected = vec![
        Instruction {
            opcode: OpCodes::MOV,
            operand_1: OperandType::Register { idx: Registers::GPA as usize },
            operand_2: OperandType::Literal { value: MemoryMappedProperties::VelocityY as i32 },
        },
        Instruction {
            opcode: OpCodes::MOV,
            operand_1: OperandType::Register { idx: Registers::GPB as usize },
            operand_2: OperandType::Literal { value: MemoryMappedProperties::Moment as i32 },
        },
        Instruction {
            opcode: OpCodes::STORE,
            operand_1: OperandType::Register { idx: Registers::GPA as usize },
            operand_2: OperandType::Literal { value: 100 },
        },
        Instruction {
            opcode: OpCodes::STORE,
            operand_1: OperandType::Register { idx: Registers::GPB as usize },
            operand_2: OperandType::Literal { value: -100 },
        }
    ];

    assert!(instructions.is_ok());

    let instructions = instructions.unwrap();
    assert_eq!(instructions.len(), expected.len(), "Instruction set does not have the correct size");

    for (i1, i2) in instructions.iter().zip(expected.iter()) {
        assert_eq!(i1, i2);
    }
}
