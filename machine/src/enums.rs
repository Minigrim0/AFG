use std::fmt;

fn register_to_string(index: usize) -> String {
    match index {
        i if i == Registers::GPA as usize => "GPA".to_string(),
        i if i == Registers::GPB as usize => "GPB".to_string(),
        i if i == Registers::GPC as usize => "GPC".to_string(),
        i if i == Registers::GPD as usize => "GPD".to_string(),
        i if i == Registers::SBP as usize => "SBP".to_string(),
        i if i == Registers::TSP as usize => "TSP".to_string(),
        i if i == Registers::FRV as usize => "FRV".to_string(),
        i if i == Registers::CIP as usize => "CIP".to_string(),
        _ => "XXX".to_string(),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OperandType {
    Literal {
        value: i32,
    },
    Register {
        idx: usize,
    },
    StackValue {
        base_register: usize,
        addition: bool,
        offset: usize,
    },
    MemoryOffset {
        base_register: usize,
        addition: bool,
        offset_register: usize,
    },
    #[default]
    None,
}

impl fmt::Display for OperandType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OperandType::Literal { value } => write!(f, "#{}", value),
            OperandType::Register { idx } => write!(f, "'{}", register_to_string(*idx)),
            OperandType::MemoryOffset {
                base_register,
                addition,
                offset_register,
            } => {
                write!(
                    f,
                    "[{} {} {}]",
                    register_to_string(*base_register),
                    if *addition { '+' } else { '-' },
                    register_to_string(*offset_register)
                )
            }
            OperandType::StackValue {
                base_register,
                addition,
                offset,
            } => write!(
                f,
                "[{} {} {}]",
                register_to_string(*base_register),
                if *addition { '+' } else { '-' },
                offset
            ),
            OperandType::None => write!(f, ""),
        }
    }
}

pub enum MemoryMappedProperties {
    // 0xFFF8 => Mask for Read-only properties (range 0xFF20 - 0xFFFF)
    Position = 0xfffe, // Read-only Lateral position (position[1] is vertical)
    Rotation = 0xfffd, // Read-only Rotation

    RayDist = 0xff40, // Ray dist here and above (up to 32 rays)
    RayType = 0xff20, // Ray type here and above (up to 32 rays)

    Velocity = 0xff1e, // Writable Lateral velocity (right+/left-) Velocity[1] is forward/backward
    Moment = 0xff1d,   // Writable Moment (clockwise+/counterclockwise-)
}

/// The list of registers in the virtual machine.
/// The accumulator and parameter pointers are used to move actual data around, perform calculations, etc.
/// SBP is the stack base pointer, It defines in the current callee the base of the stack. From this, the first element is the previously push SBP, then the return address, and then eventual parameters
/// TSP is the top-stack pointer. It points to the top of the stack. It is increased each time a value is pushed on the stack and decreased upon each pop.
/// FRV is the register used to transfer return parameters from callee to caller.
/// CIP is the current instruction pointer. It is normally increased by one after each instruction except for branching instructions
pub enum Registers {
    GPA = 0, // Accumulator
    GPB = 1, // Parameter
    GPC = 2, // Base memory pointer
    GPD = 3, // Memory Offset Pointer
    SBP = 4, // Stack base pointer, defines the stack "scope" of the current function
    TSP = 5, // Stack Pointer, the current top of the stack
    FRV = 6, // Register containing function's return values
    CIP = 7, // Instruction pointer
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum OpCodes {
    MOV,   // r<op1> = #r<op2>
    STORE, // [#r<op1>] = #r<op2>
    LOAD,  // r<op1> = [#r<op2>]
    ADD,   // r<op1> = #r<op1> + #r<op2>
    SUB,   // Subs into <Register <operand 1>> <Register <operand 2>>
    MUL,   // Mul into <Register <operand 1>> <Register <operand 2>>
    DIV,   // r<op1> = #<r<op1>> / #<r<op2>>
    MOD,   // r<op1> = #<r<op1>> % #<r<op2>>
    CMP, // Performs a comparison by subbing its two register operands, without saving the result, just changing the flags
    JMP, // Unconditional jump to instruction #<op1>
    JZ,  // Jump if previous operation resulted in 0
    JNZ, // Jump if previous operation was not 0
    JN,  // Jump if previous operation was negative
    JP,  // Jump if previous operation was positive
    CALL, // Call function at address #<r<op1>>   /!\ User is responsible for pushing and popping the stack
    RET, // Returns from function call           /!\ User is responsible for pushing and popping the stack
    POP, // Pops a value from the stack into <r<op1>>
    PUSH, // Pushes to the stack the value of <r<op1>>
    PRINT, // Prints the value of <r<op1>> to the console
    HLT, // Halts the machine
}

/// Enum for the machine status
/// Empty: The machine is empty, no program is loaded
/// Ready: A program is loaded and the machine is ready to run
/// Running: The machine is currently running (At least one tick has happened)
/// Dead: The machine has encountered an error and is no longer running
/// Complete: The machine has finished running the program
#[derive(Default)]
pub enum MachineStatus {
    #[default]
    Empty = 0x0,
    Ready = 0x1,
    Running = 0x2,
    Dead = 0x3,
    Complete = 0x4,
}

impl fmt::Display for MachineStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            MachineStatus::Empty => write!(f, "Empty"),
            MachineStatus::Ready => write!(f, "Ready"),
            MachineStatus::Running => write!(f, "Running"),
            MachineStatus::Dead => write!(f, "Dead"),
            MachineStatus::Complete => write!(f, "Complete"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Flags {
    ZeroFlag = 0b00000001,
    OverflowFlag = 0b00000010,
    NegativeFlag = 0b00000100,
    PositiveFlag = 0b00001000,
}

impl Flags {
    pub fn iter() -> impl Iterator<Item = Flags> {
        [
            Flags::ZeroFlag,
            Flags::OverflowFlag,
            Flags::NegativeFlag,
            Flags::PositiveFlag,
        ]
        .iter()
        .copied()
    }

    pub fn to_string(&self) -> String {
        match self {
            Flags::ZeroFlag => "ZF".to_string(),
            Flags::OverflowFlag => "OF".to_string(),
            Flags::NegativeFlag => "NF".to_string(),
            Flags::PositiveFlag => "PF".to_string(),
        }
    }
}
