mod assets;
mod camera;
mod map;
mod player;
mod state;

use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::DefaultPlugins;
use bevy_common_assets::toml::TomlAssetPlugin;
use bevy_rapier2d::plugin::RapierContext;
use bevy_rapier2d::prelude::*;
use state::AppState;

use map::Map;
use player::components::Bot;
use player::systems as player_systems;

#[derive(Component)]
struct IsSelected;

fn gravity_setup(mut rapier_config: Query<&mut RapierConfiguration>) {
    rapier_config.single_mut().gravity = Vec2::new(0.0, 0.0);
}

fn mouse_button_events(
    mut commands: Commands,
    mut mousebtn_evr: EventReader<MouseButtonInput>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut q_selected_entity: Query<Entity, With<IsSelected>>,
    bots: Query<(), With<Bot>>,
    rapier_context: Query<&RapierContext>,
) {
    use bevy::input::ButtonState;

    let Ok(rapier_context) = rapier_context.get_single() else {
        println!("Unable to get rapier context for mouse click");
        return;
    };

    for ev in mousebtn_evr.read() {
        if ev.state == ButtonState::Pressed {
            let (camera, camera_transform) = q_camera.single();
            let Some(mouse_position) = q_windows
                .single()
                .cursor_position()
                .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor).ok())
            else {
                continue;
            };

            rapier_context.intersections_with_point(
                mouse_position,
                QueryFilter::default(),
                |entity| {
                    if bots.get(entity).is_ok() {
                        commands.entity(entity).insert(IsSelected);
                        if let Ok(previously_selected) = q_selected_entity.get_single_mut() {
                            commands.entity(previously_selected).remove::<IsSelected>();
                        }
                    }

                    // Return `false` to stop searching for other colliders containing this point.
                    false
                },
            );
        }
    }
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            RapierDebugRenderPlugin::default(),
            TomlAssetPlugin::<Map>::new(&["map.toml"]),
        ))
        .insert_resource(Time::<Fixed>::from_hz(120.0))
        .init_asset::<machine::prelude::Program>()
        .init_asset_loader::<assets::ProgramLoader>()
        .init_state::<AppState>()
        .add_systems(
            Startup,
            (camera::camera_setup, gravity_setup, map::setup_map),
        )
        .add_systems(Update, (map::spawn_map).run_if(in_state(AppState::Loading)))
        .add_systems(
            OnEnter(AppState::Running),
            (camera::move_camera, player_systems::setup),
        )
        .add_systems(
            Update,
            (
                camera::update_camera_zoom,
                camera::update_camera,
                camera::switch_camera_mode,
                camera::update_follow_camera,
            ),
        )
        .add_systems(
            FixedUpdate,
            ((
                player_systems::attach_program_to_player,
                player_systems::update_player,
                player_systems::update_health,
                mouse_button_events,
            )
                .run_if(in_state(AppState::Running)),),
        )
        .run();
}
