use bevy::prelude::*;

use crate::virtual_machine::VirtualMachine;

use super::entities::{Player, PlayerBundle};

use super::components::{Gun, GunType, Health};

// System to setup the player entity
pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn the player entity with all its components
    commands.spawn(PlayerBundle {
        virtual_machine: VirtualMachine::new(),
        health: Health::new(100),
        gun: Gun::new(GunType::Pistol),
        sprite: Sprite::from_image(
            asset_server.load("sprites/soldier.png")
        ),
        transform: Transform::IDENTITY
    });
}

pub fn update_player(mut query: Query<(&mut VirtualMachine, &mut Transform)>) {
    println!("Updating player's virtual machine");
}
