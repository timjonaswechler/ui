use bevy::prelude::*;
use ui::{
    components::{text::Text, BoxComponent, FlexComponent, SpacingLevel},
    plugin::ForgeUiPlugin,
    theme::typography::TextSize,
    utilities::{ui_root, ComponentBuilder},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ForgeUiPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Spawn camera
    commands.spawn(Camera2d);

    // Create main container
    commands.spawn(ui_root("FlexDemo")).with_children(|parent| {
        // Title
        parent.spawn(
            Text::title("Flex Component Demo")
                .size(TextSize::X3l)
                .build(),
        );

        // Demo 1: Simple row layout
        parent
            .spawn(
                FlexComponent::row("Row Demo")
                    .gap(16.0)
                    .padding(Val::Px(20.0))
                    .background_color(Color::srgba(0.2, 0.2, 0.2, 0.5))
                    .margin_y(Val::Px(10.0))
                    .build(),
            )
            .with_children(|parent| {
                parent.spawn(Text::body("Item 1").build());
                parent.spawn(Text::body("Item 2").build());
                parent.spawn(Text::body("Item 3").build());
            });

        // Demo 2: Column layout with center alignment
        parent
            .spawn(
                FlexComponent::column("Column Demo")
                    .align_center()
                    .gap(12.0)
                    .padding(Val::Px(20.0))
                    .background_color(Color::srgba(0.2, 0.3, 0.4, 0.5))
                    .margin_y(Val::Px(10.0))
                    .build(),
            )
            .with_children(|parent| {
                parent.spawn(Text::body("Column Item 1").build());
                parent.spawn(Text::body("Column Item 2").build());
                parent.spawn(Text::body("Column Item 3").build());
            });

        // Demo 3: Centered layout
        parent
            .spawn(
                FlexComponent::center("Centered Demo")
                    .height(Val::Px(100.0))
                    .background_color(Color::srgba(0.4, 0.2, 0.3, 0.5))
                    .margin_y(Val::Px(10.0))
                    .build(),
            )
            .with_children(|parent| {
                parent.spawn(Text::body("Perfectly Centered!").build());
            });

        // Demo 4: Simple boxes with flex grow
        parent
            .spawn(
                FlexComponent::row("Flex Grow Demo")
                    .gap(8.0)
                    .padding(Val::Px(20.0))
                    .background_color(Color::srgba(0.2, 0.4, 0.3, 0.5))
                    .margin_y(Val::Px(10.0))
                    .build(),
            )
            .with_children(|parent| {
                // Fixed width box
                parent
                    .spawn(
                        BoxComponent::new("Fixed")
                            .panel()
                            .width(Val::Px(100.0))
                            .padding(Val::Px(8.0))
                            .build(),
                    )
                    .with_children(|item| {
                        item.spawn(Text::body("Fixed").build());
                    });

                // Flex grow box
                parent
                    .spawn(
                        BoxComponent::new("Flex Grow")
                            .panel()
                            .flex_1()
                            .padding(Val::Px(8.0))
                            .build(),
                    )
                    .with_children(|item| {
                        item.spawn(Text::body("Flex Grow").build());
                    });

                // Another fixed width box
                parent
                    .spawn(
                        BoxComponent::new("Fixed 2")
                            .panel()
                            .width(Val::Px(80.0))
                            .padding(Val::Px(8.0))
                            .build(),
                    )
                    .with_children(|item| {
                        item.spawn(Text::body("Fixed 2").build());
                    });
            });
    });
}
