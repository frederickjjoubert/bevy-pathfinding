use super::{GameState, MapUpdatedEvent, Position};
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{Map, MAP_HEIGHT, MAP_WIDTH};

pub const TILE_SIZE: i32 = 32;

/// === Helper Functions ===
#[must_use]
pub fn index_to_world_position(x: i32, y: i32) -> Vec2 {
    let x_offset = (x * TILE_SIZE) + TILE_SIZE / 2;
    let y_offset = (y * TILE_SIZE) + TILE_SIZE / 2;
    Vec2::new(x_offset as f32, y_offset as f32)
}

#[must_use]
pub fn world_position_to_index(position: Vec2) -> (i32, i32) {
    let x_index = position.x / TILE_SIZE as f32;
    let y_index = position.y / TILE_SIZE as f32;
    (x_index as i32, y_index as i32)
}

pub fn setup_tilemap(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("Setup TileMap...");
    let tilemap_size = TilemapSize {
        x: MAP_WIDTH as u32,
        y: MAP_HEIGHT as u32,
    };
    let tilemap_entity = commands.spawn().id(); // Need one per layer.
    let mut tile_storage = TileStorage::empty(tilemap_size); // Need one per tilemap_entity.

    // Spawn the elements of the tilemap.
    for y in 0..tilemap_size.y {
        for x in 0..tilemap_size.x {
            let world_position = index_to_world_position(x as i32, y as i32);
            let tile_position = TilePos { x, y };
            let tile_entity = commands
                .spawn()
                .insert(Name::new(format!("Tile: {}, {}", x, y)))
                .insert_bundle(TileBundle {
                    position: tile_position,
                    tilemap_id: TilemapId(tilemap_entity),
                    ..default()
                })
                .insert_bundle(TransformBundle {
                    local: Transform::from_xyz(world_position.x, world_position.y, 0.),
                    global: Default::default(),
                })
                .insert(RigidBody::Fixed)
                .insert(Collider::cuboid(
                    TILE_SIZE as f32 / 2.0,
                    TILE_SIZE as f32 / 2.0,
                ))
                .id();
            tile_storage.set(&tile_position, Some(tile_entity));
        }
    }

    let tile_size = TilemapTileSize {
        x: TILE_SIZE as f32,
        y: TILE_SIZE as f32,
    };
    let image_handle: Handle<Image> = asset_server.load("sprites/tiles.png");

    let mut tilemap_entity_transform =
        bevy_ecs_tilemap::helpers::get_centered_transform_2d(&tilemap_size, &tile_size, 0.0);
    tilemap_entity_transform.translation.x += (MAP_WIDTH / 2) as f32;
    tilemap_entity_transform.translation.y += (MAP_HEIGHT / 2) as f32;

    commands
        .entity(tilemap_entity)
        .insert(Name::new("Tilemap".to_string()))
        .insert_bundle(TilemapBundle {
            grid_size: TilemapGridSize {
                x: TILE_SIZE as f32,
                y: TILE_SIZE as f32,
            },
            size: tilemap_size,
            storage: tile_storage,
            texture: TilemapTexture(image_handle),
            tile_size,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        });
}

pub fn draw_tilemap(
    mut map_updated_event_reader: EventReader<MapUpdatedEvent>,
    tile_storage_query: Query<&TileStorage>,
    mut tile_texture_query: Query<&mut TileTexture>,
    map: Res<Map>,
    game_state: Res<GameState>,
) {
    for map_updated_event in map_updated_event_reader.iter() {
        println!("Drawing tilemap...");
        if let Ok(tile_storage) = tile_storage_query.get_single() {
            for j in 0..map.height {
                for i in 0..map.width {
                    let tile_position = TilePos::new(i as u32, j as u32);
                    if let Some(tile_entity) = tile_storage.get(&tile_position) {
                        if let Ok(mut tile_texture) = tile_texture_query.get_mut(tile_entity) {
                            let index = map.xy_idx(i, j);
                            if map.blocked[index] {
                                tile_texture.0 = 2;
                            } else {
                                tile_texture.0 = 1;
                            }
                        }
                    }
                }
            }
            for point in &game_state.path {
                let tile_position = TilePos::new(point.0 as u32, point.1 as u32);
                if let Some(tile_entity) = tile_storage.get(&tile_position) {
                    if let Ok(mut tile_texture) = tile_texture_query.get_mut(tile_entity) {
                        tile_texture.0 = 5;
                    }
                }
            }
            let start: &Position = &game_state.start;
            let tile_position = TilePos::new(start.0 as u32, start.1 as u32);
            if let Some(tile_entity) = tile_storage.get(&tile_position) {
                if let Ok(mut tile_texture) = tile_texture_query.get_mut(tile_entity) {
                    tile_texture.0 = 3;
                }
            }
            let goal: &Position = &game_state.goal;
            let tile_position = TilePos::new(goal.0 as u32, goal.1 as u32);
            if let Some(tile_entity) = tile_storage.get(&tile_position) {
                if let Ok(mut tile_texture) = tile_texture_query.get_mut(tile_entity) {
                    tile_texture.0 = 4;
                }
            }
        }
    }
}
