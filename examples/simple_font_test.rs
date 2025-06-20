use bevy::prelude::*;
use ui::{
    components::{text::Text, FontFamily, TextWeight},
    plugin::ForgeUiPlugin,
    utilities::ComponentBuilder,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ForgeUiPlugin::new())
        .add_systems(Startup, setup_font_test)
        .run();
}

fn setup_font_test(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2d);

    // Verschiedene Fonts testen
    commands.spawn(
        Text::new("Sans Regular")
            .family(FontFamily::Sans)
            .weight(TextWeight::Regular)
            .build(),
    );
    commands.spawn(
        Text::new("Sans Bold")
            .family(FontFamily::Sans)
            .weight(TextWeight::Bold)
            .build(),
    );
    commands.spawn(
        Text::new("Sans Italic")
            .family(FontFamily::Sans)
            .weight(TextWeight::Regular)
            .italic()
            .build(),
    );

    commands.spawn(
        Text::new("Serif Regular")
            .family(FontFamily::Serif)
            .weight(TextWeight::Regular)
            .build(),
    );
    commands.spawn(
        Text::new("Serif Bold")
            .family(FontFamily::Serif)
            .weight(TextWeight::Bold)
            .build(),
    );
    commands.spawn(
        Text::new("Serif Italic")
            .family(FontFamily::Serif)
            .weight(TextWeight::Regular)
            .italic()
            .build(),
    );

    commands.spawn(
        Text::new("Mono Regular")
            .family(FontFamily::Mono)
            .weight(TextWeight::Regular)
            .build(),
    );
    commands.spawn(
        Text::new("Mono Bold")
            .family(FontFamily::Mono)
            .weight(TextWeight::Bold)
            .build(),
    );
    commands.spawn(
        Text::new("Mono Italic")
            .family(FontFamily::Mono)
            .weight(TextWeight::Regular)
            .italic()
            .build(),
    );
}
