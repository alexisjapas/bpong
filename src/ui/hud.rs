use bevy::prelude::*;

use crate::constants::*;
use crate::game::ball::Ball;
use crate::game::paddle::InGameEntity;
use crate::game::scoring::Scoreboard;
use crate::game::shared::Speed;

#[derive(Component)]
pub(crate) struct ScoreLeft;

#[derive(Component)]
pub(crate) struct ScoreRight;

#[derive(Component)]
pub(crate) struct BallStats;

pub fn spawn_scores(mut commands: Commands, asset_server: Res<AssetServer>) {
    let root_node = Node {
        width: Val::Percent(100.),
        height: Val::Percent(100.),
        flex_direction: FlexDirection::Row,
        ..default()
    };

    commands
        .spawn((root_node, InGameEntity))
        .with_children(|parent| {
            // Left score
            let container_left = Node {
                width: Val::Percent(50.),
                height: Val::Percent(100.),
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::FlexEnd,
                padding: UiRect::all(Val::Px(8.)),
                ..default()
            };
            parent.spawn(container_left).with_children(|left_parent| {
                left_parent.spawn((
                    Text::new(format!("{}", INIT_HEALTH)),
                    TextFont {
                        font: asset_server.load("fonts/bpong.otf"),
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    TextLayout::new_with_justify(Justify::Right),
                    ScoreLeft,
                ));
            });

            // Right score
            let container_right = Node {
                width: Val::Percent(50.),
                height: Val::Percent(100.),
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::FlexStart,
                padding: UiRect::all(Val::Px(8.)),
                ..default()
            };
            parent.spawn(container_right).with_children(|right_parent| {
                right_parent.spawn((
                    Text::new(format!("{}", INIT_HEALTH)),
                    TextFont {
                        font: asset_server.load("fonts/bpong.otf"),
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    TextLayout::new_with_justify(Justify::Left),
                    ScoreRight,
                ));
            });
        });
}

pub fn update_scores(
    scoreboard: Res<Scoreboard>,
    mut left_text_q: Query<&mut Text, (With<ScoreLeft>, Without<ScoreRight>)>,
    mut right_text_q: Query<&mut Text, (With<ScoreRight>, Without<ScoreLeft>)>,
) {
    if !scoreboard.is_changed() {
        return;
    }

    if let Ok(mut left_text) = left_text_q.single_mut() {
        left_text.0 = format!("{}", scoreboard.left);
    }

    if let Ok(mut right_text) = right_text_q.single_mut() {
        right_text.0 = format!("{}", scoreboard.right);
    }
}

pub fn spawn_stats(mut commands: Commands, asset_server: Res<AssetServer>) {
    let root_node = Node {
        width: Val::Percent(100.),
        height: Val::Percent(100.),
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::FlexEnd,
        padding: UiRect::all(Val::Px(8.)),
        ..default()
    };

    commands.spawn((root_node, InGameEntity)).with_child((
        Text::new(format!("{}", BALL_INITIAL_SPEED)),
        TextFont {
            font: asset_server.load("fonts/bpong.otf"),
            ..default()
        },
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(Justify::Center),
        BallStats,
    ));
}

pub fn update_stats(
    ball_query: Query<&Speed, (With<Ball>, Changed<Speed>)>,
    mut ball_stats_text: Query<&mut Text, With<BallStats>>,
) {
    if let Ok(speed) = ball_query.single()
        && let Ok(mut text) = ball_stats_text.single_mut()
    {
        text.0 = format!("{:.0}", speed.0);
    }
}
