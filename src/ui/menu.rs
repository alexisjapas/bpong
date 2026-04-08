use bevy::prelude::*;

use crate::audio::SoundAssets;
use crate::state::GameState;
use crate::ui::shared::ButtonExit;

#[derive(Component)]
pub(crate) struct MenuEntity;

#[derive(Component)]
pub(crate) struct ButtonTitleEE;

#[derive(Component)]
pub(crate) struct ButtonPlay;

pub fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<MenuEntity>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn handle_button_play(
    mut next_state: ResMut<NextState<GameState>>,
    interaction_q: Query<&Interaction, (With<ButtonPlay>, Changed<Interaction>)>,
) {
    for interaction in interaction_q.iter() {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::InGame);
        }
    }
}

pub fn handle_button_title(
    sounds: Res<SoundAssets>,
    mut commands: Commands,
    interaction_q: Query<&Interaction, (With<ButtonTitleEE>, Changed<Interaction>)>,
) {
    let idx = rand::random_range(0..sounds.ee.len());
    for interaction in interaction_q.iter() {
        if *interaction == Interaction::Pressed {
            commands.spawn(AudioPlayer::new(sounds.ee[idx].clone()));
        }
    }
}

pub fn setup_menu(mut commands: Commands) {
    let root_node = Node {
        width: Val::Percent(100.),
        height: Val::Percent(100.),
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    commands
        .spawn((root_node, MenuEntity))
        .with_children(|parent| {
            // Title
            let container_title = Node {
                width: Val::Percent(100.),
                height: Val::Percent(30.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(8.)),
                ..default()
            };
            parent.spawn(container_title).with_child((
                Text::new("BPONG"),
                TextColor(Color::WHITE),
                TextLayout::new_with_justify(Justify::Center),
                TextFont {
                    font_size: 64.,
                    ..default()
                },
                ButtonTitleEE,
                Button,
            ));

            // Button Play
            let container_button_play = Node {
                width: Val::Percent(20.),
                height: Val::Percent(20.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(8.)),
                ..default()
            };
            parent.spawn(container_button_play).with_child((
                Text::new("PLAY"),
                TextColor(Color::WHITE),
                TextLayout::new_with_justify(Justify::Center),
                ButtonPlay,
                Button,
            ));

            // Button Exit
            let container_button_exit = Node {
                width: Val::Percent(20.),
                height: Val::Percent(20.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(8.)),
                ..default()
            };
            parent.spawn(container_button_exit).with_child((
                Text::new("EXIT"),
                TextColor(Color::WHITE),
                TextLayout::new_with_justify(Justify::Center),
                ButtonExit,
                Button,
            ));
        });
}
