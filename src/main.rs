mod virtual_machine;
mod player;

use virtual_machine::VirtualMachine;

use bevy::prelude::*;
use bevy::DefaultPlugins;

use player::systems as player_systems;

fn world_setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (world_setup, player_systems::setup))
        .add_systems(Update, player_systems::update_player)
        .run();
}
