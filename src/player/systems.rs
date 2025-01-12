use bevy::color::palettes::css::GREEN;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::virtual_machine::{assets::Program, VirtualMachine};
use crate::{Map, map::MapHandle};

use super::entities::PlayerBundle;
use super::components::{Gun, GunType, Health};

// System to setup the player entity
pub fn setup(
    mut commands: Commands,
    map: Res<MapHandle>,
    maps: ResMut<Assets<Map>>,
    asset_server: Res<AssetServer>
) {
    let spawn_position = if let Some(map) = maps.get(map.0.id()) {
        let possibilities = map.spawn_places.0;
        println!("Spawning within {:?}", map.spawn_places);
        (
            rand::thread_rng().gen_range(possibilities.0..possibilities.0 + possibilities.2) as f32 * map.tile_size as f32,
            rand::thread_rng().gen_range(possibilities.1..possibilities.1 + possibilities.3) as f32 * map.tile_size as f32
        )
    } else {
        println!("No position found");
        (0.0, 0.0)
    };

    let player_program: Handle<Program> = asset_server.load("programs/turn.csasm");

    // Spawn the player entity with all its components
    commands.spawn(PlayerBundle {
        virtual_machine: VirtualMachine::new(),
        health: Health::new(100),
        gun: Gun::new(GunType::Pistol),
        sprite: Sprite::from_image(asset_server.load("sprites/soldier.png")),
        transform: Transform::from_xyz(spawn_position.0, spawn_position.1, 0.0),
        collider: Collider::ball(25.0),
        body: RigidBody::Dynamic,
        velocity: Velocity::default(),
    });
}

pub fn update_player(mut query: Query<(&mut VirtualMachine, &mut Transform, &mut Velocity)>) {
    for (mut vm, mut transform, mut vel) in query.iter_mut() {
        vm.tick();
        vm.update_mmp(&mut transform, &mut vel);
    }
}

pub fn debug_player_direction(mut gizmos: Gizmos, query: Query<(&VirtualMachine, &Transform, &Velocity)>) {
    for(_vm, transform, vel) in query.iter() {
        gizmos.line(transform.translation, transform.translation + vel.linvel.extend(0.0) * 50.0, GREEN);
    }
}
