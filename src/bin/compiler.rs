use clap::Parser;
use std::fs;

use csai::lang::ast::AST;

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

    let program = AST::parse(&text)?;

    Ok(())
}
