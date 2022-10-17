use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn setup_physics(mut rapier_config: ResMut<RapierConfiguration>) {
    // Set Gravity to 0.0
    rapier_config.gravity = Vec2::ZERO;
}

// References:
// 1. Rapier Gravity
// https://rapier.rs/docs/user_guides/bevy_plugin/rigid_bodies#gravity
