use bevy::prelude::*;

use crate::constants::*;
use crate::game::paddle::{Health, InGameEntity, PlayerLeft, PlayerRight};

#[derive(Component)]
pub(crate) struct ScorePlayerLeft;

#[derive(Component)]
pub(crate) struct ScorePlayerRight;

pub fn spawn_scores(mut commands: Commands) {
    let root_node = Node {
        width: Val::Percent(100.),
        height: Val::Percent(100.),
        flex_direction: FlexDirection::Row,
        ..default()
    };

    commands
        .spawn((root_node, InGameEntity))
        .with_children(|parent| {
            // PlayerLeft score
            let container_pleft = Node {
                width: Val::Percent(50.),
                height: Val::Percent(100.),
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::FlexEnd,
                padding: UiRect::all(Val::Px(8.)),
                ..default()
            };
            parent.spawn(container_pleft).with_children(|pleft_parent| {
                pleft_parent.spawn((
                    Text::new(format!("{}", INIT_HEALTH)),
                    TextColor(Color::WHITE),
                    TextLayout::new_with_justify(Justify::Right),
                    ScorePlayerLeft,
                ));
            });

            // PlayerRight score
            let container_pright = Node {
                width: Val::Percent(50.),
                height: Val::Percent(100.),
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::FlexStart,
                padding: UiRect::all(Val::Px(8.)),
                ..default()
            };
            parent
                .spawn(container_pright)
                .with_children(|pright_parent| {
                    pright_parent.spawn((
                        Text::new(format!("{}", INIT_HEALTH)),
                        TextColor(Color::WHITE),
                        TextLayout::new_with_justify(Justify::Left),
                        ScorePlayerRight,
                    ));
                });
        });
}

pub fn update_scores(
    pleft_health_q: Query<&Health, (With<PlayerLeft>, Changed<Health>)>,
    pright_health_q: Query<&Health, (With<PlayerRight>, Changed<Health>)>,
    mut pleft_text_q: Query<&mut Text, (With<ScorePlayerLeft>, Without<ScorePlayerRight>)>,
    mut pright_text_q: Query<&mut Text, (With<ScorePlayerRight>, Without<ScorePlayerLeft>)>,
) {
    if let Ok(pleft_health) = pleft_health_q.single()
        && let Ok(mut pleft_text) = pleft_text_q.single_mut()
    {
        pleft_text.0 = format!("{}", pleft_health.0);
    }

    if let Ok(pright_health) = pright_health_q.single()
        && let Ok(mut pright_text) = pright_text_q.single_mut()
    {
        pright_text.0 = format!("{}", pright_health.0);
    }
}
