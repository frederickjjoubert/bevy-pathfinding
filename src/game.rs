use bevy::prelude::*;
use pathfinding::prelude::{astar, bfs, dijkstra};

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

// === Resources ===
#[derive(Debug)]
pub struct GameState {
    pub pathfinding_algorithm: PathfindingAlgorithm,
    pub placement_mode: PlacementMode,
    pub start: Position,
    pub goal: Position,
    pub path: Vec<Position>,
    pub step: usize,
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
    mut pathfinding_algorithm_changed_event: EventWriter<PathfindingAlgorithmChangedEvent>,
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
    pathfinding_algorithm_changed_event.send(PathfindingAlgorithmChangedEvent {});
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
    if mouse.holding_lmb {
        let (x, y) = world_position_to_index(mouse.world_position);
        let clicked_position = Position(x, y);
        // Prevent placing on start or goal:
        if clicked_position == game_state.start || clicked_position == game_state.goal {
            return;
        }
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
pub fn cost_system(
    mut user_interface_interaction_event_reader: EventReader<UserInterfaceInteractionEvent>,
    mut map_updated_event_writer: EventWriter<MapUpdatedEvent>,
    mouse: Res<Mouse>,
    mouse_input: Res<Input<MouseButton>>,
    mut game_state: ResMut<GameState>,
    mut map: ResMut<Map>,
) {
    // This is a hack to prevent placement when buttons are clicked.
    for _ in user_interface_interaction_event_reader.iter() {
        return;
    }
    if mouse_input.just_pressed(MouseButton::Left) {
        let (x, y) = world_position_to_index(mouse.world_position);
        let clicked_position = Position(x, y);
        // Prevent placing on start or goal:
        if clicked_position == game_state.start || clicked_position == game_state.goal {
            return;
        }
        match game_state.placement_mode {
            PlacementMode::IncreaseCost => {
                let index = map.xy_idx(x, y);
                if let Some(current_cost) = map.costs[index] {
                    map.costs[index] = Some(current_cost + 1);
                }
            }
            PlacementMode::DecreaseCost => {
                let index = map.xy_idx(x, y);
                if let Some(current_cost) = map.costs[index] {
                    if current_cost > 1 {
                        map.costs[index] = Some(current_cost - 1);
                    }
                }
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
        if !game_state.path.is_empty() {
            if game_state.step < game_state.path.len() - 1 {
                game_state.step = game_state.step + 1;
            } else {
                game_state.step = 2; // Why 2? Me Dumb?
            }
            map_updated_event_writer.send(MapUpdatedEvent {});
        }
    }
}

// See Reference 1
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
        match game_state.pathfinding_algorithm {
            PathfindingAlgorithm::AStar => {
                let result = astar(
                    &start,
                    |position| {
                        map.get_successors(position)
                            .iter()
                            .map(|successor| (successor.position, successor.cost))
                            .collect::<Vec<_>>()
                    },
                    |position| position.distance(&goal),
                    |position| *position == goal,
                );
                if let Some(result) = result {
                    println!("Path: {:?}", result.0);
                    println!("Cost: {:?}", result.1);
                    game_state.path = result.0;
                    game_state.step = game_state.path.len();
                } else {
                    println!("No Path Found!");
                    game_state.path = Vec::new();
                    game_state.step = 0;
                }
            }
            PathfindingAlgorithm::BFS => {
                let result = bfs(
                    &start,
                    |position| {
                        map.get_successors(position)
                            .iter()
                            .map(|successor| successor.position)
                            .collect::<Vec<_>>()
                    },
                    |position| *position == goal,
                );
                if let Some(result) = result {
                    println!("Path: {:?}", result);
                    game_state.path = result;
                    game_state.step = game_state.path.len();
                } else {
                    println!("No Path Found!");
                    game_state.path = Vec::new();
                    game_state.step = 0;
                }
            }
            PathfindingAlgorithm::Dijkstra => {
                let result = dijkstra(
                    &start,
                    |position| {
                        map.get_successors(position)
                            .iter()
                            .map(|successor| (successor.position, successor.cost))
                            .collect::<Vec<_>>()
                    },
                    |position| *position == goal,
                );
                if let Some(result) = result {
                    println!("Path: {:?}", result.0);
                    println!("Cost: {:?}", result.1);
                    game_state.path = result.0;
                    game_state.step = game_state.path.len();
                } else {
                    println!("No Path Found!");
                    game_state.path = Vec::new();
                    game_state.step = 0;
                }
            }
        }
        map_updated_event_writer.send(MapUpdatedEvent {});
    }
}

// Reset the Path Solve
pub fn reset_system(
    mut reset_event_reader: EventReader<ResetEvent>,
    mut map_updated_event_writer: EventWriter<MapUpdatedEvent>,
    mut game_state: ResMut<GameState>,
) {
    for _ in reset_event_reader.iter() {
        game_state.path = Vec::new();
        map_updated_event_writer.send(MapUpdatedEvent {});
    }
}

// Clear Everything
pub fn clear_system(
    mut clear_event_reader: EventReader<ClearEvent>,
    mut map_updated_event_writer: EventWriter<MapUpdatedEvent>,
    mut game_state: ResMut<GameState>,
    mut map: ResMut<Map>,
) {
    for _ in clear_event_reader.iter() {
        game_state.path = Vec::new();
        game_state.start = Position(16, 32);
        game_state.goal = Position(48, 32);
        map.costs = vec![Some(1); (map.width * map.height) as usize];
        map.blocked = vec![false; (map.width * map.height) as usize];
        map_updated_event_writer.send(MapUpdatedEvent {});
    }
}

pub fn change_pathfinding_algorithm_system(
    mut pathfinding_algorithm_selection_changed_event_reader: EventReader<
        PathfindingAlgorithmSelectionChangedEvent,
    >,
    mut pathfinding_algorithm_changed_event_writer: EventWriter<PathfindingAlgorithmChangedEvent>,
    mut reset_event_writer: EventWriter<ResetEvent>,
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
        reset_event_writer.send(ResetEvent {});
    }
}

// References
// 1. Pathfinding Docs
// https://docs.rs/pathfinding/latest/pathfinding/directed/astar/fn.astar.html
// https://docs.rs/pathfinding/latest/pathfinding/directed/bfs/index.html
// https://docs.rs/pathfinding/latest/pathfinding/directed/dijkstra/index.html
// 2. Pathfinding in Rust: A tutorial with examples
// https://blog.logrocket.com/pathfinding-rust-tutorial-examples/
// https://github.com/gregstoll/rust-pathfinding
