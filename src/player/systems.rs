use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;
use rand::Rng;

// use log;

use crate::player::components::{Crashed, IsSelected, SpawnPlace};
use crate::{map::MapHandle, Map};
use machine::prelude::{Program, VirtualMachine};

use super::components::{Bot, BotClass, Health};
use super::entities::{PlayerBundle, ProgramHandle};
use super::utils::compute_rays;

// System to setup the player entity
pub fn setup(
    mut commands: Commands,
    map: Res<MapHandle>,
    maps: ResMut<Assets<Map>>,
    asset_server: Res<AssetServer>,
) {
    let program = asset_server.load("programs/move_and_turn.asmfg");
    for index in 0..10 {
        let spawn_position = if let Some(map) = maps.get(map.0.id()) {
            let possibilities = if index % 2 == 0 {
                map.spawn_places.0
            } else {
                map.spawn_places.1
            };
            println!(
                "Possibilities are x: {}-{}, y: {}-{}",
                possibilities.0, possibilities.2, possibilities.1, possibilities.3
            );

            (
                rand::thread_rng().gen_range(possibilities.0..possibilities.0 + possibilities.2)
                    as f32
                    * map.tile_size as f32
                    + map.tile_size as f32 / 2.0,
                rand::thread_rng().gen_range(possibilities.1..possibilities.1 + possibilities.3)
                    as f32
                    * map.tile_size as f32
                    + map.tile_size as f32 / 2.0,
            )
        } else {
            (0.0, 0.0)
        };
        println!(
            "Spawning bot {index} at position ({}, {})",
            spawn_position.0, spawn_position.1
        );

        // Spawn the player entity with all its components
        commands.spawn(PlayerBundle {
            bot: Bot {
                class: BotClass::new_basic(),
                team_nr: index % 2,
            },
            virtual_machine: VirtualMachine::new(),
            program_handle: ProgramHandle(program.clone()),
            sprite: Sprite::from_image(asset_server.load("sprites/soldier.png")),
            transform: Transform::from_xyz(spawn_position.0, spawn_position.1, 0.0),
            spawn_place: SpawnPlace(Vec3::new(spawn_position.0, spawn_position.1, 0.0)),
            collider: Collider::ball(25.0),
            body: RigidBody::Dynamic,
            velocity: Velocity::default(),
        });
    }
}

pub fn attach_program_to_player(
    mut query: Query<(Entity, &mut VirtualMachine, &ProgramHandle)>,
    programs: Res<Assets<Program>>,
    mut commands: Commands,
) {
    for (entity, mut machine, program) in query.iter_mut() {
        if let Some(program) = programs.get(&program.0) {
            machine.load_program(program.instructions.clone());
            commands
                .entity(entity)
                .remove::<ProgramHandle>()
                .insert(super::components::ProgramLoaded)
                .insert(machine::prelude::VirtualMachineMetaData::new(
                    program.textual_instructions.clone(),
                ));
        }
    }
}

pub fn update_player(
    mut query: Query<
        (
            Entity,
            &Bot,
            &mut VirtualMachine,
            &mut Transform,
            &mut Velocity,
        ),
        (Without<Crashed>, With<super::components::ProgramLoaded>),
    >,
    rapier_context: ReadRapierContext,
    mut gizmos: Gizmos,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let Ok(rapier_context) = rapier_context.single() else {
        error!("Can't get rapier context.");
        return;
    };

    for (entity, bot, mut vm, mut transform, mut vel) in query.iter_mut() {
        if let Err(e) = vm.tick() {
            // The bot crashed or completed its execution
            error!("Oh noes {}", e);
            commands
                .entity(entity)
                .insert(Crashed)
                .remove::<Sprite>()
                .insert(Sprite::from_image(
                    asset_server.load("sprites/soldier-dead.png"),
                ));
            return;
        }
        vm.update_mmp(&mut transform, &mut vel);

        let rays = compute_rays((bot, transform, entity), &rapier_context, &mut gizmos);
        vm.update_rays(rays);
    }
}

/// System to update the health sprite of the bots
pub fn update_health(time: Res<Time>, mut bot_query: Query<(&mut Health, &Transform), With<Bot>>) {
    for (mut health, _transform) in bot_query.iter_mut() {
        if let Some(regen_timer) = &mut health.no_regen_timer {
            // Wait for the timer to expire to regenerate
            regen_timer.tick(time.delta());

            // if it finished, despawn the bomb
            if regen_timer.finished() {
                health.no_regen_timer = None;
            }
        } else if health.current < health.max {
            // Regen
            health.current += health.regen_rate as f32 * time.delta_secs()
        }

        // health.foreground_sprite.
    }
}

/// Handles selecting bots on the board
pub fn mouse_button_events(
    mut commands: Commands,
    mut mousebtn_evr: EventReader<MouseButtonInput>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut q_selected_entity: Query<Entity, With<IsSelected>>,
    bots: Query<(), With<Bot>>,
    rapier_context: ReadRapierContext,
) {
    use bevy::input::ButtonState;

    let Ok(rapier_context) = rapier_context.single() else {
        println!("Unable to get rapier context for mouse click");
        return;
    };

    for ev in mousebtn_evr.read() {
        if ev.state == ButtonState::Pressed {
            let Ok((camera, camera_transform)) = q_camera.single() else {
                continue;
            };
            let Ok(Some(mouse_position)) = q_windows.single().map(|window| {
                if let Some(cursor_position) = window.cursor_position() {
                    camera
                        .viewport_to_world_2d(camera_transform, cursor_position)
                        .ok()
                } else {
                    None
                }
            }) else {
                continue;
            };

            rapier_context.intersections_with_point(
                mouse_position,
                QueryFilter::default(),
                |entity| {
                    if bots.get(entity).is_ok() {
                        if let Ok(previously_selected) = q_selected_entity.single_mut() {
                            commands.entity(previously_selected).remove::<IsSelected>();
                        }
                        info!("Selecting bot {}", entity.index());
                        commands.entity(entity).insert(IsSelected);
                    }

                    // Return `false` to stop searching for other colliders containing this point.
                    false
                },
            );
        }
    }
}
