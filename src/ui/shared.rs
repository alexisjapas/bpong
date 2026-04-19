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
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![(
            Button,
            Node {
                width: px(150),
                height: px(65),
                border: UiRect::all(px(5)),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                border_radius: BorderRadius::MAX,
                ..default()
            },
            BorderColor::all(Color::WHITE),
            BackgroundColor(NORMAL_BUTTON),
            children![(
                Text::new(text),
                TextFont {
                    font: asset_server.load("fonts/bpong.otf"),
                    font_size: 33.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                TextShadow::default(),
            )]
        )],
    )
}
