use avian2d::prelude::*;
use bevy::prelude::*;

use crate::constants::*;
use crate::game::ball::{Ball, Speed};
use crate::state::GameState;

#[derive(Resource)]
pub struct Scoreboard {
    pub left: u32,
    pub right: u32,
}

pub fn handle_scoring(
    mut ball_query: Query<(&mut Position, &mut LinearVelocity, &mut Speed), With<Ball>>,
    mut scoreboard: ResMut<Scoreboard>,
) {
    for (mut pos, mut vel, mut speed) in ball_query.iter_mut() {
        let x = pos.0.x;

        if x < -HALF_SCREEN_WIDTH {
            scoreboard.left = scoreboard.left.saturating_sub(1);
        } else if x > HALF_SCREEN_WIDTH {
            scoreboard.right = scoreboard.right.saturating_sub(1);
        } else {
            continue;
        }

        pos.0 = Vec2::ZERO;
        let dir = if x < 0.0 { -1.0 } else { 1.0 };
        vel.0 = Vec2::new(dir * BALL_INITIAL_SPEED, 0.0);
        speed.0 = BALL_INITIAL_SPEED;
    }
}

pub fn handle_game_over(scoreboard: Res<Scoreboard>, mut next_state: ResMut<NextState<GameState>>) {
    if scoreboard.left == 0 || scoreboard.right == 0 {
        next_state.set(GameState::GameOver);
    }
}
