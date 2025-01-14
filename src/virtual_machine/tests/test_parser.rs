use std::fs;

use super::super::parser::parse;
use super::super::{Instruction, Instructions, Registers, MemoryMappedProperties};

#[test]
fn test_parser() {
    let text = fs::read_to_string("assets/programs/test.csasm");
    assert!(text.is_ok(), "Unable to read input file: {}", text.err().unwrap().to_string());
    let instructions = parse(text.unwrap());

    let expected = vec![
        Instruction {
            opcode: Instructions::MOVI,
            operand_1: (Registers::GPA as i32),
            operand_2: Some(MemoryMappedProperties::VelocityY as i32),
        },
        Instruction {
            opcode: Instructions::MOVI,
            operand_1: (Registers::GPB as i32),
            operand_2: Some(MemoryMappedProperties::Moment as i32),
        },
        Instruction {
            opcode: Instructions::STOREI,
            operand_1: (Registers::GPA as i32),
            operand_2: Some(100),
        },
        Instruction {
            opcode: Instructions::STOREI,
            operand_1: (Registers::GPB as i32),
            operand_2: Some(-100),
        }
    ];

    assert!(instructions.is_ok());

    let instructions = instructions.unwrap();
    assert_eq!(instructions.len(), expected.len(), "Instruction set does not have the correct size");

    for (i1, i2) in instructions.iter().zip(expected.iter()) {
        assert_eq!(i1, i2);
    }
}
