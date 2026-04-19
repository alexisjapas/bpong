use bevy::prelude::*;

use crate::constants::*;
use crate::state::GameState;

// Systems
pub fn handle_button_exit(_: On<Pointer<Click>>, mut exit: MessageWriter<AppExit>) {
    exit.write(AppExit::Success);
}

pub fn handle_button_restart(_: On<Pointer<Click>>, mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Restarting);
}

pub fn handle_button_menu(_: On<Pointer<Click>>, mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::MainMenu);
}

// Helpers
pub fn button(text: &str, asset_server: &AssetServer) -> impl Bundle {
    (
        Button,
        Node {
            width: Val::Px(BUTTON_WIDTH),
            height: Val::Px(BUTTON_HEIGHT),
            margin: UiRect::bottom(Val::Px(BUTTON_SPACING)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border_radius: BorderRadius::MAX,
            ..default()
        },
        BorderColor::all(Color::WHITE),
        BackgroundColor(BUTTON_COLOR),
        children![(
            Text::new(text),
            TextFont {
                font: asset_server.load("fonts/bpong.otf"),
                font_size: BUTTON_FONT_SIZE,
                ..default()
            },
            TextColor(TEXT_COLOR),
            TextShadow::default(),
        )],
    )
}
