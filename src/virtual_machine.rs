use bevy::prelude::*;

enum INSTRUCTIONS {
    MOV = 0x01, // Moves data from one location to another
    ADD = 0x02, // Adds two numbers
    SUB = 0x03,
    MUL = 0x04,
    DIV = 0x05,
    JMP = 0x06,
    JZ = 0x07,
    JNZ = 0x08,
    CALL = 0x09,
    RET = 0x0A,
    POP = 0x0B,
    PUSH = 0x0C,
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    opcode: u8,   // 1 byte
    operand_1: u16, // 2 bytes
    operand_2: u16, // 2 bytes
}

#[derive(Component)]
pub struct VirtualMachine {
    memory: [u8; 65536],              // 64KB of memory
    instructions: [Instruction; 255], // 255 instructions
}

impl VirtualMachine {
    pub fn new() -> VirtualMachine {
        VirtualMachine {
            memory: [0; 65536],
            instructions: [Instruction {
                opcode: 0,
                operand_1: 0,
                operand_2: 0,
            }; 255],
        }
    }
}
