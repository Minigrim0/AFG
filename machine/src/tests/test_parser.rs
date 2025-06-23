use crate::prelude::{Instruction, MemoryMappedProperties, OpCodes, OperandType, Registers};

use super::super::parser::parse;

#[test]
fn test_parser() {
    let text = "; Test program, used to validate the parser functionality
mov 'GPA $Velocity
mov 'GPB $Moment
store 'GPA #100
store 'GPB #-100";

    let instructions = parse(text);

    let expected = vec![
        Instruction {
            opcode: OpCodes::MOV,
            operand_1: OperandType::Register {
                idx: Registers::GPA as usize,
            },
            operand_2: OperandType::Literal {
                value: MemoryMappedProperties::Velocity as i32,
            },
        },
        Instruction {
            opcode: OpCodes::MOV,
            operand_1: OperandType::Register {
                idx: Registers::GPB as usize,
            },
            operand_2: OperandType::Literal {
                value: MemoryMappedProperties::Moment as i32,
            },
        },
        Instruction {
            opcode: OpCodes::STORE,
            operand_1: OperandType::Register {
                idx: Registers::GPA as usize,
            },
            operand_2: OperandType::Literal { value: 100 },
        },
        Instruction {
            opcode: OpCodes::STORE,
            operand_1: OperandType::Register {
                idx: Registers::GPB as usize,
            },
            operand_2: OperandType::Literal { value: -100 },
        },
    ];

    assert!(
        instructions.is_ok(),
        "Instructions parsing failed: {}",
        instructions.err().unwrap()
    );

    let instructions = instructions.unwrap();
    assert_eq!(
        instructions.len(),
        expected.len(),
        "Instruction set does not have the correct size"
    );

    for (i1, i2) in instructions.iter().zip(expected.iter()) {
        assert_eq!(i1, i2);
    }
}
