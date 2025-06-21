use bevy::prelude::*;
use ui::components::{ButtonBuilder, ButtonClickEvent, ButtonVariant};
use ui::plugin::ForgeUiPlugin;
use ui::theme::color::theme;
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

    // Demo verschiedener Button-Features using new theme system
    let solid_button = ButtonBuilder::new("Solid")
        .variant(ButtonVariant::Solid)
        .text("Primary Solid")
        .build();

    let secondary_button = ButtonBuilder::new("Secondary")
        .variant(ButtonVariant::Soft)
        .text("Secondary Soft")
        .build();

    let disabled_button = ButtonBuilder::new("Disabled")
        .variant(ButtonVariant::Solid)
        .text("Disabled Button")
        .disabled()
        .build();

    let loading_button = ButtonBuilder::new("Loading")
        .variant(ButtonVariant::Soft)
        .text("Loading Button")
        .loading()
        .build();

    let outline_button = ButtonBuilder::new("Outline")
        .variant(ButtonVariant::Outline)
        .color(theme().green)
        .text("Outline Button")
        .build();

    let ghost_button = ButtonBuilder::new("Ghost")
        .variant(ButtonVariant::Ghost)
        .text("Ghost Button")
        .build();

    // Destructive button example
    let destructive_button = ButtonBuilder::new("Destructive")
        .variant(ButtonVariant::Solid)
        .color(theme().red)
        .text("Delete")
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
            // Title
            parent.spawn((
                Text::new(format!("Game UI Demo")),
                TextColor(theme().black.text),
                Node {
                    margin: UiRect::bottom(Val::Px(32.0)),
                    ..default()
                },
            ));

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
