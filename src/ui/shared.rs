#[derive(Component)]
struct ButtonMenu;

#[derive(Component)]
struct ButtonRestart;

#[derive(Component)]
struct ButtonExit;

fn handle_button_exit(
    mut exit: MessageWriter<AppExit>,
    interaction_q: Query<&Interaction, (With<ButtonExit>, Changed<Interaction>)>,
) {
    for interaction in interaction_q.iter() {
        if *interaction == Interaction::Pressed {
            exit.write(AppExit::Success);
        }
    }
}

fn handle_button_restart(
    mut next_state: ResMut<NextState<GameState>>,
    interaction_q: Query<&Interaction, (With<ButtonRestart>, Changed<Interaction>)>,
) {
    for interaction in interaction_q.iter() {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::Restarting);
        }
    }
}

fn handle_button_menu(
    mut next_state: ResMut<NextState<GameState>>,
    interaction_q: Query<&Interaction, (With<ButtonMenu>, Changed<Interaction>)>,
) {
    for interaction in interaction_q.iter() {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::MainMenu);
        }
    }
}
