/// The debug window for the project
mod systems;

use bevy::prelude::*;

use bevy_egui::EguiContextPass;


pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(EguiContextPass, (
            systems::show_debug_window
        ));
    }
}