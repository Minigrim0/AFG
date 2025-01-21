use super::errors::ParsingError;
use super::{Instruction, Instructions, MemoryMappedProperties, Registers, OperandType};

fn parse_instr<S: AsRef<str>>(instr: S) -> Result<Instructions, String> {
    match instr.as_ref().to_lowercase().as_str() {
        "mov" => Ok(Instructions::MOV),
        "store" => Ok(Instructions::STORE),
        "load" => Ok(Instructions::LOAD),
        "add" => Ok(Instructions::ADD),
        "sub" => Ok(Instructions::SUB),
        "mul" => Ok(Instructions::MUL),
        "div" => Ok(Instructions::DIV),
        "cmp" => Ok(Instructions::CMP),
        "jmp" => Ok(Instructions::JMP),
        "jz" => Ok(Instructions::JZ),
        "jnz" => Ok(Instructions::JNZ),
        "jn" => Ok(Instructions::JN),
        "jp" => Ok(Instructions::JP),
        "call" => Ok(Instructions::CALL),
        "ret" => Ok(Instructions::RET),
        "pop" => Ok(Instructions::POP),
        "push" => Ok(Instructions::PUSH),
        _ => Err(format!("Unknown instruction: {}", instr.as_ref())),
    }
}

fn parse_operand<S: AsRef<str>>(operand: S) -> Result<OperandType, String> {
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
                "VelocityX" => Ok(OperandType::Literal { value: MemoryMappedProperties::VelocityX as i32 }),
                "VelocityY" => Ok(OperandType::Literal { value: MemoryMappedProperties::VelocityY as i32 }),
                "Moment" => Ok(OperandType::Literal { value: MemoryMappedProperties::Moment as i32 }),
                "Rotation" => Ok(OperandType::Literal { value: MemoryMappedProperties::Rotation as i32 }),
                "PositionX" => Ok(OperandType::Literal { value: MemoryMappedProperties::PositionX as i32 }),
                "PositionY" => Ok(OperandType::Literal { value: MemoryMappedProperties::PositionY as i32 }),
                "Ray0Dist" => Ok(OperandType::Literal { value: MemoryMappedProperties::Ray0Dist as i32 }),
                "Ray0Type" => Ok(OperandType::Literal { value: MemoryMappedProperties::Ray0Type as i32 }),
                "Ray1Dist" => Ok(OperandType::Literal { value: MemoryMappedProperties::Ray1Dist as i32 }),
                "Ray1Type" => Ok(OperandType::Literal { value: MemoryMappedProperties::Ray1Type as i32 }),
                "Ray2Dist" => Ok(OperandType::Literal { value: MemoryMappedProperties::Ray2Dist as i32 }),
                "Ray2Type" => Ok(OperandType::Literal { value: MemoryMappedProperties::Ray2Type as i32 }),
                "Ray3Dist" => Ok(OperandType::Literal { value: MemoryMappedProperties::Ray3Dist as i32 }),
                "Ray3Type" => Ok(OperandType::Literal { value: MemoryMappedProperties::Ray3Type as i32 }),
                "Ray4Dist" => Ok(OperandType::Literal { value: MemoryMappedProperties::Ray4Dist as i32 }),
                "Ray4Type" => Ok(OperandType::Literal { value: MemoryMappedProperties::Ray4Type as i32 }),
                "Ray5Dist" => Ok(OperandType::Literal { value: MemoryMappedProperties::Ray5Dist as i32 }),
                "Ray5Type" => Ok(OperandType::Literal { value: MemoryMappedProperties::Ray5Type as i32 }),
                "Ray6Dist" => Ok(OperandType::Literal { value: MemoryMappedProperties::Ray6Dist as i32 }),
                "Ray6Type" => Ok(OperandType::Literal { value: MemoryMappedProperties::Ray6Type as i32 }),
                var => Err(format!("Unknown variable: {}", var)),
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
                .map(|v| OperandType::Literal { value: v })
                .map_err(|e| format!("Unable to parse int : {}", e.to_string()))
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
                "GPA" => Ok(OperandType::Register { idx: Registers::GPA as i32 }),
                "GPB" => Ok(OperandType::Register { idx: Registers::GPB as i32 }),
                "GPC" => Ok(OperandType::Register { idx: Registers::GPC as i32 }),
                "GPD" => Ok(OperandType::Register { idx: Registers::GPD as i32 }),
                "FRP" => Ok(OperandType::Register { idx: Registers::FRP as i32 }),
                reg => Err(format!("Unknown register: {}", reg)),
            }
        }
        Some(_) => operand
            .as_ref()
            .chars()
            .skip(1)
            .collect::<String>()
            .parse::<i32>()
            .map(|v| OperandType::Literal { value: v })
            .map_err(|e| format!("Unable to parse int : {}", e.to_string())),
        None => Err("No operand to parse !".to_string()),
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
                    Err(e) => return Err(ParsingError::new(line_nbr as u32, e)),
                },
                None => {
                    println!("No intruction found for line '{}'", line);
                    break 'main_loop;
                }
            },
            operand_1: match splitted_line.next() {
                Some(op) => match parse_operand(op) {
                    Ok(op) => op,
                    Err(e) => return Err(ParsingError::new(line_nbr as u32, e)),
                },
                None => {
                    OperandType::None
                }
            },
            operand_2: splitted_line
                .next()
                .and_then(|operand| parse_operand(operand).ok())
                .unwrap_or(OperandType::None),
        };
        instructions.push(instruction);
    }

    Ok(instructions)
}
