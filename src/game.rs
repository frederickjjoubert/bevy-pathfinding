use bevy::prelude::*;
use pathfinding::prelude::bfs;

use super::{
    world_position_to_index, Map, MapUpdatedEvent, Mouse, Position, UserInterfaceInteractionEvent,
};

#[derive(Clone, Copy, Debug)]
pub enum PathfindingAlgorithm {
    AStar,
    BFS,
    Dijkstra,
}

#[derive(Debug)]
pub enum PlacementMode {
    Path,
    Obstacle,
    Start,
    Goal,
    IncreaseCost,
    DecreaseCost,
}

#[derive(Debug)]
pub struct GameState {
    pub pathfinding_algorithm: PathfindingAlgorithm,
    pub placement_mode: PlacementMode,
    pub start: Position,
    pub goal: Position,
    pub path: Vec<Position>,
    pub step: i32,
}

// === Events ===
pub struct SolveEvent {}

pub struct StepEvent {}

pub struct ResetEvent {}

pub struct ClearEvent {}

pub struct CycleAlgorithmLeftEvent {}

pub struct CycleAlgorithmRightEvent {}

pub struct PathfindingAlgorithmSelectionChangedEvent {
    pub pathfinding_algorithm: PathfindingAlgorithm,
}

pub struct PathfindingAlgorithmChangedEvent {}

// === Systems ===
pub fn setup_game(
    mut commands: Commands,
    mut map_updated_event_writer: EventWriter<MapUpdatedEvent>,
) {
    println!("Setup Game...");
    commands.insert_resource(GameState {
        pathfinding_algorithm: PathfindingAlgorithm::BFS,
        placement_mode: PlacementMode::Obstacle,
        start: Position(16, 32),
        goal: Position(48, 32),
        path: Vec::new(),
        step: 0,
    });
    map_updated_event_writer.send(MapUpdatedEvent {});
}

pub fn placement_system(
    mut user_interface_interaction_event_reader: EventReader<UserInterfaceInteractionEvent>,
    mut map_updated_event_writer: EventWriter<MapUpdatedEvent>,
    mouse: Res<Mouse>,
    mut game_state: ResMut<GameState>,
    mut map: ResMut<Map>,
) {
    // This is a hack to prevent placement when buttons are clicked.
    for _ in user_interface_interaction_event_reader.iter() {
        return;
    }
    // if mouse_input.just_pressed(MouseButton::Left)
    if mouse.holding_lmb {
        let (x, y) = world_position_to_index(mouse.world_position);
        // println!("clicked index x: {}, y: {}", x, y);
        match game_state.placement_mode {
            PlacementMode::Path => {
                let index = map.xy_idx(x, y);
                map.blocked[index] = false;
            }
            PlacementMode::Obstacle => {
                let index = map.xy_idx(x, y);
                map.blocked[index] = true;
            }
            PlacementMode::Start => {
                game_state.start = Position(x, y);
            }
            PlacementMode::Goal => {
                game_state.goal = Position(x, y);
            }
            _ => {
                // Do Nothing
            }
        }
        game_state.path = Vec::new();
        map_updated_event_writer.send(MapUpdatedEvent {});
    }
}

pub fn step_system(
    mut step_event_reader: EventReader<StepEvent>,
    mut map_updated_event_writer: EventWriter<MapUpdatedEvent>,
    mut game_state: ResMut<GameState>,
) {
    for _ in step_event_reader.iter() {
        // TODO: Wrap around
        game_state.step = game_state.step + 1;
        map_updated_event_writer.send(MapUpdatedEvent {});
    }
}

pub fn solve_system(
    mut solve_event_reader: EventReader<SolveEvent>,
    mut map_updated_event_writer: EventWriter<MapUpdatedEvent>,
    mut game_state: ResMut<GameState>,
    map: Res<Map>,
) {
    for _ in solve_event_reader.iter() {
        println!("Attempting to solve...");
        let start = game_state.start;
        let goal = game_state.goal;
        let result = bfs(
            &start,
            |p| {
                map.get_successors(p)
                    .iter()
                    .map(|successor| successor.position)
                    .collect::<Vec<_>>()
            },
            |p| *p == goal,
        );

        if let Some(result) = result {
            println!("Result: {:?}", result);
            game_state.path = result;
            map_updated_event_writer.send(MapUpdatedEvent {});
        } else {
            println!("No Path Found!");
        }
    }
}

pub fn reset_system(
    mut reset_event_reader: EventReader<ResetEvent>,
    mut map_updated_event_writer: EventWriter<MapUpdatedEvent>,
    mut game_state: ResMut<GameState>,
    mut map: ResMut<Map>,
) {
    for _ in reset_event_reader.iter() {
        game_state.path = Vec::new();
        game_state.start = Position(16, 32);
        game_state.goal = Position(48, 32);
        map.costs = vec![None; (map.width * map.height) as usize];
        map.blocked = vec![false; (map.width * map.height) as usize];
        map_updated_event_writer.send(MapUpdatedEvent {});
    }
}

pub fn clear_system(
    mut clear_event_reader: EventReader<ClearEvent>,
    mut map_updated_event_writer: EventWriter<MapUpdatedEvent>,
    mut game_state: ResMut<GameState>,
    mut map: ResMut<Map>,
) {
    for _ in clear_event_reader.iter() {
        game_state.path = Vec::new();
        map.blocked = vec![false; (map.width * map.height) as usize];
        map_updated_event_writer.send(MapUpdatedEvent {});
    }
}

pub fn change_pathfinding_algorithm_system(
    mut pathfinding_algorithm_selection_changed_event_reader: EventReader<
        PathfindingAlgorithmSelectionChangedEvent,
    >,
    mut pathfinding_algorithm_changed_event_writer: EventWriter<PathfindingAlgorithmChangedEvent>,
    mut game_state: ResMut<GameState>,
) {
    for pathfinding_algorithm_selection_changed_event in
        pathfinding_algorithm_selection_changed_event_reader.iter()
    {
        match pathfinding_algorithm_selection_changed_event.pathfinding_algorithm {
            PathfindingAlgorithm::AStar => {
                game_state.pathfinding_algorithm = PathfindingAlgorithm::AStar;
            }
            PathfindingAlgorithm::BFS => {
                game_state.pathfinding_algorithm = PathfindingAlgorithm::BFS;
            }
            PathfindingAlgorithm::Dijkstra => {
                game_state.pathfinding_algorithm = PathfindingAlgorithm::Dijkstra;
            }
        }
        pathfinding_algorithm_changed_event_writer.send(PathfindingAlgorithmChangedEvent {});
    }
}
