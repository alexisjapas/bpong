use bevy::prelude::*;

use crate::audio::SoundAssets;
use crate::state::GameState;
use crate::ui::shared::{button, handle_button_exit};

#[derive(Component)]
pub(crate) struct MenuEntity;

#[derive(Component)]
pub(crate) struct ButtonTitleEE;

pub fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<MenuEntity>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn handle_button_play(_: On<Pointer<Click>>, mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::InGame);
}

pub fn handle_button_title(
    _: On<Pointer<Click>>,
    sounds: Res<SoundAssets>,
    mut commands: Commands,
) {
    let idx = rand::random_range(0..sounds.ee.len());
    commands.spawn(AudioPlayer::new(sounds.ee[idx].clone()));
}

pub fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
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
            parent
                .spawn(container_title)
                .with_child((
                    Text::new("BPONG"),
                    TextColor(Color::WHITE),
                    TextLayout::new_with_justify(Justify::Center),
                    TextFont {
                        font: asset_server.load("fonts/bpong.otf"),
                        font_size: 64.,
                        ..default()
                    },
                    ButtonTitleEE,
                    Button,
                ))
                .observe(handle_button_title);

            parent
                .spawn(button("PLAY", &asset_server))
                .observe(handle_button_play);
            parent
                .spawn(button("EXIT", &asset_server))
                .observe(handle_button_exit);
        });
}
