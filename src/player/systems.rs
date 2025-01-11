use bevy::color::palettes::css::GREEN;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::virtual_machine::VirtualMachine;

use super::entities::PlayerBundle;

use super::components::{Gun, GunType, Health};

// System to setup the player entity
pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn the player entity with all its components
    commands.spawn(PlayerBundle {
        virtual_machine: VirtualMachine::new(),
        health: Health::new(100),
        gun: Gun::new(GunType::Pistol),
        sprite: Sprite::from_image(asset_server.load("sprites/soldier.png")),
        transform: Transform::from_xyz(0.0, -500.0, 0.0),
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
