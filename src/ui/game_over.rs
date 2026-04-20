use bevy::prelude::*;

use crate::{
    constants::*,
    ui::shared::{button, on_button_exit, on_button_menu, on_button_restart},
};

#[derive(Component)]
pub(crate) struct GameOverEntity;

pub fn setup_game_over(mut commands: Commands, asset_server: Res<AssetServer>) {
    let root_node = Node {
        width: Val::Percent(100.),
        height: Val::Percent(100.),
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    commands
        .spawn((GameOverEntity, root_node))
        .with_children(|parent| {
            // Title
            let container_title = Node {
                width: Val::Percent(100.),
                height: Val::Percent(TITLE_CONTAINER_HEIGHT_PERCENTAGE),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            };
            parent.spawn(container_title).with_child((
                Text::new("GAME OVER"),
                TextColor(TEXT_COLOR),
                TextLayout::new_with_justify(Justify::Center),
                TextFont {
                    font: asset_server.load("fonts/bpong.otf"),
                    font_size: TITLE_FONT_SIZE,
                    ..default()
                },
            ));

            // Buttons
            parent
                .spawn(button("RESTART", &asset_server))
                .observe(on_button_restart);

            parent
                .spawn(button("MENU", &asset_server))
                .observe(on_button_menu);

            parent
                .spawn(button("EXIT", &asset_server))
                .observe(on_button_exit);
        });
}

pub fn cleanup_game_over(mut commands: Commands, query: Query<Entity, With<GameOverEntity>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
