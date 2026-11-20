use bevy::prelude::*;

#[derive(Component)]
pub struct MainMenu;

#[derive(Component)]
pub struct MenuText;

pub fn spawn_main_menu(mut commands: Commands) {
    commands
        .spawn((
            MainMenu,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.15)),
        ))
        .with_children(|parent| {
            parent.spawn((
                MenuText,
                Text::new("*kriegsfeld*"),
                TextFont {
                    font_size: FontSize::Px(48.),
                    ..default()
                },
                TextColor::WHITE,
            ));
        });
}

pub fn despawn_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
