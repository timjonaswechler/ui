use bevy::prelude::*;
use bevy_project::{
    AccentColor, ButtonBuilder, ButtonClickEvent, ButtonSize, ButtonVariant, RadixUIPlugin,
    ThemeTokens, ThemeMode,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RadixUIPlugin)
        // 🌙 Set to Dark theme - this is all you need to change!
        .insert_resource(ThemeMode::Dark)
        .add_systems(Startup, setup)
        .add_systems(Update, handle_button_events)
        .run();
}

fn setup(mut commands: Commands, tokens: Res<ThemeTokens>, theme_mode: Res<ThemeMode>) {
    commands.spawn(Camera2d);

    // Same button setup as the light theme demo
    let primary_button = ButtonBuilder::new("Primary")
        .variant(ButtonVariant::Solid)
        .size(ButtonSize::Size2)
        .text("Primary Action")
        .build_with_theme(&tokens);

    let secondary_button = ButtonBuilder::new("Secondary")
        .variant(ButtonVariant::Soft)
        .size(ButtonSize::Size2)
        .text("Secondary")
        .build_with_theme(&tokens);

    let danger_button = ButtonBuilder::new("Danger")
        .variant(ButtonVariant::Solid)
        .size(ButtonSize::Size2)
        .color(AccentColor::Red)
        .text("Delete")
        .build_with_theme(&tokens);

    let outline_button = ButtonBuilder::new("Outline")
        .variant(ButtonVariant::Outline)
        .size(ButtonSize::Size2)
        .color(AccentColor::Green)
        .text("Save")
        .build_with_theme(&tokens);

    // Dark themed container
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(16.0),
                padding: UiRect::all(Val::Px(24.0)),
                ..default()
            },
            BackgroundColor(tokens.semantic.background),
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new(format!("🌙 Dark Theme Demo")),
                TextColor(tokens.semantic.foreground),
                Node {
                    margin: UiRect::bottom(Val::Px(32.0)),
                    ..default()
                },
            ));

            // Button Grid
            parent.spawn((
                Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(12.0),
                    margin: UiRect::bottom(Val::Px(16.0)),
                    ..default()
                },
            )).with_children(|row| {
                row.spawn(primary_button);
                row.spawn(secondary_button);
            });

            parent.spawn((
                Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(12.0),
                    ..default()
                },
            )).with_children(|row| {
                row.spawn(outline_button);
                row.spawn(danger_button);
            });

            // Info Text
            parent.spawn((
                Text::new("🎮 Dark Theme Active!\nSame theme system, different colors.\nChange ThemeMode::Dark to ThemeMode::Light in main()."),
                TextColor(tokens.semantic.muted_foreground),
                Node {
                    margin: UiRect::top(Val::Px(24.0)),
                    max_width: Val::Px(400.0),
                    ..default()
                },
            ));
        });
}

fn handle_button_events(mut events: EventReader<ButtonClickEvent>) {
    for event in events.read() {
        info!(
            "🌙 Dark theme button clicked! Variant: {:?}",
            event.button_variant
        );
    }
}