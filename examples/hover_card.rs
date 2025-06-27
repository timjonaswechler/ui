use bevy::prelude::*;
use ui::{
    components::{
        hover_card::{
            HoverCardBuilder, HoverCardContentBuilder, HoverCardSide, HoverCardTriggerBuilder,
        },
        text::Text,
    },
    plugin::ForgeUiPlugin,
    theme::color::accent_palette,
    utilities::ui_root::ui_root,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ForgeUiPlugin)
        .add_systems(Startup, setup_hover_card_demo)
        .run();
}

fn setup_hover_card_demo(mut commands: Commands) {
    // Setup camera and UI root
    commands.spawn(Camera2d);

    // Create UI root for the demo
    commands.spawn(ui_root("main"));

    // Create hover card root
    let hover_card = commands.spawn(HoverCardBuilder::new("demo").build()).id();

    // Create a container for the demo
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(20.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
        ))
        .with_children(|parent| {
            // Title
            parent.spawn(
                Text::title("HoverCard Demo")
                    .size(ui::theme::typography::TextSize::X4l)
                    .build(),
            );

            // Hover card trigger
            parent.spawn(
                HoverCardTriggerBuilder::new("demo_trigger", hover_card)
                    .text("Hover me for more information")
                    .text_size(ui::theme::typography::TextSize::Lg)
                    .build(),
            );

            // Create hover card content
            parent.spawn(
                HoverCardContentBuilder::new("demo_content", hover_card)
                    .side(HoverCardSide::Top)
                    .theme(accent_palette())
                    .arrow()
                    .build(),
            ).with_children(|content_parent| {
                content_parent.spawn(
                    Text::body("This is a hover card! It shows additional information when you hover over the trigger element.")
                        .build(),
                );
            });
        });
}
