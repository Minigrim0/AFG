mod map;
mod player;
mod state;
mod virtual_machine;

use bevy::prelude::*;
use bevy::DefaultPlugins;
use bevy_common_assets::toml::TomlAssetPlugin;
use bevy_rapier2d::prelude::*;
use map::Map;
use state::AppState;

use player::systems as player_systems;

fn world_setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn gravity_setup(mut rapier_config: Query<&mut RapierConfiguration>) {
    rapier_config.single_mut().gravity = Vec2::new(0.0, 0.0);
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            RapierDebugRenderPlugin::default(),
            TomlAssetPlugin::<Map>::new(&["map.toml"]),
        ))
        .init_state::<AppState>()
        .add_systems(
            Startup,
            (
                world_setup,
                player_systems::setup,
                gravity_setup,
                map::setup_map,
            ),
        )
        .add_systems(
            Update,
            (
                player_systems::update_player.run_if(in_state(AppState::Level)),
                player_systems::debug_player_direction.run_if(in_state(AppState::Level)),
                // player_systems::print_player_velocity.run_if(in_state(AppState::Level)),
                map::spawn_map.run_if(in_state(AppState::Loading)),
            ),
        )
        .run();
}
