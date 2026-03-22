use bevy::prelude::*;
use bevy::window::WindowResolution;

const SCREEN_WIDTH: f32 = 720.;
const SCREEN_HEIGHT: f32 = 480.;
const PLAYER_HEIGHT: f32 = 64.;
const PLAYER_WIDTH: f32 = 32.;
const BALL_SIZE: f32 = 16.;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Bin pong".into(),
            resolution: WindowResolution::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32),
            ..default()
        }),
        ..default()
    }));

    app.add_systems(Startup, (setup_camera, spawn_player, spawn_ai, spawn_ball));

    app.add_systems(Update, (move_player, move_ball));

    app.run();
}

// Components
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Ai;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Health(u32);

#[derive(Component)]
struct Direction(Vec2);

#[derive(Component)]
struct Speed(f32);

// Systems
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Player,
        Health(10),
        Transform::from_xyz(-SCREEN_WIDTH / 2. + PLAYER_WIDTH, 0.0, 0.0),
        Sprite {
            color: Color::srgb(0.5, 0.5, 0.5),
            custom_size: Some(Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT)),
            ..default()
        },
    ));
}

fn spawn_ai(mut commands: Commands) {
    commands.spawn((
        Ai,
        Health(10),
        Transform::from_xyz(328.0, 0.0, 0.0),
        Sprite {
            color: Color::srgb(0.5, 0.5, 0.5),
            custom_size: Some(Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT)),
            ..default()
        },
    ));
}

fn spawn_ball(mut commands: Commands) {
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
        Speed(100.),
        Sprite {
            color: Color::srgb(1.0, 1.0, 1.0),
            custom_size: Some(Vec2::new(BALL_SIZE, BALL_SIZE)),
            ..default()
        },
    ));
}

fn move_player(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    const SPEED: f32 = 256.;
    for mut player_transform in query.iter_mut() {
        if input.pressed(KeyCode::ArrowUp) {
            player_transform.translation.y =
                (player_transform.translation.y + SPEED * time.delta_secs()).clamp(
                    -SCREEN_HEIGHT / 2. + PLAYER_HEIGHT / 2.,
                    SCREEN_HEIGHT / 2. - PLAYER_HEIGHT / 2.,
                );
        }
        if input.pressed(KeyCode::ArrowDown) {
            player_transform.translation.y =
                (player_transform.translation.y - SPEED * time.delta_secs()).clamp(
                    -SCREEN_HEIGHT / 2. + PLAYER_HEIGHT / 2.,
                    SCREEN_HEIGHT / 2. - PLAYER_HEIGHT / 2.,
                );
        }
    }
}

fn move_ball(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Direction, &Speed), With<Ball>>,
) {
    for (mut ball_transform, mut ball_direction, speed) in query.iter_mut() {
        ball_transform.translation.x += ball_direction.0.x * speed.0 * time.delta_secs();
        ball_transform.translation.y += ball_direction.0.y * speed.0 * time.delta_secs();
    }
}
