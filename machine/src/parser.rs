use super::enums::{MemoryMappedProperties, OpCodes, OperandType, Registers};
use super::errors::ParsingError;
use super::Instruction;

fn parse_instr<S: AsRef<str>>(instr: S) -> Result<OpCodes, String> {
    match instr.as_ref().to_lowercase().as_str() {
        "mov" => Ok(OpCodes::MOV),
        "store" => Ok(OpCodes::STORE),
        "load" => Ok(OpCodes::LOAD),
        "add" => Ok(OpCodes::ADD),
        "sub" => Ok(OpCodes::SUB),
        "mul" => Ok(OpCodes::MUL),
        "div" => Ok(OpCodes::DIV),
        "mod" => Ok(OpCodes::MOD),
        "cmp" => Ok(OpCodes::CMP),
        "jmp" => Ok(OpCodes::JMP),
        "jz" => Ok(OpCodes::JZ),
        "jnz" => Ok(OpCodes::JNZ),
        "jn" => Ok(OpCodes::JN),
        "jp" => Ok(OpCodes::JP),
        "call" => Ok(OpCodes::CALL),
        "ret" => Ok(OpCodes::RET),
        "pop" => Ok(OpCodes::POP),
        "push" => Ok(OpCodes::PUSH),
        "print" => Ok(OpCodes::PRINT),
        "halt" => Ok(OpCodes::HLT),
        _ => Err(format!("Unknown instruction: {}", instr.as_ref())),
    }
}

fn parse_register<S: AsRef<str>>(register: S) -> Result<usize, String> {
    match register.as_ref()
    {
        "GPA" => Ok(Registers::GPA as usize),
        "GPB" => Ok(Registers::GPB as usize),
        "SBP" => Ok(Registers::SBP as usize),
        "TSP" => Ok(Registers::TSP as usize),
        "FRV" => Ok(Registers::FRV as usize),
        reg => Err(format!("Unknown register: {} (If you try to modify the instruction pointer, it cannot be written to direcctly, use branching instructions)", reg)),
    }
}

fn parse_literal<S: AsRef<str>>(literal: S) -> Result<i32, String> {
    literal
        .as_ref()
        .to_string()
        .parse::<i32>()
        .map(|v| v)
        .map_err(|e| format!("Unable to parse int : {}", e.to_string()))
}

