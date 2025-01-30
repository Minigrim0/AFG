use super::enums::{Flags, MachineStatus, OpCodes, OperandType, Registers};
use crate::Instruction;

#[cfg_attr(feature = "bevy", derive(bevy::prelude::Component))]
pub struct VirtualMachine {
    registers: [i32; 6],  // 5 registers
    stack: [i32; 256],    // 1kB of stack (each value on the stack is 4 bytes)
    flags: u8,            // CPU flags
    next_flags: u8,       // CPU flags at next instruction
    memory: [i32; 65536], // 64KB of memory
    status: MachineStatus,
    program: Option<Vec<Instruction>>,
    current_output: Option<String>,
}

impl Default for VirtualMachine {
    fn default() -> Self {
        Self {
            registers: [0; 6],  // 5 registers
            stack: [0; 256],    // 1kB of stack (each value on the stack is 4 bytes)
            flags: 0,           // CPU flags
            next_flags: 0,      // CPU flags at next instruction
            memory: [0; 65536], // 64KB of memory
            status: MachineStatus::Empty,
            program: None,
            current_output: None,
        }
    }
}

impl VirtualMachine {
    pub fn new() -> VirtualMachine {
        let mut vm = VirtualMachine::default();

        // Stack pointer
        vm.registers[Registers::TSP as usize] = vm.stack.len() as i32;
        vm
    }

    pub fn with_program(mut self, program: Vec<Instruction>) -> VirtualMachine {
        self.program = Some(program);
        self.status = MachineStatus::Ready;
        self
    }

    pub fn get_status(&self) -> String {
        match self.status {
            MachineStatus::Empty => "Empty".to_string(),
            MachineStatus::Ready => "Ready".to_string(),
            MachineStatus::Running => "Running".to_string(),
            MachineStatus::Dead => "Dead".to_string(),
            MachineStatus::Complete => "Complete".to_string(),
        }
    }

    /// Checks if a flag is currently set.
    fn check_flag(&self, flag: Flags) -> bool {
        self.flags & flag as u8 != 0
    }

    /// Checks if a flag is currently set.
    fn check_next_flag(&self, flag: Flags) -> bool {
        self.next_flags & flag as u8 != 0
    }

    fn update_flags(&mut self, value: i32) {
        self.next_flags = match value {
            0 => self.next_flags | Flags::ZeroFlag as u8,
            n if n < 0 => self.next_flags | Flags::NegativeFlag as u8,
            _ => self.next_flags | Flags::PositiveFlag as u8,
        };
    }

    pub fn has_completed(&self) -> bool {
        matches!(self.status, MachineStatus::Complete)
    }

    pub fn get_flags(&self) -> Vec<(String, String, String)> {
        Flags::iter()
            .map(|f| {
                (
                    f.to_string(),
                    if self.check_flag(f) {
                        "t".to_string()
                    } else {
                        "f".to_string()
                    },
                    if self.check_next_flag(f) {
                        "t".to_string()
                    } else {
                        "f".to_string()
                    },
                )
            })
            .collect()
    }

    // Update memory mapped properties to reflect the bot's sensors & react to the program's instructions
    #[cfg(feature = "bevy")]
    pub fn update_mmp(
        &mut self,
        transform: &mut bevy::prelude::Transform,
        vel: &mut bevy_rapier2d::prelude::Velocity,
    ) {
        use super::enums::MemoryMappedProperties;
        use std::f32::consts::PI;

        use bevy::prelude::*;

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
    }

    /// Updates the rays values in memory
    #[cfg(feature = "bevy")]
    pub fn update_rays(&mut self, rays: Vec<Option<(bevy::prelude::Entity, f32)>>) {
        use super::enums::MemoryMappedProperties;

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
        Err(format!(
            "FATAL: {} (CIP: {})",
            msg.as_ref(),
            self.registers[Registers::CIP as usize]
        ))
    }

