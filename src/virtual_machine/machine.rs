use std::f32::consts::PI;
use std::fmt::Display;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::assets::Program;
use super::{Instructions, MachineStatus, MemoryMappedProperties, Registers, OperandType};

enum Flags {
    ZeroFlag     = 0b00000001,
    OverflowFlag = 0b00000010,
    NegativeFlag = 0b00000100,
    PositiveFlag = 0b00001000,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Instruction {
    pub opcode: Instructions,
    pub operand_1: OperandType,
    pub operand_2: OperandType,
}

#[derive(Component)]
pub struct VirtualMachine {
    pub registers: [i32; 12], // 8 registers
    pub stack: Vec<i32>,
    pub flags: u8,
    pub memory: [i32; 65536], // 64KB of memory
    program_handle: Handle<Program>,
    pub status: MachineStatus,
}

impl VirtualMachine {
    pub fn new() -> VirtualMachine {
        VirtualMachine {
            registers: [0; 12],
            stack: vec![],
            flags: 0,
            memory: [0; 65536],
            program_handle: Handle::default(),
            status: MachineStatus::Ready,
        }
    }

    pub fn with_program(mut self, program: Handle<Program>) -> VirtualMachine {
        self.program_handle = program;
        self
    }

    fn check_flag(&self, flag: Flags) -> bool {
        self.flags & flag as u8 != 0
    }

    fn is_valid_register(&self) -> Box<dyn Fn(&i32) -> (bool, String)> {
        let register_len = self.registers.len();
        Box::from(move |val: &i32| ((*val as usize) < register_len, "Invalid register".to_string()))
    }

