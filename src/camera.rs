use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;

use crate::IsSelected;

use super::map::{Map, MapHandle};

#[derive(Component)]
pub struct Follow;

pub fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn move_camera(
    mut query: Query<(&mut Transform, &mut Projection), (With<Camera2d>, Without<Follow>)>,
    map: Res<MapHandle>,
    maps: ResMut<Assets<Map>>,
) {
    let Some(map) = maps.get(map.0.id()) else {
        println!("Unable to get the map to setup the camera");
        return;
    };
    let Ok((mut camera_transform, camera_projection)) = query.single_mut() else {
        println!("Unable to get the camera to set its position");
        return;
    };

    let map_size = map.size;
    camera_transform.translation = Vec3::new(
        map_size.0 as f32 * map.tile_size as f32 / 2.0,
        map_size.1 as f32 * map.tile_size as f32 / 2.0,
        10.0,
    );

    if let Projection::Orthographic(ref mut orthographic) = *camera_projection.into_inner() {
        orthographic.scale = -18.0;
    }
}

/// Allows user to zoom in/out if the camera is not in follow mode
pub fn update_camera_zoom(
    mut query: Query<&mut Projection, (With<Camera2d>, Without<Follow>)>,
    time: Res<Time>,
    mut evr_scroll: EventReader<MouseWheel>,
) {
    use bevy::input::mouse::MouseScrollUnit;
    for ev in evr_scroll.read() {
        match ev.unit {
            MouseScrollUnit::Line => {
                for projection in query.iter_mut() {
                    if let Projection::Orthographic(ref mut orthographic) = *projection.into_inner()
                    {
                        orthographic.scale += ev.y * 1.0 * time.delta_secs();
                    }
                }
            }
            MouseScrollUnit::Pixel => {
                for projection in query.iter_mut() {
                    if let Projection::Orthographic(ref mut orthographic) = *projection.into_inner()
                    {
                        orthographic.scale += ev.y * 1.0 * time.delta_secs();
                    }
                }
            }
        }
    }
}

pub fn switch_camera_mode(
    mut camera_entity_follow: Query<(Entity, &mut Projection), (With<Camera2d>, With<Follow>)>,
    mut camera_entity: Query<(Entity, &mut Projection), (With<Camera2d>, Without<Follow>)>,
    kb_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
) {
    if kb_input.just_pressed(KeyCode::Space) {
        if let Ok((camera, projection)) = camera_entity_follow.single_mut() {
            commands.entity(camera).remove::<Follow>();
            if let Projection::Orthographic(ref mut orthographic) = *projection.into_inner() {
                orthographic.scale = -18.0;
            }
        }
        if let Ok((camera, projection)) = camera_entity.single_mut() {
            commands.entity(camera).insert(Follow);
            if let Projection::Orthographic(ref mut orthographic) = *projection.into_inner() {
                orthographic.scale = -1.0;
            }
        }
    }
}

pub fn update_follow_camera(
    mut camera: Query<(&mut Transform, &mut Projection), (With<Camera2d>, With<Follow>)>,
    selected_bot: Query<&Transform, (With<IsSelected>, Without<Camera2d>)>,
    time: Res<Time>,
) {
    let mut camera = if let Ok(camera) = camera.single_mut() {
        camera
    } else {
        return;
    };

    let selected_bot_transform = if let Ok(transform) = selected_bot.single() {
        transform
    } else {
        return;
    };

    camera.0.translation = camera.0.translation.lerp(
        selected_bot_transform.translation,
        time.delta_secs_f64() as f32 * 5.0,
    );
}

/// Updates the camera position if the camera is not in follow mode.
pub fn update_camera(
    mut camera: Query<(&mut Transform, &mut Projection), (With<Camera2d>, Without<Follow>)>,
    time: Res<Time>,
    kb_input: Res<ButtonInput<KeyCode>>,
) {
    let Ok((mut transform, projection)) = camera.single_mut() else {
        return;
    };

    let mut direction = Vec2::ZERO;

    if kb_input.pressed(KeyCode::KeyW) {
        direction.y += 1.;
    }

    if kb_input.pressed(KeyCode::KeyS) {
        direction.y -= 1.;
    }

    if kb_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.;
    }

    if kb_input.pressed(KeyCode::KeyD) {
        direction.x += 1.;
    }

    if let Projection::Orthographic(ref mut orthographic) = *projection.into_inner() {
        if kb_input.pressed(KeyCode::KeyE) {
            orthographic.scale += 1.;
            println!("Projection scale: {}", orthographic.scale);
        }
        if kb_input.pressed(KeyCode::KeyQ) {
            orthographic.scale -= 1.;
            println!("Projection scale: {}", orthographic.scale);
        }
    }

    // Progressively update the camera's position over time. Normalize the
    // direction vector to prevent it from exceeding a magnitude of 1 when
    // moving diagonally.
    let move_delta = direction.normalize_or_zero() * 2000.0 * time.delta_secs();
    transform.translation += move_delta.extend(0.);
}
