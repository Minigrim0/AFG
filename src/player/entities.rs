use bevy::{
    asset::Handle,
    ecs::component::Component,
    prelude::{Bundle, Transform},
    sprite::Sprite,
};
use bevy_rapier2d::prelude::{Collider, RigidBody, Velocity};

use machine::{prelude::VirtualMachine, Program};

use crate::player::components::SpawnPlace;

use super::components::Bot;

#[derive(Component)]
pub struct ProgramHandle(pub Handle<Program>);

/// A player's bundle
#[derive(Bundle)]
pub struct PlayerBundle {
    pub bot: Bot,
    pub virtual_machine: VirtualMachine,
    pub program_handle: ProgramHandle,
    pub sprite: Sprite,
    pub transform: Transform,
    pub spawn_place: SpawnPlace,
    pub collider: Collider,
    pub body: RigidBody,
    pub velocity: Velocity,
}
