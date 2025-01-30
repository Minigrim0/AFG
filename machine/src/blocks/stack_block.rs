use crossterm::event::{KeyCode, KeyEvent};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{self, Span};
use ratatui::widgets::{Block, Paragraph, Wrap};
use ratatui::{layout::Rect, Frame};

use std::cmp::max;

use super::AppBlock;
use machine::prelude::{Registers, VirtualMachine};

pub struct StackBlock {
    offset: usize,                         // Scroll in the stack block
    most_recently_modified: Option<usize>, // Index of the most recently modified value
}

impl StackBlock {
    pub fn new() -> StackBlock {
        StackBlock {
            offset: 0,
            most_recently_modified: None,
        }
    }
}

impl AppBlock for StackBlock {
    fn draw(
        &mut self,
        frame: &mut Frame,
        machine: &mut VirtualMachine,
        is_selected: bool,
        area: &Rect,
    ) {
        let sbp = machine.get_register(Registers::SBP as usize);
        let tsp = machine.get_register(Registers::TSP as usize);

        let lines = machine
            .get_stack_slice(self.offset, area.height as usize)
            .iter()
            .map(|(idx, value)| {
                let mut line_vec = vec![Span::from(format!("{:02X}", idx)), Span::from(" ")];
                if Some(*idx) == self.most_recently_modified {
                    line_vec.push(Span::styled(
                        format!("{:04X}", value),
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    ));
                } else {
                    line_vec.push(Span::from(format!("{:04X}", value)));
                }
                if *idx == sbp as usize {
                    line_vec.push(Span::from(" < SBP"));
                }
                if *idx == tsp as usize {
                    line_vec.push(Span::from(" < TSP"));
                }
                if *idx > sbp as usize && *idx < (sbp + 5) as usize {
                    let offset = *idx as i32 - sbp;
                    line_vec.push(Span::from(" <"));
                    line_vec.push(Span::from(format!(" SBP {:02}", offset)));
                    if offset == 1 {
                        line_vec.push(Span::from(" <= Return pointer"));
                    }
                }
                if *idx < sbp as usize && *idx > tsp as usize {
                    let offset = *idx as i32 - sbp;
                    line_vec.push(Span::from(" <"));
                    line_vec.push(Span::from(format!(" SBP {:02}", offset)));
                }
                text::Line::from(line_vec)
            })
            .collect::<Vec<_>>();

        let block = Block::bordered()
            .title(Span::styled(
                "Stack",
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
