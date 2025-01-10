mod machine;

pub enum MemoryMappedProperties {
    // 0xFFF8 => Mask for Read-only properties (range 0xFFF8 - 0xFFFF)
    PositionX = 0xffff, // Read-only Lateral position
    PositionY = 0xfffe, // Read-only Vertical position
    Rotation = 0xfffd,  // Read-only Rotation

    // 0xFFF0 => Mask for Writable properties (range 0xFFF0 - 0xFFF7)
    VelocityX = 0xfff7, // Writable Lateral velocity (right+/left-)
    VelocityY = 0xfff6, // Writable Vertical velocity (front+/back-)
    MOMENT = 0xfff5,    // Writable Moment (clockwise+/counterclockwise-)
}

pub enum Registers {
    GPA = 0x00, // General Purpose
    GPB = 0x01, // General Purpose
    GPC = 0x02, // General Purpose
    GPD = 0x03, // General Purpose
    FPA = 0x04, // Function Parameter A
    FPB = 0x05, // Function Parameter B
    FPC = 0x06, // Function Parameter C
    FPD = 0x07, // Function Parameter D
    PC = 0x08,  // Program Counter
    MP = 0x09,  // Memory Pointer
    SP = 0x0A,  // Stack Pointer (used for function calls)
    RP = 0x0B,  // Return Pointer (used for function calls)
}

#[derive(Debug, Copy, Clone)]
pub enum Instructions {
    NOP = 0x00,   // No operation
    MOVRI = 0x01, // Moves data from one location to another
    MOVRR = 0x02, // Moves data from one location to another
    MOVRM = 0x03, // Moves data from one location to another
    MOVMR = 0x04, // Moves data from one location to another
    ADD = 0x05,   // Adds two numbers
    SUB = 0x06,
    MUL = 0x07,
    DIV = 0x08,
    JMP = 0x09,
    JZ = 0x0A,
    JNZ = 0x0B,
    CALL = 0x0C, // Call function
    RET = 0x0D,
    POP = 0x0E,
    PUSH = 0x0F,
}

pub enum MachineStatus {
    Ready = 0x0,
    Running = 0x1,
    Dead = 0x2,
    Complete = 0x3,
}

pub use machine::*;
