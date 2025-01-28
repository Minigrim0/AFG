use core::fmt;
use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::assets::Program;
use super::{Instructions, MachineStatus, MemoryMappedProperties, OperandType, Registers};

enum Flags {
    ZeroFlag = 0b00000001,
    _OverflowFlag = 0b00000010,
    NegativeFlag = 0b00000100,
    PositiveFlag = 0b00001000,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Instruction {
    pub opcode: Instructions,
    pub operand_1: OperandType,
    pub operand_2: OperandType,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {} {}", self.opcode, self.operand_1, self.operand_2)
    }
}

#[derive(Component)]
pub struct VirtualMachine {
    pub registers: [i32; 6],   // 5 registers
    pub stack: [i32; 256],     // 1kB of stack (each value on the stack is 4 bytes)
    pub flags: u8,             // CPU flags
    pub memory: [i32; 65536],  // 64KB of memory
    pub status: MachineStatus,
    program_handle: Handle<Program>,
}

impl VirtualMachine {
    pub fn new() -> VirtualMachine {
        let mut vm = VirtualMachine {
            registers: [0; 6],
            stack: [0; 256],
            flags: 0,
            memory: [0; 65536],
            program_handle: Handle::default(),
            status: MachineStatus::Ready,
        };

        // Stack pointer
        vm.registers[Registers::TSP as usize] = 255;
        vm
    }

    pub fn with_program(mut self, program: Handle<Program>) -> VirtualMachine {
        self.program_handle = program;
        self
    }

    fn check_flag(&self, flag: Flags) -> bool {
        self.flags & flag as u8 != 0
    }

    pub fn update_mmp(&mut self, transform: &mut Transform, vel: &mut Velocity) {
        let mut rotation_angle =
            transform.rotation.to_axis_angle().0.z * transform.rotation.to_axis_angle().1;

        // Keep angles between 0 and 2PI
        if rotation_angle < 0.0 {
            rotation_angle += 2.0 * PI;
        }

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
            (
                MemoryMappedProperties::Ray0Dist,
                MemoryMappedProperties::Ray0Type,
            ),
            (
                MemoryMappedProperties::Ray1Dist,
                MemoryMappedProperties::Ray1Type,
            ),
            (
                MemoryMappedProperties::Ray2Dist,
                MemoryMappedProperties::Ray2Type,
            ),
            (
                MemoryMappedProperties::Ray3Dist,
                MemoryMappedProperties::Ray3Type,
            ),
            (
                MemoryMappedProperties::Ray4Dist,
                MemoryMappedProperties::Ray4Type,
            ),
            (
                MemoryMappedProperties::Ray5Dist,
                MemoryMappedProperties::Ray5Type,
            ),
            (
                MemoryMappedProperties::Ray6Dist,
                MemoryMappedProperties::Ray6Type,
            ),
        ];

