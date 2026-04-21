use avian2d::prelude::{Collider, Position, RigidBody};
use bevy::prelude::*;

use crate::constants::*;
use crate::game::shared::Speed;

#[derive(Component)]
pub struct Paddle;

#[derive(Component)]
pub struct Left;

#[derive(Component)]
pub struct Right;

#[derive(Component)]
pub struct InGameEntity;

#[derive(Component)]
pub struct PaddleControls {
    pub up: KeyCode,
    pub down: KeyCode,
}

pub fn spawn_paddles(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn((
        Paddle,
        Left,
        PaddleControls {
            up: KeyCode::KeyW,
            down: KeyCode::KeyS,
        },
        RigidBody::Kinematic,
        Collider::rectangle(PADDLE_WIDTH, PADDLE_HEIGHT),
        Transform::from_xyz(-HALF_SCREEN_WIDTH + PADDLE_WIDTH, 0.0, 0.0),
        Sprite::from_image(asset_server.load("imgs/paddle.png")),
        InGameEntity,
        Speed(PADDLE_INITIAL_SPEED),
    ));
    commands.spawn((
        Paddle,
        Right,
        PaddleControls {
            up: KeyCode::ArrowUp,
            down: KeyCode::ArrowDown,
        },
        RigidBody::Kinematic,
        Collider::rectangle(PADDLE_WIDTH, PADDLE_HEIGHT),
        Transform::from_xyz(HALF_SCREEN_WIDTH - PADDLE_WIDTH, 0.0, 0.0),
        Sprite::from_image(asset_server.load("imgs/paddle.png")),
        InGameEntity,
        Speed(PADDLE_INITIAL_SPEED),
    ));
}

pub fn move_paddles(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Position, &PaddleControls, &Speed)>,
) {
    for (mut pos, controls, speed) in query.iter_mut() {
        if input.pressed(controls.up) {
            pos.0.y = (pos.0.y + speed.0 * time.delta_secs()).clamp(
                -HALF_SCREEN_HEIGHT + HALF_PADDLE_HEIGHT,
                HALF_SCREEN_HEIGHT - HALF_PADDLE_HEIGHT,
            );
        }
        if input.pressed(controls.down) {
            pos.0.y = (pos.0.y - speed.0 * time.delta_secs()).clamp(
                -HALF_SCREEN_HEIGHT + HALF_PADDLE_HEIGHT,
                HALF_SCREEN_HEIGHT - HALF_PADDLE_HEIGHT,
            );
        }
    }
}
