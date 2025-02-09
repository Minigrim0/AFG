use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::player::components::Crashed;
use crate::{map::MapHandle, Map};
use machine::prelude::{Program, VirtualMachine};

use super::components::{Bot, BotClass, Gun, GunType, Health};
use super::entities::{PlayerBundle, ProgramHandle};
use super::utils::compute_rays;

// System to setup the player entity
pub fn setup(
    mut commands: Commands,
    map: Res<MapHandle>,
    maps: ResMut<Assets<Map>>,
    asset_server: Res<AssetServer>,
) {
    let program = asset_server.load("programs/new_turn.asmfg");
    for index in 0..1 {
        let spawn_position = if let Some(map) = maps.get(map.0.id()) {
            let possibilities = if index % 2 == 0 {
                map.spawn_places.0
            } else {
                map.spawn_places.1
            };
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
                .insert(super::components::ProgramLoaded);
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
    rapier_context: Query<&RapierContext>,
    mut gizmos: Gizmos,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for (entity, bot, mut vm, mut transform, mut vel) in query.iter_mut() {
        if let Err(e) = vm.tick() {
            // The bot crashed or completed its execution
            println!("Oh noes {}", e);
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

        if let Ok(context) = rapier_context.get_single() {
            let rays = compute_rays((bot, transform, entity), context, &mut gizmos);
            vm.update_rays(rays);
        }
    }
}

/// System to update the health sprite of the bots
pub fn update_health(time: Res<Time>, mut bot_query: Query<(&mut Health, &Transform), With<Bot>>) {
    for (mut health, transform) in bot_query.iter_mut() {
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
