use bevy::prelude::*;
use ui::{
    assets::icon::{Icon, IconAtlases, IconBuilder, IconId},
    plugin::ForgeUiPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ForgeUiPlugin::new())
        .add_systems(Startup, setup_camera)
        .add_systems(Update, setup_icon_demo)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn setup_icon_demo(
    mut commands: Commands,
    atlases: Option<Res<IconAtlases>>,
    mut setup_done: Local<bool>,
) {
    // Only run once when atlases are available
    if *setup_done {
        return;
    }
    
    // Wait for atlases to load
    let Some(atlases) = atlases else {
        return;
    };
    
    *setup_done = true;

    // Root container
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(20.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("Icon System Demo"),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            // Icon size demonstration
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(20.0),
                        align_items: AlignItems::Center,
                        ..default()
                    },
                ))
                .with_children(|size_row| {
                    // Small icons
                    size_row
                        .spawn((
                            Node {
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                row_gap: Val::Px(10.0),
                                ..default()
                            },
                        ))
                        .with_children(|small_col| {
                            small_col.spawn((
                                Text::new("Small (16px)"),
                                TextFont {
                                    font_size: 14.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                            ));

                            small_col
                                .spawn((
                                    Node {
                                        flex_direction: FlexDirection::Row,
                                        column_gap: Val::Px(5.0),
                                        ..default()
                                    },
                                ))
                                .with_children(|icons| {
                                    icons.spawn(IconBuilder::small(IconId::Home).bundle(&atlases));
                                    icons.spawn(IconBuilder::small(IconId::Gear).bundle(&atlases));
                                    icons.spawn(IconBuilder::small(IconId::MagnifyingGlass).bundle(&atlases));
                                    icons.spawn(IconBuilder::small(IconId::Person).bundle(&atlases));
                                });
                        });

                    // Medium icons
                    size_row
                        .spawn((
                            Node {
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                row_gap: Val::Px(10.0),
                                ..default()
                            },
                        ))
                        .with_children(|medium_col| {
                            medium_col.spawn((
                                Text::new("Medium (24px)"),
                                TextFont {
                                    font_size: 14.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                            ));

                            medium_col
                                .spawn((
                                    Node {
                                        flex_direction: FlexDirection::Row,
                                        column_gap: Val::Px(5.0),
                                        ..default()
                                    },
                                ))
                                .with_children(|icons| {
                                    icons.spawn(IconBuilder::medium(IconId::Backpack).bundle(&atlases));
                                    icons.spawn(IconBuilder::medium(IconId::Heart).bundle(&atlases));
                                    icons.spawn(IconBuilder::medium(IconId::LightningBolt).bundle(&atlases));
                                    icons.spawn(IconBuilder::medium(IconId::Target).bundle(&atlases));
                                });
                        });

                    // Large icons
                    size_row
                        .spawn((
                            Node {
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                row_gap: Val::Px(10.0),
                                ..default()
                            },
                        ))
                        .with_children(|large_col| {
                            large_col.spawn((
                                Text::new("Large (32px)"),
                                TextFont {
                                    font_size: 14.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                            ));

                            large_col
                                .spawn((
                                    Node {
                                        flex_direction: FlexDirection::Row,
                                        column_gap: Val::Px(5.0),
                                        ..default()
                                    },
                                ))
                                .with_children(|icons| {
                                    icons.spawn(IconBuilder::large(IconId::Scissors).bundle(&atlases));
                                    icons.spawn(IconBuilder::large(IconId::Badge).bundle(&atlases));
                                    icons.spawn(IconBuilder::large(IconId::MagicWand).bundle(&atlases));
                                    icons.spawn(IconBuilder::large(IconId::Star).bundle(&atlases));
                                });
                        });
                });

            // Colored icons demonstration
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        row_gap: Val::Px(10.0),
                        ..default()
                    },
                ))
                .with_children(|color_section| {
                    color_section.spawn((
                        Text::new("Colored Icons"),
                        TextFont {
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));

                    color_section
                        .spawn((
                            Node {
                                flex_direction: FlexDirection::Row,
                                column_gap: Val::Px(10.0),
                                ..default()
                            },
                        ))
                        .with_children(|color_row| {
                            // Red sun icon
                            color_row.spawn(
                                Icon::medium(IconId::Sun)
                                    .tint(Color::srgb(1.0, 0.3, 0.3))
                                    .bundle(&atlases)
                            );

                            // Blue globe icon
                            color_row.spawn(
                                Icon::medium(IconId::Globe)
                                    .tint(Color::srgb(0.3, 0.6, 1.0))
                                    .bundle(&atlases)
                            );

                            // Green heart icon
                            color_row.spawn(
                                Icon::medium(IconId::HeartFilled)
                                    .tint(Color::srgb(0.3, 0.8, 0.3))
                                    .bundle(&atlases)
                            );

                            // Yellow lightning icon
                            color_row.spawn(
                                Icon::medium(IconId::LightningBolt)
                                    .tint(Color::srgb(1.0, 1.0, 0.3))
                                    .bundle(&atlases)
                            );
                        });
                });

            // Interactive button with icon
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(10.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.3, 0.7)),
                ))
                .with_children(|button| {
                    button.spawn(
                        Icon::medium(IconId::Play)
                            .bundle(&atlases)
                    );

                    button.spawn((
                        Text::new("Start Game"),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });

            // Instructions
            parent.spawn((
                Text::new("Icons loaded from texture atlas at assets/ui/texture_atlas_20x16_*.png\nSupports 318 icons in 16px, 24px, 32px, and 64px sizes"),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
                TextLayout::new_with_justify(JustifyText::Center),
            ));
        });
}