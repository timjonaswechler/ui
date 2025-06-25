use bevy::prelude::*;
use ui::{
    components::{
        button::{ButtonBuilder, ButtonVariant},
        card::{CardBuilder, CardVariant},
        heading::Heading,
        text::TextBuilder,
    },
    plugin::{ForgeUiPlugin, UiState},
    theme::{
        color::theme,
        typography::{TextSize, TextWeight},
    },
    utilities::{ui_root, ComponentBuilder},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ForgeUiPlugin)
        .add_systems(OnEnter(UiState::Ready), setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2d);

    // UI Root
    commands.spawn(ui_root("Card_Root")).with_children(|parent| {
        // Main container
        parent.spawn(
            CardBuilder::new("MainContainer")
                .size_3()
                .surface()
                .max_width(Val::Px(800.0))
                .padding(32.0)
                .build()
        ).with_children(|parent| {
            // Title
            parent.spawn(
                Heading::h1("Card Component Demo")
                    .size(TextSize::X6l)
                    .weight(TextWeight::Bold)
                    .build()
            );

            // Demo grid container
            parent.spawn((
                Node {
                    display: Display::Grid,
                    grid_template_columns: vec![RepeatedGridTrack::fr(1, 1.0); 3],
                    column_gap: Val::Px(24.0),
                    row_gap: Val::Px(24.0),
                    margin: UiRect::top(Val::Px(32.0)),
                    ..default()
                },
            )).with_children(|parent| {
                // Surface Variant
                create_demo_card(
                    parent,
                    "Surface Card",
                    "Subtle background for content areas. Default variant with gentle styling.",
                    CardVariant::Surface,
                    theme().gray
                );

                // Classic Variant
                create_demo_card(
                    parent,
                    "Classic Card",
                    "Enhanced border and background for prominent content. Stronger visual presence.",
                    CardVariant::Classic,
                    theme().indigo,
                );

                // Ghost Variant
                create_demo_card(
                    parent,
                    "Ghost Card",
                    "Transparent design with hover effects. Minimal visual footprint.",
                    CardVariant::Ghost,
                    theme().green,
                );
            });

            // Interactive Cards Section
            parent.spawn(
                Heading::h2("Interactive Cards")
                    .size(TextSize::X3l)
                    .weight(TextWeight::Medium)
                    .build()
            );

            parent.spawn((
                Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(16.0),
                    margin: UiRect::top(Val::Px(16.0)),
                    ..default()
                },
            )).with_children(|parent| {
                // Interactive Card with Button
                parent.spawn(
                    CardBuilder::new("InteractiveCard")
                        .classic()
                        .size_2()
                        .interactive()
                        .color_palette(theme().red)
                        .width(Val::Px(300.0))
                        .build()
                ).with_children(|parent| {
                    parent.spawn(
                        Heading::h3("Action Card")
                            .size(TextSize::Xl)
                            .weight(TextWeight::Bold)
                            .build()
                    );

                    parent.spawn(
                        TextBuilder::new("This card contains interactive elements and responds to clicks.")
                            .size(TextSize::Sm)
                            .build()
                    );

                    parent.spawn((
                        Node {
                            margin: UiRect::top(Val::Px(16.0)),
                            ..default()
                        },
                    )).with_children(|parent| {
                        parent.spawn(
                            ButtonBuilder::new("Click Me")
                                .variant(ButtonVariant::Solid)
                                .color(theme().red)
                                .build()
                        );
                    });
                });

                // Size Demonstration
                parent.spawn((
                    Node {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(12.0),
                        ..default()
                    },
                )).with_children(|parent| {
                    // Size 1 Card
                    parent.spawn(
                        CardBuilder::new("Size1Card")
                            .surface()
                            .size_1()
                            .width(Val::Px(200.0))
                            .build()
                    ).with_children(|parent| {
                        parent.spawn(
                            TextBuilder::new("Size 1 - Compact")
                                .size(TextSize::Sm)
                                .weight(TextWeight::Medium)
                                .build()
                        );
                    });

                    // Size 2 Card
                    parent.spawn(
                        CardBuilder::new("Size2Card")
                            .surface()
                            .size_2()
                            .width(Val::Px(200.0))
                            .build()
                    ).with_children(|parent| {
                        parent.spawn(
                            TextBuilder::new("Size 2 - Default")
                                .size(TextSize::Sm)
                                .weight(TextWeight::Medium)
                                .build()
                        );
                    });

                    // Size 3 Card
                    parent.spawn(
                        CardBuilder::new("Size3Card")
                            .surface()
                            .size_3()
                            .width(Val::Px(200.0))
                            .build()
                    ).with_children(|parent| {
                        parent.spawn(
                            TextBuilder::new("Size 3 - Large")
                                .size(TextSize::Sm)
                                .weight(TextWeight::Medium)
                                .build()
                        );
                    });
                });
            });
        });
    });
}

fn create_demo_card(
    parent: &mut ChildSpawnerCommands,
    title: &str,
    description: &str,
    variant: CardVariant,
    color_palette: ui::theme::color::UiColorPalette,
) {
    parent
        .spawn(
            CardBuilder::new("DemoCard")
                .variant(variant)
                .size_2()
                .color_palette(color_palette)
                .height(Val::Px(200.0))
                .build(),
        )
        .with_children(|parent| {
            parent.spawn(
                Heading::h3(title)
                    .size(TextSize::Lg)
                    .weight(TextWeight::Bold)
                    .build(),
            );

            parent
                .spawn((Node {
                    margin: UiRect::top(Val::Px(12.0)),
                    ..default()
                },))
                .with_children(|parent| {
                    parent.spawn(TextBuilder::new(description).size(TextSize::Sm).build());
                });

            // Variant-specific content
            match variant {
                CardVariant::Surface => {
                    parent
                        .spawn((Node {
                            margin: UiRect::top(Val::Px(16.0)),
                            ..default()
                        },))
                        .with_children(|parent| {
                            parent.spawn(
                                TextBuilder::new("✨ Perfect for content areas")
                                    .size(TextSize::Xs)
                                    .weight(TextWeight::Medium)
                                    .build(),
                            );
                        });
                }
                CardVariant::Classic => {
                    parent
                        .spawn((Node {
                            margin: UiRect::top(Val::Px(16.0)),
                            ..default()
                        },))
                        .with_children(|parent| {
                            parent.spawn(
                                TextBuilder::new("🎯 Enhanced visual prominence")
                                    .size(TextSize::Xs)
                                    .weight(TextWeight::Medium)
                                    .build(),
                            );
                        });
                }
                CardVariant::Ghost => {
                    parent
                        .spawn((Node {
                            margin: UiRect::top(Val::Px(16.0)),
                            ..default()
                        },))
                        .with_children(|parent| {
                            parent.spawn(
                                TextBuilder::new("👻 Minimal and clean")
                                    .size(TextSize::Xs)
                                    .weight(TextWeight::Medium)
                                    .build(),
                            );
                        });
                }
            }
        });
}
