use bevy::prelude::*;
use ui::{
    components::{
        heading::{Heading, HeadingExt, HeadingLevel},
        text::{FontFamily, Text, TextColor, TextSize, TextWeight},
        ButtonBuilder,
    },
    plugin::ForgeUiPlugin,
    utilities::{ui_root, ComponentBuilder},
};
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ForgeUiPlugin)
        .add_systems(Startup, (setup_typography_showcase, setup_camera))
        .run();
}

fn setup_typography_showcase(mut commands: Commands) {
    // Create main UI root
    commands.spawn(ui_root("main_ui"));

    // Spawn typography showcase
    commands.spawn(
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(40.0)),
            row_gap: Val::Px(24.0),
            ..default()
        }
    ).with_children(|parent| {
        // Page Title (H1)
        parent.spawn(
            Heading::h1("Typography Showcase")
                .build()
        );

        // Subtitle with custom styling
        parent.spawn(
            Text::new("Demonstrating the power of our typography system")
                .size(TextSize::Lg)
                .color(TextColor::Muted)
                .italic()
                .build()
        );

        // Headings Section
        parent.spawn(
            Heading::h2("Heading Hierarchy")
                .build()
        );

        // All heading levels
        parent.spawn(Heading::h1("Heading 1 - Main Title").build());
        parent.spawn(Heading::h2("Heading 2 - Section Title").build());
        parent.spawn(Heading::h3("Heading 3 - Subsection").build());
        parent.spawn(Heading::h4("Heading 4 - Minor Section").build());
        parent.spawn(Heading::h5("Heading 5 - Small Heading").build());
        parent.spawn(Heading::h6("Heading 6 - Smallest Heading").build());

        // Text Variants Section
        parent.spawn(
            Heading::h2("Text Variants")
                .build()
        );

        parent.spawn(
            Text::display("Display Text - Hero Content")
                .build()
        );

        parent.spawn(
            Text::title("Title Text - Important Sections")
                .build()
        );

        parent.spawn(
            Text::body("Body Text - This is the standard text used for most content. It's readable and comfortable for longer passages of text.")
                .build()
        );

        parent.spawn(
            Text::label("Label Text - Form Labels and Descriptions")
                .build()
        );

        parent.spawn(
            Text::caption("Caption Text - Small supplementary information and footnotes")
                .build()
        );

        // Font Families Section
        parent.spawn(
            Heading::h2("Font Families")
                .build()
        );

        parent.spawn(
            Text::new("Sans-serif font family (default)")
                .family(FontFamily::Sans)
                .build()
        );

        parent.spawn(
            Text::new("Serif font family for elegant text")
                .family(FontFamily::Serif)
                .build()
        );

        parent.spawn(
            Text::new("Monospace font family for code")
                .family(FontFamily::Mono)
                .build()
        );

        // Text Weights Section
        parent.spawn(
            Heading::h2("Font Weights")
                .build()
        );

        parent.spawn(
            Text::new("Light weight text")
                .weight(TextWeight::Light)
                .build()
        );

        parent.spawn(
            Text::new("Regular weight text (default)")
                .weight(TextWeight::Regular)
                .build()
        );

        parent.spawn(
            Text::new("Medium weight text")
                .weight(TextWeight::Medium)
                .build()
        );

        parent.spawn(
            Text::new("Bold weight text")
                .weight(TextWeight::Bold)
                .build()
        );

        // Italic Text Section
        parent.spawn(
            Heading::h2("Text Styles")
                .build()
        );

        parent.spawn(
            Text::new("Regular text without italic")
                .build()
        );

        parent.spawn(
            Text::new("Italic text for emphasis")
                .italic()
                .build()
        );

        parent.spawn(
            Text::new("Bold italic combination")
                .weight(TextWeight::Bold)
                .italic()
                .build()
        );

        // Text Colors Section
        parent.spawn(
            Heading::h2("Text Colors")
                .build()
        );

        parent.spawn(
            Text::new("Default text color")
                .color(TextColor::Default)
                .build()
        );

        parent.spawn(
            Text::new("Muted text color for secondary information")
                .color(TextColor::Muted)
                .build()
        );

        parent.spawn(
            Text::new("Accent color for highlights")
                .color(TextColor::Accent)
                .weight(TextWeight::Medium)
                .build()
        );

        parent.spawn(
            Text::new("Error color for warnings")
                .color(TextColor::Error)
                .weight(TextWeight::Medium)
                .build()
        );

        parent.spawn(
            Text::new("Success color for confirmations")
                .color(TextColor::Success)
                .weight(TextWeight::Medium)
                .build()
        );

        parent.spawn(
            Text::new("Warning color for cautions")
                .color(TextColor::Warning)
                .weight(TextWeight::Medium)
                .build()
        );

        // Text Alignment Section
        parent.spawn(
            Heading::h2("Text Alignment")
                .build()
        );

        parent.spawn(
            Text::new("Left aligned text (default)")
                .build()
        );

        parent.spawn(
            Text::new("Center aligned text")
                .center()
                .build()
        );

        parent.spawn(
            Text::new("Right aligned text")
                .right()
                .build()
        );

        // Mixed Examples Section
        parent.spawn(
            Heading::h2("Mixed Examples")
                .build()
        );

        parent.spawn(
            Text::new("This is a complex example with ")
                .build()
        );
        // Note: In a real scenario, you'd use text spans for mixed styling within one text block

        // Button with Text Integration
        parent.spawn(
            Heading::h3("Button Integration")
                .build()
        );

        parent.spawn(
            Text::body("Buttons automatically use the typography system:")
                .build()
        );

        // Container for buttons
        parent.spawn(
            Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(16.0),
                margin: UiRect::top(Val::Px(16.0)),
                ..default()
            }
        ).with_children(|button_parent| {
            button_parent.spawn(
                ButtonBuilder::new("Small Button")
                    .size(ui::components::ButtonSize::Small)
                    .text("Small Button")
                    .build()
            );

            button_parent.spawn(
                ButtonBuilder::new("Default Button")
                    .size(ui::components::ButtonSize::Default)
                    .text("Default Button")
                    .build()
            );

            button_parent.spawn(
                ButtonBuilder::new("Large Button")
                    .size(ui::components::ButtonSize::Large)
                    .text("Large Button")
                    .build()
            );
        });

        // Extension Trait Example
        parent.spawn(
            Heading::h3("Extension Trait Usage")
                .build()
        );

        parent.spawn(
            Text::new("This text becomes a heading")
                .as_heading(HeadingLevel::H4)
                .color(TextColor::Accent)
                .build()
        );

        // Final Note
        parent.spawn(
            Text::body("All typography components automatically inherit theme colors, font families, and sizing scales.")
                .color(TextColor::Muted)
                .italic()
                .build()
        );
    });
}
