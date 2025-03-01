use clap::Parser;
use std::collections::HashMap;
use std::fs;

use colog;
use log::{error, info, warn};

use afgcompiler::prelude::*;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, help = "The input file to compile")]
    input: String,
    #[arg(short, long, help = "The output file to write the compiled program")]
    output: Option<String>,
    #[arg(short, long, help = "Save intermediate files (ast, pseudo-asm, ...)")]
    save_intermediate: bool,
    #[arg(short = 'O', long, help = "Tries to delete redundant instructions")]
    optmimize: bool,
}

fn main() -> Result<(), String> {
    colog::init();

    let args = Args::parse();

    info!("Reading source code from {}", &args.input);
    let text = fs::read_to_string(&args.input).map_err(|e| e.to_string())?;

    info!("Extracting tokens");
    let tokens = lex(text);
    if args.save_intermediate {
        let token_output = args.input.clone() + ".tokens";
        info!("Saving tokens to {}", token_output);
        fs::write(
            &token_output,
            format!(
                "{}",
                tokens
                    .iter()
                    .map(|t| format!("{}", t))
                    .collect::<Vec<String>>()
                    .join("\n")
            ),
        )
        .map_err(|e| e.to_string())?;
    }

    info!("Parsing AST from tokens");
    let program = AST::parse(&mut tokens.into_iter().peekable())?;
    if args.save_intermediate {
        let ast_output = args.input.clone() + ".ast";
        info!("Saving AST to {}", ast_output);
        fs::write(&ast_output, format!("{}", program)).map_err(|e| e.to_string())?;
    }

    info!("Analyzing AST");
    analyze(&program).map_err(|e| format!("{}", e))?;

    info!("Generating pseudo-asm");
    let pasm = PASMProgram::parse(program)?;
    if args.save_intermediate {
        let pasm_output = args.input.clone() + ".pasm";
        info!("Saving pseudo-asm to {}", pasm_output);
        fs::write(&pasm_output, format!("{}", pasm)).map_err(|e| e.to_string())?;
    }

    info!("Allocating static memory");
    let allocated_program = PASMAllocatedProgram {
        functions: pasm
            .functions
            .iter()
            .map(
                |(function_name, function)| -> Result<(String, Vec<PASMInstruction>), String> {
                    Ok((function_name.clone(), allocate(function)?))
                },
            )
            .collect::<Result<HashMap<String, Vec<PASMInstruction>>, String>>()?,
    };
    if args.save_intermediate {
        let pasm_output = args.input.clone() + ".pasm_allocated";
        info!("Saving allocated pseudo-asm to {}", pasm_output);
        fs::write(&pasm_output, format!("{}", allocated_program)).map_err(|e| e.to_string())?;
    }

    if args.optmimize {
        warn!("Code optimization is not complete yet and might lead to buggy programs");
        error!("Code optimization is not implemented yet");
        return Err("Code optimization not implemented for this compiler's version".to_string());
        // let analyzed = PASMProgramWithInterferenceGraph::analyse(&pasm)?;
    }

    // Final step; resolve labels and write to output file
    let mut final_code = allocated_program
        .functions
        .get("main")
        .ok_or("No main function")?
        .clone();

    for (function_name, function) in allocated_program.functions.into_iter() {
        if function_name == "main" {
            continue;
        }
        final_code.push(PASMInstruction::new_comment(format!(
            "Function {}",
            function_name
        )));
        final_code.extend(function);
    }

    info!("Resolving labels");
    resolve_labels(final_code)
        .map_err(|e| e.to_string())
        .and_then(|resolved| {
            let output = args.output.unwrap_or("a.asmfg".to_string());
            info!("Writing output to {}", output);
            fs::write(
                output,
                format!(
                    "{}",
                    resolved
                        .iter()
                        .map(|i| format!("{}", i))
                        .collect::<Vec<String>>()
                        .join("\n")
                ),
            )
            .map_err(|e| e.to_string())
        })?;

    Ok(())
}
