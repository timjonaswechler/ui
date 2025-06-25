use bevy::prelude::*;
use ui::{
    components::{text::Text, Heading, Section},
    plugin::{ForgeUiPlugin, UiState},
    utilities::{ui_root, ComponentBuilder},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ForgeUiPlugin)
        .add_systems(OnEnter(UiState::Ready), setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2d);

    // UI Root
    commands
        .spawn(ui_root("section_demo"))
        .with_children(|parent| {
            // Main container with some padding
            parent
                .spawn(
                    Section::size_4("main_container")
                        .padding_x(Val::Px(40.0))
                        .fill_width()
                        .build(),
                )
                .with_children(|parent| {
                    // Header Section - Large spacing
                    parent
                        .spawn(Section::size_4("header").build())
                        .with_children(|parent| {
                            parent.spawn(Heading::h1("Section Component Demo").build());
                            parent.spawn(
                                Text::body("Demonstrating Section component with different sizes and content types.")
                                    .build(),
                            );
                        });

                    // Features Section - Default spacing
                    parent
                        .spawn(Section::size_3("features").gap(16.0).build())
                        .with_children(|parent| {
                            parent.spawn(Heading::h2("Features").build());
                            parent.spawn(Text::body("Section provides semantic content structure with consistent vertical rhythm.").build());

                            // Subsection with smaller spacing
                            parent
                                .spawn(Section::size_2("feature_list").gap(12.0).build())
                                .with_children(|parent| {
                                    parent.spawn(Heading::h3("Key Benefits").build());
                                    parent.spawn(Text::body("• Consistent vertical spacing").build());
                                    parent.spawn(Text::body("• Semantic HTML section element").build());
                                    parent.spawn(Text::body("• Theme-integrated sizing").build());
                                    parent.spawn(Text::body("• Flexible content organization").build());
                                });
                        });

                    // Usage Section - Default spacing
                    parent
                        .spawn(Section::size_3("usage").build())
                        .with_children(|parent| {
                            parent.spawn(Heading::h2("Usage Examples").build());
                            parent.spawn(Text::body("Section components are ideal for organizing content hierarchically.").build());
                        });

                    // Size Demonstration Section
                    parent
                        .spawn(Section::size_3("size_demo").build())
                        .with_children(|parent| {
                            parent.spawn(Heading::h2("Size Variants").build());
                            parent.spawn(Text::body("Different section sizes create visual hierarchy:").build());

                            // Size 1 - Compact
                            parent
                                .spawn(
                                    Section::size_1("compact")
                                        .background_color(Color::srgba(0.2, 0.3, 0.4, 0.1))
                                        .rounded()
                                        .build(),
                                )
                                .with_children(|parent| {
                                    parent.spawn(Heading::h4("Size 1 - Compact").build());
                                    parent.spawn(Text::caption("Minimal vertical spacing (16px)").build());
                                });

                            // Size 2 - Small
                            parent
                                .spawn(
                                    Section::size_2("small")
                                        .background_color(Color::srgba(0.3, 0.4, 0.2, 0.1))
                                        .rounded()
                                        .build(),
                                )
                                .with_children(|parent| {
                                    parent.spawn(Heading::h4("Size 2 - Small").build());
                                    parent.spawn(Text::caption("Small vertical spacing (24px)").build());
                                });

                            // Size 3 - Default
                            parent
                                .spawn(
                                    Section::size_3("default")
                                        .background_color(Color::srgba(0.4, 0.2, 0.3, 0.1))
                                        .rounded()
                                        .build(),
                                )
                                .with_children(|parent| {
                                    parent.spawn(Heading::h4("Size 3 - Default").build());
                                    parent.spawn(Text::caption("Standard vertical spacing (32px)").build());
                                });

                            // Size 4 - Large
                            parent
                                .spawn(
                                    Section::size_4("large")
                                        .background_color(Color::srgba(0.2, 0.4, 0.3, 0.1))
                                        .rounded()
                                        .build(),
                                )
                                .with_children(|parent| {
                                    parent.spawn(Heading::h4("Size 4 - Large").build());
                                    parent.spawn(Text::caption("Generous vertical spacing (48px)").build());
                                });
                        });

                    // Content Styling Section
                    parent
                        .spawn(Section::size_3("styling").build())
                        .with_children(|parent| {
                            parent.spawn(Heading::h2("Styling Options").build());
                            parent.spawn(Text::body("Sections can be styled with backgrounds, borders, and gaps.").build());

                            // Example with border and background
                            parent
                                .spawn(
                                    Section::size_2("styled_section")
                                        .background_color(Color::srgba(0.1, 0.2, 0.3, 0.2))
                                        .border(2.0)
                                        .border_color(Color::srgba(0.3, 0.4, 0.5, 0.5))
                                        .rounded()
                                        .gap(8.0)
                                        .build(),
                                )
                                .with_children(|parent| {
                                    parent.spawn(Heading::h4("Styled Section").build());
                                    parent.spawn(Text::body("This section has a background, border, and custom gap.").build());
                                    parent.spawn(Text::caption("Perfect for highlighting important content.").build());
                                });
                        });

                    // Layout Options Section
                    parent
                        .spawn(Section::size_3("layout").build())
                        .with_children(|parent| {
                            parent.spawn(Heading::h2("Layout Control").build());
                            parent.spawn(Text::body("Sections support flexible layout options.").build());

                            // Centered content section
                            parent
                                .spawn(
                                    Section::size_2("centered")
                                        .center_content()
                                        .background_color(Color::srgba(0.2, 0.2, 0.2, 0.1))
                                        .rounded()
                                        .build(),
                                )
                                .with_children(|parent| {
                                    parent.spawn(Heading::h4("Centered Content").build());
                                    parent.spawn(Text::body("This section centers its content both horizontally and vertically.").build());
                                });

                            // Max-width constrained section
                            parent
                                .spawn(
                                    Section::size_2("constrained")
                                        .max_width(Val::Px(400.0))
                                        .background_color(Color::srgba(0.3, 0.2, 0.1, 0.1))
                                        .rounded()
                                        .build(),
                                )
                                .with_children(|parent| {
                                    parent.spawn(Heading::h4("Width Constrained").build());
                                    parent.spawn(Text::body("This section has a maximum width of 400px for better readability.").build());
                                });
                        });

                    // Footer Section - Compact spacing
                    parent
                        .spawn(Section::size_1("footer").margin_y(Val::Px(40.0)).build())
                        .with_children(|parent| {
                            parent.spawn(Text::caption("Section Demo - Built with Bevy UI Components").build());
                        });
                });
        });
}
