use bevy::prelude::*;
use ui::{
    components::text::{Text as UiText, TextColor as UiTextColor, TextSize},
    plugin::ForgeUiPlugin,
    utilities::ComponentBuilder,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ForgeUiPlugin)
        .add_systems(Startup, setup_auto_contrast_demo)
        .run();
}

fn setup_auto_contrast_demo(mut commands: Commands) {
    commands.spawn(Camera2d);
    let demo_entity = commands
        .spawn((
            Name::new("AutoContrastDemo"),
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
        parent.spawn(UiText::display("Auto-Contrast Text Demo").center().build());

        parent.spawn(
            UiText::body("Default behavior: automatic WCAG AA contrast optimization")
                .center()
                .build(),
        );

        // Comparison section
        parent
            .spawn(Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(24.0),
                ..default()
            })
            .with_children(|row| {
                // Auto-contrast column (NEW DEFAULT BEHAVIOR)
                row.spawn(Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(16.0),
                    ..default()
                })
                .with_children(|auto_column| {
                    auto_column.spawn(UiText::title("🤖 Auto-Contrast (Default)").center().build());

                    auto_column.spawn(
                        UiText::body("Just Text::new() - automatically optimal!")
                            .center()
                            .size(TextSize::Sm)
                            .build(),
                    );

                    // Examples of auto-contrast (no .color() calls)
                    auto_column.spawn(UiText::new("Primary text - auto WCAG AA").build());

                    auto_column.spawn(
                        UiText::new("Secondary text")
                            .variant(ui::components::text::TextVariant::Caption)
                            .build(),
                    );

                    auto_column.spawn(
                        UiText::new("Accessible text - auto WCAG AAA")
                            .accessible()
                            .build(),
                    );

                    // With explicit background context
                    auto_column
                        .spawn((
                            Node {
                                width: Val::Px(200.0),
                                height: Val::Px(60.0),
                                padding: UiRect::all(Val::Px(12.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.2, 0.3, 0.7)), // Dark blue background
                        ))
                        .with_children(|card| {
                            card.spawn(
                                UiText::new("Smart on blue background")
                                    .on_background(Color::srgb(0.2, 0.3, 0.7))
                                    .center()
                                    .build(),
                            );
                        });
                });

                // Manual color column (EXPLICIT COLOR MODE)
                row.spawn(Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(16.0),
                    ..default()
                })
                .with_children(|manual_column| {
                    manual_column.spawn(
                        UiText::title("🎨 Manual Colors (Explicit)")
                            .center()
                            .build(),
                    );

                    manual_column.spawn(
                        UiText::body("Using .color() - disables auto-contrast")
                            .center()
                            .size(TextSize::Sm)
                            .build(),
                    );

                    // Examples with explicit colors (traditional behavior)
                    manual_column.spawn(
                        UiText::new("Explicit accent color")
                            .color(UiTextColor::Accent) // Explicit color = no auto-contrast
                            .build(),
                    );

                    manual_column.spawn(
                        UiText::new("Explicit error color")
                            .color(UiTextColor::Error) // Explicit color = no auto-contrast
                            .build(),
                    );

                    manual_column.spawn(
                        UiText::new("Explicit custom color")
                            .color(UiTextColor::Custom(Color::srgb(0.8, 0.4, 0.9))) // Purple
                            .build(),
                    );

                    // Same background with manual color (might not be readable)
                    manual_column
                        .spawn((
                            Node {
                                width: Val::Px(200.0),
                                height: Val::Px(60.0),
                                padding: UiRect::all(Val::Px(12.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.2, 0.3, 0.7)), // Same dark blue
                        ))
                        .with_children(|card| {
                            card.spawn(
                                UiText::new("Manual color (might be unreadable)")
                                    .color(UiTextColor::Accent) // Explicit color, no auto-contrast
                                    .center()
                                    .build(),
                            );
                        });
                });
            });

        // API Examples
        parent.spawn(UiText::title("🚀 New API Examples").center().build());

        parent
            .spawn(Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Px(8.0),
                ..default()
            })
            .with_children(|api_examples| {
                api_examples.spawn(
                    UiText::code("Text::new(\"Auto-optimized text\") // Default: WCAG AA")
                        .size(TextSize::Sm)
                        .build(),
                );

                api_examples.spawn(
                    UiText::code("Text::new(\"text\").accessible() // Force WCAG AAA")
                        .size(TextSize::Sm)
                        .build(),
                );

                api_examples.spawn(
                    UiText::code("Text::new(\"text\").color(TextColor::Accent) // Explicit color")
                        .size(TextSize::Sm)
                        .build(),
                );

                api_examples.spawn(
                    UiText::code("Text::new(\"text\").on_background(bg_color) // Context-aware")
                        .size(TextSize::Sm)
                        .build(),
                );
            });
    });
}
