use super::{Instruction, Instructions, MemoryMappedProperties, Registers};

fn parse_instr<S: AsRef<str>>(instr: S) -> Instructions {
    match instr.as_ref() {
        "mov" => Instructions::MOV,
        "movi" => Instructions::MOVI,
        "store" => Instructions::STORE,
        "storei" => Instructions::STOREI,
        "load" => Instructions::LOAD,
        "loadi" => Instructions::LOADI,
        _ => {
            println!("Unknown instruction: {}", instr.as_ref());
            Instructions::NOP
        }
    }
}

fn parse_operand<S: AsRef<str>>(operand: S) -> Result<i32, ()> {
    match operand.as_ref().chars().next() {
        Some('$') => {
            println!("Operand is a special variable");
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
                var => {
                    println!("Unknown variable: {}", var);
                    Err(())
                }
            }
        }
        Some('#') => {
            println!("Operand is a litteral");
            operand
                .as_ref()
                .chars()
                .skip(1)
                .collect::<String>()
                .parse::<i32>()
                .map_err(|e| {
                    println!("Unable to parse int : {}", e.to_string());
                })
        }
        Some('\'') => {
            println!("Operand is a register");
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
                    println!("Unknown register: {}", reg);
                    Err(())
                }
            }
        }
        Some(_) => operand
            .as_ref()
            .chars()
            .skip(1)
            .collect::<String>()
            .parse::<i32>()
            .map_err(|e| {
                println!("Unable to parse int : {}", e.to_string());
            }),
        None => {
            println!("No operand to parse !");
            Err(())
        }
    }
}

pub fn parse<S: AsRef<str>>(text: S) -> Vec<Instruction> {
    let mut instructions = vec![];
    'main_loop: for line in text.as_ref().split("\n") {
        println!("Working on line: {}", line);
        if line.chars().next() == Some(';') {
            println!("Comment line, skipping");
            continue;
        }
        if line.len() == 0 {
            println!("Empty line, skipping");
            continue;
        }

        let splitted_line = line.split(" ").collect::<Vec<&str>>();
        let mut splitted_line = splitted_line.iter();
        let instruction = Instruction {
            opcode: match splitted_line.next() {
                Some(instr) => parse_instr(instr),
                None => {
                    println!("No intruction found for line '{}'", line);
                    break 'main_loop;
                }
            },
            operand_1: match splitted_line.next() {
                Some(op) => match parse_operand(op) {
                    Ok(op) => op,
                    Err(()) => {
                        println!("Error in program code");
                        break 'main_loop;
                    }
                },
                None => {
                    println!("No intruction found for line '{}'", line);
                    break 'main_loop;
                }
            },
            operand_2: splitted_line
                .next()
                .and_then(|operand| parse_operand(operand).ok()),
        };
        instructions.push(instruction);
    }

    instructions
}
