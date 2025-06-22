use bevy::prelude::*;
use ui::{
    components::{text::Text, BoxComponent, ContainerSize, FlexComponent},
    plugin::ForgeUiPlugin,
    theme::typography::TextSize,
    utilities::{ui_root, ComponentBuilder},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ForgeUiPlugin)
        .add_systems(Startup, setup_ui)
        .run();
}

fn setup_ui(mut commands: Commands) {
    // Create camera
    commands.spawn(Camera2d);

    // Create UI root with centering
    commands
        .spawn(ui_root("Container Example"))
        .with_children(|parent| {
            // Title
            parent.spawn(
                Text::title("Container Component Examples")
                    .size(TextSize::X3l)
                    .build(),
            );

            // Section spacing
            parent.spawn(
                BoxComponent::new("spacing-1")
                    .height(Val::Px(32.0))
                    .build(),
            );

            // Section title
            parent.spawn(
                Text::title("Container Sizes Comparison")
                    .size(TextSize::Xl)
                    .build(),
            );

            // Demo 1: All container sizes comparison
                // Create centering wrapper for each container
                for (i, (size, label, description)) in [
                    (ContainerSize::Size1, "Size 1", "448px - Mobile first"),
                    (ContainerSize::Size2, "Size 2", "688px - Tablet"),
                    (ContainerSize::Size3, "Size 3", "880px - Desktop"),
                    (ContainerSize::Size4, "Size 4", "1136px - Wide desktop"),
                ]
                .iter()
                .enumerate()
                {
                    // Centering wrapper
                    parent
                        .spawn(
                            FlexComponent::center("container-wrapper")
                                .fill_width()
                                .margin_y(Val::Px(8.0))
                                .build(),
                        )
                        .with_children(|centered_parent| {
                            // Container with content
                            centered_parent
                                .spawn(
                                    BoxComponent::new(&format!("container-{}", i))
                                        .container(*size)
                                        .panel()
                                        .padding(Val::Px(20.0))
                                        .build(),
                                )
                                .with_children(|container| {
                                    // Header
                                    container.spawn(
                                        Text::title(*label)
                                            .size(TextSize::Lg)
                                            .build(),
                                    );

                                    // Description
                                    container.spawn(
                                        Text::body(*description)
                                            .size(TextSize::Sm)
                                            .build(),
                                    );

                                    // Sample content
                                    container.spawn(
                                        Text::body("This container demonstrates the max-width constraint. The container will take 100% width until it reaches its maximum width limit, then it stops growing.")
                                            .build(),
                                    );
                                });
                        });
                }

            // Section spacing
            parent.spawn(
                BoxComponent::new("spacing-2")
                    .height(Val::Px(32.0))
                    .build(),
            );

            // Section title
            parent.spawn(
                Text::title("Container with Content Layouts")
                    .size(TextSize::Xl)
                    .build(),
            );

            // Demo 2: Container with different content layouts
                // Centering wrapper
                parent
                    .spawn(
                        FlexComponent::center("layout-demo-wrapper")
                            .fill_width()
                            .build(),
                    )
                    .with_children(|centered_parent| {
                        // Container with nested flex layouts
                        centered_parent
                            .spawn(
                                BoxComponent::container_3("content-demo")
                                    .panel()
                                    .padding(Val::Px(24.0))
                                    .build(),
                            )
                            .with_children(|container| {
                                // Header section
                                container
                                    .spawn(
                                        FlexComponent::row("header")
                                            .justify_between()
                                            .align_center()
                                            .margin_y(Val::Px(16.0))
                                            .build(),
                                    )
                                    .with_children(|header| {
                                        header.spawn(
                                            Text::title("Article Title")
                                                .size(TextSize::X2l)
                                                .build(),
                                        );
                                        header.spawn(
                                            Text::body("2024-06-22")
                                                .size(TextSize::Sm)
                                                .build(),
                                        );
                                    });

                                // Content section
                                container
                                    .spawn(
                                        FlexComponent::column("content")
                                            .gap(16.0)
                                            .build(),
                                    )
                                    .with_children(|content| {
                                        content.spawn(
                                            Text::body("This is a demonstration of how containers work with complex content layouts. The container provides the max-width constraint while allowing full flexibility for internal layouts.")
                                                .build(),
                                        );

                                        // Card-like sections within container
                                        for (i, section_title) in [
                                            "Introduction",
                                            "Main Content", 
                                            "Conclusion"
                                        ].iter().enumerate() {
                                            content
                                                .spawn(
                                                    BoxComponent::new(&format!("section-{}", i))
                                                        .card()
                                                        .padding(Val::Px(16.0))
                                                        .build(),
                                                )
                                                .with_children(|section| {
                                                    section.spawn(
                                                        Text::title(*section_title)
                                                            .size(TextSize::Lg)
                                                            .build(),
                                                    );
                                                    section.spawn(
                                                        Text::body("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.")
                                                            .build(),
                                                    );
                                                });
                                        }
                                    });
                            });
                    });

            // Section spacing
            parent.spawn(
                BoxComponent::new("spacing-3")
                    .height(Val::Px(32.0))
                    .build(),
            );

            // Section title
            parent.spawn(
                Text::title("Container Convenience Constructors")
                    .size(TextSize::Xl)
                    .build(),
            );

            // Demo 3: Container convenience constructors
                parent
                    .spawn(
                        FlexComponent::column("constructors-demo")
                            .gap(12.0)
                            .align_center()
                            .build(),
                    )
                    .with_children(|column| {
                        // Show different ways to create containers
                        for (constructor, size_name) in [
                            ("BoxComponent::container_1()", "Size 1"),
                            ("BoxComponent::container_2()", "Size 2"),
                            ("BoxComponent::container_3()", "Size 3"),
                            ("BoxComponent::container_4()", "Size 4"),
                        ] {
                            column
                                .spawn(
                                    BoxComponent::new("demo-constructor")
                                        .container(match size_name {
                                            "Size 1" => ContainerSize::Size1,
                                            "Size 2" => ContainerSize::Size2,
                                            "Size 3" => ContainerSize::Size3,
                                            _ => ContainerSize::Size4,
                                        })
                                        .outline()
                                        .padding(Val::Px(12.0))
                                        .build(),
                                )
                                .with_children(|container| {
                                    container.spawn(
                                        Text::body(&format!("{} → {}", constructor, size_name))
                                            .size(TextSize::Sm)
                                            .build(),
                                    );
                                });
                        }
                    });
        });
}

