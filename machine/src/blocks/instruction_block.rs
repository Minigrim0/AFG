use color_eyre::owo_colors::colors::css::LightGray;
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
    offset: usize, // Selected instruction
    cursor_position: i32,
    breakpoints: Vec<usize>,
}

impl InstructionsBlock {
    pub fn new() -> InstructionsBlock {
        InstructionsBlock {
            offset: 0,
            cursor_position: 0,
            breakpoints: vec![],
        }
    }

    pub fn get_selected_cip(&self) -> usize {
        self.offset + max(0, self.cursor_position) as usize
    }

    pub fn update_breakpoints(&mut self, bp: Vec<usize>) {
        self.breakpoints = bp;
    }
}

impl AppBlock for InstructionsBlock {
    fn draw(
        &mut self,
        frame: &mut Frame,
        machine: &mut VirtualMachine,
        is_selected: bool,
        area: &Rect,
    ) {
        if self.cursor_position < 0 {
            // Move offset down
            self.offset = max(self.offset as i32 - 1, 0) as usize;
        }
        if self.cursor_position > area.height as i32 - 3 {
            // Move offset up
            self.offset = max((self.offset as i32) + 1, 0) as usize
        }

        // Bind cursor position
        self.cursor_position = max(0, min(self.cursor_position, area.height as i32 - 3));
        let instruction_start_offset = max(self.offset as i32, 0) as usize;
        let instructions =
            machine.get_instruction_slice(instruction_start_offset, area.height as usize);
        let current_cip = machine.get_cip();

        let lines = instructions
            .iter()
            .map(|(idx, instr)| {
                let mut line_vec = vec![Span::from(format!("{:04X}", idx))];
                if self.breakpoints.contains(idx) {
                    line_vec.push(Span::styled("x", Style::default().fg(Color::Red)))
                } else {
                    line_vec.push(Span::from(" "))
                }

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

                if self.cursor_position as usize + self.offset == *idx {
                    line_vec.push(Span::styled(" <", Style::default().fg(Color::LightGreen)));
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
            KeyCode::Down => self.cursor_position += 1,
            KeyCode::Up => self.cursor_position -= 1,
            _ => (),
        }
    }
}
