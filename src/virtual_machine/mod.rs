mod machine;
mod parser;
pub mod errors;
pub mod assets;

#[cfg(test)]
mod tests;

pub fn get_special_variables() -> Vec<String> {
    vec![
        "$PositionX".to_string(),
        "$PositionY".to_string(), // Read-only Vertical position
        "$Rotation".to_string(),  // Read-only Rotation
        "$Ray0Dist".to_string(),
        "$Ray0Type".to_string(),
        "$Ray1Dist".to_string(),
        "$Ray1Type".to_string(),
        "$Ray2Dist".to_string(),
        "$Ray2Type".to_string(),
        "$Ray3Dist".to_string(),
        "$Ray3Type".to_string(),
        "$Ray4Dist".to_string(),
        "$Ray4Type".to_string(),
        "$Ray5Dist".to_string(),
        "$Ray5Type".to_string(),
        "$Ray6Dist".to_string(),
        "$Ray6Type".to_string(),
        "$VelocityX".to_string(),
        "$VelocityY".to_string(),
        "$Moment".to_string(),
    ]
}

pub enum MemoryMappedProperties {
    // 0xFFF8 => Mask for Read-only properties (range 0xFF20 - 0xFFFF)
    PositionX = 0xffff, // Read-only Lateral position
    PositionY = 0xfffe, // Read-only Vertical position
    Rotation = 0xfffd,  // Read-only Rotation
    Ray0Dist = 0xfffc,
    Ray0Type = 0xfffb,
    Ray1Dist = 0xfffa,
    Ray1Type = 0xfff9,
    Ray2Dist = 0xfff8,
    Ray2Type = 0xfff7,
    Ray3Dist = 0xfff6,
    Ray3Type = 0xfff5,
    Ray4Dist = 0xfff4,
    Ray4Type = 0xfff3,
    Ray5Dist = 0xfff2,
    Ray5Type = 0xfff1,
    Ray6Dist = 0xfff0,
    Ray6Type = 0xffef,

    // 0xFFF0 => Mask for Writable properties (range 0xFFF0 - 0xFFF7)
    VelocityX = 0xff1f, // Writable Lateral velocity (right+/left-)
    VelocityY = 0xff1e, // Writable Vertical velocity (front+/back-)
    Moment = 0xff1d,    // Writable Moment (clockwise+/counterclockwise-)
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
    SP = 0x0A,  // Stack Pointer (used for function calls)
    RP = 0x0B,  // Return Pointer (used for function calls)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Instructions {
    NOP,    // No operation
    MOV,    // r<op1> = #r<op2>
    MOVI,   // r<op1> = #<op2>
    STORE,  // [#r<op1>] = #r<op2>
    STOREI, // [#r<op1>] = #<op2>
    LOAD,   // r<op1> = [#r<op2>]
    LOADI,  // r<op1> = [#<op2>]
    ADD,    // r<op1> = #r<op1> + #r<op2>
    ADDI,   // r<op1> = #r<op1> + #<op2>
    SUB,    // Subs into <Register <operand 1>> <Register <operand 2>>
    SUBI,   // Subs into <Register <operand 1>> #<operand 2>
    MUL,    // Mul into <Register <operand 1>> <Register <operand 2>>
    MULI,   // Mul into <Register <operand 1>> #<operand 2>
    DIV,    // r<op1> = #<r<op1>> / #<r<op2>>
    DIVI,   // r<op1> = #<r<op1>> / #<op2>
    CMP,    // Performs a comparison by subbing its two register operands, without saving the result, just changing the flags
    CMPI,    // Performs a comparison by subbing its register operands with an immediate value, without saving the result, just changing the flags
    JMP,    // Unconditional jump to instruction #<op1>
    JZ,     // Jump if previous operation resulted in 0
    JNZ,    // Jump if previous operation was not 0
    JN,     // Jump if previous operation was negative
    JP,     // Jump if previous operation was positive
    CALL,   // Call function at address #<r<op1>>  /!\ User is responsible for pushing and popping the stack
    RET,    // Returns from function call          /!\ User is responsible for pushing and popping the stack
    POP,    // Pops a value from the stack into <r<op1>>
    PUSH,   // Pushes to the stack the value of <r<op1>>
}

pub enum MachineStatus {
    Ready = 0x0,
    Running = 0x1,
    Dead = 0x2,
    Complete = 0x3,
}

pub use machine::*;
