use bevy::prelude::*;
use ui::{
    components::{
        BoxComponent, SpacingLevel,
        text::Text,
    },
    plugin::ForgeUiPlugin,
    theme::color::accent_palette,
    utilities::ui_root,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ForgeUiPlugin)
        .add_systems(Startup, setup_simple_box_demo)
        .run();
}

fn setup_simple_box_demo(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2d);

    // UI Root
    commands
        .spawn(ui_root("SimpleBoxDemo"))
        .with_children(|parent| {
            // Main container
            parent
                .spawn(
                    BoxComponent::new("MainContainer")
                        .fill()
                        .pad(SpacingLevel::Base)
                        .build(),
                )
                .with_children(|parent| {
                    // Title
                    parent.spawn(Text::title("Box Component Demo").build());

                    // Simple surface box
                    parent
                        .spawn(
                            BoxComponent::new("SurfaceDemo")
                                .surface()
                                .pad(SpacingLevel::Base)
                                .margin_y(Val::Px(16.0))
                                .rounded()
                                .build(),
                        )
                        .with_children(|parent| {
                            parent.spawn(Text::body("Surface Box - Subtle background").build());
                        });

                    // Panel box with different color
                    parent
                        .spawn(
                            BoxComponent::new("PanelDemo")
                                .panel()
                                .color(accent_palette())
                                .pad(SpacingLevel::Lg)
                                .margin_y(Val::Px(16.0))
                                .rounded_lg()
                                .build(),
                        )
                        .with_children(|parent| {
                            parent.spawn(Text::body("Panel Box - Prominent background").build());
                        });

                    // Card box with shadow
                    parent
                        .spawn(
                            BoxComponent::new("CardDemo")
                                .card()
                                .pad(SpacingLevel::X2l)
                                .margin_y(Val::Px(16.0))
                                .rounded_lg()
                                .shadow()
                                .build(),
                        )
                        .with_children(|parent| {
                            parent.spawn(
                                Text::body("Card Box - Elevated appearance with shadow").build(),
                            );
                        });

                    // Outline box
                    parent
                        .spawn(
                            BoxComponent::new("OutlineDemo")
                                .outline()
                                .pad(SpacingLevel::Base)
                                .margin_y(Val::Px(16.0))
                                .border_width(2.0)
                                .rounded()
                                .build(),
                        )
                        .with_children(|parent| {
                            parent.spawn(Text::body("Outline Box - Border only").build());
                        });
                });
        });
}
