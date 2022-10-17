use bevy::prelude::*;

use super::{MAP_HEIGHT, MAP_WIDTH, TILE_SIZE};

const CAMERA_MOVEMENT_SPEED: f32 = 10.0;

/// === Startup Systems ===
pub fn setup_camera(mut commands: Commands) {
    let x = MAP_WIDTH as f32 / 2.0 * TILE_SIZE as f32;
    let y = MAP_HEIGHT as f32 / 2.0 * TILE_SIZE as f32;
    let position = Transform::from_xyz(x, y, 1000.0);
    commands
        .spawn_bundle(Camera2dBundle {
            transform: position,
            ..default()
        })
        .insert(OrthographicProjection { ..default() })
        .insert(Name::new("Camera"));
}

/// === Systems ===
// TODO: Make sure this works frame rate independently with a fixed time step.
pub fn camera_movement_system(
    keyboard: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    let mut camera_transform = camera_query.single_mut();

    // Update the camera position based on the keyboard input.
    let mut movement_delta = Vec3::new(0.0, 0.0, 0.0);
    // Up
    if keyboard.pressed(KeyCode::W) || keyboard.pressed(KeyCode::Up) {
        movement_delta.y += 1.0;
    }
    // Down
    else if keyboard.pressed(KeyCode::S) || keyboard.pressed(KeyCode::Down) {
        movement_delta.y -= 1.0;
    }
    // Left
    if keyboard.pressed(KeyCode::A) || keyboard.pressed(KeyCode::Left) {
        movement_delta.x -= 1.0;
    }
    // Right
    else if keyboard.pressed(KeyCode::D) || keyboard.pressed(KeyCode::Right) {
        movement_delta.x += 1.0;
    }

    if movement_delta != Vec3::ZERO {
        // Normalize
        movement_delta /= movement_delta.length();
        movement_delta *= CAMERA_MOVEMENT_SPEED;
    }
    camera_transform.translation += movement_delta;
}

// References
// 1. Orthographic Projection Scale for Zooming
// https://docs.rs/bevy/latest/bevy/render/camera/struct.OrthographicProjection.html
