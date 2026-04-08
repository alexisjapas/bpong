use bevy::prelude::*;

#[derive(Component)]
struct ScorePlayerLeft;

#[derive(Component)]
struct ScorePlayerRight;

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

fn handle_game_over(query: Query<&Health>, mut next_state: ResMut<NextState<GameState>>) {
    for health in query.iter() {
        if health.0 == 0 {
            next_state.set(GameState::GameOver);
        }
    }
}
