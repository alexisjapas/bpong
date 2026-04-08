use bevy::prelude::*;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Direction(Vec2);

#[derive(Component)]
struct Speed(f32);

fn spawn_ball(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn((
        Ball,
        Transform::from_xyz(0.0, 0.0, 0.0),
        Direction(
            Vec2::new(
                rand::random_range(-1.0..=1.0),
                rand::random_range(-1.0..=1.0),
            )
            .normalize(),
        ),
        Speed(BALL_INITIAL_SPEED),
        Sprite::from_image(asset_server.load("imgs/ball.png")),
        InGameEntity,
    ));
}

fn move_ball(time: Res<Time>, mut query: Query<(&mut Transform, &Direction, &Speed), With<Ball>>) {
    for (mut ball_transform, ball_direction, speed) in query.iter_mut() {
        ball_transform.translation.x += ball_direction.0.x * speed.0 * time.delta_secs();
        ball_transform.translation.y += ball_direction.0.y * speed.0 * time.delta_secs();
    }
}

fn handle_ball_collisions(
    sounds: Res<SoundAssets>,
    mut commands: Commands,
    mut ball_query: Query<(&mut Transform, &mut Direction, &mut Speed), With<Ball>>,
    paddle_query: Query<&Transform, (Or<(With<PlayerLeft>, With<PlayerRight>)>, Without<Ball>)>,
) {
    for (mut ball_transform, mut ball_direction, mut ball_speed) in ball_query.iter_mut() {
        // Paddles collision
        for paddle_transform in paddle_query.iter() {
            let dx = (ball_transform.translation.x - paddle_transform.translation.x).abs();
            let dy = (ball_transform.translation.y - paddle_transform.translation.y).abs();
            let overlap_x = DEMI_BALL_SIZE + DEMI_PADDLE_WIDTH - dx;
            let overlap_y = DEMI_BALL_SIZE + DEMI_PADDLE_HEIGHT - dy;

            if dx < DEMI_BALL_SIZE + DEMI_PADDLE_WIDTH && dy < DEMI_BALL_SIZE + DEMI_PADDLE_HEIGHT {
                let is_left_paddle = paddle_transform.translation.x < 0.0;
                if is_left_paddle && ball_direction.0.x > 0.0 {
                    continue;
                }
                if !is_left_paddle && ball_direction.0.x < 0.0 {
                    continue;
                }

                if overlap_x < overlap_y {
                    if is_left_paddle {
                        commands.spawn(AudioPlayer::new(sounds.ping.clone()));
                    } else {
                        commands.spawn(AudioPlayer::new(sounds.pong.clone()));
                    }

                    // Bounce vertically, dependent on the ball's position relative to the paddle
                    let delta_y = (ball_transform.translation.y - paddle_transform.translation.y)
                        / DEMI_PADDLE_HEIGHT;
                    let angle = delta_y * std::f32::consts::FRAC_PI_3;
                    let bounce_direction_x = if is_left_paddle { 1.0 } else { -1.0 };
                    let x = angle.cos() * bounce_direction_x;
                    let y = angle.sin();
                    ball_direction.0 = Vec2::new(x, y).normalize();

                    // Move the ball to just outside the paddle
                    ball_transform.translation.x = paddle_transform.translation.x
                        + (DEMI_PADDLE_WIDTH + DEMI_BALL_SIZE) * bounce_direction_x;

                    // Accelerate ball
                    ball_speed.0 = (ball_speed.0 * BALL_MULTIPLIER).clamp(0., BALL_MAX_SPEED);
                } else {
                    ball_direction.0.y *= -1.;
                    let side_y =
                        (ball_transform.translation.y - paddle_transform.translation.y).signum();
                    ball_transform.translation.y = paddle_transform.translation.y
                        + (DEMI_PADDLE_HEIGHT + DEMI_BALL_SIZE) * side_y;
                }
            }
        }

        // Walls collision (top and bottom)
        if ball_transform.translation.y < -DEMI_SCREEN_HEIGHT + DEMI_BALL_SIZE {
            ball_transform.translation.y = -DEMI_SCREEN_HEIGHT + DEMI_BALL_SIZE;
            ball_direction.0.y *= -1.;
        }
        if ball_transform.translation.y > DEMI_SCREEN_HEIGHT - DEMI_BALL_SIZE {
            ball_transform.translation.y = DEMI_SCREEN_HEIGHT - DEMI_BALL_SIZE;
            ball_direction.0.y *= -1.;
        }
    }
}
