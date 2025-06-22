use bevy::prelude::*;
use ui::{
    components::{
        button::Button, text::Text, ButtonBuilder, ButtonClickEvent, ButtonSize, ButtonVariant,
        Heading, HeadingExt,
    },
    plugin::ForgeUiPlugin,
    theme::{
        color::{error_palette, theme, TextColor as TextColorEnum},
        typography::{FontFamily, TextSize, TextWeight},
    },
    utilities::ComponentBuilder,
};
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ForgeUiPlugin)
        // Set theme mode at startup - change this to ThemeMode::Dark for dark theme
        .add_systems(Startup, setup)
        .add_systems(Update, handle_button_events)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // Demo verschiedener Button-Features using new theme system and TextBuilder integration
    let solid_button = ButtonBuilder::new("Solid")
        .variant(ButtonVariant::Solid)
        .text("Primary Solid")
        .text_weight(TextWeight::Bold)
        .text_accessible()
        .build();

    let secondary_button = ButtonBuilder::new("Secondary")
        .variant(ButtonVariant::Soft)
        .text("Secondary Soft")
        .text_accessible()
        .build();

    let disabled_button = ButtonBuilder::new("Disabled")
        .variant(ButtonVariant::Solid)
        .text("Disabled Button")
        .text_weight(TextWeight::Medium)
        .disabled()
        .text_accessible()
        .build();

    let loading_button = ButtonBuilder::new("Loading")
        .variant(ButtonVariant::Soft)
        .text("Loading Button")
        .text_italic()
        .loading()
        .build();

    let outline_button = ButtonBuilder::new("Outline")
        .variant(ButtonVariant::Outline)
        .color(theme().green)
        .text("Outline Button")
        .text_family(FontFamily::Serif)
        .text_center()
        .text_accessible()
        .build();

    let ghost_button = ButtonBuilder::new("Ghost")
        .variant(ButtonVariant::Ghost)
        .text("Ghost Button")
        .size(ButtonSize::Large)
        .text_accessible()
        .build();

    // Destructive button example with enhanced text styling
    let destructive_button = ButtonBuilder::new("Destructive")
        .variant(ButtonVariant::Solid)
        .color(error_palette())
        .text("Delete")
        .text_weight(TextWeight::Bold)
        .text_accessible()
        .build();

    // New: Code-style button example
    let code_button = ButtonBuilder::new("Code")
        .variant(ButtonVariant::Outline)
        .text("console.log()")
        .text_family(FontFamily::Mono)
        .size(ButtonSize::Large)
        .build();

    // Test text_size method
    let size_test_button = ButtonBuilder::new("SizeTest")
        .variant(ButtonVariant::Solid)
        .text("Custom Size X4l")
        .size(ButtonSize::Small) // Button size small but text should be X4l
        .build();

    // Container für die Buttons with themed background
    commands
        .spawn((Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(16.0),
            padding: UiRect::all(Val::Px(24.0)),
            ..default()
        },))
        .with_children(|parent| {
            // Title using new Heading component with TextBuilder features
            parent
                .spawn(Node {
                    margin: UiRect::bottom(Val::Px(32.0)),
                    ..default()
                })
                .with_children(|title_parent| {
                    title_parent.spawn(
                        Heading::h1("Button TextBuilder Demo")
                            .heading_accessible()
                            .heading_center()
                            .build(),
                    );
                });

            // Subtitle with enhanced text styling
            parent
                .spawn(Node {
                    margin: UiRect::bottom(Val::Px(24.0)),
                    ..default()
                })
                .with_children(|subtitle_parent| {
                    subtitle_parent.spawn(
                        Text::body("Demonstrating advanced text styling in buttons")
                            .italic()
                            .color(TextColorEnum::Muted)
                            .center()
                            .build(),
                    );
                });

            // Button Grid - Main Actions
            parent
                .spawn((Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(12.0),
                    margin: UiRect::bottom(Val::Px(16.0)),
                    ..default()
                },))
                .with_children(|row| {
                    row.spawn(solid_button);
                    row.spawn(secondary_button);
                    row.spawn(outline_button);
                });

            // Button Grid - States
            parent
                .spawn((Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(12.0),
                    margin: UiRect::bottom(Val::Px(16.0)),
                    ..default()
                },))
                .with_children(|row| {
                    row.spawn(disabled_button);
                    row.spawn(loading_button);
                    row.spawn(ghost_button);
                });

            // Button Grid - Advanced Text Features
            parent
                .spawn((Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(12.0),
                    margin: UiRect::bottom(Val::Px(16.0)),
                    ..default()
                },))
                .with_children(|row| {
                    row.spawn(code_button);
                    row.spawn(size_test_button);
                });

            // Destructive action
            parent.spawn(destructive_button);
        });
}

fn handle_button_events(mut events: EventReader<ButtonClickEvent>) {
    for event in events.read() {
        info!(
            "🎮 Button clicked! Entity: {:?}, Variant: {:?}",
            event.button_entity, event.button_variant
        );
    }
}
