use crossterm::event::KeyEvent;
use log::warn;
use std::fmt;

use machine::prelude::*;

use crate::blocks::{AppBlock, InstructionsBlock, RegisterBlock, StackBlock};

use ratatui::{
    layout::{Constraint, Layout},
    Frame,
};

#[derive(Default)]
pub enum AppStatus {
    #[default]
    Ready,
    Ticking,
    Continuing,
    Err(String),
}

impl fmt::Display for AppStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppStatus::Ready => write!(f, "Ready"),
            AppStatus::Ticking => write!(f, "Ticking"),
            AppStatus::Continuing => write!(f, "Continuing"),
            AppStatus::Err(e) => write!(f, "Error: {}", e),
        }
    }
}

pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub machine: VirtualMachine,
    pub status: AppStatus,
    pub selected_block: usize, // Selected block for modifications (Instructions, Stack, registers)
    pub blocks: [Box<dyn AppBlock>; 3],
}

impl App<'_> {
    pub fn new(title: &str, machine: VirtualMachine) -> App {
        App {
            title,
            should_quit: false,
            machine,
            status: AppStatus::default(),
            selected_block: 0, // Instructions
            blocks: [
                Box::new(InstructionsBlock::new()),
                Box::new(StackBlock::new()),
                Box::new(RegisterBlock::new()),
            ],
        }
    }

    pub fn on_tick(&mut self) {
        match self.machine.tick() {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }

    pub fn on_key(&mut self, key: KeyEvent) {
        self.blocks[self.selected_block].on_key(key);
    }

    pub fn on_next_block(&mut self) {
        self.selected_block = (self.selected_block + 1) % self.blocks.len();
    }

    /// Toggles the app between "Ticking" and "Continuing" states
    pub fn on_continue(&mut self) {
        self.status = match self.status {
            AppStatus::Continuing => AppStatus::Ticking,
            AppStatus::Ticking => AppStatus::Continuing,
            _ => {
                warn!("Unable to toggle between `Continuing` and `Ticking`, App is not in a valid state {}", self.status);
                return;
            }
        }
    }

    /// Update the machine if the app is in the "Continuing" state
    pub fn update(&mut self) {
        if matches!(self.status, AppStatus::Continuing) {
            if let Err(e) = self.machine.tick() {
                self.status = AppStatus::Err(e.to_string());
            };
        }
    }

    pub fn on_quit(&mut self) {
        self.should_quit = true;
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        let chunks = Layout::horizontal([
            Constraint::Min(20),
            Constraint::Min(15),
            Constraint::Min(11),
        ]);
        for (idx, (block, area)) in self
            .blocks
            .iter()
            .zip(chunks.split(frame.area()).iter())
            .enumerate()
        {
            block.draw(frame, &mut self.machine, idx == self.selected_block, area);
        }
    }
}
