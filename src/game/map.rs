use bevy::prelude::*;
use avian2d::prelude::*;

use crate::constants::*;
use crate::game::paddle::InGameEntity;

#[derive(Component)]
struct Wall;

pub fn spawn_walls(mut commands: Commands) {
    // Top-bot walls
    commands.spawn((
        Wall,
        InGameEntity,
        RigidBody::Static,
        Collider::rectangle(SCREEN_WIDTH, WALL_THICKNESS),
        Restitution::new(1.0),
        Friction::new(0.0),
        Transform::from_xyz(0.0, HALF_SCREEN_HEIGHT + HALF_WALL_THICKNESS, 0.0),
    ));
    commands.spawn((
        Wall,
        InGameEntity,
        RigidBody::Static,
        Collider::rectangle(SCREEN_WIDTH, WALL_THICKNESS),
        Restitution::new(1.0),
        Friction::new(0.0),
        Transform::from_xyz(0.0, -HALF_SCREEN_HEIGHT - HALF_WALL_THICKNESS, 0.0),
    ));
}
