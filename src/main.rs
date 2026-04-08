use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_embedded_assets::{EmbeddedAssetPlugin, PluginMode};

fn main() {
    // Generic setup
    let mut app = App::new();
    app.add_plugins((
        EmbeddedAssetPlugin {
            mode: PluginMode::ReplaceDefault,
        },
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "BPONG".into(),
                resolution: WindowResolution::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32),
                ..default()
            }),
            ..default()
        }),
    ));
    app.insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.2)));
    app.add_systems(Startup, setup_camera);
    app.init_state::<GameState>().add_sub_state::<InGameState>();

    // InGame
    app.add_systems(
        Update,
        (
            handle_pause,
            handle_game_over,
        )
            .run_if(in_state(InGameState::Playing)),
    );

    // Run
    app.run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
