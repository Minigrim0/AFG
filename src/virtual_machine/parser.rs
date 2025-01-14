use core::fmt;

use super::{Instruction, Instructions, MemoryMappedProperties, Registers};
use super::errors::ParsingError;


fn parse_instr<S: AsRef<str>>(instr: S) -> Result<Instructions, String> {
    match instr.as_ref().to_lowercase().as_str() {
        "mov" => Ok(Instructions::MOV),
        "movi" => Ok(Instructions::MOVI),
        "store" => Ok(Instructions::STORE),
        "storei" => Ok(Instructions::STOREI),
        "load" => Ok(Instructions::LOAD),
        "loadi" => Ok(Instructions::LOADI),
        "add" => Ok(Instructions::ADD),
        "addi" => Ok(Instructions::ADDI),
        "sub" => Ok(Instructions::SUB),
        "subi" => Ok(Instructions::SUBI),
        "mul" => Ok(Instructions::MUL),
        "muli" => Ok(Instructions::MULI),
        "div" => Ok(Instructions::DIV),
        "divi" => Ok(Instructions::DIVI),
        "cmp" => Ok(Instructions::CMP),
        "cmpi" => Ok(Instructions::CMPI),
        "jmp" => Ok(Instructions::JMP),
        "jz" => Ok(Instructions::JZ),
        "jnz" => Ok(Instructions::JNZ),
        "jn" => Ok(Instructions::JN),
        "call" => Ok(Instructions::CALL),
        "calli" => Ok(Instructions::CALLI),
        "ret" => Ok(Instructions::RET),
        "pop" => Ok(Instructions::POP),
        "push" => Ok(Instructions::PUSH),
        _ => {
            Err(format!("Unknown instruction: {}", instr.as_ref()))
        }
    }
}

fn parse_operand<S: AsRef<str>>(operand: S) -> Result<i32, String> {
    match operand.as_ref().chars().next() {
        Some('$') => {
            println!("\tOperand is a special variable");
            match operand
                .as_ref()
                .chars()
                .skip(1)
                .collect::<String>()
                .as_str()
            {
                "VelocityX" => Ok(MemoryMappedProperties::VelocityX as i32),
                "VelocityY" => Ok(MemoryMappedProperties::VelocityY as i32),
                "Moment" => Ok(MemoryMappedProperties::Moment as i32),
                "Rotation" => Ok(MemoryMappedProperties::Rotation as i32),
                "PositionX" => Ok(MemoryMappedProperties::PositionX as i32),
                "PositionY" => Ok(MemoryMappedProperties::PositionY as i32),
                "Ray0Dist" => Ok(MemoryMappedProperties::Ray0Dist as i32),
                "Ray0Type" => Ok(MemoryMappedProperties::Ray0Type as i32),
                "Ray1Dist" => Ok(MemoryMappedProperties::Ray1Dist as i32),
                "Ray1Type" => Ok(MemoryMappedProperties::Ray1Type as i32),
                "Ray2Dist" => Ok(MemoryMappedProperties::Ray2Dist as i32),
                "Ray2Type" => Ok(MemoryMappedProperties::Ray2Type as i32),
                "Ray3Dist" => Ok(MemoryMappedProperties::Ray3Dist as i32),
                "Ray3Type" => Ok(MemoryMappedProperties::Ray3Type as i32),
                "Ray4Dist" => Ok(MemoryMappedProperties::Ray4Dist as i32),
                "Ray4Type" => Ok(MemoryMappedProperties::Ray4Type as i32),
                "Ray5Dist" => Ok(MemoryMappedProperties::Ray5Dist as i32),
                "Ray5Type" => Ok(MemoryMappedProperties::Ray5Type as i32),
                "Ray6Dist" => Ok(MemoryMappedProperties::Ray6Dist as i32),
                "Ray6Type" => Ok(MemoryMappedProperties::Ray6Type as i32),
                var => {
                    Err(format!("Unknown variable: {}", var))
                }
            }
        }
        Some('#') => {
            println!("\tOperand is a litteral");
            operand
                .as_ref()
                .chars()
                .skip(1)
                .collect::<String>()
                .parse::<i32>()
                .map_err(|e| {
                    format!("Unable to parse int : {}", e.to_string())
                })
        }
        Some('\'') => {
            println!("\tOperand is a register");
            match operand
                .as_ref()
                .chars()
                .skip(1)
                .collect::<String>()
                .as_str()
            {
                "GPA" => Ok(Registers::GPA as i32),
                "GPB" => Ok(Registers::GPB as i32),
                "GPC" => Ok(Registers::GPC as i32),
                "GPD" => Ok(Registers::GPD as i32),
                "FPA" => Ok(Registers::FPA as i32),
                "FPB" => Ok(Registers::FPB as i32),
                "FPC" => Ok(Registers::FPC as i32),
                "FPD" => Ok(Registers::FPD as i32),
                reg => {
                    Err(format!("Unknown register: {}", reg))
                }
            }
        }
        Some(_) => operand
            .as_ref()
            .chars()
            .skip(1)
            .collect::<String>()
            .parse::<i32>()
            .map_err(|e| format!("Unable to parse int : {}", e.to_string())),
        None => {
            Err("No operand to parse !".to_string())
        }
    }
}

pub fn parse<S: AsRef<str>>(text: S) -> Result<Vec<Instruction>, ParsingError> {
    let mut instructions = vec![];
    'main_loop: for (line_nbr, line) in text.as_ref().split("\n").enumerate() {
        println!("Working on line: {}", line);
        if line.chars().next() == Some(';') {
            println!("\tComment line, skipping");
            continue;
        }
        if line.len() == 0 {
            println!("\tEmpty line, skipping");
            continue;
        }

        let splitted_line = line.split(" ").collect::<Vec<&str>>();
        let mut splitted_line = splitted_line.iter();
        let instruction = Instruction {
            opcode: match splitted_line.next() {
                Some(instr) => match parse_instr(instr) {
                    Ok(instr) => instr,
                    Err(e) => return Err(ParsingError::new(line_nbr as u32, e))
                },
                None => {
                    println!("No intruction found for line '{}'", line);
                    break 'main_loop;
                }
            },
            operand_1: match splitted_line.next() {
                Some(op) => match parse_operand(op) {
                    Ok(op) => op,
                    Err(e) => return Err(ParsingError::new(line_nbr as u32, e))
                },
                None => return Err(ParsingError::new(line_nbr as u32, "Missing operand".to_string()))
            },
            operand_2: splitted_line
                .next()
                .and_then(|operand| parse_operand(operand).ok()),
        };
        instructions.push(instruction);
    }

    Ok(instructions)
}
