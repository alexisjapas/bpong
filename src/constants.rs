use bevy::prelude::*;

pub const SCREEN_WIDTH: f32 = 1280.;
pub const SCREEN_HEIGHT: f32 = 720.;
pub const PADDLE_HEIGHT: f32 = 80.;
pub const PADDLE_WIDTH: f32 = 16.;
pub const PADDLE_SPEED: f32 = 512.;
pub const BALL_SIZE: f32 = 16.;
pub const BALL_INITIAL_SPEED: f32 = 320.;
pub const BALL_MAX_SPEED: f32 = 32768.;
pub const BALL_BOOST_EDGE: f32 = 1.01;
pub const BALL_BOOST_CENTER: f32 = 1.20;
pub const BALL_MAX_BOUNCE_ANGLE: f32 = std::f32::consts::FRAC_PI_3; // 60°
pub const BALL_CENTER_MARGIN: f32 = 0.1;
pub const INIT_HEALTH: u32 = 9;
pub const WALL_THICKNESS: f32 = 4.;

pub const HALF_SCREEN_WIDTH: f32 = SCREEN_WIDTH / 2.;
pub const HALF_SCREEN_HEIGHT: f32 = SCREEN_HEIGHT / 2.;
pub const HALF_PADDLE_HEIGHT: f32 = PADDLE_HEIGHT / 2.;
pub const HALF_WALL_THICKNESS: f32 = WALL_THICKNESS / 2.;

// UI
pub const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);
