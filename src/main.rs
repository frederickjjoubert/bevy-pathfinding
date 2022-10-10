#![warn(clippy::all, clippy::pedantic)]

mod camera;
mod game;
mod map;
mod mouse;
mod physics;
mod tilemap;
mod user_interface;

use bevy::prelude::*;
use bevy::render::texture::ImageSettings;
use bevy::window::PresentMode;
use bevy_ecs_tilemap::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

pub use camera::*;
pub use game::*;
pub use map::*;
pub use mouse::*;
pub use physics::*;
pub use tilemap::*;
pub use user_interface::*;

// #[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
// enum Setup {
//     Game,
//     Map,
//     DrawMap,
// }

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(WindowDescriptor {
            title: "Bevy Pathfinding".to_string(),
            width: 1600.0,
            height: 900.0,
            present_mode: PresentMode::Fifo,
            ..default()
        })
        .insert_resource(ImageSettings::default_nearest()) // prevents blurry sprites
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new()) // bevy_inspector_egui
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
            TILE_SIZE as f32,
        )) // bevy_rapier2d
        // .add_plugin(RapierDebugRenderPlugin::default())// bevy_rapier2d debugger
        .add_plugin(TilemapPlugin) // bevy_ecs_tilemap
        .add_event::<MapUpdatedEvent>()
        .add_event::<UserInterfaceInteractionEvent>()
        .add_event::<StepEvent>()
        .add_event::<SolveEvent>()
        .add_event::<ResetEvent>()
        .add_startup_system(setup_physics)
        .add_startup_system(setup_map)
        .add_startup_system(setup_tilemap)
        .add_startup_system(setup_mouse)
        .add_startup_system(setup_game)
        .add_startup_system(setup_camera)
        .add_startup_system(setup_user_interface)
        .add_system(draw_tilemap)
        .add_system(placement_system)
        .add_system(step_system)
        .add_system(solve_system)
        .add_system(reset_system)
        .add_system(camera_movement_system)
        .add_system(update_mouse)
        .add_system(process_mouse_events)
        .add_system(path_button_system)
        .add_system(obstacle_button_system)
        .add_system(start_button_system)
        .add_system(goal_button_system)
        .add_system(step_button_system)
        .add_system(solve_button_system)
        .add_system(reset_button_system)
        .run();
}
