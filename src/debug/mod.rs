/// The debug window for the project
mod systems;
mod events;

use bevy::prelude::*;

use bevy_egui::EguiContextPass;


pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<events::DebugBotUpdate>()
            .add_systems(EguiContextPass, (
                systems::show_debug_window,
            ))
            .add_systems(Update,
                systems::bot_react_to_reset_event
        );
    }
}