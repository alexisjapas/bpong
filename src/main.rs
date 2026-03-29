use bevy::prelude::*;
use bevy::window::WindowResolution;

const SCREEN_WIDTH: f32 = 720.;
const SCREEN_HEIGHT: f32 = 480.;
const PADDLE_HEIGHT: f32 = 80.;
const PADDLE_WIDTH: f32 = 16.;
const PADDLE_SPEED: f32 = 512.;
const BALL_SIZE: f32 = 8.;
const BALL_INITIAL_SPEED: f32 = 320.;
const BALL_MAX_SPEED: f32 = 2048.;
const BALL_MULTIPLIER: f32 = 1.1;
const INIT_HEALTH: u32 = 9;

const DEMI_SCREEN_WIDTH: f32 = SCREEN_WIDTH / 2.;
const DEMI_SCREEN_HEIGHT: f32 = SCREEN_HEIGHT / 2.;
const DEMI_PADDLE_WIDTH: f32 = PADDLE_WIDTH / 2.;
const DEMI_PADDLE_HEIGHT: f32 = PADDLE_HEIGHT / 2.;
const DEMI_BALL_SIZE: f32 = BALL_SIZE / 2.;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "BPONG".into(),
            resolution: WindowResolution::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32),
            ..default()
        }),
        ..default()
    }));
    app.insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.2)));

    app.add_systems(
        Startup,
        (setup_camera, spawn_players, spawn_ball, load_sounds),
    );

    app.add_systems(
        Update,
        (
            move_player_left,
            move_player_right,
            move_ball,
            handle_scoring.after(move_ball),
            handle_ball_collisions.after(handle_scoring),
        ),
    );

    // UI
    app.add_systems(Startup, spawn_scores);
    app.add_systems(Update, update_scores);

    app.run();
}

// Components
// Game
#[derive(Component)]
struct PlayerLeft;

#[derive(Component)]
struct PlayerRight;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Health(u32);

#[derive(Component)]
struct Direction(Vec2);

#[derive(Component)]
struct Speed(f32);

// UI
#[derive(Component)]
struct ScorePlayerLeft;

#[derive(Component)]
struct ScorePlayerRight;

// Sound
#[derive(Resource)]
struct SoundAssets {
    ping: Handle<AudioSource>,
    pong: Handle<AudioSource>,
}

// Systems
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_players(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn((
        PlayerLeft,
        Health(INIT_HEALTH),
        Transform::from_xyz(-DEMI_SCREEN_WIDTH + PADDLE_WIDTH, 0.0, 0.0),
        Sprite::from_image(asset_server.load("paddle.png")),
    ));
    commands.spawn((
        PlayerRight,
        Health(INIT_HEALTH),
        Transform::from_xyz(DEMI_SCREEN_WIDTH - PADDLE_WIDTH, 0.0, 0.0),
        Sprite::from_image(asset_server.load("paddle.png")),
    ));
}

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
        Sprite::from_image(asset_server.load("ball.png")),
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

fn handle_scoring(
    mut ball_query: Query<(&mut Transform, &mut Direction, &mut Speed), With<Ball>>,
    mut pleft_query: Query<&mut Health, (With<PlayerLeft>, Without<PlayerRight>)>,
    mut pright_query: Query<&mut Health, (With<PlayerRight>, Without<PlayerLeft>)>,
) {
    for (mut ball_transform, mut ball_direction, mut ball_speed) in ball_query.iter_mut() {
        let out_left = ball_transform.translation.x < -DEMI_SCREEN_WIDTH + DEMI_BALL_SIZE;
        let out_right = ball_transform.translation.x > DEMI_SCREEN_WIDTH - DEMI_BALL_SIZE;
        if out_left {
            ball_direction.0.x = -1.;
            for mut health in pleft_query.iter_mut() {
                health.0 = health.0.saturating_sub(1);
            }
        }
        if out_right {
            ball_direction.0.x = 1.;
            for mut health in pright_query.iter_mut() {
                health.0 = health.0.saturating_sub(1);
            }
        }
        if out_left || out_right {
            ball_transform.translation.x = 0.;
            ball_transform.translation.y = 0.;
            ball_direction.0.y = 0.;
            ball_speed.0 = BALL_INITIAL_SPEED;
        }
    }
}

fn spawn_scores(mut commands: Commands) {
    let root_node = Node {
        width: Val::Percent(100.),
        height: Val::Percent(100.),
        flex_direction: FlexDirection::Row,
        ..default()
    };

    commands.spawn(root_node).with_children(|parent| {
        // PlayerLeft score
        let container_pleft = Node {
            width: Val::Percent(50.),
            height: Val::Percent(100.),
            align_items: AlignItems::FlexStart,
            justify_content: JustifyContent::FlexEnd,
            padding: UiRect::all(Val::Px(8.)),
            ..default()
        };
        parent.spawn(container_pleft).with_children(|pleft_parent| {
            pleft_parent.spawn((
                Text::new(format!("{}", INIT_HEALTH)),
                TextColor(Color::WHITE),
                TextLayout::new_with_justify(Justify::Right),
                ScorePlayerLeft,
            ));
        });

        // PlayerRight score
        let container_pright = Node {
            width: Val::Percent(50.),
            height: Val::Percent(100.),
            align_items: AlignItems::FlexStart,
            justify_content: JustifyContent::FlexStart,
            padding: UiRect::all(Val::Px(8.)),
            ..default()
        };
        parent
            .spawn(container_pright)
            .with_children(|pright_parent| {
                pright_parent.spawn((
                    Text::new(format!("{}", INIT_HEALTH)),
                    TextColor(Color::WHITE),
                    TextLayout::new_with_justify(Justify::Left),
                    ScorePlayerRight,
                ));
            });
    });
}

fn update_scores(
    pleft_health_q: Query<&Health, (With<PlayerLeft>, Changed<Health>)>,
    pright_health_q: Query<&Health, (With<PlayerRight>, Changed<Health>)>,
    mut pleft_text_q: Query<&mut Text, (With<ScorePlayerLeft>, Without<ScorePlayerRight>)>,
    mut pright_text_q: Query<&mut Text, (With<ScorePlayerRight>, Without<ScorePlayerLeft>)>,
) {
    if let Ok(pleft_health) = pleft_health_q.single() {
        if let Ok(mut pleft_text) = pleft_text_q.single_mut() {
            pleft_text.0 = format!("{}", pleft_health.0);
        }
    }
    if let Ok(pright_health) = pright_health_q.single() {
        if let Ok(mut pright_text) = pright_text_q.single_mut() {
            pright_text.0 = format!("{}", pright_health.0);
        }
    }
}

fn load_sounds(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(SoundAssets {
        ping: asset_server.load("ping.ogg"),
        pong: asset_server.load("pong.ogg"),
    });
}
