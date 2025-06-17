use bevy::prelude::*;
use bevy_project::{RadixUIPlugin, ButtonBuilder, ButtonVariant, ButtonSize, AccentColor};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RadixUIPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // Button mit verschiedenen Varianten
    let button1 = ButtonBuilder::new("Solid Button")
        .variant(ButtonVariant::Solid)
        .size(ButtonSize::Size2)
        .text("Solid Button")
        .build();

    let button2 = ButtonBuilder::new("Soft Button")
        .variant(ButtonVariant::Soft)
        .size(ButtonSize::Size2)
        .color(AccentColor::Indigo)
        .text("Soft Button")
        .build();

    let button3 = ButtonBuilder::new("Outline Button")
        .variant(ButtonVariant::Outline)
        .size(ButtonSize::Size3)
        .high_contrast()
        .text("Outline Button")
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
            parent.spawn(button1).with_children(|child| {
                child.spawn(Text::new("Solid Button"));
            });
            parent.spawn(button2).with_children(|child| {
                child.spawn(Text::new("Soft Button"));
            });
            parent.spawn(button3).with_children(|child| {
                child.spawn(Text::new("Outline Button"));
            });
        });
}