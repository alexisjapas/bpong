use bevy::prelude::*;

use crate::constants::*;

#[derive(Component)]
pub struct PlayerLeft;

#[derive(Component)]
pub struct PlayerRight;

#[derive(Component)]
pub struct Health(pub u32);

#[derive(Component)]
pub struct InGameEntity;

#[derive(Component)]
pub struct PaddleControls {
    pub up: KeyCode,
    pub down: KeyCode,
}

pub fn spawn_players(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn((
        PlayerLeft,
        Health(INIT_HEALTH),
        PaddleControls {
            up: KeyCode::KeyW,
            down: KeyCode::KeyS,
        },
        Transform::from_xyz(-DEMI_SCREEN_WIDTH + PADDLE_WIDTH, 0.0, 0.0),
        Sprite::from_image(asset_server.load("imgs/paddle.png")),
        InGameEntity,
    ));
    commands.spawn((
        PlayerRight,
        Health(INIT_HEALTH),
        PaddleControls {
            up: KeyCode::ArrowUp,
            down: KeyCode::ArrowDown,
        },
        Transform::from_xyz(DEMI_SCREEN_WIDTH - PADDLE_WIDTH, 0.0, 0.0),
        Sprite::from_image(asset_server.load("imgs/paddle.png")),
        InGameEntity,
    ));
}

pub fn move_players(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &PaddleControls)>,
) {
    for (mut transform, controls) in query.iter_mut() {
        if input.pressed(controls.up) {
            transform.translation.y = (transform.translation.y + PADDLE_SPEED * time.delta_secs())
                .clamp(
                    -DEMI_SCREEN_HEIGHT + DEMI_PADDLE_HEIGHT,
                    DEMI_SCREEN_HEIGHT - DEMI_PADDLE_HEIGHT,
                );
        }
        if input.pressed(controls.down) {
            transform.translation.y = (transform.translation.y - PADDLE_SPEED * time.delta_secs())
                .clamp(
                    -DEMI_SCREEN_HEIGHT + DEMI_PADDLE_HEIGHT,
                    DEMI_SCREEN_HEIGHT - DEMI_PADDLE_HEIGHT,
                );
        }
    }
}
