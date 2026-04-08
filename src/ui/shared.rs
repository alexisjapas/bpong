use bevy::prelude::*;

use crate::state::GameState;

#[derive(Component)]
pub struct ButtonMenu;

#[derive(Component)]
pub struct ButtonRestart;

#[derive(Component)]
pub struct ButtonExit;

pub fn handle_button_exit(
    mut exit: MessageWriter<AppExit>,
    interaction_q: Query<&Interaction, (With<ButtonExit>, Changed<Interaction>)>,
) {
    for interaction in interaction_q.iter() {
        if *interaction == Interaction::Pressed {
            exit.write(AppExit::Success);
        }
    }
}

pub fn handle_button_restart(
    mut next_state: ResMut<NextState<GameState>>,
    interaction_q: Query<&Interaction, (With<ButtonRestart>, Changed<Interaction>)>,
) {
    for interaction in interaction_q.iter() {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::Restarting);
        }
    }
}

pub fn handle_button_menu(
    mut next_state: ResMut<NextState<GameState>>,
    interaction_q: Query<&Interaction, (With<ButtonMenu>, Changed<Interaction>)>,
) {
    for interaction in interaction_q.iter() {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::MainMenu);
        }
    }
}
