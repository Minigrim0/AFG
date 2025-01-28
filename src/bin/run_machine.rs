use afg::virtual_machine::VirtualMachine;
use bevy::asset::AssetLoader;

fn main() {
    let machine = VirtualMachine::new();

    let program = AssetLoader::load("programs/new_turn.asmfg").unwrap();

    machine.with_program(program)
}
