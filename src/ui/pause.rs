use avian2d::prelude::*;
use bevy::prelude::*;

use crate::state::InGameState;
use crate::ui::shared::{button, handle_button_exit, handle_button_menu, handle_button_restart};

#[derive(Component)]
pub(crate) struct PausedEntity;

pub fn setup_pause(
    mut commands: Commands,
    mut physics: ResMut<Time<Physics>>,
    asset_server: Res<AssetServer>,
) {
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
            parent
                .spawn(button("RESUME", &asset_server))
                .observe(handle_button_resume);

            parent
                .spawn(button("RESTART", &asset_server))
                .observe(handle_button_restart);

            parent
                .spawn(button("MENU", &asset_server))
                .observe(handle_button_menu);

            parent
                .spawn(button("EXIT", &asset_server))
                .observe(handle_button_exit);
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

pub fn handle_button_resume(_: On<Pointer<Click>>, mut next_state: ResMut<NextState<InGameState>>) {
    next_state.set(InGameState::Playing);
}
