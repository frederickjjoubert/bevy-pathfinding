use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn setup_physics(mut rapier_config: ResMut<RapierConfiguration>) {
    // Set Gravity to 0.0
    rapier_config.gravity = Vec2::ZERO;
}
