use std::time::{Duration, Instant};

use clap::Parser;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{prelude::Backend, Terminal};

use colog;
use log::{error, info};

use machine::prelude::{Program, VirtualMachine};

mod app;
mod blocks;

use app::{App, AppStatus};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, help = "The input file to execute")]
    input: String,
    #[arg(short, long, help = "The output file to write the output to")]
    output: Option<String>,
    #[arg(
        short,
        long,
        help = "Executes the program all at once, outputting only the program's outout"
    )]
    no_tui: bool,
}

fn main() -> Result<(), String> {
    colog::init();

    info!("Parsing arguments");
    let args = Args::parse();

    info!("Parsing program: {}", args.input);
    let program = Program::new(args.input)?;

    info!("Building machine");
    let mut machine = VirtualMachine::new().with_program(program.instructions);

    if !args.no_tui {
        let app = App::new("Virtual Machine", machine);

        color_eyre::install().map_err(|e| e.to_string())?;
        let mut terminal = ratatui::init();

        let result = run_app(
            &mut terminal,
            app,
            Duration::from_millis((1000.0 / 60.0) as u64),
        );

        ratatui::restore();
        result.map_err(|e| e.to_string())
    } else {
        info!("Starting execution");
        println!("Output:");
        loop {
            if let Err(e) = machine.tick() {
                error!("Machine encountered an error: {}", e);
                break Err(e);
            }
            if let Some(output) = machine.get_current_output() {
                println!("> {}", output);
            }
            if machine.has_completed() {
                info!("Machine has completed its execution");
                break Ok(());
            }
        }
    }
}

fn run_app<S: Backend>(
    terminal: &mut Terminal<S>,
    mut app: App,
    tick_rate: Duration,
) -> Result<()> {
    let mut last_tick = Instant::now();
    app.status = AppStatus::Ticking;
    loop {
        terminal.draw(|f| app.draw(f))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char(' ') => app.on_tick(),
                        KeyCode::Char('c') => app.on_continue(),
                        KeyCode::Tab => app.on_next_block(),
                        KeyCode::Esc | KeyCode::Char('q') => app.on_quit(),
                        _ => app.on_key(key),
                    }
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            // Only updates the machine if the app is in the continue state
            app.update();
            last_tick = Instant::now();
        }
        if app.should_quit {
            return Ok(());
        }
    }
}
