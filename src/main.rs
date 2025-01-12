mod map;
mod player;
mod state;
mod virtual_machine;
mod camera;

use bevy::prelude::*;
use bevy::DefaultPlugins;
use bevy_common_assets::toml::TomlAssetPlugin;
use bevy_rapier2d::prelude::*;
use state::AppState;

use map::Map;
use player::systems as player_systems;

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
                camera::camera_setup,
                gravity_setup,
                map::setup_map,
            ),
        )
        .add_systems(OnEnter (AppState::Level), player_systems::setup)
        .add_systems(
            Update,
            (
                player_systems::update_player.run_if(in_state(AppState::Level)),
                player_systems::debug_player_direction.run_if(in_state(AppState::Level)),
                camera::update_camera.run_if(in_state(AppState::Level)),
                camera::update_camera_zoom,
                // player_systems::print_player_velocity.run_if(in_state(AppState::Level)),
                map::spawn_map.run_if(in_state(AppState::Loading)),
            ),
        )
        .run();
}
