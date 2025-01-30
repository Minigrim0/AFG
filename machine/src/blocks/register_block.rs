use crossterm::event::KeyEvent;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{self, Span};
use ratatui::widgets::{Block, Paragraph, Wrap};
use ratatui::{layout::Rect, Frame};

use super::AppBlock;
use machine::prelude::VirtualMachine;

pub struct RegisterBlock {
    most_recently_modified: Option<usize>,
}

impl RegisterBlock {
    pub fn new() -> RegisterBlock {
        RegisterBlock {
            most_recently_modified: None,
        }
    }
}

impl AppBlock for RegisterBlock {
    fn draw(
        &mut self,
        frame: &mut Frame,
        machine: &mut VirtualMachine,
        is_selected: bool,
        area: &Rect,
    ) {
        let mut lines = machine
            .get_registers()
            .iter()
            .map(|(reg_name, value)| {
                text::Line::from(vec![
                    Span::styled(
                        format!("{:?}", reg_name),
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::from(": "),
                    Span::styled(
                        format!("{:04X}", value),
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD),
                    ),
                ])
            })
            .collect::<Vec<_>>();

        lines.push(text::Line::from(Span::from("")));

        for line in machine.get_flags() {
            lines.push(text::Line::from(vec![
                Span::styled(
                    format!("{:?}", line.0),
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::from(": "),
                Span::styled(
                    format!("{:?}", line.1),
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::from(": "),
                Span::styled(
                    format!("{:?}", line.2),
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
            ]));
        }

        let block = Block::bordered()
            .title(Span::styled(
                "Registers",
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
