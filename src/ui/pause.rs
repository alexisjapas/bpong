use avian2d::prelude::*;
use bevy::prelude::*;

use crate::state::InGameState;
use crate::ui::shared::{ButtonExit, ButtonMenu, ButtonRestart};

#[derive(Component)]
pub(crate) struct PausedEntity;

#[derive(Component)]
pub(crate) struct ButtonResume;

pub fn setup_pause(mut commands: Commands, mut physics: ResMut<Time<Physics>>) {
    physics.pause();
    let root_node = Node {
        width: Val::Percent(100.),
        height: Val::Percent(100.),
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    commands
        .spawn((root_node, PausedEntity))
        .with_children(|parent| {
            // Button Resume
            let container_button_resume = Node {
                width: Val::Percent(20.),
                height: Val::Percent(20.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(8.)),
                ..default()
            };
            parent.spawn(container_button_resume).with_child((
                Text::new("RESUME"),
                TextColor(Color::WHITE),
                TextLayout::new_with_justify(Justify::Center),
                ButtonResume,
                Button,
            ));

            // Button Restart
            let container_button_restart = Node {
                width: Val::Percent(20.),
                height: Val::Percent(20.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(8.)),
                ..default()
            };
            parent.spawn(container_button_restart).with_child((
                Text::new("RESTART"),
                TextColor(Color::WHITE),
                TextLayout::new_with_justify(Justify::Center),
                ButtonRestart,
                Button,
            ));

            // Button Menu
            let container_button_menu = Node {
                width: Val::Percent(20.),
                height: Val::Percent(20.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(8.)),
                ..default()
            };
            parent.spawn(container_button_menu).with_child((
                Text::new("MENU"),
                TextColor(Color::WHITE),
                TextLayout::new_with_justify(Justify::Center),
                ButtonMenu,
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

pub fn cleanup_pause(
    mut commands: Commands,
    query: Query<Entity, With<PausedEntity>>,
    mut physics: ResMut<Time<Physics>>,
) {
    physics.unpause();
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn handle_pause(
    mut next_state: ResMut<NextState<InGameState>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        next_state.set(InGameState::Paused);
    }
}

pub fn handle_depause(
    mut next_state: ResMut<NextState<InGameState>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        next_state.set(InGameState::Playing);
    }
}

pub fn handle_button_resume(
    mut next_state: ResMut<NextState<InGameState>>,
    interaction_q: Query<&Interaction, (With<ButtonResume>, Changed<Interaction>)>,
) {
    for interaction in interaction_q.iter() {
        if *interaction == Interaction::Pressed {
            next_state.set(InGameState::Playing);
        }
    }
}
