#![allow(clippy::type_complexity)]

mod constants;
mod state;
mod game;
mod ui;
mod audio;

use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_embedded_assets::{EmbeddedAssetPlugin, PluginMode};

use crate::constants::*;
use crate::state::{GameState, InGameState};

fn main() {
    // Generic setup
    let mut app = App::new();
    app.add_plugins((
        EmbeddedAssetPlugin {
            mode: PluginMode::ReplaceDefault,
        },
        game::GamePlugin,
        ui::UiPlugin,
        audio::AudioPlugin,
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

    // Run
    app.run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
