use bevy::prelude::*;
use ui::{
    components::text::{Text as UiText, TextColor as UiTextColor, TextSize},
    plugin::ForgeUiPlugin,
    theme::color::{
        accent_palette, error_palette, success_palette, warning_palette, TextContrastLevel,
    },
    utilities::ComponentBuilder,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ForgeUiPlugin)
        .add_systems(Startup, setup_smart_colors_demo)
        .run();
}

fn setup_smart_colors_demo(mut commands: Commands) {
    // Create simple root container instead of UIRoot
    commands.spawn(Camera2d);
    let demo_entity = commands
        .spawn((
            Name::new("SmartColorsDemo"),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                padding: UiRect::all(Val::Px(32.0)),
                row_gap: Val::Px(24.0),
                ..default()
            },
        ))
        .id();

    commands.entity(demo_entity).with_children(|parent| {
        // Title
        parent.spawn(
            UiText::display("Smart Color Contrast Demo")
                .center()
                .build(),
        );

        parent.spawn(
            UiText::body("Demonstrating intelligent text color calculation")
                .color(UiTextColor::Muted)
                .center()
                .build(),
        );

        // Traditional vs Smart Comparison
        parent.spawn(UiText::title("Traditional vs Smart Approach").build());

        // Demo row showing different backgrounds
        parent
            .spawn(Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(16.0),
                ..default()
            })
            .with_children(|row| {
                let backgrounds = vec![
                    ("Dark Blue", Color::srgb(0.1, 0.2, 0.4)),
                    ("Light Yellow", Color::srgb(0.9, 0.9, 0.4)),
                    ("Medium Gray", Color::srgb(0.5, 0.5, 0.5)),
                    ("Dark Red", Color::srgb(0.4, 0.1, 0.1)),
                ];

                for (_name, bg_color) in backgrounds {
                    // Comparison card: Traditional vs Smart
                    row.spawn((
                        Node {
                            width: Val::Px(200.0),
                            height: Val::Px(120.0),
                            padding: UiRect::all(Val::Px(8.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(8.0),
                            ..default()
                        },
                        BackgroundColor(bg_color),
                    ))
                    .with_children(|card| {
                        // Traditional approach: Static color (might be unreadable)
                        card.spawn(
                            UiText::new("Traditional Text")
                                .color(UiTextColor::Default) // Always same color
                                .size(TextSize::Sm)
                                .center()
                                .build(),
                        );

                        // Smart approach: Context-aware color
                        card.spawn(
                            UiText::on_background("Smart Text", bg_color)
                                .size(TextSize::Sm)
                                .center()
                                .build(),
                        );
                    });
                }
            });

        // Semantic colors demo
        parent.spawn(UiText::title("Semantic Colors with Auto-Contrast").build());

        parent
            .spawn(Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(16.0),
                ..default()
            })
            .with_children(|row| {
                // Accent example
                let accent_bg = accent_palette().solid;
                row.spawn((
                    Node {
                        width: Val::Px(140.0),
                        height: Val::Px(80.0),
                        padding: UiRect::all(Val::Px(12.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(accent_bg),
                ))
                .with_children(|card| {
                    card.spawn(
                        UiText::on_background("Accent", accent_bg)
                            .color(UiTextColor::Accent)
                            .center()
                            .build(),
                    );
                });

                // Warning example
                let warning_bg = warning_palette().solid;
                row.spawn((
                    Node {
                        width: Val::Px(140.0),
                        height: Val::Px(80.0),
                        padding: UiRect::all(Val::Px(12.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(warning_bg),
                ))
                .with_children(|card| {
                    card.spawn(
                        UiText::on_background("Warning", warning_bg)
                            .color(UiTextColor::Warning)
                            .center()
                            .build(),
                    );
                });

                // Success example
                let success_bg = success_palette().solid;
                row.spawn((
                    Node {
                        width: Val::Px(140.0),
                        height: Val::Px(80.0),
                        padding: UiRect::all(Val::Px(12.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(success_bg),
                ))
                .with_children(|card| {
                    card.spawn(
                        UiText::on_background("Success", success_bg)
                            .color(UiTextColor::Success)
                            .center()
                            .build(),
                    );
                });

                // Error example
                let error_bg = error_palette().solid;
                row.spawn((
                    Node {
                        width: Val::Px(140.0),
                        height: Val::Px(80.0),
                        padding: UiRect::all(Val::Px(12.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(error_bg),
                ))
                .with_children(|card| {
                    card.spawn(
                        UiText::on_background("Error", error_bg)
                            .color(UiTextColor::Error)
                            .center()
                            .build(),
                    );
                });
            });

        // Accessibility levels demo
        parent.spawn(UiText::title("WCAG Accessibility Levels").build());

        parent
            .spawn(Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(16.0),
                ..default()
            })
            .with_children(|row| {
                let dark_bg = Color::srgb(0.2, 0.2, 0.3);

                // Medium contrast
                row.spawn((
                    Node {
                        width: Val::Px(120.0),
                        height: Val::Px(80.0),
                        padding: UiRect::all(Val::Px(8.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(4.0),
                        ..default()
                    },
                    BackgroundColor(dark_bg),
                ))
                .with_children(|card| {
                    card.spawn(
                        UiText::new("Medium")
                            .on_background(dark_bg)
                            .contrast_level(TextContrastLevel::Medium)
                            .size(TextSize::Sm)
                            .center()
                            .build(),
                    );
                    card.spawn(
                        UiText::new("Contrast")
                            .on_background(dark_bg)
                            .contrast_level(TextContrastLevel::Medium)
                            .size(TextSize::Xs)
                            .center()
                            .build(),
                    );
                });

                // High contrast (WCAG AA)
                row.spawn((
                    Node {
                        width: Val::Px(120.0),
                        height: Val::Px(80.0),
                        padding: UiRect::all(Val::Px(8.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(4.0),
                        ..default()
                    },
                    BackgroundColor(dark_bg),
                ))
                .with_children(|card| {
                    card.spawn(
                        UiText::new("High AA")
                            .on_background(dark_bg)
                            .high_contrast()
                            .size(TextSize::Sm)
                            .center()
                            .build(),
                    );
                    card.spawn(
                        UiText::new("4.5:1 Ratio")
                            .on_background(dark_bg)
                            .high_contrast()
                            .size(TextSize::Xs)
                            .center()
                            .build(),
                    );
                });

                // Accessible (WCAG AAA)
                row.spawn((
                    Node {
                        width: Val::Px(120.0),
                        height: Val::Px(80.0),
                        padding: UiRect::all(Val::Px(8.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(4.0),
                        ..default()
                    },
                    BackgroundColor(dark_bg),
                ))
                .with_children(|card| {
                    card.spawn(
                        UiText::new("AAA")
                            .on_background(dark_bg)
                            .accessible()
                            .size(TextSize::Sm)
                            .center()
                            .build(),
                    );
                    card.spawn(
                        UiText::new("7:1 Ratio")
                            .on_background(dark_bg)
                            .accessible()
                            .size(TextSize::Xs)
                            .center()
                            .build(),
                    );
                });
            });

        // API Example showcase
        parent.spawn(UiText::title("API Examples").build());

        parent
            .spawn(Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Px(8.0),
                ..default()
            })
            .with_children(|examples| {
                examples.spawn(
                    UiText::code("Text::on_background(\"Auto contrast\", bg_color)")
                        .size(TextSize::Sm)
                        .build(),
                );
                examples.spawn(
                    UiText::code("Text::accessible(\"WCAG AAA\", bg_color)")
                        .size(TextSize::Sm)
                        .build(),
                );
                examples.spawn(
                    UiText::code("Text::new().contrast_level(TextContrastLevel::High)")
                        .size(TextSize::Sm)
                        .build(),
                );
            });
    });
}
