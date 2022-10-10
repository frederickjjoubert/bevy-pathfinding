use bevy::prelude::*;
// use pathfinding::prelude::{astar, bfs, dijkstra};

pub const MAP_WIDTH: i32 = 10;
pub const MAP_HEIGHT: i32 = 10;

/// === Events ===
pub struct MapUpdatedEvent {
    pub(crate) path: Vec<Position>,
}

#[derive(Debug)]
pub struct Map {
    pub width: i32,
    pub height: i32,
    pub costs: Vec<Option<i32>>,
    pub blocked: Vec<bool>,
    pub allow_diagonals: bool,
}

impl Map {
    pub fn new(width: i32, height: i32, allow_diagonals: bool) -> Map {
        Map {
            width,
            height,
            costs: vec![None; (width * height) as usize],
            blocked: vec![false; (width * height) as usize],
            allow_diagonals,
        }
    }

    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    pub fn get_successors(&self, position: &Position) -> Vec<Successor> {
        let mut successors = Vec::new();

        for dy in -1..=1 {
            for dx in -1..=1 {
                let x = position.0 + dx;
                let y = position.1 + dy;
                if dx == 0 && dy == 0 { continue; } // Exclude current position.
                if !self.allow_diagonals {
                    if (dx + dy).abs() != 1 { continue; } // Exclude diagonals.
                }
                if x < 0 || x > self.width - 1 { continue; } // Make sure we are within width bounds.
                if y < 0 || y > self.height - 1 { continue; } // Make sure we are within height bounds.

                let neighbor_position = Position(x, y);
                let neighbor_index = self.xy_idx(x, y);
                if self.blocked[neighbor_index] { continue; }
                let neighbor_cost = self.costs[neighbor_index];
                successors.push(Successor { position: neighbor_position, cost: neighbor_cost })
            }
        }

        successors
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Position(pub i32, pub i32);

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd)]
pub struct Successor {
    pub position: Position,
    pub cost: Option<i32>,
}

pub fn setup_map(mut commands: Commands) {
    println!("Setup Map...");
    let mut map = Map::new(5, 5, true);
    map.blocked = vec![
        false, false, false, true, false,
        false, true, false, false, false,
        false, true, false, false, false,
        false, true, false, false, false,
        false, false, false, true, false,
    ];
    commands.insert_resource(map);
}
