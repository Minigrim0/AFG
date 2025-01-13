use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;

use super::map::{Map, MapHandle};

pub fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn move_camera(
    mut query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera2d>>,
    map: Res<MapHandle>,
    maps: ResMut<Assets<Map>>,
) {
    let Some(map) = maps.get(map.0.id()) else {
        println!("Unable to get the map to setup the camera");
        return;
    };
    let Ok((mut camera_transform, mut camera_projection)) = query.get_single_mut() else {
        println!("Unable to get the camera to set its position");
        return;
    };

    let map_size = map.size;
    camera_transform.translation = Vec3::new(
        map_size.0 as f32 * map.tile_size as f32 / 2.0,
        map_size.1 as f32 * map.tile_size as f32 / 2.0,
        10.0,
    );

    camera_projection.scale = -18.0;
}

pub fn update_camera_zoom(
    mut query: Query<&mut OrthographicProjection, With<Camera2d>>,
    time: Res<Time>,
    mut evr_scroll: EventReader<MouseWheel>,
) {
    use bevy::input::mouse::MouseScrollUnit;
    for ev in evr_scroll.read() {
        match ev.unit {
            MouseScrollUnit::Line => {
                for mut projection in query.iter_mut() {
                    projection.scale += ev.y * 1.0 * time.delta_secs();
                }
            }
            MouseScrollUnit::Pixel => {
                for mut projection in query.iter_mut() {
                    projection.scale += ev.y * 1.0 * time.delta_secs();
                }
            }
        }
    }
}

pub fn update_camera(
    mut camera: Query<(&mut Transform, &mut OrthographicProjection), With<Camera2d>>,
    time: Res<Time>,
    kb_input: Res<ButtonInput<KeyCode>>,
) {
    let Ok((mut transform, mut projection)) = camera.get_single_mut() else {
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

    if kb_input.pressed(KeyCode::KeyE) {
        projection.scale += 1.;
        println!("Projection scale: {}", projection.scale);
    }
    if kb_input.pressed(KeyCode::KeyQ) {
        projection.scale -= 1.;
        println!("Projection scale: {}", projection.scale);
    }

    // Progressively update the camera's position over time. Normalize the
    // direction vector to prevent it from exceeding a magnitude of 1 when
    // moving diagonally.
    let move_delta = direction.normalize_or_zero() * 2000.0 * time.delta_secs();
    transform.translation += move_delta.extend(0.);
}
