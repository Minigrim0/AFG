use bevy::prelude::*;
use std::sync::{Arc, Mutex};

#[derive(Component)]
/// Metadata for a program (that is mainly the original source code and its asm equivalent).
/// By default, when writing a program in AFG, hitting compile will change the ASM equivalent.
/// If writing the program directly in ASM, the AFG version will either stay unchanged or be
/// removed.
/// TODO: Determine behaviour when writing ASM directly.
pub struct VirtualMachineMetaData {
    program: Arc<Mutex<Option<String>>>,
    program_asm: Arc<Mutex<String>>,
}

impl VirtualMachineMetaData {
    pub fn new(afg: Option<String>, asm: String) -> Self {
        Self {
            program: Arc::new(Mutex::new(afg)),
            program_asm: Arc::new(Mutex::new(asm)),
        }
    }

    // Returns the AFG source code of the program. This code is behind an
    // Arc Mutex for inner mutability.
    pub fn afg(&self) -> Arc<Mutex<Option<String>>> {
        self.program.clone()
    }

    // Returns the ASM source of the program. This code is behind an
    // Arc Mutex for inner mutability
    pub fn asm(&self) -> Arc<Mutex<String>> {
        self.program_asm.clone()
    }
}
