use crossterm::event::KeyEvent;
use log::warn;
use std::fmt;

use machine::prelude::*;

use crate::blocks::{
    AppBlock, InstructionsBlock, MachineOutputBlock, MachineStatusBlock, RegisterBlock, StackBlock,
};

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
    pub _title: &'a str,
    pub should_quit: bool,
    pub machine: VirtualMachine,
    pub status: AppStatus,
    pub selected_block: usize, // Selected block for modifications (Instructions, Stack, registers)
    pub blocks: (
        InstructionsBlock,
        StackBlock,
        RegisterBlock,
        MachineOutputBlock,
        MachineStatusBlock,
    ),
    pub breakpoints: Vec<usize>,
}

impl App<'_> {
    pub fn new(title: &str, machine: VirtualMachine) -> App {
        App {
            _title: title,
            should_quit: false,
            machine,
            status: AppStatus::default(),
            selected_block: 0, // Instructions
            blocks: (
                InstructionsBlock::new(),
                StackBlock::new(),
                RegisterBlock::new(),
                MachineOutputBlock::new(),
                MachineStatusBlock::new(),
            ),
            breakpoints: vec![],
        }
    }

    pub fn on_tick(&mut self) {
        if !self.machine.has_completed() {
            match self.machine.tick() {
                Ok(_) => (),
                Err(e) => {
                    self.status = AppStatus::Err(e.to_string());
                }
            }
        }
    }

    pub fn on_key(&mut self, key: KeyEvent) {
        match key.code {
            crossterm::event::KeyCode::Char('b') => {
                let index = self.blocks.0.get_selected_cip();
                if let Some(position) = self.breakpoints.iter().position(|v| *v == index) {
                    self.breakpoints.remove(position);
                } else {
                    self.breakpoints.push(index);
                }
                self.blocks.0.update_breakpoints(self.breakpoints.clone());
            }
            _ => match self.selected_block {
                0 => self.blocks.0.on_key(key),
                1 => self.blocks.1.on_key(key),
                2 => self.blocks.2.on_key(key),
                3 => self.blocks.3.on_key(key),
                4 => self.blocks.4.on_key(key),
                _ => unreachable!(),
            },
        }
    }

    pub fn on_next_block(&mut self) {
        self.selected_block = (self.selected_block + 1) % 5;
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
            if self
                .breakpoints
                .contains(&(self.machine.get_cip() as usize))
            {
                self.on_continue();
                return;
            }
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
            Constraint::Length(40),
            Constraint::Length(30),
        ])
        .split(frame.area());

        self.blocks.0.draw(
            frame,
            &mut self.machine,
            self.selected_block == 0,
            &chunks[0],
        );

        self.blocks.1.draw(
            frame,
            &mut self.machine,
            self.selected_block == 1,
            &chunks[1],
        );

        let sub_layout = Layout::vertical([
            Constraint::Min(12),
            Constraint::Min(5),
            Constraint::Length(3),
        ])
        .split(chunks[2]);

        self.blocks.2.draw(
            frame,
            &mut self.machine,
            self.selected_block == 2,
            &sub_layout[0],
        );

        self.blocks.3.draw(
            frame,
            &mut self.machine,
            self.selected_block == 3,
            &sub_layout[1],
        );

        self.blocks.4.draw(
            frame,
            &mut self.machine,
            self.selected_block == 4,
            &sub_layout[2],
        );
    }
}
