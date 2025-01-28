use std::fs;
use petgraph::dot::{Dot, Config};

use super::{
    asm::PASMInstruction,
    PASMProgram,
};


mod block;
mod liveness_tree;

use block::Block;

/// Represents a PASM program where each funcion has an associated interference graph,
/// used to perform the register allocation in the next stage.
pub struct PASMProgramWithInterferenceGraph {
    pub functions: Vec<(Vec<PASMInstruction>, ())>,
}

impl PASMProgramWithInterferenceGraph {
    fn extract_labels(function: &Vec<PASMInstruction>) -> Vec<String> {
        function
            .iter()
            .filter_map(|inst| {
                if inst.is_label {
                    Some(inst.opcode.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    fn function_live_analysis(function: &Vec<PASMInstruction>) {
        let labels = Self::extract_labels(function);
        println!("Found labels: {}", labels.join(", "));

        let stats = (
            labels.iter().filter(|l| l.contains("_loop_label_")).count(),
            labels
                .iter()
                .filter(|l| l.contains("_while_condition_"))
                .count(),
            labels.iter().filter(|l| l.contains("_if_exit_")).count(),
        );

        println!(
            "These correspond to {} loop(s), {} while(s) and {} if statement(s)",
            stats.0, stats.1, stats.2
        );
    }

    /// For each function's PASM, performs the undead analysis and attaches to the Program
    pub fn analyse(program: &PASMProgram) -> Result<Self, String> {
        for (fname, function) in program.functions.iter() {
            // Summarizes amount of loops & such
            Self::function_live_analysis(&function.1);

            let blocks = Block::from_function(&function.1)?;

            let ast_output = format!("graph_{}.viz", fname);
            if let Err(e) = fs::write(
                &ast_output,
                format!("{:#?}", Dot::with_config(&blocks, &[Config::EdgeNoLabel])),
            ) {
                println!("Error writing graph: {}", e);
            }
        }

        Ok(Self { functions: vec![] })
    }
}
