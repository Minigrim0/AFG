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

pub struct InstructionsBlock {
    offset: usize,
}

impl InstructionsBlock {
    pub fn new() -> InstructionsBlock {
        InstructionsBlock { offset: 0 }
    }
}

impl AppBlock for InstructionsBlock {
    fn draw(
        &self,
        frame: &mut Frame,
        machine: &mut VirtualMachine,
        is_selected: bool,
        area: &Rect,
    ) {
        let instructions = machine.get_instruction_slice(self.offset, area.height as usize);
        let current_cip = machine.get_cip();

        let lines = instructions
            .iter()
            .map(|(idx, instr)| {
                let mut line_vec = vec![Span::from(format!("{:04X}", idx)), Span::from(" ")];
                if *idx as i32 == current_cip {
                    line_vec.push(Span::styled(
                        format!("> {}", instr),
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    ))
                } else {
                    line_vec.push(Span::from(format!("   {}", instr)));
                }
                text::Line::from(line_vec)
            })
            .collect::<Vec<_>>();

        let block = Block::bordered()
            .title(Span::styled(
                "Instructions",
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

    fn on_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Down => self.offset += 1,
            KeyCode::Up => self.offset = max((self.offset as i32) - 1, 0) as usize,
            _ => (),
        }
    }
}
