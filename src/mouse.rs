use bevy::prelude::*;
use bevy::render::camera::RenderTarget;

// === Resources ===
pub struct Mouse {
    pub is_in_window: bool,
    pub window_position: Vec2,
    pub world_position: Vec2,
    pub holding_lmb: bool,
    pub holding_mmb: bool,
    pub holding_rmb: bool,
}

// === Systems ===
pub fn setup_mouse(mut commands: Commands) {
    commands.insert_resource(Mouse {
        is_in_window: false,
        window_position: Default::default(),
        world_position: Default::default(),
        holding_lmb: false,
        holding_mmb: false,
        holding_rmb: false,
    })
}

pub fn update_mouse_position(
    mut mouse: ResMut<Mouse>,
    windows: Res<Windows>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = camera_query.single();

    // Get the window that the camera is displaying to (or the primary window)
    let window = if let RenderTarget::Window(id) = camera.target {
        windows.get(id).unwrap()
    } else {
        windows.get_primary().unwrap()
    };

    // Check if the cursor is inside or outside the window.
    if let Some(screen_position) = window.cursor_position() {
        // Cursor is inside the window.
        mouse.is_in_window = true;
        mouse.window_position = screen_position;

        // Get the size of the window
        let window_size = Vec2::new(window.width(), window.height());

        // Convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_position / window_size) * 2.0 - Vec2::ONE;

        // Matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        // Use it to convert ndc to world-space coordinates
        let world_position = ndc_to_world.project_point3(ndc.extend(-1.0));

        // Reduce it to a 2D value and assign.
        mouse.world_position = world_position.truncate();
    } else {
        // Cursor is not inside the window.
        mouse.is_in_window = false;
    }
}

pub fn update_mouse_input(mut mouse: ResMut<Mouse>, mouse_input: Res<Input<MouseButton>>) {
    // Left Mouse Button
    if mouse_input.just_pressed(MouseButton::Left) {
        mouse.holding_lmb = true;
    } else if mouse_input.just_released(MouseButton::Left) {
        mouse.holding_lmb = false;
    }
    // Middle Mouse Button
    if mouse_input.just_pressed(MouseButton::Middle) {
        mouse.holding_mmb = true;
    } else if mouse_input.just_released(MouseButton::Middle) {
        mouse.holding_mmb = false;
    }
    // Right Mouse Button
    if mouse_input.just_pressed(MouseButton::Right) {
        mouse.holding_rmb = true;
    } else if mouse_input.just_released(MouseButton::Right) {
        mouse.holding_rmb = false;
    }
}

pub fn process_mouse_events(
    mut mouse: ResMut<Mouse>,
    // mut mouse_movement_events: EventReader<CursorMoved>,
    mut mouse_entered_window_events: EventReader<CursorEntered>,
    mut mouse_left_window_events: EventReader<CursorLeft>,
) {
    // for moved_event in mouse_movement_events.iter() {
    //     println!("mouse moved.");
    // }
    for _ in mouse_entered_window_events.iter() {
        println!("mouse entered the window.");
        mouse.is_in_window = true;
    }
    for _ in mouse_left_window_events.iter() {
        println!("mouse left the window.");
        mouse.is_in_window = false;
    }
}

// References
// 1. Mouse Input
// https://bevy-cheatbook.github.io/input/mouse.html
// 2. Convert cursor window position to world coordinates.
// https://bevy-cheatbook.github.io/cookbook/cursor2world.html
