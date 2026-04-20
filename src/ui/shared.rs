use bevy::prelude::*;

use crate::constants::*;
use crate::state::GameState;

// Components
#[derive(Component)]
pub struct InteractiveButton;

// Helpers
pub fn button(text: &str, asset_server: &AssetServer) -> impl Bundle {
    (
        InteractiveButton,
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

// Systems
pub fn handle_button_interaction(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (With<InteractiveButton>, Changed<Interaction>),
    >,
) {
    for (interaction, mut bg_color) in button_query.iter_mut() {
        match *interaction {
            Interaction::Hovered => {
                bg_color.0 = BUTTON_COLOR_HOVER;
            }
            Interaction::Pressed => {
                bg_color.0 = BUTTON_COLOR_PRESSED;
            }
            Interaction::None => {
                bg_color.0 = BUTTON_COLOR;
            }
        }
    }
}

pub fn on_button_exit(_: On<Pointer<Click>>, mut exit: MessageWriter<AppExit>) {
    exit.write(AppExit::Success);
}

pub fn on_button_restart(_: On<Pointer<Click>>, mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Restarting);
}

pub fn on_button_menu(_: On<Pointer<Click>>, mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::MainMenu);
}
