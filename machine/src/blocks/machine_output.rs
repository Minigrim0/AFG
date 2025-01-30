use crossterm::event::KeyEvent;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{self, Span};
use ratatui::widgets::{Block, Paragraph, Wrap};
use ratatui::{layout::Rect, Frame};

use std::cmp::{max, min};
use std::usize;

use super::AppBlock;
use machine::prelude::VirtualMachine;

pub struct MachineOutputBlock {
    output: Vec<String>, // All the outputs of the machine
}

impl MachineOutputBlock {
    pub fn new() -> Self {
        Self { output: vec![] }
    }
}

impl AppBlock for MachineOutputBlock {
    fn draw(
        &mut self,
        frame: &mut Frame,
        machine: &mut VirtualMachine,
        is_selected: bool,
        area: &Rect,
    ) {
        if let Some(current_output) = machine.get_current_output() {
            self.output.push(current_output);
        }

        let lines = self
            .output
            .iter()
            .rev()
            .map(|output| text::Line::from(output.as_str()))
            .take(area.height as usize)
            .rev()
            .collect::<Vec<_>>();

        let block = Block::bordered()
            .title(Span::styled(
                "Outputs",
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::UNDERLINED),
            ))
            .border_style(Style::default().fg(if is_selected {
                Color::Yellow
            } else {
                Color::LightGreen
            }));
        let paragraph = Paragraph::new(lines).block(block).wrap(Wrap { trim: true });
        frame.render_widget(paragraph, *area);
    }

    fn on_key(&mut self, key: KeyEvent) {}
}