    fn stack_index(
        &mut self,
        base_register: usize,
        addition: bool,
        offset: usize,
    ) -> Result<usize, String> {
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

    pub fn get_instruction_slice(&self, offset: usize, amount: usize) -> Vec<(usize, Instruction)> {
        if let Some(program) = &self.program {
            program
                .iter()
                .skip(offset) // Take five instructions before the offset
                .take(amount) // Take the needed amount
                .enumerate()
                .map(|(idx, i)| (idx + offset, i.clone()))
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn get_stack_slice(&self, offset: usize, amount: usize) -> Vec<(usize, i32)> {
        self.stack
            .iter()
            .rev()
            .skip(offset)
            .take(amount)
            .enumerate()
            .map(|(idx, i)| (self.stack.len() - 1 - idx - offset, *i))
            .collect()
    }

    pub fn get_register(&self, register: usize) -> i32 {
        if register >= self.registers.len() {
            return 0;
        }
        self.registers[register]
    }

    pub fn get_registers(&self) -> [(String, i32); 6] {
        [
            ("GPA".to_string(), self.registers[Registers::GPA as usize]),
            ("GPB".to_string(), self.registers[Registers::GPB as usize]),
            ("SBP".to_string(), self.registers[Registers::SBP as usize]),
            ("TSP".to_string(), self.registers[Registers::TSP as usize]),
            ("FRV".to_string(), self.registers[Registers::FRV as usize]),
            ("CIP".to_string(), self.registers[Registers::CIP as usize]),
        ]
    }

    pub fn get_current_instruction(&self) -> Option<Instruction> {
        if let Some(program) = &self.program {
            if let Some(inst) = program.get(self.registers[Registers::CIP as usize] as usize) {
                Some(inst.clone())
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_stack_frame(&self) -> String {
        let mut stack_frame = String::new();
        for i in self.registers[Registers::TSP as usize]..self.registers[Registers::SBP as usize] {
            stack_frame.push_str(&format!("[{}] = {}\n", i, self.stack[i as usize]));
        }
        stack_frame
    }

    pub fn get_cip(&self) -> i32 {
        self.registers[Registers::CIP as usize]
    }

    pub fn get_registers_display(&self) -> String {
        format!(
            "GPA: {:5}\nGPB: {:5}\nSBP: {:5}\nTSP: {:5}\nFRV: {:5}\nCIP: {:5}",
            self.registers[Registers::GPA as usize],
            self.registers[Registers::GPB as usize],
            self.registers[Registers::SBP as usize],
            self.registers[Registers::TSP as usize],
            self.registers[Registers::FRV as usize],
            self.registers[Registers::CIP as usize]
        )
    }

    fn get_stack(
        &mut self,
        base_register: usize,
        addition: bool,
        offset: usize,
    ) -> Result<i32, String> {
        let stack_index: usize = self.stack_index(base_register, addition, offset)?; // Offset is incremented by one here because the stack pointer actually points one above the last value
        if let Some(value) = self.stack.get(stack_index) {
            Ok(*value)
        } else {
            self.status = MachineStatus::Dead;
            Err(format!(
                "Unable to get stack value at index: {}",
                stack_index
            ))
        }
    }

    fn set_stack(
        &mut self,
        base_register: usize,
        addition: bool,
        offset: usize,
        value: i32,
    ) -> Result<(), String> {
        let stack_index: usize = self.stack_index(base_register, addition, offset)?; // Offset is incremented by one here because the stack pointer actually points one above the last value
        if stack_index < self.stack.len() {
            self.stack[stack_index] = value;
            Ok(())
        } else {
            self.status = MachineStatus::Dead;
            Err(format!(
                "Unable to set stack value at index: {}",
                stack_index
            ))
        }
    }

    /// Tries to push a new value on the stack, returns an error if a stack overflow happens
    fn push_stack(&mut self, value: i32) -> Result<(), String> {
        if self.registers[Registers::TSP as usize] - 1 < 0 {
            return Err("Stack overflow".to_string());
        }

        self.registers[Registers::TSP as usize] -= 1;
        self.stack[self.registers[Registers::TSP as usize] as usize] = value;

        Ok(())
    }

    /// Tries to pop a value from the stack, returns an error if a stack underflow happens
    fn pop_stack(&mut self) -> Result<i32, String> {
        if (self.registers[Registers::TSP as usize] + 1) as usize >= self.stack.len() {
            return Err("Stack underflow".to_string());
        }

        let value = self.stack[self.registers[Registers::TSP as usize] as usize];
        self.registers[Registers::TSP as usize] += 1;

        Ok(value)
    }

    pub fn get_current_output(&self) -> Option<String> {
        self.current_output.clone()
    }

    pub fn tick(&mut self) -> Result<(), String> {
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

        let instruction: Instruction = if let Some(instruction) = self.get_current_instruction() {
            Ok(instruction)
        } else {
            Err(format!(
                "Unable to fetch instruction at index {}",
                self.registers[Registers::CIP as usize] as usize
            ))
        }?;

        let mut next_jump: i32 = 1;
        self.current_output = None;

        match instruction.opcode {
            OpCodes::MOV => {
                let to_store = match instruction.operand_2 {
                    OperandType::Register { idx: op2 } => self.registers[op2 as usize],
                    OperandType::Literal { value: op2 } => op2,
                    OperandType::StackValue {
                        base_register,
                        addition,
                        offset,
                    } => self.get_stack(base_register, addition, offset)?,
                    OperandType::None => {
                        self.invalid_instruction("Missing second operand for mov instruction")?
                    }
                };

                if let OperandType::Register { idx: op1 } = instruction.operand_1 {
                    self.registers[op1 as usize] = to_store;
                } else if let OperandType::StackValue {
                    base_register,
                    addition,
                    offset,
                } = instruction.operand_1
                {
                    self.set_stack(base_register as usize, addition, offset, to_store)?;
                } else {
                    self.invalid_instruction("Missing first operand for mov instruction")?
                }
            }
            OpCodes::STORE => {
                let to_store = match instruction.operand_2 {
                    OperandType::Register { idx: op2 } => self.registers[op2 as usize],
                    OperandType::Literal { value: op2 } => op2,
                    OperandType::StackValue {
                        base_register,
                        addition,
                        offset,
                    } => self.get_stack(base_register, addition, offset)?,
                    OperandType::None => {
                        self.invalid_instruction("Missing second operand for store instruction")?
                    }
                };

                match instruction.operand_1 {
                    OperandType::Register { idx: op1 } => {
                        self.memory[self.registers[op1 as usize] as usize] = to_store
                    }
                    OperandType::Literal { value: op1 } => self.memory[op1 as usize] = to_store,
                    OperandType::StackValue {
                        base_register,
                        addition,
                        offset,
                    } => {
                        self.memory[self.get_stack(base_register, addition, offset)? as usize] =
                            to_store
                    }
                    OperandType::None => {
                        self.invalid_instruction("Missing first operand for store instruction")?
                    }
                }
            }
            OpCodes::LOAD => {
                if let OperandType::Register { idx: op1 } = instruction.operand_1 {
                    match instruction.operand_2 {
                        OperandType::Register { idx: op2 } => {
                            self.registers[op1 as usize] =
                                self.memory[self.registers[op2 as usize] as usize]
                        }
                        OperandType::Literal { value: op2 } => {
                            self.registers[op1 as usize] = self.memory[op2 as usize]
                        }
                        OperandType::StackValue {
                            base_register,
                            addition,
                            offset,
                        } => {
                            self.registers[op1 as usize] = self.memory
                                [self.get_stack(base_register, addition, offset)? as usize]
                        }
                        OperandType::None => self
                            .invalid_instruction("Missing second operand for store instruction")?,
                    }
                } else {
                    self.invalid_instruction("Missing first operand for store instruction")?;
                }
            }
            OpCodes::ADD => {
                if let OperandType::Register { idx: op1 } = instruction.operand_1 {
                    match instruction.operand_2 {
                        OperandType::Register { idx: op2 } => {
                            self.registers[op1 as usize] += self.registers[op2 as usize]
                        }
                        OperandType::Literal { value: op2 } => self.registers[op1 as usize] += op2,
                        OperandType::StackValue {
                            base_register: _,
                            addition: _,
                            offset: _,
                        } => self.invalid_instruction(
                            "Cannot use stack operation as operand for arithmetic instruction",
                        )?,
                        OperandType::None => {
                            self.invalid_instruction("Missing second operand for add instruction")?
                        }
                    }
                    self.update_flags(self.registers[op1 as usize]);
                } else {
                    self.invalid_instruction("Missing first operand for add instruction")?
                }
            }
            OpCodes::SUB => {
                if let OperandType::Register { idx: op1 } = instruction.operand_1 {
                    match instruction.operand_2 {
                        OperandType::Register { idx: op2 } => {
                            self.registers[op1 as usize] -= self.registers[op2 as usize]
                        }
                        OperandType::Literal { value: op2 } => self.registers[op1 as usize] -= op2,
                        OperandType::StackValue {
                            base_register: _,
                            addition: _,
                            offset: _,
                        } => self.invalid_instruction(
                            "Cannot use stack operation as operand for arithmetic instruction",
                        )?,
                        OperandType::None => {
                            self.invalid_instruction("Missing second operand for sub instruction")?
                        }
                    }
                    self.update_flags(self.registers[op1 as usize]);
                } else {
                    self.invalid_instruction("Missing first operand for sub instruction")?
                }
            }
            OpCodes::MUL => {
                if let OperandType::Register { idx: op1 } = instruction.operand_1 {
                    match instruction.operand_2 {
                        OperandType::Register { idx: op2 } => {
                            self.registers[op1 as usize] *= self.registers[op2 as usize]
                        }
                        OperandType::Literal { value: op2 } => self.registers[op1 as usize] *= op2,
                        OperandType::StackValue {
                            base_register: _,
                            addition: _,
                            offset: _,
                        } => self.invalid_instruction(
                            "Cannot use stack operation as operand for arithmetic instruction",
                        )?,
                        OperandType::None => {
                            self.invalid_instruction("Missing second operand for mul instruction")?
                        }
                    }
                    self.update_flags(self.registers[op1 as usize]);
                } else {
                    self.invalid_instruction("Missing first operand for mul instruction")?
                }
            }
            OpCodes::DIV => {
                if let OperandType::Register { idx: op1 } = instruction.operand_1 {
                    match instruction.operand_2 {
                        OperandType::Register { idx: op2 } => {
                            self.registers[op1 as usize] /= self.registers[op2 as usize]
                        }
                        OperandType::Literal { value: op2 } => self.registers[op1 as usize] /= op2,
                        OperandType::StackValue {
                            base_register: _,
                            addition: _,
                            offset: _,
                        } => self.invalid_instruction(
                            "Cannot use stack operation as operand for arithmetic instruction",
                        )?,
                        OperandType::None => {
                            self.invalid_instruction("Missing second operand for div instruction")?
                        }
                    }
                    self.update_flags(self.registers[op1 as usize]);
                } else {
                    self.invalid_instruction("Missing first operand for div instruction")?
                }
            }
            OpCodes::MOD => {
                if let OperandType::Register { idx: op1 } = instruction.operand_1 {
                    match instruction.operand_2 {
                        OperandType::Register { idx: op2 } => {
                            self.registers[op1 as usize] %= self.registers[op2 as usize]
                        }
                        OperandType::Literal { value: op2 } => self.registers[op1 as usize] %= op2,
                        OperandType::StackValue {
                            base_register: _,
                            addition: _,
                            offset: _,
                        } => self.invalid_instruction(
                            "Cannot use stack operation as operand for arithmetic instruction",
                        )?,
                        OperandType::None => {
                            self.invalid_instruction("Missing second operand for mod instruction")?
                        }
                    }
                    self.update_flags(self.registers[op1 as usize]);
                } else {
                    self.invalid_instruction("Missing first operand for mod instruction")?
                }
            }
            OpCodes::CMP => {
                if let OperandType::Register { idx: op1 } = instruction.operand_1 {
                    match instruction.operand_2 {
                        OperandType::Register { idx: op2 } => {
                            self.update_flags(
                                self.registers[op1 as usize] - self.registers[op2 as usize],
                            );
                        }
                        OperandType::Literal { value: op2 } => {
                            self.update_flags(self.registers[op1 as usize] - op2);
                        }
                        OperandType::StackValue {
                            base_register: _,
                            addition: _,
                            offset: _,
                        } => self.invalid_instruction(
                            "Cannot use stack operation as operand for comparison instruction",
                        )?,
                        OperandType::None => {
                            self.invalid_instruction("Missing second operand for sub instruction")?
                        }
                    }
                } else {
                    self.invalid_instruction("Missing first operand for sub instruction")?
                }
            }
            OpCodes::JMP => {
                next_jump = match instruction.operand_1 {
                    OperandType::Register { idx: op1 } => self.registers[op1 as usize],
                    OperandType::Literal { value: op1 } => op1,
                    OperandType::StackValue {
                        base_register,
                        addition,
                        offset,
                    } => self.get_stack(base_register, addition, offset)?,
                    OperandType::None => {
                        self.invalid_instruction("Missing first operand for store instruction")?
                    }
                };
            }
            OpCodes::JZ => {
                if self.check_flag(Flags::ZeroFlag) {
                    next_jump = match instruction.operand_1 {
                        OperandType::Register { idx: op1 } => self.registers[op1 as usize],
                        OperandType::Literal { value: op1 } => op1,
                        OperandType::StackValue {
                            base_register,
                            addition,
                            offset,
                        } => self.get_stack(base_register, addition, offset)?,
                        OperandType::None => {
                            self.invalid_instruction("Missing first operand for store instruction")?
                        }
                    };
                }
            }
            OpCodes::JNZ => {
                if !self.check_flag(Flags::ZeroFlag) {
                    next_jump = match instruction.operand_1 {
                        OperandType::Register { idx: op1 } => self.registers[op1 as usize],
                        OperandType::Literal { value: op1 } => op1,
                        OperandType::StackValue {
                            base_register,
                            addition,
                            offset,
                        } => self.get_stack(base_register, addition, offset)?,
                        OperandType::None => {
                            self.invalid_instruction("Missing first operand for store instruction")?
                        }
                    };
                }
            }
            OpCodes::JN => {
                if self.check_flag(Flags::NegativeFlag) {
                    next_jump = match instruction.operand_1 {
                        OperandType::Register { idx: op1 } => self.registers[op1 as usize],
                        OperandType::Literal { value: op1 } => op1,
                        OperandType::StackValue {
                            base_register,
                            addition,
                            offset,
                        } => self.get_stack(base_register, addition, offset)?,
                        OperandType::None => {
                            self.invalid_instruction("Missing first operand for store instruction")?
                        }
                    };
                }
            }
            OpCodes::JP => {
                if self.check_flag(Flags::PositiveFlag) {
                    next_jump = match instruction.operand_1 {
                        OperandType::Register { idx: op1 } => self.registers[op1 as usize],
                        OperandType::Literal { value: op1 } => op1,
                        OperandType::StackValue {
                            base_register,
                            addition,
                            offset,
                        } => self.get_stack(base_register, addition, offset)?,
                        OperandType::None => {
                            self.invalid_instruction("Missing first operand for store instruction")?
                        }
                    };
                }
            }
            OpCodes::CALL => {
                // Glorified JMP
                next_jump = match instruction.operand_1 {
                    OperandType::Register { idx: op1 } => self.registers[op1 as usize],
                    OperandType::Literal { value: op1 } => op1,
                    OperandType::StackValue {
                        base_register,
                        addition,
                        offset,
                    } => self.get_stack(base_register, addition, offset)?,
                    OperandType::None => {
                        self.invalid_instruction("Missing first operand for store instruction")?
                    }
                };
                self.push_stack(self.registers[Registers::CIP as usize] + 1)?;
            }
            OpCodes::RET => {
                let rp = self.pop_stack()?;
                next_jump = rp - self.registers[Registers::CIP as usize];
            }
            OpCodes::POP => match instruction.operand_1 {
                OperandType::Register { idx: op1 } => {
                    self.registers[op1 as usize] = self.pop_stack()?
                }
                OperandType::None => {
                    let _ = self.pop_stack()?;
                }
                _ => self
                    .invalid_instruction("Can't pop the stack into the stack or into a literal")?,
            },
            OpCodes::PUSH => match instruction.operand_1 {
                OperandType::Register { idx: op1 } => {
                    self.push_stack(self.registers[op1 as usize])?
                }
                OperandType::Literal { value: op1 } => self.push_stack(op1)?,
                _ => self.invalid_instruction(
                    "Can't push a value from the stack onto the stack or no value",
                )?,
            },
            OpCodes::PRINT => {
                if let OperandType::Register { idx: op1 } = instruction.operand_1 {
                    self.current_output = Some(format!("{}", self.registers[op1 as usize]));
                } else {
                    self.invalid_instruction("Missing operand for print instruction")?
                }
            }
        }

        self.flags = self.next_flags;
        self.next_flags = 0;
        self.registers[Registers::CIP as usize] += next_jump;
        if self.registers[Registers::CIP as usize] as usize
            >= self
                .program
                .as_ref()
                .and_then(|p| Some(p.len()))
                .unwrap_or(0)
        {
            self.status = MachineStatus::Complete;
        }
        Ok(())
    }
}
