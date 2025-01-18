use std::f32::consts::PI;
use std::fmt::Display;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::assets::Program;
use super::{Instructions, MachineStatus, MemoryMappedProperties, Registers};

enum Flags {
    ZeroFlag     = 0b00000001,
    OverflowFlag = 0b00000010,
    NegativeFlag = 0b00000100,
}

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

    fn check_valid_value<T>(&mut self, val: T, check: Box<dyn Fn(&T) -> (bool, String)>) -> Result<T, ()>
        where T: Display {
        if let (false, err) = check(&val) {
            println!("ERR: {}", format!("{}: {}", err, val));
            self.status = MachineStatus::Dead;
            Err(())
        } else {
            Ok(val)
        }
    }

    fn check_some_valid_value<T>(&mut self, val: Option<T>, check: Box<dyn Fn(&T) -> (bool, String)>) -> Result<T, ()>
        where T: Display {
        if let Some(val) = val {
            self.check_valid_value(val, check)
        } else {
            Err(())
        }
    }

    fn update_flags(flags: u8, value: i32) -> u8 {
        match value {
            0 => flags | Flags::ZeroFlag as u8,
            n if n < 0 => flags | Flags::NegativeFlag as u8,
            _ => flags
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
            println!("Unable to find program");
            // self.invalid_instruction("Could not find program");
            return false;
        };

        let instruction = instructions[self.registers[Registers::PC as usize] as usize];
        let mut next_jump: i32 = 1;
        let mut next_flags: u8 = 0;

        match instruction.opcode {
            Instructions::MOV => {
                if let (Ok(op1), Ok(op2)) = (
                    self.check_valid_value(instruction.operand_1, self.is_valid_register()),
                    self.check_some_valid_value(instruction.operand_2, self.is_valid_register())
                ) {
                    println!("MOV <{}>, <{}>", op1, op2);
                    self.registers[op1 as usize] = self.registers[op2 as usize];
                } else {
                    return false;
                }
            }
            Instructions::MOVI => {
                if let (Ok(op1), Ok(op2)) = (
                    self.check_valid_value(instruction.operand_1, self.is_valid_register()),
                    self.check_some_valid_value(instruction.operand_2, Box::from(|_: &i32| (true, "Uh oh unexpected".to_string())))
                ) {
                    println!("MOVI <{}>, ${}", op1, op2);
                    self.registers[op1 as usize] = op2;
                } else {
                    return false;
                }
            }
            Instructions::STORE => {
                if let (Ok(op1), Ok(op2)) = (
                    self.check_valid_value(instruction.operand_1, self.is_valid_register()),
                    self.check_some_valid_value(instruction.operand_2, self.is_valid_register())
                ) {
                    println!("STORE [<{}>], <{}>", op1, op2);
                    self.memory[self.registers[op1 as usize] as usize] =
                        self.registers[op2 as usize];
                } else {
                    return false;
                }
            }
            Instructions::STOREI => {
                if let (Ok(op1), Ok(op2)) = (
                    self.check_valid_value(instruction.operand_1, self.is_valid_register()),
                    self.check_some_valid_value(instruction.operand_2, Box::from(|_: &i32| (true, "Uh oh unexpected".to_string())))
                ) {
                    println!("STOREI [<{}>], ${}", op1, op2);
                    self.memory[self.registers[op1 as usize] as usize] = op2;
                } else {
                    return false;
                }
            }
            Instructions::LOAD => {
                if let (Ok(op1), Ok(op2)) = (
                    self.check_valid_value(instruction.operand_1, self.is_valid_register()),
                    self.check_some_valid_value(instruction.operand_2, self.is_valid_register())
                ) {
                    println!("LOAD <{}>, [<{}>]", op1, op2);
                    self.registers[op1 as usize] = self.memory[self.registers[op2 as usize] as usize];
                } else {
                    return false;
                }
            }
            Instructions::LOADI => {
                if let (Ok(op1), Ok(op2)) = (
                    self.check_valid_value(instruction.operand_1, self.is_valid_register()),
                    self.check_some_valid_value(instruction.operand_2, self.is_valid_memory_address())
                ) {
                    println!("LOADI <{}>, [#{}]", op1, op2);
                    self.registers[op1 as usize] = self.memory[op2 as usize];
                } else {
                    return false;
                }
            }
            Instructions::ADD => {
                if let (Ok(op1), Ok(op2)) = (
                    self.check_valid_value(instruction.operand_1, self.is_valid_register()),
                    self.check_some_valid_value(instruction.operand_2, self.is_valid_register())
                ) {
                    println!("ADD <{}>, <{}>", op1, op2);
                    self.registers[op1 as usize] += self.registers[op2 as usize];
                } else {
                    return false;
                }
            }
            Instructions::ADDI => {
                if let (Ok(op1), Ok(op2)) = (
                    self.check_valid_value(instruction.operand_1, self.is_valid_register()),
                    self.check_some_valid_value(instruction.operand_2, Box::from(|_: &i32| (true, "Uh oh unexpected".to_string())))
                ) {
                    println!("ADDI <{}>, #{}", op1, op2);
                    self.registers[op1 as usize] += op2;
                } else {
                    return false;
                }
            }
            Instructions::SUB => {
                if let (Ok(op1), Ok(op2)) = (
                    self.check_valid_value(instruction.operand_1, self.is_valid_register()),
                    self.check_some_valid_value(instruction.operand_2, self.is_valid_register())
                ) {
                    print!("SUB <{}>, <{}>", op1, op2);
                    self.registers[op1 as usize] -= self.registers[op2 as usize];
                    next_flags = Self::update_flags(next_flags, self.registers[op1 as usize]);
                    println!("flags: {:8b}", next_flags);
                } else {
                    return false;
                }
            }
            Instructions::SUBI => {
                if let (Ok(op1), Ok(op2)) = (
                    self.check_valid_value(instruction.operand_1, self.is_valid_register()),
                    self.check_some_valid_value(instruction.operand_2, Box::from(|_: &i32| (true, "Uh oh unexpected".to_string())))
                ) {
                    print!("SUBI <{}>, #{}", op1, op2);
                    self.registers[op1 as usize] -= op2;
                    next_flags = Self::update_flags(next_flags, self.registers[op1 as usize]);
                    println!("flags: {:8b}", next_flags);
                } else {
                    return false;
                }
            }
            Instructions::MUL => {}
            Instructions::MULI => {}
            Instructions::DIV => {}
            Instructions::DIVI => {}
            Instructions::CMP => {
                if let (Ok(op1), Ok(op2)) = (
                    self.check_valid_value(instruction.operand_1, self.is_valid_register()),
                    self.check_some_valid_value(instruction.operand_2, self.is_valid_register())
                ) {
                    print!("CMP <{}>, <{}>", op1, op2);
                    next_flags = Self::update_flags(next_flags, self.registers[op1 as usize] - self.registers[op2 as usize]);
                    println!("flags: {:8b}", next_flags);
                }
            }
            Instructions::CMPI => {
                if let (Ok(op1), Ok(op2)) = (
                    self.check_valid_value(instruction.operand_1, self.is_valid_register()),
                    self.check_some_valid_value(instruction.operand_2, Box::from(|_: &i32| (true, "Uh Oh unexpected".to_string())))
                ) {
                    print!("CMPI <{}>, #{}", op1, op2);
                    next_flags = Self::update_flags(next_flags, self.registers[op1 as usize] - op2);
                    println!("flags: {:8b}", next_flags);
                }
            }
            Instructions::JMP => {
                println!("JMP {}", instruction.operand_1);
                next_jump = instruction.operand_1;
            }
            Instructions::JZ => {
                if self.check_flag(Flags::ZeroFlag) {
                    println!("JZ {}", instruction.operand_1);
                    next_jump = instruction.operand_1;
                }
            }
            Instructions::JNZ => {}
            Instructions::JN => {
                if self.check_flag(Flags::NegativeFlag) {
                    println!("JN {}", instruction.operand_1);
                    next_jump = instruction.operand_1;
                }
            }
            Instructions::CALL => {}  // TODO: Implement context switching for function calls
            Instructions::RET => {}
            Instructions::POP => {}
            Instructions::PUSH => {}
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
