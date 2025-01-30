use crossterm::event::KeyEvent;
use machine::prelude::VirtualMachine;
use ratatui::{layout::Rect, Frame};

mod instruction_block;
mod machine_output;
mod machine_status;
mod register_block;
mod stack_block;

pub trait AppBlock {
    fn draw(
        &mut self,
        frame: &mut Frame,
        machine: &mut VirtualMachine,
        is_selected: bool,
        area: &Rect,
    );
    fn on_key(&mut self, key: KeyEvent);
}

pub use instruction_block::InstructionsBlock;
pub use machine_output::MachineOutputBlock;
pub use machine_status::MachineStatusBlock;
pub use register_block::RegisterBlock;
pub use stack_block::StackBlock;
