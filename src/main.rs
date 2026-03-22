use bevy::prelude::*;
use bevy::window::WindowResolution;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Bin pong".into(),
            resolution: WindowResolution::new(720, 480),
            ..default()
        }),
        ..default()
    }));

    app.add_systems(Startup, (setup_camera, spawn_player));

    app.add_systems(Update, move_player);

    app.run();
}

// Components
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Health(u32);

// Systems
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Player,
        Health(10),
        Transform::from_xyz(-328.0, 0.0, 0.0),
        Sprite {
            color: Color::srgb(0.5, 0.5, 0.5),
            custom_size: Some(Vec2::new(32.0, 64.0)),
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
            player_transform.translation.y += SPEED * time.delta_secs();
        }
        if input.pressed(KeyCode::ArrowDown) {
            player_transform.translation.y -= SPEED * time.delta_secs();
        }
    }
}
