use crossterm::event::KeyEvent;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{self, Span};
use ratatui::widgets::{Block, Paragraph, Wrap};
use ratatui::{layout::Rect, Frame};

use super::AppBlock;
use machine::prelude::VirtualMachine;

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
        &self,
        frame: &mut Frame,
        machine: &mut VirtualMachine,
        is_selected: bool,
        area: &Rect,
    ) {
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
        let paragraph = Paragraph::new(vec![])
            .block(block)
            .wrap(Wrap { trim: true });
        frame.render_widget(paragraph, *area);
    }

    fn on_key(&mut self, key: KeyEvent) {}
}
