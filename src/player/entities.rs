use bevy::{prelude::{Bundle, Transform}, sprite::Sprite};

use crate::virtual_machine::VirtualMachine;

use super::components::{Gun, Health};

// Define the player entity
pub struct Player;

/// A player's bundle
#[derive(Bundle)]
pub struct PlayerBundle {
    pub virtual_machine: VirtualMachine,
    pub health: Health,
    pub gun: Gun,
    pub sprite: Sprite,
    pub transform: Transform
}
