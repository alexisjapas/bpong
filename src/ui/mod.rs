pub mod game_over;
pub mod hud;
pub mod menu;
pub mod pause;
pub mod shared;

use crate::state::{GameState, InGameState};
use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        // HUD
        app.add_systems(OnEnter(GameState::InGame), hud::spawn_scores)
            .add_systems(
                Update,
                hud::update_scores.run_if(in_state(InGameState::Playing)),
            );

        // Menu
        app.add_systems(OnEnter(GameState::MainMenu), menu::setup_menu)
            .add_systems(
                Update,
                (
                    menu::handle_button_play,
                    menu::handle_button_title,
                    shared::handle_button_exit,
                )
                    .run_if(in_state(GameState::MainMenu)),
            )
            .add_systems(OnExit(GameState::MainMenu), menu::cleanup_menu);

        // Paused
        app.add_systems(OnEnter(InGameState::Paused), pause::setup_pause)
            .add_systems(OnExit(InGameState::Paused), pause::cleanup_pause)
            .add_systems(
                Update,
                (
                    pause::handle_depause,
                    pause::handle_button_resume,
                    shared::handle_button_restart,
                    shared::handle_button_menu,
                    shared::handle_button_exit,
                )
                    .run_if(in_state(InGameState::Paused)),
            );

        // Game Over
        app.add_systems(OnEnter(GameState::GameOver), game_over::setup_game_over)
            .add_systems(OnExit(GameState::GameOver), game_over::cleanup_game_over)
            .add_systems(
                Update,
                (
                    shared::handle_button_restart,
                    shared::handle_button_menu,
                    shared::handle_button_exit,
                )
                    .run_if(in_state(GameState::GameOver)),
            );
    }
}
