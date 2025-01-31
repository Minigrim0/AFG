use std::f32::consts::PI;

use bevy::color::palettes::css::{BLUE, GREEN, RED};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::{map::MapHandle, Map};
use machine::prelude::{Program, VirtualMachine};

use super::components::{Bot, Gun, GunType, Health};
use super::entities::{PlayerBundle, ProgramHandle};

// System to setup the player entity
pub fn setup(
    mut commands: Commands,
    map: Res<MapHandle>,
    maps: ResMut<Assets<Map>>,
    asset_server: Res<AssetServer>,
) {
    for _ in 0..1 {
        let spawn_position = if let Some(map) = maps.get(map.0.id()) {
            let possibilities = map.spawn_places.0;
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

        let program = asset_server.load("programs/new_turn.asmfg");

        // Spawn the player entity with all its components
        commands
            .spawn(PlayerBundle {
                bot: Bot,
                virtual_machine: VirtualMachine::new(),
                program_handle: ProgramHandle(program),
                health: Health::new(100),
                gun: Gun::new(GunType::Pistol),
                sprite: Sprite::from_image(asset_server.load("sprites/soldier.png")),
                transform: Transform::from_xyz(spawn_position.0, spawn_position.1, 0.0),
                collider: Collider::ball(25.0),
                body: RigidBody::Dynamic,
                velocity: Velocity::default(),
            })
            .insert(super::super::IsSelected);
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
            commands.entity(entity).remove::<ProgramHandle>();
        }
    }
}

pub fn update_player(
    mut query: Query<(Entity, &mut VirtualMachine, &mut Transform, &mut Velocity)>,
    rapier_context: Query<&RapierContext>,
    mut gizmos: Gizmos,
) {
    let view_angle = 120.0 * PI / 180.0;
    let ray_amount = 7;
    let viewing_distance = 2000.0;

    for (current_bot, mut vm, mut transform, mut vel) in query.iter_mut() {
        if let Err(e) = vm.tick() {
            println!("Oh noes {}", e);
        }
        vm.update_mmp(&mut transform, &mut vel);

        let initial_angle = transform.rotation.to_axis_angle().0.z
            * transform.rotation.to_axis_angle().1
            + (PI / 2.0);

        if let Ok(context) = rapier_context.get_single() {
            let rays = (0..ray_amount)
                .map(|ray_id| {
                    let ray_dir = Vec2::from_angle(
                        initial_angle - (view_angle / 2.0)
                            + ray_id as f32 * (view_angle / ((ray_amount - 1) as f32)),
                    );
                    if let Some((entity, toi)) = context.cast_ray(
                        transform.translation.truncate(),
                        ray_dir,
                        viewing_distance,
                        false,
                        QueryFilter::new().exclude_collider(current_bot),
                    ) {
                        let hit_point = transform.translation.truncate() + ray_dir * toi;
                        gizmos.line(transform.translation, hit_point.extend(0.0), RED);
                        Some((entity, toi))
                    } else {
                        gizmos.line(
                            transform.translation,
                            (transform.translation.truncate() + ray_dir * viewing_distance)
                                .extend(0.0),
                            BLUE,
                        );
                        None
                    }
                })
                .collect::<Vec<Option<(Entity, f32)>>>();
            vm.update_rays(rays);
        }
    }
}
