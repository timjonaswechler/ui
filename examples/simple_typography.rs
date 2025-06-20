use bevy::prelude::*;
use ui::{
    components::{
        heading::Heading,
        text::{FontFamily, Text, TextColor, TextSize, TextWeight},
    },
    plugin::ForgeUiPlugin,
    utilities::{ui_root, ComponentBuilder},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ForgeUiPlugin)
        .add_systems(Startup, (setup_camera, setup_simple_typography))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn setup_simple_typography(mut commands: Commands) {
    info!("Setting up simple typography showcase");
    // Create the main UI container
    commands.spawn(ui_root("main_ui"));

    // Create a simple content layout
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(20.0),
                ..default()
            },
            BackgroundColor::from(Color::WHITE),
        ))
        .with_children(|parent| {
            // Main title using Heading
            parent.spawn(Heading::h1("Welcome to Our App").build());

            // Subtitle using Text with custom styling
            parent.spawn(
                Text::new("A simple example of typography in action")
                    .size(TextSize::Lg)
                    .color(TextColor::Muted)
                    .build(),
            );

            // Section heading
            parent.spawn(Heading::h2("Getting Started").build());

            // Body text
            parent.spawn(Text::body("This is regular body text that's easy to read.").build());

            // Emphasized text
            parent.spawn(
                Text::new("This text is bold and italic")
                    .weight(TextWeight::Bold)
                    .italic()
                    .build(),
            );

            // Code-style text
            parent.spawn(
                Text::new("fn main() { println!(\"Hello, World!\"); }")
                    .family(FontFamily::Mono)
                    .color(TextColor::Accent)
                    .build(),
            );

            // Small caption
            parent.spawn(Text::caption("This is a small caption with muted color").build());
        });
}

// Alternative usage examples for documentation:

fn _text_examples() {
    // Basic text
    let _simple = Text::new("Hello World").build();

    // Styled text
    let _styled = Text::new("Important Message")
        .size(TextSize::Lg)
        .weight(TextWeight::Bold)
        .color(TextColor::Error)
        .center()
        .build();

    // Semantic variants
    let _display = Text::display("Hero Title").build();
    let _title = Text::title("Section Title").build();
    let _body = Text::body("Body content").build();
    let _label = Text::label("Form Label").build();
    let _caption = Text::caption("Small text").build();

    // Font families
    let _sans = Text::new("Sans-serif text")
        .family(FontFamily::Sans)
        .build();
    let _serif = Text::new("Serif text").family(FontFamily::Serif).build();
    let _mono = Text::new("Monospace text").family(FontFamily::Mono).build();

    // All weights with italic
    let _light_italic = Text::new("Light Italic")
        .weight(TextWeight::Light)
        .italic()
        .build();
    let _bold_italic = Text::new("Bold Italic")
        .weight(TextWeight::Bold)
        .italic()
        .build();
}

fn _heading_examples() {
    // Direct heading creation
    let _h1 = Heading::h1("Main Title").build();
    let _h2 = Heading::h2("Section Title").build();
    let _h3 = Heading::h3("Subsection").build();

    // All heading levels
    let _h1 = Heading::h1("Heading 1").build(); // X5l + Bold
    let _h2 = Heading::h2("Heading 2").build(); // X3l + Bold
    let _h3 = Heading::h3("Heading 3").build(); // X2l + Medium
    let _h4 = Heading::h4("Heading 4").build(); // Xl + Medium
    let _h5 = Heading::h5("Heading 5").build(); // Lg + Medium
    let _h6 = Heading::h6("Heading 6").build(); // Base + Medium

    // Custom styling on headings (still possible)
    let _custom_heading = Heading::h2("Custom Heading")
        .color(TextColor::Accent)
        .italic()
        .build();
}
