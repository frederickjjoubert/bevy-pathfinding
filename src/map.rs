use bevy::prelude::*;

pub const MAP_WIDTH: i32 = 64;
pub const MAP_HEIGHT: i32 = 64;

/// === Events ===
pub struct MapUpdatedEvent {}

/// === Resources ===
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
            costs: vec![Some(1); (width * height) as usize],
            blocked: vec![false; (width * height) as usize],
            allow_diagonals,
        }
    }

    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    pub fn get_successors(&self, position: &Position, allow_diagonals: bool) -> Vec<Successor> {
        let mut successors = Vec::new();

        for dy in -1..=1 {
            for dx in -1..=1 {
                let x = position.0 + dx;
                let y = position.1 + dy;
                if dx == 0 && dy == 0 {
                    continue;
                } // Exclude current position.
                if !allow_diagonals {
                    if (dx + dy).abs() != 1 {
                        continue;
                    } // Exclude diagonals.
                }
                if x < 0 || x > self.width - 1 {
                    continue;
                } // Make sure we are within width bounds.
                if y < 0 || y > self.height - 1 {
                    continue;
                } // Make sure we are within height bounds.

                let neighbor_position = Position(x, y);
                let neighbor_index = self.xy_idx(x, y);
                if self.blocked[neighbor_index] {
                    continue;
                }
                let neighbor_cost = self.costs[neighbor_index];
                if let Some(neighbor_cost) = neighbor_cost {
                    successors.push(Successor {
                        position: neighbor_position,
                        cost: neighbor_cost,
                    })
                } else {
                    successors.push(Successor {
                        position: neighbor_position,
                        cost: 1,
                    })
                }
            }
        }
        successors
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Position(pub i32, pub i32);

impl Position {
    pub fn distance(&self, other: &Position) -> i32 {
        (self.0.abs_diff(other.0) + self.1.abs_diff(other.1)) as i32
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd)]
pub struct Successor {
    pub position: Position,
    pub cost: i32,
}

/// === Systems ===
pub fn setup_map(mut commands: Commands) {
    println!("Setup Map...");
    let map = Map::new(MAP_WIDTH, MAP_HEIGHT, false);
    commands.insert_resource(map);
}
