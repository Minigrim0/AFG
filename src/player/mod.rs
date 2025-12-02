use bevy::prelude::*;

pub mod components;
pub mod entities;
mod systems;
mod utils;

use super::state::AppState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::Running),
            systems::setup,
        ).add_systems(
            FixedUpdate,
            (
                systems::attach_program_to_player,
                systems::update_player,
                systems::update_health,
                systems::mouse_button_events,
            )
            .run_if(in_state(AppState::Running))
        );
    }
}