fn parse_operand<S: AsRef<str>>(operand: S) -> Result<OperandType, String> {
    match operand.as_ref().chars().next() {
        Some('$') => {
            match operand
                .as_ref()
                .chars()
                .skip(1)
                .collect::<String>()
                .as_str()
            {
                "VelocityX" => Ok(OperandType::Literal {
                    value: MemoryMappedProperties::VelocityX as i32,
                }),
                "VelocityY" => Ok(OperandType::Literal {
                    value: MemoryMappedProperties::VelocityY as i32,
                }),
                "Moment" => Ok(OperandType::Literal {
                    value: MemoryMappedProperties::Moment as i32,
                }),
                "Rotation" => Ok(OperandType::Literal {
                    value: MemoryMappedProperties::Rotation as i32,
                }),
                "PositionX" => Ok(OperandType::Literal {
                    value: MemoryMappedProperties::PositionX as i32,
                }),
                "PositionY" => Ok(OperandType::Literal {
                    value: MemoryMappedProperties::PositionY as i32,
                }),
                "Ray0Dist" => Ok(OperandType::Literal {
                    value: MemoryMappedProperties::Ray0Dist as i32,
                }),
                "Ray0Type" => Ok(OperandType::Literal {
                    value: MemoryMappedProperties::Ray0Type as i32,
                }),
                "Ray1Dist" => Ok(OperandType::Literal {
                    value: MemoryMappedProperties::Ray1Dist as i32,
                }),
                "Ray1Type" => Ok(OperandType::Literal {
                    value: MemoryMappedProperties::Ray1Type as i32,
                }),
                "Ray2Dist" => Ok(OperandType::Literal {
                    value: MemoryMappedProperties::Ray2Dist as i32,
                }),
                "Ray2Type" => Ok(OperandType::Literal {
                    value: MemoryMappedProperties::Ray2Type as i32,
                }),
                "Ray3Dist" => Ok(OperandType::Literal {
                    value: MemoryMappedProperties::Ray3Dist as i32,
                }),
                "Ray3Type" => Ok(OperandType::Literal {
                    value: MemoryMappedProperties::Ray3Type as i32,
                }),
                "Ray4Dist" => Ok(OperandType::Literal {
                    value: MemoryMappedProperties::Ray4Dist as i32,
                }),
                "Ray4Type" => Ok(OperandType::Literal {
                    value: MemoryMappedProperties::Ray4Type as i32,
                }),
                "Ray5Dist" => Ok(OperandType::Literal {
                    value: MemoryMappedProperties::Ray5Dist as i32,
                }),
                "Ray5Type" => Ok(OperandType::Literal {
                    value: MemoryMappedProperties::Ray5Type as i32,
                }),
                "Ray6Dist" => Ok(OperandType::Literal {
                    value: MemoryMappedProperties::Ray6Dist as i32,
                }),
                "Ray6Type" => Ok(OperandType::Literal {
                    value: MemoryMappedProperties::Ray6Type as i32,
                }),
                var => Err(format!("Unknown variable: {}", var)),
            }
        }
        Some('#') => Ok(OperandType::Literal {
            value: parse_literal(operand.as_ref().chars().skip(1).collect::<String>())?,
        }),
        Some('\'') => Ok(OperandType::Register {
            idx: parse_register(
                operand
                    .as_ref()
                    .chars()
                    .skip(1)
                    .collect::<String>()
                    .as_str(),
            )?,
        }),
        Some('[') => {
            let operand = operand
                .as_ref()
                .chars()
                .filter_map(|c| if c == '[' || c == ']' { None } else { Some(c) })
                .collect::<String>();
            let splitted = operand
                .as_str()
                .split(" ")
                .filter_map(|s| {
                    if s.trim().is_empty() {
                        None
                    } else {
                        Some(s.to_string())
                    }
                })
                .collect::<Vec<String>>();

            if splitted.len() == 3 {
                Ok(OperandType::StackValue {
                    base_register: parse_register(
                        &splitted[0].as_str().chars().skip(1).collect::<String>(),
                    )?,
                    addition: &splitted[1] == "+",
                    offset: parse_literal(&splitted[2])? as usize,
                })
            } else {
                Err("Stack access must be composed of three operands".to_string())
            }
        }
        Some(_) => Ok(OperandType::Literal {
            value: parse_literal(operand.as_ref().chars().skip(1).collect::<String>())?,
        }),
        None => Err("No operand to parse !".to_string()),
    }
}

pub fn parse<S: AsRef<str>>(text: S) -> Result<Vec<Instruction>, ParsingError> {
    let mut instructions = vec![];
    'main_loop: for (line_nbr, line) in text.as_ref().split("\n").enumerate() {
        if line.chars().next() == Some(';') || line.len() == 0 {
            continue;
        }

        let mut char_iter = line.chars().peekable();
        let opcode = char_iter
            .by_ref()
            .take_while(|c| *c != ' ')
            .collect::<String>();
        let operand1 = {
            if char_iter.peek() == Some(&'[') {
                let res = char_iter
                    .by_ref()
                    .take_while(|c| *c != ']')
                    .collect::<String>()
                    + "]";
                char_iter.next(); // Consume the space
                res
            } else {
                char_iter
                    .by_ref()
                    .take_while(|c| *c != ' ')
                    .collect::<String>()
            }
        };

        let operand2 = {
            if char_iter.peek() == Some(&'[') {
                char_iter
                    .by_ref()
                    .take_while(|c| *c != ']')
                    .collect::<String>()
                    + "]"
            } else {
                char_iter
                    .by_ref()
                    .take_while(|c| *c != ' ')
                    .collect::<String>()
            }
        };

        let instruction = Instruction {
            opcode: match opcode {
                instr if !instr.is_empty() => match parse_instr(instr) {
                    Ok(instr) => instr,
                    Err(e) => return Err(ParsingError::new(line_nbr as u32, e)),
                },
                _ => {
                    println!("No intruction found for line '{}'", line);
                    break 'main_loop;
                }
            },
            operand_1: match operand1 {
                op if !op.is_empty() => match parse_operand(op) {
                    Ok(op) => op,
                    Err(e) => return Err(ParsingError::new(line_nbr as u32, e)),
                },
                _ => OperandType::None,
            },
            operand_2: match operand2 {
                op if !op.is_empty() => match parse_operand(op) {
                    Ok(op) => op,
                    Err(e) => return Err(ParsingError::new(line_nbr as u32, e)),
                },
                _ => OperandType::None,
            },
        };
        instructions.push(instruction);
    }

    Ok(instructions)
}
