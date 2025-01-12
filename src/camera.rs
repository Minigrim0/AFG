use bevy::prelude::*;
use bevy::input::mouse::MouseWheel;

pub fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn update_camera_zoom(
    mut query: Query<&mut OrthographicProjection, With<Camera2d>>,
    time: Res<Time>,
    mut evr_scroll: EventReader<MouseWheel>
) {
    use bevy::input::mouse::MouseScrollUnit;
    for ev in evr_scroll.read() {
        match ev.unit {
            MouseScrollUnit::Line => {
                for mut projection in query.iter_mut() {
                    projection.scale -= ev.y * 0.5 * time.delta_secs();
                }
            }
            MouseScrollUnit::Pixel => {
                for mut projection in query.iter_mut() {
                    projection.scale -= ev.y * 0.5 * time.delta_secs();
                }
            }
        }
    }
}

pub fn update_camera(
    mut camera: Query<&mut Transform, With<Camera2d>>,
    time: Res<Time>,
    kb_input: Res<ButtonInput<KeyCode>>
) {
    let Ok(mut camera) = camera.get_single_mut() else {
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

    // Progressively update the camera's position over time. Normalize the
    // direction vector to prevent it from exceeding a magnitude of 1 when
    // moving diagonally.
    let move_delta = direction.normalize_or_zero() * 200.0 * time.delta_secs();
    camera.translation += move_delta.extend(0.);
}
