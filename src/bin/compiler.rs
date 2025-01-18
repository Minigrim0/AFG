use clap::Parser;
use std::fs;

use csai::lang::ast::parser::parse_program;
use csai::lang::ast::node::print_block;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, help = "The input file to compile")]
    input: String,
    #[arg(short, long, help = "The output file to write the compiled program")]
    output: Option<String>,
    #[arg(short, long, help = "Save intermediate files")]
    save_intermediate: bool,
}

fn main() -> Result<(), String> {
    let args = Args::parse();

    let text = fs::read_to_string(args.input).map_err(|e| e.to_string())?;

    let mut program = parse_program(&text)?;

    if let Some(fun) = program.get("main") {
        if fun.parameters.len() != 0 {
            return Err("function main cannot contain parameters".to_string());
        }
    } else {
        return Err("Program must contain a main function".to_string());
    }

    if let Some(function) = program.remove("main") {
        println!("AST for main function");
        print_block(function.content)
    }
    if let Some(function) = program.remove("turn_90") {
        println!("AST for turn_90 function");
        print_block(function.content)
    }

    Ok(())
}
