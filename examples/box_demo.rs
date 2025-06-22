use bevy::prelude::*;
use ui::{
    components::{text::Text, BoxComponent, SpacingLevel},
    plugin::ForgeUiPlugin,
    theme::{color::theme, typography::TextSize},
    utilities::{ui_root, ComponentBuilder},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ForgeUiPlugin)
        .add_systems(Startup, setup_box_demo)
        .run();
}

fn setup_box_demo(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2d);

    // UI Root
    commands.spawn(ui_root("BoxDemo")).with_children(|parent| {
        // Main container with flex layout
        parent
            .spawn(
                BoxComponent::new("MainContainer")
                    .fill()
                    .pad(SpacingLevel::Base)
                    .build(),
            )
            .with_children(|parent| {
                // Title
                parent.spawn(
                    Text::title("Box Component Demo")
                        .size(TextSize::X3l)
                        .build(),
                );

                // Surface Box Demo
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

                // Panel Box Demo
                parent
                    .spawn(
                        BoxComponent::new("PanelDemo")
                            .panel()
                            .color(theme().blue)
                            .pad(SpacingLevel::Lg)
                            .margin_y(Val::Px(16.0))
                            .rounded_lg()
                            .build(),
                    )
                    .with_children(|parent| {
                        parent.spawn(
                            Text::body("Panel Box - Prominent background with blue theme").build(),
                        );
                    });

                // Card Box Demo
                parent
                    .spawn(
                        BoxComponent::new("CardDemo")
                            .card()
                            .color(theme().green)
                            .pad(SpacingLevel::X2l)
                            .margin_y(Val::Px(16.0))
                            .rounded_lg()
                            .shadow()
                            .build(),
                    )
                    .with_children(|parent| {
                        parent.spawn(
                            Text::body(
                                "Card Box - Elevated appearance with shadow and green theme",
                            )
                            .build(),
                        );
                    });

                // Outline Box Demo
                parent
                    .spawn(
                        BoxComponent::new("OutlineDemo")
                            .outline()
                            .color(theme().red)
                            .pad(SpacingLevel::Base)
                            .margin_y(Val::Px(16.0))
                            .border_width(2.0)
                            .rounded()
                            .build(),
                    )
                    .with_children(|parent| {
                        parent
                            .spawn(Text::body("Outline Box - Border only with red theme").build());
                    });

                // Size Demo Row
                parent
                    .spawn(
                        BoxComponent::new("SizeDemo")
                            .surface()
                            .pad(SpacingLevel::Base)
                            .margin_y(Val::Px(16.0))
                            .build(),
                    )
                    .with_children(|parent| {
                        parent.spawn(Text::body("Size Demonstrations:").build());

                        // Row of different sized boxes
                        parent
                            .spawn(Node {
                                display: Display::Flex,
                                flex_direction: FlexDirection::Row,
                                column_gap: Val::Px(16.0),
                                margin: UiRect::top(Val::Px(8.0)),
                                ..default()
                            })
                            .with_children(|parent| {
                                // Small square
                                parent.spawn(
                                    BoxComponent::new("SmallSquare")
                                        .card()
                                        .square(Val::Px(50.0))
                                        .rounded()
                                        .build(),
                                );

                                // Medium rectangle
                                parent.spawn(
                                    BoxComponent::new("MediumRect")
                                        .panel()
                                        .size(Val::Px(100.0), Val::Px(60.0))
                                        .rounded()
                                        .build(),
                                );

                                // Flexible box
                                parent
                                    .spawn(
                                        BoxComponent::new("FlexBox")
                                            .surface()
                                            .flex_1()
                                            .height(Val::Px(40.0))
                                            .pad(SpacingLevel::Sm)
                                            .rounded()
                                            .build(),
                                    )
                                    .with_children(|parent| {
                                        parent.spawn(Text::caption("Flexible").build());
                                    });
                            });
                    });

                // Positioning Demo
                parent
                    .spawn(
                        BoxComponent::new("PositioningDemo")
                            .outline()
                            .size(Val::Px(200.0), Val::Px(100.0))
                            .margin_y(Val::Px(16.0))
                            .position_relative()
                            .rounded()
                            .build(),
                    )
                    .with_children(|parent| {
                        parent.spawn(Text::caption("Relative container").build());

                        // Absolutely positioned child
                        parent
                            .spawn(
                                BoxComponent::new("AbsoluteChild")
                                    .card()
                                    .color(theme().red)
                                    .size(Val::Px(40.0), Val::Px(30.0))
                                    .position_absolute()
                                    .top(Val::Px(10.0))
                                    .right(Val::Px(10.0))
                                    .rounded_sm()
                                    .build(),
                            )
                            .with_children(|parent| {
                                parent.spawn(Text::caption("Abs").size(TextSize::Xs).build());
                            });
                    });

                // Custom styling demo
                parent
                    .spawn(
                        BoxComponent::new("CustomDemo")
                            .surface()
                            .background_color(Color::srgba(0.2, 0.8, 0.4, 0.3))
                            .border_color(Color::srgb(0.1, 0.6, 0.2))
                            .border_width(3.0)
                            .pad(SpacingLevel::Base)
                            .margin_y(Val::Px(16.0))
                            .rounded_lg()
                            .build(),
                    )
                    .with_children(|parent| {
                        parent.spawn(Text::body("Custom colors override theme colors").build());
                    });
            });
    });
}
