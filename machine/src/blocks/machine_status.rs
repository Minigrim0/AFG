use crossterm::event::{KeyCode, KeyEvent};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{self, Span};
use ratatui::widgets::{Block, Paragraph, Wrap};
use ratatui::{layout::Rect, Frame};

use super::AppBlock;
use machine::prelude::VirtualMachine;

pub struct MachineStatusBlock;

impl MachineStatusBlock {
    pub fn new() -> Self {
        Self
    }
}

impl AppBlock for MachineStatusBlock {
    fn draw(
        &mut self,
        frame: &mut Frame,
        machine: &mut VirtualMachine,
        is_selected: bool,
        area: &Rect,
    ) {
        let lines = vec![text::Line::from(machine.get_status())];

        let block = Block::bordered()
            .title(Span::styled(
                "Status",
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

    fn on_key(&mut self, _key: KeyEvent) {}
}
