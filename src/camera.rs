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
    windows: ResMut<Windows>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    let mut camera_transform = camera_query.single_mut();

    // Update the camera position based on the keyboard input.
    let mut movement_delta = Vec3::new(0.0, 0.0, 0.0);
    // Up
    if keyboard.pressed(KeyCode::W) {
        movement_delta.y += 1.0;
    }
    // Down
    else if keyboard.pressed(KeyCode::S) {
        movement_delta.y -= 1.0;
    }
    // Left
    if keyboard.pressed(KeyCode::A) {
        movement_delta.x -= 1.0;
    }
    // Right
    else if keyboard.pressed(KeyCode::D) {
        movement_delta.x += 1.0;
    }

    if movement_delta != Vec3::ZERO {
        // Normalize
        movement_delta /= movement_delta.length();
        movement_delta *= CAMERA_MOVEMENT_SPEED;
    }
    camera_transform.translation += movement_delta;

    // Get the primary window.
    let window = windows.get_primary().unwrap();
    // Get the size of the window.
    let window_width = window.width();
    let window_height = window.height();

    let buffer = 16.0;
    let min_x = 0.0 + (window_width / 2.0) - buffer;
    let min_y = 0.0 + (window_height / 2.0) - buffer;
    let max_x = (MAP_WIDTH as f32 * TILE_SIZE as f32) - (window_width / 2.0) + buffer;
    let max_y = (MAP_HEIGHT as f32 * TILE_SIZE as f32) - (window_height / 2.0) + buffer;
    // println!("min_x: {}, min_y: {}, max_x: {}, max_y: {}", min_x, min_y, max_x, max_y);

    // Bound the Camera Movement
    camera_transform.translation.x = max_x.min(min_x.max(camera_transform.translation.x));
    camera_transform.translation.y = max_y.min(min_y.max(camera_transform.translation.y));

    // println!("Camera Position: {:?}", camera_transform.translation);
}

// References
// 1. Orthographic Projection Scale for Zooming
// https://docs.rs/bevy/latest/bevy/render/camera/struct.OrthographicProjection.html
