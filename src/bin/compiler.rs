use clap::Parser;
use std::fs;

use csai::lang::{TokenStream, AST, analyze, SemanticError};

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

    let mut tokens = TokenStream::lex(text);
    let program = AST::parse(&mut tokens)?;

    println!("{}", program);

    analyze(&program).map_err(|e| match e {
        SemanticError::UnknownVariable(e) => e,
        SemanticError::InvalidOperation(e) => e,
    })?;

    Ok(())
}
