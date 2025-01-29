use clap::Parser;

use color_eyre::Result;
use crossterm::event::{self, Event, KeyEvent};
use ratatui::{DefaultTerminal, Frame};

use machine::prelude::{VirtualMachine, Program};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, help = "The input file to execute")]
    input: String,
    #[arg(short, long, help = "The output file to write the output to")]
    output: Option<String>,
}


fn main() -> Result<(), String> {
    let args = Args::parse();

    println!("Parsing program: {}", args.input);
    let program = Program::new(args.input)?;

    println!("Building machine");
    let mut machine = VirtualMachine::new()
        .with_program(program.instructions);

    color_eyre::install().map_err(|e| e.to_string())?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result.map_err(|e| e.to_string())
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(render)?;
        if matches!(event::read()?, Event::Key(KeyEvent { code: event::KeyCode::Esc, .. })) {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    frame.render_widget("hello world", frame.area());
}

fn run_machine(machine: &mut VirtualMachine) {

    println!("Starting the machine");
    loop {
        if let Some(current_instruction) = machine.get_current_instruction() {
            println!("CI {}: {}", machine.get_cip(), current_instruction);
        }
        match machine.tick() {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
        }
    }
}
