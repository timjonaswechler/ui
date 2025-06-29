use bevy::prelude::*;
use ui::{
    assets::{icons::interface::Interface, interface},
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
                    .text("text")
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

            // Icon Toggles Section
            parent.spawn((
                Name::new("IconSectionTitle"),
                Text::new("Icon Toggles"),
                TextColor(Color::WHITE),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
            ));

            // Icon Only Toggle
            parent.spawn((
                Name::new("IconOnlyToggle"),
                ToggleComponent::new("icon_only")
                    .text(Interface::new(interface::AArrowDown))
                    .variant(ToggleVariant::Soft)
                    .build(),
            ));

            // Icon + Text Toggle
            parent.spawn((
                Name::new("IconTextToggle"),
                ToggleComponent::new("icon_text")
                    .icon(InterfaceIconId::StarFilled)
                    .text("Favorite")
                    .variant(ToggleVariant::Outline)
                    .color(warning_palette())
                    .build(),
            ));

            // Icon Toggle - Pressed
            parent.spawn((
                Name::new("IconPressedToggle"),
                ToggleComponent::new("icon_pressed")
                    .icon(InterfaceIconId::Bookmark)
                    .text("Bookmarked")
                    .variant(ToggleVariant::Solid)
                    .pressed()
                    .build(),
            ));

            // Various Icon Examples
            parent.spawn((
                Name::new("GearToggle"),
                ToggleComponent::new("gear")
                    .icon(InterfaceIconId::Gear)
                    .text("Settings")
                    .variant(ToggleVariant::Surface)
                    .build(),
            ));

            parent.spawn((
                Name::new("BellToggle"),
                ToggleComponent::new("bell")
                    .icon(InterfaceIconId::Bell)
                    .text("Notifications")
                    .color(error_palette())
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
