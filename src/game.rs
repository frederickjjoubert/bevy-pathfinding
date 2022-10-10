use bevy::prelude::*;
use pathfinding::prelude::{bfs};
use crate::MapUpdatedEvent;

use super::{Map, Position};

#[derive(Debug)]
pub enum PathfindingMode {
    Astar,
    BFS,
    Dijkstra,
}

#[derive(Debug)]
pub enum MouseMode {
    Open,
    Obstacle,
    IncreaseCost,
    DecreaseCost,
}

#[derive(Debug)]
pub struct GameState {
    pub pathfinding_mode: PathfindingMode,
    pub mouse_mode: MouseMode,
    pub start: Position,
    pub goal: Position,
}

pub fn setup_game(mut commands: Commands) {
    println!("Setup Game...");
    commands.insert_resource(GameState {
        pathfinding_mode: PathfindingMode::BFS,
        mouse_mode: MouseMode::Obstacle,
        start: Position(0, 2),
        goal: Position(4, 2),
    });
}

pub fn solve(
    mut map_updated_event_writer: EventWriter<MapUpdatedEvent>,
    game_state: Res<GameState>,
    keyboard: ResMut<Input<KeyCode>>,
    map: Res<Map>)
{
    if keyboard.just_pressed(KeyCode::Space) {
        println!("Attempting to solve...");
        let start = game_state.start;
        let goal = game_state.goal;
        let result = bfs(
            &start,
            |p| map.get_successors(p).iter().map(|successor| successor.position).collect::<Vec<_>>(),
            |p| *p == goal);
        let result = result.expect("No path found");
        println!("Result: {:?}", result);
        map_updated_event_writer.send(MapUpdatedEvent { path: result });
    }
}