    fn is_valid_memory_address(&self) -> Box<dyn Fn(&i32) -> (bool, String)> {
        let memory_size = self.memory.len();
        Box::from(move |val: &i32| ((*val as usize) < memory_size, "Invalid memory addresss".to_string()))
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

    pub fn update_rays(&mut self, rays: Vec<Option<(Entity, f32)>>) {
        let ray_addr = vec![
            (MemoryMappedProperties::Ray0Dist, MemoryMappedProperties::Ray0Type),
            (MemoryMappedProperties::Ray1Dist, MemoryMappedProperties::Ray1Type),
            (MemoryMappedProperties::Ray2Dist, MemoryMappedProperties::Ray2Type),
            (MemoryMappedProperties::Ray3Dist, MemoryMappedProperties::Ray3Type),
            (MemoryMappedProperties::Ray4Dist, MemoryMappedProperties::Ray4Type),
            (MemoryMappedProperties::Ray5Dist, MemoryMappedProperties::Ray5Type),
            (MemoryMappedProperties::Ray6Dist, MemoryMappedProperties::Ray6Type),
        ];

        for (ray_data, (dist_addr, type_addr)) in rays.iter().zip(ray_addr) {
            if let Some((ent, dist)) = ray_data {
                self.memory[dist_addr as usize] = *dist as i32;
                self.memory[type_addr as usize] = 1;
            } else {
                self.memory[dist_addr as usize] = 0;
                self.memory[type_addr as usize] = 0;
            }
        }
    }

    fn invalid_instruction<S: AsRef<str>>(&mut self, msg: S) {
        println!("FATAL: {}", msg.as_ref());
        self.status = MachineStatus::Dead;
    }

    fn update_flags(flags: u8, value: i32) -> u8 {
        match value {
            0 => flags | Flags::ZeroFlag as u8,
            n if n < 0 => flags | Flags::NegativeFlag as u8,
            _ => flags | Flags::PositiveFlag as u8
        }
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
            // self.invalid_instruction("Could not find program");
            return false;
        };

        let instruction = instructions[self.registers[Registers::PC as usize] as usize];
        let mut next_jump: i32 = 1;
        let mut next_flags: u8 = 0;

        match instruction.opcode {
            Instructions::MOV => {
                println!("mov {:?} {:?}", instruction.operand_1, instruction.operand_2);
                if let OperandType::Register { idx: op1 } = instruction.operand_1 {
                    match instruction.operand_2 {
                        OperandType::Register { idx: op2 } => self.registers[op1 as usize] = self.registers[op2 as usize],
                        OperandType::Literal { value: op2 } => self.registers[op1 as usize] = op2,
                        OperandType::None => self.invalid_instruction("Missing second operand for mov instruction"),
                    }
                } else {
                    self.invalid_instruction("Missing first operand for mov instruction");
                }
            }
            Instructions::STORE => {
                let to_store = match instruction.operand_2 {
                    OperandType::Register { idx: op2 } => self.registers[op2 as usize],
                    OperandType::Literal { value: op2 } => op2,
                    OperandType::None => { self.invalid_instruction("Missing second operand for store instruction"); return false },
                };

                match instruction.operand_1 {
                    OperandType::Register { idx: op1 } => self.memory[self.registers[op1 as usize] as usize] = to_store,
                    OperandType::Literal { value: op1 } => self.memory[op1 as usize] = to_store,
                    OperandType::None => { self.invalid_instruction("Missing first operand for store instruction"); return false},
                }
            }
            Instructions::LOAD => {
                if let OperandType::Register { idx: op1 } = instruction.operand_1 {
                    match instruction.operand_2 {
                        OperandType::Register { idx: op2 } => self.registers[op1 as usize] = self.memory[self.registers[op2 as usize] as usize],
                        OperandType::Literal { value: op2 } => self.registers[op1 as usize] = self.memory[op2 as usize],
                        OperandType::None => self.invalid_instruction("Missing second operand for store instruction"),
                    }
                } else {
                    self.invalid_instruction("Missing first operand for store instruction");
                }
            }
            Instructions::ADD => {
                if let OperandType::Register { idx: op1 } = instruction.operand_1 {
                    match instruction.operand_2 {
                        OperandType::Register { idx: op2 } => self.registers[op1 as usize] += self.registers[op2 as usize],
                        OperandType::Literal { value: op2 } => self.registers[op1 as usize] += op2,
                        OperandType::None => {self.invalid_instruction("Missing second operand for add instruction"); return false},
                    }
                    next_flags = Self::update_flags(next_flags, self.registers[op1 as usize]);
                } else {
                    self.invalid_instruction("Missing first operand for add instruction");
                    return false;
                }
            }
            Instructions::SUB => {
                if let OperandType::Register { idx: op1 } = instruction.operand_1 {
                    match instruction.operand_2 {
                        OperandType::Register { idx: op2 } => self.registers[op1 as usize] -= self.registers[op2 as usize],
                        OperandType::Literal { value: op2 } => self.registers[op1 as usize] -= op2,
                        OperandType::None => {self.invalid_instruction("Missing second operand for sub instruction"); return false},
                    }
                    next_flags = Self::update_flags(next_flags, self.registers[op1 as usize]);
                } else {
                    self.invalid_instruction("Missing first operand for sub instruction");
                    return false;
                }
            }
            Instructions::MUL => {
                if let OperandType::Register { idx: op1 } = instruction.operand_1 {
                    match instruction.operand_2 {
                        OperandType::Register { idx: op2 } => self.registers[op1 as usize] *= self.registers[op2 as usize],
                        OperandType::Literal { value: op2 } => self.registers[op1 as usize] *= op2,
                        OperandType::None => {self.invalid_instruction("Missing second operand for mul instruction"); return false},
                    }
                    next_flags = Self::update_flags(next_flags, self.registers[op1 as usize]);
                } else {
                    self.invalid_instruction("Missing first operand for mul instruction");
                    return false;
                }
            }
            Instructions::DIV => {
                if let OperandType::Register { idx: op1 } = instruction.operand_1 {
                        match instruction.operand_2 {
                            OperandType::Register { idx: op2 } => self.registers[op1 as usize] /= self.registers[op2 as usize],
                            OperandType::Literal { value: op2 } => self.registers[op1 as usize] /= op2,
                            OperandType::None => {self.invalid_instruction("Missing second operand for div instruction"); return false},
                        }
                        next_flags = Self::update_flags(next_flags, self.registers[op1 as usize]);
                    } else {
                        self.invalid_instruction("Missing first operand for div instruction");
                        return false;
                    }
            }
            Instructions::CMP => {
                if let OperandType::Register { idx: op1 } = instruction.operand_1 {
                    match instruction.operand_2 {
                        OperandType::Register { idx: op2 } => next_flags = Self::update_flags(next_flags, self.registers[op1 as usize] - self.registers[op2 as usize]),
                        OperandType::Literal { value: op2 } => next_flags = Self::update_flags(next_flags, self.registers[op1 as usize] - op2),
                        OperandType::None => {self.invalid_instruction("Missing second operand for sub instruction"); return false},
                    }
                } else {
                    self.invalid_instruction("Missing first operand for sub instruction");
                    return false;
                }
            }
            Instructions::JMP => {
                next_jump = match instruction.operand_1 {
                    OperandType::Register { idx: op1 } => self.registers[op1 as usize],
                    OperandType::Literal { value: op1 } => op1,
                    OperandType::None => { self.invalid_instruction("Missing first operand for store instruction"); return false},
                };
            }
            Instructions::JZ => {
                if self.check_flag(Flags::ZeroFlag) {
                    next_jump = match instruction.operand_1 {
                        OperandType::Register { idx: op1 } => self.registers[op1 as usize],
                        OperandType::Literal { value: op1 } => op1,
                        OperandType::None => { self.invalid_instruction("Missing first operand for store instruction"); return false},
                    };
                }
            }
            Instructions::JNZ => {
                if !self.check_flag(Flags::ZeroFlag) {
                    next_jump = match instruction.operand_1 {
                        OperandType::Register { idx: op1 } => self.registers[op1 as usize],
                        OperandType::Literal { value: op1 } => op1,
                        OperandType::None => { self.invalid_instruction("Missing first operand for store instruction"); return false},
                    };
                }
            }
            Instructions::JN => {
                if self.check_flag(Flags::NegativeFlag) {
                    next_jump = match instruction.operand_1 {
                        OperandType::Register { idx: op1 } => self.registers[op1 as usize],
                        OperandType::Literal { value: op1 } => op1,
                        OperandType::None => { self.invalid_instruction("Missing first operand for store instruction"); return false},
                    };
                }
            }
            Instructions::JP => {
                if self.check_flag(Flags::PositiveFlag) {
                    next_jump = match instruction.operand_1 {
                        OperandType::Register { idx: op1 } => self.registers[op1 as usize],
                        OperandType::Literal { value: op1 } => op1,
                        OperandType::None => { self.invalid_instruction("Missing first operand for store instruction"); return false},
                    };
                }
            }
            Instructions::CALL => {}  // TODO: Implement context switching for function calls
            Instructions::RET => {
                next_jump = self.registers[Registers::PC as usize] - self.registers[Registers::FRP as usize];
            }
            Instructions::POP => {
                if let OperandType::Register { idx: op1 } = instruction.operand_1 {
                    self.registers[op1 as usize] = self.stack.pop().unwrap_or(0);
                } else if let OperandType::None = instruction.operand_1 {
                    // Just pop into void
                    self.stack.pop();
                } else {
                    self.invalid_instruction("Can't pop the stack into a literal");
                    return false;
                }
            }
            Instructions::PUSH => {
                match instruction.operand_1 {
                    OperandType::Register { idx: op1 } => self.stack.push(self.registers[op1 as usize]),
                    OperandType::Literal { value: op1 } => self.stack.push(op1),
                    OperandType::None => { self.invalid_instruction("Missing operand for push instruction"); return false; }
                }
            }
            Instructions::NOP => {}
        }

        self.flags = next_flags;
        self.registers[Registers::PC as usize] += next_jump;
        if self.registers[Registers::PC as usize] as usize >= instructions.len() {
            self.status = MachineStatus::Complete;
        }
        true
    }
}
