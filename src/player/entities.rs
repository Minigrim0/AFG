use bevy::{
    prelude::{Bundle, Transform},
    sprite::Sprite,
};
use bevy_rapier2d::prelude::{Collider, RigidBody, Velocity};

use crate::virtual_machine::VirtualMachine;

use super::components::{Gun, Health, Bot};

/// A player's bundle
#[derive(Bundle)]
pub struct PlayerBundle {
    pub bot: Bot,
    pub virtual_machine: VirtualMachine,
    pub health: Health,
    pub gun: Gun,
    pub sprite: Sprite,
    pub transform: Transform,
    pub collider: Collider,
    pub body: RigidBody,
    pub velocity: Velocity,
}
