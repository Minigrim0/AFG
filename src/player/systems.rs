use std::f32::consts::PI;

use bevy::color::palettes::css::{BLUE, GREEN, RED};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::virtual_machine::{assets::Program, VirtualMachine};
use crate::{map::MapHandle, Map};

use super::components::{Bot, Gun, GunType, Health};
use super::entities::PlayerBundle;

#[derive(Component)]
pub struct DebugMachineText;

#[derive(Component)]
pub struct StackText;

#[derive(Component)]
pub struct RegisterText;

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

        let player_program: Handle<Program> = asset_server.load("programs/new_turn.asmfg");

        // Spawn the player entity with all its components
        commands.spawn(PlayerBundle {
            bot: Bot,
            virtual_machine: VirtualMachine::new().with_program(player_program),
            health: Health::new(100),
            gun: Gun::new(GunType::Pistol),
            sprite: Sprite::from_image(asset_server.load("sprites/soldier.png")),
            transform: Transform::from_xyz(spawn_position.0, spawn_position.1, 0.0),
            collider: Collider::ball(25.0),
            body: RigidBody::Dynamic,
            velocity: Velocity::default(),
        }).insert(super::super::IsSelected);
    }

    let font: Handle<Font> = asset_server.load("fonts/toxigenesis bd.otf");
    let text_font = TextFont {
        font: font.clone(),
        font_size: 50.0,
        ..default()
    };
    commands.spawn((
        Text2d::new("translation"),
        text_font.clone(),
        TextLayout::new_with_justify(JustifyText::Left),
        StackText
    ));

    commands.spawn((
        Text2d::new("translation"),
        text_font.clone(),
        TextLayout::new_with_justify(JustifyText::Left),
        DebugMachineText
    ));

    commands.spawn((
        Text2d::new("translation"),
        text_font.clone(),
        TextLayout::new_with_justify(JustifyText::Left),
        RegisterText
    ));
}

pub fn update_player(
    mut query: Query<(Entity, &mut VirtualMachine, &mut Transform, &mut Velocity)>,
    rapier_context: Query<&RapierContext>,
    programs: Res<Assets<Program>>,
    mut gizmos: Gizmos,
) {
    let view_angle = 120.0 * PI / 180.0;
    let ray_amount = 7;
    let viewing_distance = 2000.0;

    for (current_bot, mut vm, mut transform, mut vel) in query.iter_mut() {
        if let Err(e) = vm.tick(&programs) {
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

pub fn debug_current_instruction(
    mut query: Query<(&mut Transform, &mut Text2d), With<DebugMachineText>>,
    vm_query: Query<&VirtualMachine, With<super::super::IsSelected>>,
    q_camera: Query<&GlobalTransform, With<Camera>>,
    programs: Res<Assets<Program>>,
) {
    let machine = vm_query.single();

    for (mut transform, mut text) in query.iter_mut() {
        transform.translation = q_camera.single().translation() + Transform::from_xyz(0.0, 500.0, 0.0).translation;
        text.0 = format!("Current Instruction\n{}", machine.get_current_instruction(&programs));
    }
}


pub fn debug_stack_frame(
    mut query: Query<(&mut Transform, &mut Text2d), With<StackText>>,
    vm_query: Query<&VirtualMachine, With<super::super::IsSelected>>,
    q_camera: Query<&GlobalTransform, With<Camera>>,
) {
    let machine = vm_query.single();

    for (mut transform, mut text) in query.iter_mut() {
        transform.translation = q_camera.single().translation() + Transform::from_xyz(-300.0, 0.0, 0.0).translation;
        text.0 = format!("Stack Frame\n{}", machine.get_stack_frame());
    }
}

pub fn debug_registers(
    mut query: Query<(&mut Transform, &mut Text2d), With<RegisterText>>,
    vm_query: Query<&VirtualMachine, With<super::super::IsSelected>>,
    q_camera: Query<&GlobalTransform, With<Camera>>,
) {
    let machine = vm_query.single();

    for (mut transform, mut text) in query.iter_mut() {
        transform.translation = q_camera.single().translation() + Transform::from_xyz(200.0, 0.0, 0.0).translation;
        text.0 = format!("Registers\n{}", machine.get_registers_display());
    }
}
