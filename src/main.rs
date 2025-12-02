mod assets;
mod camera;
mod editor;
mod map;
mod player;
mod state;

#[cfg(debug_assertions)]
mod debug;

use bevy_egui::{egui, EguiContextPass, EguiContexts, EguiPlugin};
use bevy::prelude::*;
use bevy::DefaultPlugins;
use bevy_common_assets::toml::TomlAssetPlugin;
use bevy_rapier2d::prelude::*;
use state::AppState;

use editor::{afg_code_editor_system, AfgSourceCode};
use map::Map;

use crate::player::PlayerPlugin;

fn gravity_setup(mut rapier_config: Query<&mut RapierConfiguration>) {
    if let Ok(mut rconfig) = rapier_config.single_mut() {
        rconfig.gravity = Vec2::new(0.0, 0.0);
    }
}

fn main() {
    let mut app: App = App::new();
    app.add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            RapierDebugRenderPlugin::default(),
            TomlAssetPlugin::<Map>::new(&["map.toml"]),
        ))
        .add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: true,
        })
        .add_plugins(PlayerPlugin)
        .insert_resource(Time::<Fixed>::from_hz(120.0))
        .init_asset::<machine::prelude::Program>()
        .init_asset_loader::<assets::ProgramLoader>()
        .init_state::<AppState>()
        .add_systems(
            Startup,
            (camera::camera_setup, gravity_setup, map::setup_map),
        )
        .add_systems(
            OnEnter(AppState::Running),
            camera::move_camera
        )
        .add_systems(Update, (map::spawn_map).run_if(in_state(AppState::Loading)))
        .insert_resource(AfgSourceCode::default())
        .add_systems(EguiContextPass, afg_code_editor_system)
        .add_systems(
            Update,
            (
                camera::update_camera_zoom,
                camera::update_camera,
                camera::switch_camera_mode,
                camera::update_follow_camera,
            ),
        );

    #[cfg(debug_assertions)]
    app.add_plugins(debug::DebugPlugin);

    app.run();
}
