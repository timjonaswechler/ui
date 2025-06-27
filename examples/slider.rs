use bevy::prelude::*;
use ui::{
    components::{
        heading::Heading,
        slider::{SliderBuilder, SliderComponent, SliderValueChangeEvent, SliderValueCommitEvent},
        text::Text,
    },
    plugin::{ForgeUiPlugin, UiState},
    theme::color::{accent_palette, theme_mode, ThemeMode, UiColorPalettes},
    utilities::ui_root::ui_root,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ForgeUiPlugin)
        .add_systems(OnEnter(UiState::Ready), setup_ui)
        .add_systems(Update, handle_slider_events)
        .run();
}

fn setup_ui(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands
        .spawn(ui_root("Slider Root"))
        .with_children(|parent| {
            // Title
            parent.spawn(Heading::h1("Slider Component Examples").build());

            // Basic slider section
            parent.spawn(Heading::h2("Basic Sliders").build());

            // Default slider (0-100, value 50)
            parent.spawn(Text::label("Default Slider (0-100, value 50)").build());
            parent.spawn(
                SliderBuilder::new()
                    .name("BasicSlider")
                    .value(50.0)
                    .range(0.0, 100.0)
                    .size2()
                    .build(),
            );

            // Size variants section
            parent.spawn(Heading::h2("Size Variants").build());

            // Size 1 - Small
            parent.spawn(Text::label("Size 1 (Small)").build());
            parent.spawn(
                SliderBuilder::new()
                    .name("SliderSmall")
                    .size1()
                    .value(25.0)
                    .build(),
            );

            // Size 2 - Medium (default)
            parent.spawn(Text::label("Size 2 (Medium - Default)").build());
            parent.spawn(
                SliderBuilder::new()
                    .name("SliderMedium")
                    .size2()
                    .value(50.0)
                    .build(),
            );

            // Size 3 - Large
            parent.spawn(Text::label("Size 3 (Large)").build());
            parent.spawn(
                SliderBuilder::new()
                    .name("SliderLarge")
                    .size3()
                    .value(75.0)
                    .build(),
            );

            // Orientation section
            parent.spawn(Heading::h2("Orientation").build());

            // Horizontal (default)
            parent.spawn(Text::label("Horizontal (Default)").build());
            parent.spawn(
                SliderBuilder::new()
                    .name("SliderHorizontal")
                    .horizontal()
                    .value(60.0)
                    .size2()
                    .build(),
            );

            // Vertical
            parent.spawn(Text::label("Vertical").build());
            parent
                .spawn(Node {
                    height: Val::Px(150.0),
                    width: Val::Px(50.0),
                    margin: UiRect::all(Val::Px(10.0)),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(
                        SliderBuilder::new()
                            .name("SliderVertical")
                            .vertical()
                            .value(40.0)
                            .size2()
                            .build(),
                    );
                });

            // Custom ranges section
            parent.spawn(Heading::h2("Custom Ranges").build());

            // Temperature slider (-20 to 40 Celsius)
            parent.spawn(Text::label("Temperature (-20°C to 40°C, current: 22°C)").build());
            parent.spawn(
                SliderBuilder::new()
                    .name("TemperatureSlider")
                    .range(-20.0, 40.0)
                    .value(22.0)
                    .step(1.0)
                    .size2()
                    .build(),
            );

            // Volume slider (0 to 1, step 0.1)
            parent.spawn(Text::label("Volume (0.0 to 1.0, step 0.1, current: 0.7)").build());
            parent.spawn(
                SliderBuilder::new()
                    .name("VolumeSlider")
                    .range(0.0, 1.0)
                    .value(0.7)
                    .step(0.1)
                    .size2()
                    .build(),
            );

            // Progress/Score slider (0 to 10)
            parent.spawn(Text::label("Score (0 to 10, current: 8.5)").build());
            parent.spawn(
                SliderBuilder::new()
                    .name("ScoreSlider")
                    .range(0.0, 10.0)
                    .value(8.5)
                    .step(0.5)
                    .size2()
                    .build(),
            );

            // Color variants section
            parent.spawn(Heading::h2("Color Variants").build());

            // Accent color (default)
            parent.spawn(Text::label("Accent Color (Default)").build());
            parent.spawn(
                SliderBuilder::new()
                    .name("SliderAccent")
                    .value(70.0)
                    .color(accent_palette())
                    .size2()
                    .build(),
            );

            // Blue color
            parent.spawn(Text::label("Blue Color").build());
            parent.spawn(
                SliderBuilder::new()
                    .name("SliderBlue")
                    .value(60.0)
                    .color(if theme_mode() == ThemeMode::Dark {
                        UiColorPalettes::dark_mode().blue
                    } else {
                        UiColorPalettes::light_mode().blue
                    })
                    .size2()
                    .build(),
            );

            // Green color
            parent.spawn(Text::label("Green Color").build());
            parent.spawn(
                SliderBuilder::new()
                    .name("SliderGreen")
                    .value(80.0)
                    .color(if theme_mode() == ThemeMode::Dark {
                        UiColorPalettes::dark_mode().green
                    } else {
                        UiColorPalettes::light_mode().green
                    })
                    .size2()
                    .build(),
            );

            // Disabled state section
            parent.spawn(Heading::h2("States").build());

            // Disabled slider
            parent.spawn(Text::label("Disabled Slider").build());
            parent.spawn(
                SliderBuilder::new()
                    .name("SliderDisabled")
                    .value(45.0)
                    .disabled()
                    .size2()
                    .build(),
            );

            // Interactive demonstration
            parent.spawn(Heading::h2("Interactive Demonstration").build());
            parent.spawn(
                Text::body("Try dragging the sliders above to see value changes in the console")
                    .build(),
            );

            // Practical use cases
            parent.spawn(Heading::h2("Practical Use Cases").build());

            // Settings panel simulation
            parent.spawn(Text::body("Settings Panel Example:").build());
            parent
                .spawn((
                    Node {
                        padding: UiRect::all(Val::Px(20.0)),
                        row_gap: Val::Px(15.0),
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    BackgroundColor(if theme_mode() == ThemeMode::Dark {
                        UiColorPalettes::dark_mode().gray.bg
                    } else {
                        UiColorPalettes::light_mode().gray.bg
                    }),
                    BorderRadius::all(Val::Px(8.0)),
                ))
                .with_children(|parent| {
                    parent.spawn(Text::label("Audio Settings").build());

                    // Master Volume
                    parent.spawn(Text::body("Master Volume").build());
                    parent.spawn(
                        SliderBuilder::new()
                            .name("MasterVolume")
                            .range(0.0, 100.0)
                            .value(75.0)
                            .step(5.0)
                            .size2()
                            .build(),
                    );

                    // Music Volume
                    parent.spawn(Text::body("Music Volume").build());
                    parent.spawn(
                        SliderBuilder::new()
                            .name("MusicVolume")
                            .range(0.0, 100.0)
                            .value(60.0)
                            .step(5.0)
                            .color(if theme_mode() == ThemeMode::Dark {
                                UiColorPalettes::dark_mode().blue
                            } else {
                                UiColorPalettes::light_mode().blue
                            })
                            .size2()
                            .build(),
                    );

                    // SFX Volume
                    parent.spawn(Text::body("SFX Volume").build());
                    parent.spawn(
                        SliderBuilder::new()
                            .name("SfxVolume")
                            .range(0.0, 100.0)
                            .value(85.0)
                            .step(5.0)
                            .color(if theme_mode() == ThemeMode::Dark {
                                UiColorPalettes::dark_mode().green
                            } else {
                                UiColorPalettes::light_mode().green
                            })
                            .size2()
                            .build(),
                    );
                });
        });
}

// System to handle slider events and log them
fn handle_slider_events(
    mut value_change_events: EventReader<SliderValueChangeEvent>,
    mut value_commit_events: EventReader<SliderValueCommitEvent>,
    slider_query: Query<&SliderComponent>,
) {
    // Handle live value changes (while dragging)
    for event in value_change_events.read() {
        if let Ok(slider) = slider_query.get(event.slider_entity) {
            println!(
                "Slider value changed - Entity: {:?}, Value: {:.2}, Range: {:.1}-{:.1}",
                event.slider_entity, event.value, slider.min, slider.max
            );
        }
    }

    // Handle value commits (when drag ends or track is clicked)
    for event in value_commit_events.read() {
        if let Ok(slider) = slider_query.get(event.slider_entity) {
            println!(
                "Slider value committed - Entity: {:?}, Final Value: {:.2}, Range: {:.1}-{:.1}",
                event.slider_entity, event.value, slider.min, slider.max
            );
        }
    }
}
