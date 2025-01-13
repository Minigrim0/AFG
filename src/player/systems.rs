use std::f32::consts::PI;

use bevy::color::palettes::css::{GREEN, RED, BLUE};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::virtual_machine::{assets::Program, VirtualMachine};
use crate::{map::MapHandle, Map};

use super::components::{Gun, GunType, Health, Bot};
use super::entities::PlayerBundle;

// System to setup the player entity
pub fn setup(
    mut commands: Commands,
    map: Res<MapHandle>,
    maps: ResMut<Assets<Map>>,
    asset_server: Res<AssetServer>,
) {
    let spawn_position = if let Some(map) = maps.get(map.0.id()) {
        let possibilities = map.spawn_places.0;
        println!("Spawning within {:?}", map.spawn_places);
        (
            rand::thread_rng().gen_range(possibilities.0..possibilities.0 + possibilities.2) as f32
                * map.tile_size as f32,
            rand::thread_rng().gen_range(possibilities.1..possibilities.1 + possibilities.3) as f32
                * map.tile_size as f32,
        )
    } else {
        println!("No position found");
        (0.0, 0.0)
    };

    let player_program: Handle<Program> = asset_server.load("programs/turn.csasm");

    // Spawn the player entity with all its components
    commands.spawn(PlayerBundle {
        bot: Bot,
        virtual_machine: VirtualMachine::new_with_program(player_program),
        health: Health::new(100),
        gun: Gun::new(GunType::Pistol),
        sprite: Sprite::from_image(asset_server.load("sprites/soldier.png")),
        transform: Transform::from_xyz(spawn_position.0, spawn_position.1, 0.0),
        collider: Collider::ball(25.0),
        body: RigidBody::Dynamic,
        velocity: Velocity::default(),
    });
}

pub fn update_player(
    mut query: Query<(&mut VirtualMachine, &mut Transform, &mut Velocity)>,
    rapier_context: Query<&RapierContext>,
    programs: Res<Assets<Program>>,
    mut gizmos: Gizmos,
) {
    let view_angle = 110.0 * PI / 180.0;
    let ray_amount = 7;

    for (mut vm, mut transform, mut vel) in query.iter_mut() {
        vm.tick(&programs);
        vm.update_mmp(&mut transform, &mut vel);

        if let Ok(context) = rapier_context.get_single() {
            let rays = (0..ray_amount).map(|ray_id| {
                let ray_dir = Vec2::from_angle(-(view_angle / 2.0) + ray_id as f32 * (view_angle / ray_amount as f32) + PI / 2.0);
                println!("{}) angle: {}", ray_id, ray_dir);
                if let Some((entity, toi)) = context.cast_ray(transform.translation.truncate(), ray_dir, f32::MAX, false, QueryFilter::new()) {
                    let hit_point = transform.translation.truncate() + ray_dir * toi;
                    gizmos.line(transform.translation, hit_point.extend(0.0), RED);
                    Some((entity, toi))
                } else {
                    gizmos.line(
                        transform.translation,
                        (transform.translation.truncate() + ray_dir * 200.0).extend(0.0),
                        BLUE
                    );
                    None
                }
            }).collect::<Vec<Option<(Entity, f32)>>>();
            vm.update_rays(rays);
        }
    }
}

pub fn debug_player_direction(
    mut gizmos: Gizmos,
    query: Query<(&VirtualMachine, &Transform, &Velocity)>,
) {
    for (_vm, transform, vel) in query.iter() {
        gizmos.line(
            transform.translation,
            transform.translation + vel.linvel.extend(0.0) * 50.0,
            GREEN,
        );
    }
}
