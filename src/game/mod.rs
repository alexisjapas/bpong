pub mod ball;
pub mod map;
pub mod paddle;
pub mod scoring;

use crate::constants::INIT_HEALTH;
use crate::game::paddle::InGameEntity;
use crate::state::{GameState, InGameState};
use crate::ui::pause::handle_pause;
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(scoring::Scoreboard {
            left: INIT_HEALTH,
            right: INIT_HEALTH,
        });
        app.add_systems(
            OnEnter(GameState::InGame),
            (map::spawn_walls, paddle::spawn_paddles, ball::spawn_ball),
        )
        .add_systems(OnEnter(GameState::Restarting), restart_game)
        .add_systems(OnExit(GameState::InGame), cleanup_ingame)
        .add_systems(
            Update,
            (
                paddle::move_paddles,
                scoring::handle_scoring,
                scoring::handle_game_over,
                handle_pause,
            )
                .run_if(in_state(InGameState::Playing)),
        );
    }
}

fn cleanup_ingame(mut commands: Commands, query: Query<Entity, With<InGameEntity>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn restart_game(mut next_state: ResMut<NextState<GameState>>, mut scoreboard: ResMut<scoring::Scoreboard>) {
    scoreboard.left = INIT_HEALTH;
    scoreboard.right = INIT_HEALTH;
    next_state.set(GameState::InGame);
}
