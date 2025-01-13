use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::assets::Program;
use super::{Instructions, MachineStatus, MemoryMappedProperties, Registers};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Instruction {
    pub opcode: Instructions,   // 1 byte
    pub operand_1: i32,         // 4 bytes
    pub operand_2: Option<i32>, // 4 bytes
}

#[derive(Component)]
pub struct VirtualMachine {
    pub registers: [i32; 12], // 8 registers
    pub stack: Vec<i32>,
    pub memory: [i32; 65536], // 64KB of memory
    program_handle: Handle<Program>,
    pub status: MachineStatus,
}

impl VirtualMachine {
    pub fn new() -> VirtualMachine {
        VirtualMachine {
            registers: [0; 12],
            stack: vec![],
            memory: [0; 65536],
            program_handle: Handle::default(),
            status: MachineStatus::Ready,
        }
    }

    pub fn new_with_program(program: Handle<Program>) -> VirtualMachine {
        println!("Constructing new machine with handle {:?}", program);
        VirtualMachine {
            registers: [0; 12],
            stack: vec![],
            memory: [0; 65536],
            program_handle: program,
            status: MachineStatus::Ready,
        }
    }

    pub fn update_mmp(&mut self, transform: &mut Transform, vel: &mut Velocity) {
        let rotation_angle =
            transform.rotation.to_axis_angle().0.z * transform.rotation.to_axis_angle().1;

        // Write read-only to memory, read writeable from memory
        self.memory[MemoryMappedProperties::PositionX as usize] = transform.translation.x as i32;
        self.memory[MemoryMappedProperties::PositionY as usize] = transform.translation.y as i32;
        self.memory[MemoryMappedProperties::Rotation as usize] =
            (rotation_angle * (180.0 / PI)) as i32;

        let velocity: Vec2 = Vec2::new(
            self.memory[MemoryMappedProperties::VelocityX as usize] as f32,
            self.memory[MemoryMappedProperties::VelocityY as usize] as f32,
        );

        vel.linvel = Vec2::from_angle(rotation_angle).rotate(velocity);

        vel.angvel = self.memory[MemoryMappedProperties::Moment as usize] as f32 * (PI / 180.0);

        // println!(
        //     "PosX: {:5} PosY: {:5} Rot: {:5} Vel: ({:3}, {:3}, {:3})",
        //     self.memory[MemoryMappedProperties::PositionX as usize],
        //     self.memory[MemoryMappedProperties::PositionY as usize],
        //     self.memory[MemoryMappedProperties::Rotation as usize],
        //     self.memory[MemoryMappedProperties::VelocityX as usize],
        //     self.memory[MemoryMappedProperties::VelocityY as usize],
        //     self.memory[MemoryMappedProperties::Moment as usize]
        // );
    }

    fn invalid_instruction<S: AsRef<str>>(&mut self, msg: S) {
        println!("ERR: {}", msg.as_ref());
        self.status = MachineStatus::Dead;
    }

    pub fn tick(&mut self, programs: &Res<Assets<Program>>) -> bool {
        match self.status {
            MachineStatus::Dead | MachineStatus::Complete => {
                return false;
            }
            MachineStatus::Ready => {
                self.registers[Registers::PC as usize] = 0i32;
                self.status = MachineStatus::Running;
            }
            _ => {}
        }

        let instructions = if let Some(program) = programs.get(&self.program_handle) {
            &program.instructions
        } else {
            println!("Unable to find program");
            // self.invalid_instruction("Could not find program");
            return false;
        };

        let instruction = instructions[self.registers[Registers::PC as usize] as usize];
        match instruction.opcode {
            Instructions::MOVI => {
                let op1: i32 = instruction.operand_1;
                if let Some(op2) = instruction.operand_2 {
                    if op1 as usize > self.registers.len() {
                        self.invalid_instruction(format!("Invalid register {}", op1));
                        return false;
                    }
                    println!("MOVI <{}>, ${}", op1, op2);
                    self.registers[op1 as usize] = op2;
                } else {
                    self.invalid_instruction("Missing operand for MOVI instruction");
                    return false;
                }
            }
            Instructions::MOV => {
                let op1: i32 = instruction.operand_1;
                if let Some(op2) = instruction.operand_2 {
                    if op1 as usize > self.registers.len() || op2 as usize > self.registers.len() {
                        self.invalid_instruction(format!("Invalid register {}, {}", op1, op2));
                        return false;
                    }
                    println!("MOV <{}>, <{}>", op1, op2);
                    self.registers[op1 as usize] = self.registers[op2 as usize];
                } else {
                    self.invalid_instruction("Missing operand for MOV instruction");
                    return false;
                }
            }
            Instructions::STORE => {
                let op1 = instruction.operand_1;
                if op1 as usize > self.memory.len() {
                    self.invalid_instruction(format!("Invalid memory address {}", op1));
                    return false;
                }
                if let Some(op2) = instruction.operand_2 {
                    if op2 as usize > self.registers.len() {
                        self.invalid_instruction(format!("Invalid register {}", op2));
                        return false;
                    }
                    println!("STORE [<{}>], <{}>", op1, op2);
                    self.memory[self.registers[op1 as usize] as usize] =
                        self.registers[op2 as usize];
                } else {
                    self.invalid_instruction("Missing operand for STORE instruction");
                    return false;
                }
            }
            Instructions::STOREI => {
                let op1 = instruction.operand_1;
                if op1 as usize > self.memory.len() {
                    self.invalid_instruction(format!("Invalid memory address {}", op1));
                    return false;
                }
                if let Some(op2) = instruction.operand_2 {
                    println!("STOREI [<{}>], ${}", op1, op2);
                    self.memory[self.registers[op1 as usize] as usize] = op2;
                } else {
                    self.invalid_instruction("Missing operand for STOREI instruction");
                    return false;
                }
            }
            Instructions::LOAD => {
                let op1: i32 = instruction.operand_1;
                if op1 as usize > self.registers.len() {
                    self.invalid_instruction(format!("Invalid register {}", op1));
                    return false;
                }
                if let Some(op2) = instruction.operand_2 {
                    if op2 as usize > self.registers.len() {
                        self.invalid_instruction(format!("Invalid memory address {}", op2));
                        return false;
                    }

                    println!("LOAD <{}>, [<{}>]", op1, op2);
                    self.registers[op1 as usize] = self.memory[op2 as usize];
                } else {
                    self.invalid_instruction("Missing operand for LOAD instruction");
                }
            }
            Instructions::NOP => {}
            _ => {
                println!("Not yet implemented");
            }
        }

        self.registers[Registers::PC as usize] += 1;
        if self.registers[Registers::PC as usize] as usize >= instructions.len() {
            self.status = MachineStatus::Complete;
        }
        true
    }
}
