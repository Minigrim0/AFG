use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{self, Span};
use ratatui::widgets::{Block, Paragraph, Wrap};
use ratatui::{layout::Rect, Frame};

use std::cmp::{max, min};
use std::usize;

use super::AppBlock;
use machine::prelude::{Instruction, OpCodes, OperandType, VirtualMachine};

pub struct InstructionsBlock {
    offset: usize, // Selected instruction
    cursor_position: i32,
    follow_cip: bool,
    breakpoints: Vec<usize>,
}

impl InstructionsBlock {
    pub fn new() -> InstructionsBlock {
        InstructionsBlock {
            offset: 0,
            cursor_position: 0,
            follow_cip: false,
            breakpoints: vec![],
        }
    }

    pub fn get_selected_cip(&self) -> usize {
        self.offset + max(0, self.cursor_position) as usize
    }

    pub fn update_breakpoints(&mut self, bp: Vec<usize>) {
        self.breakpoints = bp;
    }

    /// Returns Some(value) when the instruction currently pointed at might jump to a literal
    fn get_jump_index(
        &self,
        _current_cip: i32,
        instructions: &Vec<(usize, Instruction)>,
    ) -> Option<usize> {
        let mut target = None;

        if let Some(instruction) = instructions.get(self.cursor_position as usize) {
            if matches!(
                instruction.1.opcode,
                OpCodes::JMP
                    | OpCodes::JZ
                    | OpCodes::JNZ
                    | OpCodes::JP
                    | OpCodes::JN
                    | OpCodes::CALL
            ) {
                if let OperandType::Literal { value } = instruction.1.operand_1 {
                    target = Some((self.cursor_position + self.offset as i32 + value) as usize);
                }
            }
        }

        target
    }

    /// Displays the jump lines (a '|' if the index is between the target and the cursor, a '┌' or '└' if the index is the target or the cursor)
    fn display_jump(&self, idx: usize, target: Option<usize>) -> Vec<Span> {
        let mut line_vec = vec![];

        if let Some(target) = target {
            // Current line is between current instr & target
            if idx > min(self.cursor_position as usize + self.offset, target)
                && idx < max(self.cursor_position as usize + self.offset, target)
            {
                line_vec.push(Span::styled(
                    "|",
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                ))
            } else if idx == target {
                let character = if target > self.cursor_position as usize {
                    "└"
                } else {
                    "┌"
                };
                line_vec.push(Span::styled(
                    character,
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                ))
            } else if idx == self.cursor_position as usize + self.offset {
                let character = if target > self.cursor_position as usize {
                    "┌"
                } else {
                    "└"
                };
                line_vec.push(Span::styled(
                    character,
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                ))
            } else {
                line_vec.push(Span::from(" "));
            }
        } else {
            line_vec.push(Span::from(" "));
        }

        line_vec
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

        if self.follow_cip {
            self.cursor_position = machine.get_cip() as i32 - self.offset as i32;
        }

        // Bind cursor position
        self.cursor_position = max(0, min(self.cursor_position, area.height as i32 - 3));
        let instruction_start_offset = max(self.offset as i32, 0) as usize;
        let instructions =
            machine.get_instruction_slice(instruction_start_offset, area.height as usize);
        let current_cip = machine.get_cip();
        self.cursor_position = min(self.cursor_position, instructions.len() as i32 - 1);

        let jump_to_target = self.get_jump_index(current_cip, &instructions);

        let lines = instructions
            .iter()
            .map(|(idx, instr)| {
                let mut line_vec = vec![Span::from(format!("{:04X}", idx))];

                // Show breakpoint
                line_vec.push(if self.breakpoints.contains(idx) {
                    Span::styled("●", Style::default().fg(Color::Red))
                } else {
                    Span::from(" ")
                });

                // Show jump lines
                line_vec.extend(self.display_jump(*idx, jump_to_target));

                // Show instruction
                if *idx as i32 == current_cip {
                    line_vec.push(Span::styled(
                        format!("➤ {}", instr),
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    ))
                } else {
                    line_vec.push(Span::from(format!("  {}", instr)));
                }

                // Show cursor
                if self.cursor_position as usize + self.offset == *idx {
                    line_vec.push(Span::styled(" ☚", Style::default().fg(Color::LightGreen)));
                    if self.follow_cip {
                        line_vec.push(Span::styled(" 🔒", Style::default().fg(Color::LightGreen)));
                    }
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
            KeyCode::Down if !self.follow_cip => self.cursor_position += 1,
            KeyCode::Up if !self.follow_cip => self.cursor_position -= 1,
            KeyCode::PageDown if !self.follow_cip => self.cursor_position += 10,
            KeyCode::PageUp if !self.follow_cip => self.cursor_position -= 10,
            KeyCode::Char('f') => self.follow_cip = !self.follow_cip,
            _ => (),
        }
    }
}