        for (ray_data, (dist_addr, type_addr)) in rays.iter().zip(ray_addr) {
            if let Some((_ent, dist)) = ray_data {
                self.memory[dist_addr as usize] = *dist as i32;
                self.memory[type_addr as usize] = 1;
            } else {
                self.memory[dist_addr as usize] = 0;
                self.memory[type_addr as usize] = 0;
            }
        }
    }

    fn invalid_instruction<S: AsRef<str>, R>(&mut self, msg: S) -> Result<R, String> {
        self.status = MachineStatus::Dead;
        Err(format!("FATAL: {}", msg.as_ref()))
    }

    fn update_flags(flags: u8, value: i32) -> u8 {
        match value {
            0 => flags | Flags::ZeroFlag as u8,
            n if n < 0 => flags | Flags::NegativeFlag as u8,
            _ => flags | Flags::PositiveFlag as u8,
        }
    }

    fn stack_index(&mut self, base_register: usize, addition: bool, offset: usize) -> Result<usize, String> {
        let res = if addition {
            self.registers[base_register] + offset as i32
        } else {
            self.registers[base_register] - offset as i32
        };
        if (res) < 0 {
            self.status = MachineStatus::Dead;
            Err("Stack underflow".to_string())
        } else {
            Ok(res as usize)
        }
    }

    pub fn get_current_instruction(&self, programs: &Res<Assets<Program>>) -> String {
        if let Some(program) = programs.get(&self.program_handle) {
            if let Some(inst) = program.instructions.get(self.registers[Registers::CIP as usize] as usize) {
                format!("{}", inst)
            } else {
                "nono".to_string()
            }
        } else {
            "nope".to_string()
        }
    }

    fn get_stack(&mut self, base_register: usize, addition: bool, offset: usize) -> Result<i32, String> {
        let stack_index: usize = self.stack_index(base_register, addition, offset + 1)?;  // Offset is incremented by one here because the stack pointer actually points one above the last value
        if let Some(value) = self.stack.get(stack_index) {
            println!("[{} {} {}] = {}",  base_register, if addition { '+' } else { '-' }, offset, value);
            Ok(*value)
        } else {
            self.status = MachineStatus::Dead;
            Err(format!("Unable to get stack value at index: {}", stack_index))
        }
    }

    /// Tries to push a new value on the stack, returns an error if a stack overflow happens
    fn push_stack(&mut self, value: i32) -> Result<(), String> {
        if self.registers[Registers::TSP as usize] - 1 < 0 {
            return Err("Stack overflow".to_string());
        }

        self.stack[self.registers[Registers::TSP as usize] as usize] = value;
        self.registers[Registers::TSP as usize] -= 1;

        Ok(())
    }

    /// Tries to pop a value from the stack, returns an error if a stack underflow happens
    fn pop_stack(&mut self) -> Result<i32, String> {
        if (self.registers[Registers::TSP as usize] + 1) as usize >= self.stack.len() {
            return Err("Stack underflow".to_string())
        }

        self.registers[Registers::TSP as usize] -= 1;
        let value = self.stack[self.registers[Registers::TSP as usize] as usize];

        Ok(value)
    }

    pub fn tick(&mut self, programs: &Res<Assets<Program>>) -> Result<(), String> {
        match self.status {
            MachineStatus::Dead | MachineStatus::Complete => {
                return Err("Machine is dead".to_string());
            }
            MachineStatus::Ready => {
                self.registers[Registers::CIP as usize] = 0i32;
                self.status = MachineStatus::Running;
            }
            _ => {}
        }

        let instructions = if let Some(program) = programs.get(&self.program_handle) {
            &program.instructions
        } else {
            // self.invalid_instruction("Could not find program");
            return Err("Unable to find program".to_string());
        };

        let instruction: &Instruction = if let Some(instruction) = instructions.get(self.registers[Registers::CIP as usize] as usize) {
            Ok(instruction)
        } else {
            Err(format!("Unable to fetch instruction at index {}", self.registers[Registers::CIP as usize] as usize))
        }?;

        let mut next_jump: i32 = 1;
        let mut next_flags: u8 = 0;

        match instruction.opcode {
            Instructions::MOV => {
                if let OperandType::Register { idx: op1 } = instruction.operand_1 {
                    match instruction.operand_2 {
                        OperandType::Register { idx: op2 } => {
                            self.registers[op1 as usize] = self.registers[op2 as usize]
                        }
                        OperandType::Literal { value: op2 } => self.registers[op1 as usize] = op2,
                        OperandType::StackValue { base_register, addition, offset } => self.registers[op1 as usize] = self.get_stack(base_register, addition, offset)?,
                        OperandType::None => self.invalid_instruction("Missing second operand for mov instruction")?
                    }
                } else {
                    self.invalid_instruction("Missing first operand for mov instruction")?
                }
            }
            Instructions::STORE => {
                let to_store = match instruction.operand_2 {
                    OperandType::Register { idx: op2 } => self.registers[op2 as usize],
                    OperandType::Literal { value: op2 } => op2,
                    OperandType::StackValue { base_register, addition, offset } => self.get_stack(base_register, addition, offset)?,
                    OperandType::None => self.invalid_instruction("Missing second operand for store instruction")?
                };

                match instruction.operand_1 {
                    OperandType::Register { idx: op1 } => {
                        self.memory[self.registers[op1 as usize] as usize] = to_store
                    }
                    OperandType::Literal { value: op1 } => self.memory[op1 as usize] = to_store,
                    OperandType::StackValue { base_register, addition, offset } => self.memory[self.get_stack(base_register, addition, offset)? as usize] = to_store,
                    OperandType::None => self.invalid_instruction("Missing first operand for store instruction")?
                }
            }
            Instructions::LOAD => {
                if let OperandType::Register { idx: op1 } = instruction.operand_1 {
                    match instruction.operand_2 {
                        OperandType::Register { idx: op2 } => {
                            self.registers[op1 as usize] =
                                self.memory[self.registers[op2 as usize] as usize]
                        }
                        OperandType::Literal { value: op2 } => {
                            self.registers[op1 as usize] = self.memory[op2 as usize]
                        }
                        OperandType::StackValue { base_register, addition, offset } => {
                            self.registers[op1 as usize] = self.memory[self.get_stack(base_register, addition, offset)? as usize]
                        }
                        OperandType::None => self.invalid_instruction("Missing second operand for store instruction")?
                    }
                } else {
                    self.invalid_instruction("Missing first operand for store instruction")?;
                }
            }
            Instructions::ADD => {
                if let OperandType::Register { idx: op1 } = instruction.operand_1 {
                    match instruction.operand_2 {
                        OperandType::Register { idx: op2 } => {
                            self.registers[op1 as usize] += self.registers[op2 as usize]
                        }
                        OperandType::Literal { value: op2 } => self.registers[op1 as usize] += op2,
                        OperandType::StackValue { base_register, addition, offset } =>  self.invalid_instruction("Cannot use stack operation as operand for arithmetic instruction")?,
                        OperandType::None => self.invalid_instruction("Missing second operand for add instruction")?
                    }
                    next_flags = Self::update_flags(next_flags, self.registers[op1 as usize]);
                } else {
                    self.invalid_instruction("Missing first operand for add instruction")?
                }
            }
            Instructions::SUB => {
                if let OperandType::Register { idx: op1 } = instruction.operand_1 {
                    match instruction.operand_2 {
                        OperandType::Register { idx: op2 } => {
                            self.registers[op1 as usize] -= self.registers[op2 as usize]
                        }
                        OperandType::Literal { value: op2 } => self.registers[op1 as usize] -= op2,
                        OperandType::StackValue { base_register, addition, offset } =>  self.invalid_instruction("Cannot use stack operation as operand for arithmetic instruction")?,
                        OperandType::None => self.invalid_instruction("Missing second operand for sub instruction")?
                    }
                    next_flags = Self::update_flags(next_flags, self.registers[op1 as usize]);
                } else {
                    self.invalid_instruction("Missing first operand for sub instruction")?
                }
            }
            Instructions::MUL => {
                if let OperandType::Register { idx: op1 } = instruction.operand_1 {
                    match instruction.operand_2 {
                        OperandType::Register { idx: op2 } => {
                            self.registers[op1 as usize] *= self.registers[op2 as usize]
                        }
                        OperandType::Literal { value: op2 } => self.registers[op1 as usize] *= op2,
                        OperandType::StackValue { base_register, addition, offset } =>  self.invalid_instruction("Cannot use stack operation as operand for arithmetic instruction")?,
                        OperandType::None => self.invalid_instruction("Missing second operand for mul instruction")?
                    }
                    next_flags = Self::update_flags(next_flags, self.registers[op1 as usize]);
                } else {
                    self.invalid_instruction("Missing first operand for mul instruction")?
                }
            }
            Instructions::DIV => {
                if let OperandType::Register { idx: op1 } = instruction.operand_1 {
                    match instruction.operand_2 {
                        OperandType::Register { idx: op2 } => {
                            self.registers[op1 as usize] /= self.registers[op2 as usize]
                        }
                        OperandType::Literal { value: op2 } => self.registers[op1 as usize] /= op2,
                        OperandType::StackValue { base_register, addition, offset } =>  self.invalid_instruction("Cannot use stack operation as operand for arithmetic instruction")?,
                        OperandType::None => self.invalid_instruction("Missing second operand for div instruction")?
                    }
                    next_flags = Self::update_flags(next_flags, self.registers[op1 as usize]);
                } else {
                    self.invalid_instruction("Missing first operand for div instruction")?
                }
            }
            Instructions::MOD => {
                if let OperandType::Register { idx: op1 } = instruction.operand_1 {
                    match instruction.operand_2 {
                        OperandType::Register { idx: op2 } => {
                            self.registers[op1 as usize] %= self.registers[op2 as usize]
                        }
                        OperandType::Literal { value: op2 } => self.registers[op1 as usize] %= op2,
                        OperandType::StackValue { base_register, addition, offset } =>  self.invalid_instruction("Cannot use stack operation as operand for arithmetic instruction")?,
                        OperandType::None => self.invalid_instruction("Missing second operand for mod instruction")?
                    }
                    next_flags = Self::update_flags(next_flags, self.registers[op1 as usize]);
                } else {
                    self.invalid_instruction("Missing first operand for mod instruction")?
                }
            }
            Instructions::CMP => {
                if let OperandType::Register { idx: op1 } = instruction.operand_1 {
                    match instruction.operand_2 {
                        OperandType::Register { idx: op2 } => {
                            next_flags = Self::update_flags(
                                next_flags,
                                self.registers[op1 as usize] - self.registers[op2 as usize],
                            )
                        }
                        OperandType::Literal { value: op2 } => {
                            next_flags =
                                Self::update_flags(next_flags, self.registers[op1 as usize] - op2)
                        }
                        OperandType::StackValue { base_register, addition, offset } => self.invalid_instruction("Cannot use stack operation as operand for comparison instruction")?,
                        OperandType::None => self.invalid_instruction("Missing second operand for sub instruction")?
                    }
                } else {
                    self.invalid_instruction("Missing first operand for sub instruction")?
                }
            }
            Instructions::JMP => {
                next_jump = match instruction.operand_1 {
                    OperandType::Register { idx: op1 } => self.registers[op1 as usize],
                    OperandType::Literal { value: op1 } => op1,
                    OperandType::StackValue { base_register, addition, offset } => self.get_stack(base_register, addition, offset)?,
                    OperandType::None => self.invalid_instruction("Missing first operand for store instruction")?
                };
            }
            Instructions::JZ => {
                if self.check_flag(Flags::ZeroFlag) {
                    next_jump = match instruction.operand_1 {
                        OperandType::Register { idx: op1 } => self.registers[op1 as usize],
                        OperandType::Literal { value: op1 } => op1,
                        OperandType::StackValue { base_register, addition, offset } => self.get_stack(base_register, addition, offset)?,
                        OperandType::None => self.invalid_instruction("Missing first operand for store instruction")?
                    };
                }
            }
            Instructions::JNZ => {
                if !self.check_flag(Flags::ZeroFlag) {
                    next_jump = match instruction.operand_1 {
                        OperandType::Register { idx: op1 } => self.registers[op1 as usize],
                        OperandType::Literal { value: op1 } => op1,
                        OperandType::StackValue { base_register, addition, offset } => self.get_stack(base_register, addition, offset)?,
                        OperandType::None => self.invalid_instruction("Missing first operand for store instruction")?
                    };
                }
            }
            Instructions::JN => {
                if self.check_flag(Flags::NegativeFlag) {
                    next_jump = match instruction.operand_1 {
                        OperandType::Register { idx: op1 } => self.registers[op1 as usize],
                        OperandType::Literal { value: op1 } => op1,
                        OperandType::StackValue { base_register, addition, offset } => self.get_stack(base_register, addition, offset)?,
                        OperandType::None => self.invalid_instruction("Missing first operand for store instruction")?
                    };
                }
            }
            Instructions::JP => {
                if self.check_flag(Flags::PositiveFlag) {
                    next_jump = match instruction.operand_1 {
                        OperandType::Register { idx: op1 } => self.registers[op1 as usize],
                        OperandType::Literal { value: op1 } => op1,
                        OperandType::StackValue { base_register, addition, offset } => self.get_stack(base_register, addition, offset)?,
                        OperandType::None => self.invalid_instruction("Missing first operand for store instruction")?
                    };
                }
            }
            Instructions::CALL => {
                // Glorified JMP
                next_jump = match instruction.operand_1 {
                    OperandType::Register { idx: op1 } => self.registers[op1 as usize],
                    OperandType::Literal { value: op1 } => op1,
                    OperandType::StackValue { base_register, addition, offset } => self.get_stack(base_register, addition, offset)?,
                    OperandType::None => self.invalid_instruction("Missing first operand for store instruction")?
                };
                self.push_stack(self.registers[Registers::CIP as usize] + 1)?;
            }
            Instructions::RET => {
                let rp = self.pop_stack()?;
                next_jump = rp - self.registers[Registers::CIP as usize];
            }
            Instructions::POP => {
                match instruction.operand_1 {
                    OperandType::Register { idx: op1 } => self.registers[op1 as usize] = self.pop_stack()?,
                    OperandType::None => {let _ = self.pop_stack()?;},
                    _ => self.invalid_instruction("Can't pop the stack into the stack or into a literal")?
                }
            }
            Instructions::PUSH => match instruction.operand_1 {
                OperandType::Register { idx: op1 } => self.push_stack(self.registers[op1 as usize])?,
                OperandType::Literal { value: op1 } => self.push_stack(op1)?,
                _ => self.invalid_instruction("Can't push a value from the stack onto the stack or no value")?
            },
            Instructions::PRINT => {
                if let OperandType::Register { idx: op1 } = instruction.operand_1 {
                    println!("PRINT {}", self.registers[op1 as usize]);
                } else {
                    self.invalid_instruction("Missing operand for print instruction")?
                }
            }
        }

        self.flags = next_flags;
        self.registers[Registers::CIP as usize] += next_jump;
        if self.registers[Registers::CIP as usize] as usize >= instructions.len() {
            self.status = MachineStatus::Complete;
        }
        Ok(())
    }
}
