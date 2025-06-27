use bevy::prelude::*;
use ui::{
    components::{heading::Heading, separator::SeparatorBuilder, text::Text},
    plugin::{ForgeUiPlugin, UiState},
    theme::color::{accent_palette, theme_mode, ThemeMode, UiColorPalettes},
    utilities::ui_root::ui_root,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ForgeUiPlugin)
        .add_systems(OnEnter(UiState::Ready), setup_ui)
        .run();
}

fn setup_ui(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands
        .spawn(ui_root("Seperator Root"))
        .with_children(|parent| {
            // Title
            parent.spawn(Heading::h1("Separator Component Examples").build());

            // Horizontal separators section
            parent.spawn(Heading::h2("Horizontal Separators").build());

            parent.spawn(Text::body("Different sizes:").build());

            // Size 1 - Small
            parent.spawn(Text::label("Size 1 (Small)").build());
            parent.spawn(
                SeparatorBuilder::new()
                    .name("HorizontalSmall")
                    .horizontal()
                    .build(),
            );

            // Size 2 - Medium
            parent.spawn(Text::label("Size 2 (Medium)").build());
            parent.spawn(
                SeparatorBuilder::new()
                    .name("HorizontalMedium")
                    .horizontal()
                    .build(),
            );

            // Size 3 - Large
            parent.spawn(Text::label("Size 3 (Large)").build());
            parent.spawn(
                SeparatorBuilder::new()
                    .name("HorizontalLarge")
                    .horizontal()
                    .build(),
            );

            // Size 4 - Full width
            parent.spawn(Text::label("Size 4 (Full Width)").build());
            parent.spawn(
                SeparatorBuilder::new()
                    .name("HorizontalFull")
                    .horizontal()
                    .build(),
            );

            // Vertical separators section
            parent.spawn(Heading::h2("Vertical Separators").build());

            // Container for vertical separators (needs height)
            parent
                .spawn(Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(20.0),
                    height: Val::Px(100.0),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(Text::label("Size 1").build());
                    parent.spawn(
                        SeparatorBuilder::new()
                            .name("VerticalSmall")
                            .vertical()
                            .build(),
                    );

                    parent.spawn(Text::label("Size 2").build());
                    parent.spawn(
                        SeparatorBuilder::new()
                            .name("VerticalMedium")
                            .vertical()
                            .build(),
                    );

                    parent.spawn(Text::label("Size 3").build());
                    parent.spawn(
                        SeparatorBuilder::new()
                            .name("VerticalLarge")
                            .vertical()
                            .build(),
                    );

                    parent.spawn(Text::label("Size 4").build());
                    parent.spawn(
                        SeparatorBuilder::new()
                            .name("VerticalFull")
                            .vertical()
                            .build(),
                    );
                });

            // Color variants section
            parent.spawn(Heading::h2("Color Variants").build());

            parent.spawn(Text::label("Gray (Default)").build());
            parent.spawn(
                SeparatorBuilder::new()
                    .name("ColorGray")
                    .horizontal()
                    .color(if theme_mode() == ThemeMode::Dark {
                        UiColorPalettes::dark_mode().gray
                    } else {
                        UiColorPalettes::light_mode().gray
                    })
                    .build(),
            );

            parent.spawn(Text::label("Accent").build());
            parent.spawn(
                SeparatorBuilder::new()
                    .name("ColorAccent")
                    .horizontal()
                    .color(accent_palette())
                    .build(),
            );

            parent.spawn(Text::label("Blue").build());
            parent.spawn(
                SeparatorBuilder::new()
                    .name("ColorBlue")
                    .horizontal()
                    .color(if theme_mode() == ThemeMode::Dark {
                        UiColorPalettes::dark_mode().blue
                    } else {
                        UiColorPalettes::light_mode().blue
                    })
                    .build(),
            );

            parent.spawn(Text::label("Red").build());
            parent.spawn(
                SeparatorBuilder::new()
                    .name("ColorRed")
                    .horizontal()
                    .color(if theme_mode() == ThemeMode::Dark {
                        UiColorPalettes::dark_mode().red
                    } else {
                        UiColorPalettes::light_mode().red
                    })
                    .build(),
            );

            // Practical usage example
            parent.spawn(Heading::h2("Practical Usage Example").build());

            parent.spawn(Text::body("Tools for building high-quality, accessible UI.").build());

            parent.spawn(
                SeparatorBuilder::new()
                    .name("PracticalSeparator")
                    .horizontal()
                    .build(),
            );

            // Inline separated content
            parent
                .spawn(Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(12.0),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(Text::body("Themes").build());
                    parent.spawn(SeparatorBuilder::new().vertical().build());
                    parent.spawn(Text::body("Primitives").build());
                    parent.spawn(SeparatorBuilder::new().vertical().build());
                    parent.spawn(Text::body("Icons").build());
                    parent.spawn(SeparatorBuilder::new().vertical().build());
                    parent.spawn(Text::body("Colors").build());
                });
        });
}
