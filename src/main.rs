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

//fn move()
