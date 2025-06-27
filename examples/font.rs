use bevy::prelude::*;
use ui::{
    components::text::Text,
    plugin::{ForgeUiPlugin, UiState},
    theme::typography::{FontFamily, TextWeight},
    utilities::ui_root,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ForgeUiPlugin::new())
        .add_systems(OnEnter(UiState::Ready), setup_font_test)
        .run();
}

fn setup_font_test(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2d);
    commands.spawn(ui_root("main_ui"));
    // Root container
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(20.0),
                overflow: Overflow::scroll(),
                ..default()
            },
            BackgroundColor::from(Color::WHITE),
        ))
        .with_children(|parent| {
            // Test verschiedene Font-Familien und Gewichtungen

            // Sans Serif Tests
            parent.spawn(
                Text::new("Sans Light - Regular")
                    .family(FontFamily::Sans)
                    .weight(TextWeight::Light)
                    .build(),
            );
            parent.spawn(
                Text::new("Sans Regular - Regular")
                    .family(FontFamily::Sans)
                    .weight(TextWeight::Regular)
                    .build(),
            );
            parent.spawn(
                Text::new("Sans Medium - Regular")
                    .family(FontFamily::Sans)
                    .weight(TextWeight::Medium)
                    .build(),
            );
            parent.spawn(
                Text::new("Sans Bold - Regular")
                    .family(FontFamily::Sans)
                    .weight(TextWeight::Bold)
                    .build(),
            );

            // Sans Serif Italic Tests
            parent.spawn(
                Text::new("Sans Light - Italic")
                    .family(FontFamily::Sans)
                    .weight(TextWeight::Light)
                    .italic()
                    .build(),
            );
            parent.spawn(
                Text::new("Sans Regular - Italic")
                    .family(FontFamily::Sans)
                    .weight(TextWeight::Regular)
                    .italic()
                    .build(),
            );
            parent.spawn(
                Text::new("Sans Medium - Italic")
                    .family(FontFamily::Sans)
                    .weight(TextWeight::Medium)
                    .italic()
                    .build(),
            );
            parent.spawn(
                Text::new("Sans Bold - Italic")
                    .family(FontFamily::Sans)
                    .weight(TextWeight::Bold)
                    .italic()
                    .build(),
            );

            // Serif Tests
            parent.spawn(
                Text::new("Serif Light - Regular")
                    .family(FontFamily::Serif)
                    .weight(TextWeight::Light)
                    .build(),
            );
            parent.spawn(
                Text::new("Serif Regular - Regular")
                    .family(FontFamily::Serif)
                    .weight(TextWeight::Regular)
                    .build(),
            );
            parent.spawn(
                Text::new("Serif Medium - Regular")
                    .family(FontFamily::Serif)
                    .weight(TextWeight::Medium)
                    .build(),
            );
            parent.spawn(
                Text::new("Serif Bold - Regular")
                    .family(FontFamily::Serif)
                    .weight(TextWeight::Bold)
                    .build(),
            );

            // Serif Italic Tests
            parent.spawn(
                Text::new("Serif Light - Italic")
                    .family(FontFamily::Serif)
                    .weight(TextWeight::Light)
                    .italic()
                    .build(),
            );
            parent.spawn(
                Text::new("Serif Regular - Italic")
                    .family(FontFamily::Serif)
                    .weight(TextWeight::Regular)
                    .italic()
                    .build(),
            );
            parent.spawn(
                Text::new("Serif Medium - Italic")
                    .family(FontFamily::Serif)
                    .weight(TextWeight::Medium)
                    .italic()
                    .build(),
            );
            parent.spawn(
                Text::new("Serif Bold - Italic")
                    .family(FontFamily::Serif)
                    .weight(TextWeight::Bold)
                    .italic()
                    .build(),
            );

            // Mono Tests
            parent.spawn(
                Text::new("Mono Light - Regular")
                    .family(FontFamily::Mono)
                    .weight(TextWeight::Light)
                    .build(),
            );
            parent.spawn(
                Text::new("Mono Regular - Regular")
                    .family(FontFamily::Mono)
                    .weight(TextWeight::Regular)
                    .build(),
            );
            parent.spawn(
                Text::new("Mono Medium - Regular")
                    .family(FontFamily::Mono)
                    .weight(TextWeight::Medium)
                    .build(),
            );
            parent.spawn(
                Text::new("Mono Bold - Regular")
                    .family(FontFamily::Mono)
                    .weight(TextWeight::Bold)
                    .build(),
            );

            // Mono Italic Tests
            parent.spawn(
                Text::new("Mono Light - Italic")
                    .family(FontFamily::Mono)
                    .weight(TextWeight::Light)
                    .italic()
                    .build(),
            );
            parent.spawn(
                Text::new("Mono Regular - Italic")
                    .family(FontFamily::Mono)
                    .weight(TextWeight::Regular)
                    .italic()
                    .build(),
            );
            parent.spawn(
                Text::new("Mono Medium - Italic")
                    .family(FontFamily::Mono)
                    .weight(TextWeight::Medium)
                    .italic()
                    .build(),
            );
            parent.spawn(
                Text::new("Mono Bold - Italic")
                    .family(FontFamily::Mono)
                    .weight(TextWeight::Bold)
                    .italic()
                    .build(),
            );
        });
}
