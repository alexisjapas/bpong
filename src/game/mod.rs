pub mod ball;
pub mod paddle;
pub mod scoring;

use bevy::prelude::*;
use crate::state::{GameState, InGameState};
use crate::ui::pause::handle_pause;
use crate::game::paddle::InGameEntity;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::InGame), (paddle::spawn_players, ball::spawn_ball))
            .add_systems(OnEnter(GameState::Restarting), restart_game)
            .add_systems(OnExit(GameState::InGame), cleanup_ingame)
            .add_systems(Update, (
                paddle::move_player_left,
                paddle::move_player_right,
                ball::move_ball,
                scoring::handle_scoring.after(ball::move_ball),
                ball::handle_ball_collisions.after(scoring::handle_scoring),
                scoring::handle_game_over,
                handle_pause,
            ).run_if(in_state(InGameState::Playing)));
    }
}

fn cleanup_ingame(mut commands: Commands, query: Query<Entity, With<InGameEntity>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn restart_game(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::InGame);
}
