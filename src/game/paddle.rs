use bevy::prelude::*;

#[derive(Component)]
struct PlayerLeft;

#[derive(Component)]
struct PlayerRight;

#[derive(Component)]
struct Health(u32);

#[derive(Component)]
struct InGameEntity;

fn spawn_players(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn((
        PlayerLeft,
        Health(INIT_HEALTH),
        Transform::from_xyz(-DEMI_SCREEN_WIDTH + PADDLE_WIDTH, 0.0, 0.0),
        Sprite::from_image(asset_server.load("imgs/paddle.png")),
        InGameEntity,
    ));
    commands.spawn((
        PlayerRight,
        Health(INIT_HEALTH),
        Transform::from_xyz(DEMI_SCREEN_WIDTH - PADDLE_WIDTH, 0.0, 0.0),
        Sprite::from_image(asset_server.load("imgs/paddle.png")),
        InGameEntity,
    ));
}

fn move_player_left(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<PlayerLeft>>,
) {
    for mut pleft_transform in query.iter_mut() {
        if input.pressed(KeyCode::KeyW) {
            pleft_transform.translation.y =
                (pleft_transform.translation.y + PADDLE_SPEED * time.delta_secs()).clamp(
                    -DEMI_SCREEN_HEIGHT + DEMI_PADDLE_HEIGHT,
                    DEMI_SCREEN_HEIGHT - DEMI_PADDLE_HEIGHT,
                );
        }
        if input.pressed(KeyCode::KeyS) {
            pleft_transform.translation.y =
                (pleft_transform.translation.y - PADDLE_SPEED * time.delta_secs()).clamp(
                    -DEMI_SCREEN_HEIGHT + DEMI_PADDLE_HEIGHT,
                    DEMI_SCREEN_HEIGHT - DEMI_PADDLE_HEIGHT,
                );
        }
    }
}

fn move_player_right(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<PlayerRight>>,
) {
    for mut pright_transform in query.iter_mut() {
        if input.pressed(KeyCode::ArrowUp) {
            pright_transform.translation.y =
                (pright_transform.translation.y + PADDLE_SPEED * time.delta_secs()).clamp(
                    -DEMI_SCREEN_HEIGHT + DEMI_PADDLE_HEIGHT,
                    DEMI_SCREEN_HEIGHT - DEMI_PADDLE_HEIGHT,
                );
        }
        if input.pressed(KeyCode::ArrowDown) {
            pright_transform.translation.y =
                (pright_transform.translation.y - PADDLE_SPEED * time.delta_secs()).clamp(
                    -DEMI_SCREEN_HEIGHT + DEMI_PADDLE_HEIGHT,
                    DEMI_SCREEN_HEIGHT - DEMI_PADDLE_HEIGHT,
                );
        }
    }
}
