use bevy::prelude::*;
use bevy_project::{AccentColor, ButtonBuilder, ButtonSize, ButtonVariant, RadixUIPlugin, ButtonClickEvent};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RadixUIPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, handle_button_events)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // Demo verschiedener Button-Features
    let solid_button = ButtonBuilder::new("Solid")
        .variant(ButtonVariant::Solid)
        .size(ButtonSize::Size2)
        .text("Solid Button")
        .build();

    let disabled_button = ButtonBuilder::new("Disabled")
        .variant(ButtonVariant::Soft)
        .size(ButtonSize::Size2)
        .text("Disabled Button")
        .disabled()
        .build();

    let loading_button = ButtonBuilder::new("Loading")
        .variant(ButtonVariant::Surface)
        .size(ButtonSize::Size2)
        .text("Loading Button")
        .loading()
        .build();

    let outline_button = ButtonBuilder::new("Outline")
        .variant(ButtonVariant::Outline)
        .size(ButtonSize::Size3)
        .color(AccentColor::Indigo)
        .text("Outline Button")
        .build();

    let ghost_button = ButtonBuilder::new("Ghost")
        .variant(ButtonVariant::Ghost)
        .size(ButtonSize::Size1)
        .text("Ghost Button")
        .build();

    // Container für die Buttons
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(16.0),
            ..default()
        })
        .with_children(|parent| {
            // Titel
            parent.spawn((
                Text::new("Radix UI Button Demo"),
                TextColor(Color::BLACK),
                Node {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                },
            ));

            // Buttons
            parent.spawn(solid_button);
            parent.spawn(disabled_button);
            parent.spawn(loading_button);
            parent.spawn(outline_button);
            parent.spawn(ghost_button);

            // Info Text
            parent.spawn((
                Text::new("Hover over buttons to see hover effects!\nClick active buttons to see events in console.\nLoading button shows animated spinner!"),
                TextColor(Color::srgb(0.5, 0.5, 0.5)),
                Node {
                    margin: UiRect::top(Val::Px(20.0)),
                    ..default()
                },
            ));
        });
}

fn handle_button_events(mut events: EventReader<ButtonClickEvent>) {
    for event in events.read() {
        info!("🎉 Custom Button Event! Entity: {:?}, Variant: {:?}", 
              event.button_entity, event.button_variant);
    }
}
