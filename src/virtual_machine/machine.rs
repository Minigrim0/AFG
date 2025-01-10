use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{Instructions, MachineStatus, MemoryMappedProperties, Registers};

#[derive(Debug, Clone, Copy)]
struct Instruction {
    opcode: Instructions, // 1 byte65536
    operand_1: u16,       // 2 bytes
    operand_2: u16,       // 2 bytes
}

#[derive(Component)]
pub struct VirtualMachine {
    registers: [u16; 12],             // 8 registers
    memory: [u16; 65536],             // 64KB of memory
    instructions: [Instruction; 255], // 255 instructions
    status: MachineStatus,
}

impl VirtualMachine {
    pub fn new() -> VirtualMachine {
        let mut machine = VirtualMachine {
            registers: [0; 12],
            memory: [0; 65536],
            instructions: [Instruction {
                opcode: Instructions::NOP,
                operand_1: 0,
                operand_2: 0,
            }; 255],
            status: MachineStatus::Ready,
        };

        machine.instructions[0] = Instruction {
            opcode: Instructions::MOVRI,
            operand_1: Registers::GPA as u16,
            operand_2: 100,
        };
        machine.instructions[1] = Instruction {
            opcode: Instructions::MOVMR,
            operand_1: MemoryMappedProperties::VelocityX as u16,
            operand_2: Registers::GPA as u16,
        };

        machine
    }

    pub fn update_mmp(&mut self, transform: &mut Transform, vel: &mut Velocity) {
        // Write read-only to memory, read writeable from memory
        self.memory[MemoryMappedProperties::PositionX as usize] =
            (transform.translation.x * 1000.0) as u16;
        self.memory[MemoryMappedProperties::PositionY as usize] =
            (transform.translation.y * 1000.0) as u16;
        self.memory[MemoryMappedProperties::Rotation as usize] =
            (transform.rotation.to_axis_angle().1 * 1000.0) as u16;

        let velocity: Vec2 = Vec2::new(
            self.memory[MemoryMappedProperties::VelocityX as usize] as f32 / 1000.0,
            self.memory[MemoryMappedProperties::VelocityY as usize] as f32 / 1000.0,
        );
        vel.linvel = velocity.rotate(Vec2::new(
            transform.rotation.to_axis_angle().1 * 1000.0,
            0.0,
        ));

        // println!(
        //     "PosX: {:5} PosY: {:5} Rot: {:5} Vel: ({:3.2}, {:3.2})",
        //     self.memory[MemoryMappedProperties::PositionX as usize],
        //     self.memory[MemoryMappedProperties::PositionY as usize],
        //     self.memory[MemoryMappedProperties::Rotation as usize],
        //     vel.linvel.x,
        //     vel.linvel.y
        // );
    }

    pub fn tick(&mut self) -> bool {
        match self.status {
            MachineStatus::Dead | MachineStatus::Complete => {
                return false;
            }
            MachineStatus::Ready => {
                self.registers[Registers::PC as usize] = 0;
                self.status = MachineStatus::Running;
            }
            _ => {}
        }

        let instruction = self.instructions[self.registers[Registers::PC as usize] as usize];
        match instruction.opcode {
            Instructions::MOVRI => {
                let operand_1: u16 = instruction.operand_1;
                if operand_1 > 0x0B {
                    println!("Invalid register");
                    self.status = MachineStatus::Dead;
                    return false;
                }
                println!("MOVRI {}, {}", operand_1, instruction.operand_2);
                self.registers[operand_1 as usize] = instruction.operand_2;
            }
            Instructions::MOVRR => {
                let operand_1: u16 = instruction.operand_1;
                let operand_2: u16 = instruction.operand_2;
                if operand_1 > 0x0B || operand_2 > 0x0B {
                    println!("Invalid register");
                    self.status = MachineStatus::Dead;
                    return false;
                }
                println!("MOVRR {}, {}", operand_1, operand_2);
                self.registers[operand_1 as usize] = self.registers[operand_2 as usize];
            }
            Instructions::MOVRM => {
                let operand_1: u16 = instruction.operand_1;
                let operand_2: u16 = instruction.operand_2;
                if operand_1 > 0x0B {
                    println!("Invalid register");
                    self.status = MachineStatus::Dead;
                    return false;
                }
                println!("MOVRM {}, {}", operand_1, operand_2);
                self.registers[operand_1 as usize] = self.memory[operand_2 as usize];
            }
            Instructions::MOVMR => {
                let operand_1: u16 = instruction.operand_1;
                let operand_2: u16 = instruction.operand_2;
                if operand_2 > 0x0B {
                    println!("Invalid register");
                    self.status = MachineStatus::Dead;
                    return false;
                }
                println!("MOVMR {}, {}", operand_1, operand_2);
                self.memory[operand_1 as usize] = self.registers[operand_2 as usize];
            }
            Instructions::NOP => {}
            _ => {
                println!("Not yet implemented");
            }
        }

        self.registers[Registers::PC as usize] += 1;
        if self.registers[Registers::PC as usize] as usize >= self.instructions.len() {
            self.status = MachineStatus::Complete;
        }
        true
    }
}
