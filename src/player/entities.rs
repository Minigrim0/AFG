use bevy::{
    asset::Handle,
    ecs::component::Component,
    prelude::{Bundle, Transform},
    sprite::Sprite,
};
use bevy_rapier2d::prelude::{Collider, RigidBody, Velocity};

use machine::{prelude::VirtualMachine, Program};

use super::components::{Bot, Gun, Health};

#[derive(Component)]
pub struct ProgramHandle(pub Handle<Program>);

/// A player's bundle
#[derive(Bundle)]
pub struct PlayerBundle {
    pub bot: Bot,
    pub virtual_machine: VirtualMachine,
    pub program_handle: ProgramHandle,
    pub health: Health,
    pub gun: Gun,
    pub sprite: Sprite,
    pub transform: Transform,
    pub collider: Collider,
    pub body: RigidBody,
    pub velocity: Velocity,
}
