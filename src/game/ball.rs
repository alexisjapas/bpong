use avian2d::prelude::*;
use bevy::prelude::*;

use crate::audio::SoundAssets;
use crate::constants::*;
use crate::game::paddle::{InGameEntity, Left, Paddle};
use crate::game::shared::Speed;

#[derive(Component)]
pub struct Ball;

pub fn spawn_ball(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands
        .spawn((
            Ball,
            LinearVelocity(Vec2::new(
                if rand::random::<bool>() {
                    BALL_INITIAL_SPEED
                } else {
                    -BALL_INITIAL_SPEED
                },
                0.0,
            )),
            Speed(BALL_INITIAL_SPEED),
            Sprite::from_image(asset_server.load("imgs/ball.png")),
            RigidBody::Dynamic,
            Collider::rectangle(BALL_SIZE, BALL_SIZE),
            LockedAxes::ROTATION_LOCKED,
            Restitution::new(1.0),
            Friction::new(0.0),
            CollisionEventsEnabled,
            InGameEntity,
        ))
        .observe(on_paddle_collision);
}

fn on_paddle_collision(
    collision: On<CollisionStart>,
    mut paddle_query: Query<(Has<Left>, &Position, &mut Speed), (With<Paddle>, Without<Ball>)>,
    mut ball_query: Query<
        (&Position, &mut LinearVelocity, &mut Speed),
        (With<Ball>, Without<Paddle>),
    >,
    sounds: Res<SoundAssets>,
    mut commands: Commands,
) {
    let ball = collision.collider1;
    let paddle = collision.collider2;

    let Ok((is_left, paddle_pos, mut paddle_speed)) = paddle_query.get_mut(paddle) else {
        return;
    };
    let Ok((ball_pos, mut velocity, mut speed)) = ball_query.get_mut(ball) else {
        return;
    };

    // Sound
    if is_left {
        commands.spawn(AudioPlayer::new(sounds.ping.clone()));
    } else {
        commands.spawn(AudioPlayer::new(sounds.pong.clone()));
    }

    // Impact ratio: -1.0 (bottom) to 1.0 (top)
    let ratio = ((ball_pos.0.y - paddle_pos.0.y) / HALF_PADDLE_HEIGHT).clamp(-1.0, 1.0);
    let abs_ratio = ratio.abs();

    // Bounce angle: center → straight, edges → up to MAX_BOUNCE_ANGLE
    let angle = if abs_ratio <= BALL_CENTER_MARGIN {
        0.0
    } else {
        let t = (abs_ratio - BALL_CENTER_MARGIN) / (1.0 - BALL_CENTER_MARGIN);
        t * BALL_MAX_BOUNCE_ANGLE
    };

    let dir_x = if is_left { 1.0 } else { -1.0 };
    let dir_y = if ratio >= 0.0 {
        angle.sin()
    } else {
        -angle.sin()
    };
    let direction = Vec2::new(dir_x * angle.cos(), dir_y).normalize();

    // Speed boost: center → BALL_BOOST_CENTER, edges → BALL_BOOST_EDGE
    let ball_boost = BALL_BOOST_CENTER - (BALL_BOOST_CENTER - BALL_BOOST_EDGE) * abs_ratio;
    speed.0 = (speed.0 * ball_boost).min(BALL_MAX_SPEED);
    velocity.0 = direction * speed.0;

    // Paddle speed ball_boost
    let paddle_boost = 1. + (ball_boost - 1.) * PADDLE_BOOST_RATIO;
    paddle_speed.0 = (paddle_speed.0 * paddle_boost).min(PADDLE_MAX_SPEED);
}
