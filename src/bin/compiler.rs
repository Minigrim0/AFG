use clap::Parser;
use std::fs;

use AFG::lang::{analyze, PASMProgram, SemanticError, TokenStream, AST};

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

    let text = fs::read_to_string(&args.input).map_err(|e| e.to_string())?;

    let mut tokens = TokenStream::lex(text);
    if args.save_intermediate {
        let token_output = args.input.clone() + ".tokens";
        fs::write(&token_output, format!("{}", tokens)).map_err(|e| e.to_string())?;
    }

    let program = AST::parse(&mut tokens)?;
    if args.save_intermediate {
        let ast_output = args.input.clone() + ".ast";
        fs::write(&ast_output, format!("{}", program)).map_err(|e| e.to_string())?;
    }

    analyze(&program).map_err(|e| match e {
        SemanticError::UnknownVariable(e) => e,
        SemanticError::InvalidOperation(e) => e,
    })?;

    let pasm = PASMProgram::parse(program)?;
    if args.save_intermediate {
        let pasm_output = args.input.clone() + ".pasm";
        fs::write(&pasm_output, format!("{}", pasm)).map_err(|e| e.to_string())?;
    }

    Ok(())
}
