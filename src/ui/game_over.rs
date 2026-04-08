use bevy::prelude::*;

#[derive(Component)]
struct GameOverEntity;

fn setup_game_over(mut commands: Commands) {
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
                height: Val::Percent(30.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(8.)),
                ..default()
            };
            parent.spawn(container_title).with_child((
                Text::new(format!("GAME OVER")),
                TextColor(Color::WHITE),
                TextLayout::new_with_justify(Justify::Center),
                TextFont {
                    font_size: 64.,
                    ..default()
                },
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
                Text::new(format!("PLAY AGAIN")),
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
                Text::new(format!("MENU")),
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
                Text::new(format!("EXIT")),
                TextColor(Color::WHITE),
                TextLayout::new_with_justify(Justify::Center),
                ButtonExit,
                Button,
            ));
        });
}

fn cleanup_game_over(mut commands: Commands, query: Query<Entity, With<GameOverEntity>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
