use bevy::prelude::*;
use ui::{
    components::toggle::*,
    plugin::ForgeUiPlugin,
    theme::color::{accent_palette, error_palette, success_palette, warning_palette},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ForgeUiPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, handle_toggle_events)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // Simple demo with basic toggles
    commands
        .spawn((
            Name::new("ToggleDemo"),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(32.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(16.0),
                align_items: AlignItems::FlexStart,
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.11)),
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Name::new("Title"),
                Text::new("Toggle Component Demo"),
                TextColor(Color::WHITE),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
            ));

            // Basic Toggle
            parent.spawn((
                Name::new("BasicToggle"),
                ToggleComponent::new("basic")
                    .variant(ToggleVariant::Soft)
                    .build(),
            ));

            // Pressed Toggle
            parent.spawn((
                Name::new("PressedToggle"),
                ToggleComponent::new("pressed")
                    .variant(ToggleVariant::Solid)
                    .pressed()
                    .build(),
            ));

            // With Text Toggle
            parent.spawn((
                Name::new("TextToggle"),
                ToggleComponent::new("text")
                    .text("Bold")
                    .variant(ToggleVariant::Outline)
                    .build(),
            ));

            // Different Colors
            parent.spawn((
                Name::new("SuccessToggle"),
                ToggleComponent::new("success")
                    .color(success_palette())
                    .pressed()
                    .build(),
            ));

            parent.spawn((
                Name::new("ErrorToggle"),
                ToggleComponent::new("error")
                    .color(error_palette())
                    .text("Error State")
                    .build(),
            ));

            // Disabled Toggle
            parent.spawn((
                Name::new("DisabledToggle"),
                ToggleComponent::new("disabled")
                    .disabled()
                    .pressed()
                    .build(),
            ));
        });
}

fn handle_toggle_events(mut toggle_events: EventReader<ToggleChangeEvent>) {
    for event in toggle_events.read() {
        info!(
            "Toggle changed: Entity {:?}, pressed: {}, size: {:?}",
            event.toggle_entity, event.pressed, event.size
        );
    }
}
