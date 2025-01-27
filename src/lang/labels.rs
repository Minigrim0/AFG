use std::collections::HashMap;

use super::asm::{OperandType, PASMInstruction};

pub fn resolve_labels(function: Vec<PASMInstruction>) -> Result<Vec<PASMInstruction>, String> {
    let mut current_line: usize = 0;
    let mut label_map: HashMap<String, usize> = HashMap::new();
    let mut resolved = Vec::new();

    for inst in function {
        if inst.is_label {
            label_map.insert(inst.opcode.clone(), current_line);
        } else {
            resolved.push(inst);
            current_line += 1;
        }
    }

    for (current_line, inst) in resolved.iter_mut().enumerate() {
        if let Some(jump_to) = inst.jump_to() {
            if let Some(line) = label_map.get(&jump_to) {
                inst.operands = vec![OperandType::Literal {
                    value: *line as i32 - current_line as i32,
                }];
            } else {
                return Err(format!("Unknown label {}", jump_to));
            }
        }
    }

    Ok(resolved)
}
