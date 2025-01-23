use std::collections::HashSet;

use super::{asm::{PASMInstruction, OperandType}, PASMProgram};

mod liveness_tree;

fn get_live_and_dead(instruction: &PASMInstruction) -> (Vec<String>, Vec<String>) {
    let mut operand_0 = if let Some(OperandType::Identifier { name }) = instruction.operands.get(0) {
        if !name.starts_with("$") && !name.starts_with("'") {
            vec![name.clone()]
        } else {
            vec![]
        }
    } else {
        vec![]
    };

    let operand_1 = if let Some(OperandType::Identifier { name }) = instruction.operands.get(1) {
        if !name.starts_with("$") && !name.starts_with("'") {
            vec![name.clone()]
        } else {
            vec![]
        }
    } else {
        vec![]
    };

    match instruction.opcode.as_str() {
        "load" | "pop" | "mov" => (operand_1, operand_0),
        "add" | "sub" | "mul" | "div" | "mod" | "cmp" | "store" => {
            operand_0.extend(operand_1);
            (operand_0, vec![])
        },
        _ => (vec![], vec![])
    }
}

/// Represents a PASM program where each funcion has an associated interference graph,
/// used to perform the register allocation in the next stage.
pub struct PASMProgramWithInterferenceGraph  {
    pub functions: Vec<(Vec<PASMInstruction>, ())>
}

impl PASMProgramWithInterferenceGraph {
    fn extract_labels(function: &Vec<PASMInstruction>)  -> Vec<String> {
        function.iter().filter_map(|inst|
            if inst.is_label {
                Some(inst.opcode.clone())
            } else {
                None
            }
        )
        .collect()
    }

    fn function_live_analysis(function: &Vec<PASMInstruction>) {
        let labels = Self::extract_labels(function);
        println!("Found labels: {}", labels.join(", "));

        let stats = (
            labels.iter().filter(|l| l.contains("_loop_label_")).count(),
            labels.iter().filter(|l| l.contains("_while_condition_")).count(),
            labels.iter().filter(|l| l.contains("_if_exit_")).count(),
        );

        println!("These correspond to {} loop(s), {} while(s) and {} if statement(s)", stats.0, stats.1, stats.2);


        let mut live_set = HashSet::new();
        for instruction in function.iter().rev() {
            let (live, dead) = get_live_and_dead(instruction);

            for live_item in live {
                live_set.insert(live_item);
            }
            for dead_item in dead {
                live_set.remove(&dead_item);
            }
            println!("{:30} | {:?}", instruction, live_set);
        }
    }

    /// For each function's PASM, performs the undead analysis and attaches to the Program
    pub fn analyse(program: &PASMProgram) -> Result<Self, String> {
        for (_, function) in program.functions.iter() {
            Self::function_live_analysis(function);
        }

        Ok(Self {
            functions: vec![]
        })
    }
}
