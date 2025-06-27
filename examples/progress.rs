use bevy::prelude::*;
use ui::{
    components::{
        heading::Heading,
        progress::{ProgressBuilder, ProgressComponent},
        text::Text,
    },
    plugin::{ForgeUiPlugin, UiState},
    theme::color::{theme_mode, ThemeMode, UiColorPalettes},
    utilities::ui_root::ui_root,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ForgeUiPlugin)
        .add_systems(OnEnter(UiState::Ready), setup_ui)
        .add_systems(Update, simulate_progress)
        .run();
}

fn setup_ui(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands
        .spawn(ui_root("Progress Root"))
        .with_children(|parent| {
            // Title
            parent.spawn(Heading::h1("Progress Component Examples").build());

            // // Size variants section
            // parent.spawn(Heading::h2("Size Variants").build());

            // // Size 1 - Small (4px height)
            // parent.spawn(Text::label("Size 1 (Small - 4px height)").build());
            // parent.spawn(
            //     ProgressBuilder::new()
            //         .name("ProgressSmall")
            //         .size1()
            //         .value(0.75)
            //         .build(),
            // );

            // // Size 2 - Medium (6px height)
            // parent.spawn(Text::label("Size 2 (Medium - 6px height)").build());
            // parent.spawn(
            //     ProgressBuilder::new()
            //         .name("ProgressMedium")
            //         .size2()
            //         .value(0.45)
            //         .build(),
            // );

            // // Size 3 - Large (8px height)
            // parent.spawn(Text::label("Size 3 (Large - 8px height)").build());
            // parent.spawn(
            //     ProgressBuilder::new()
            //         .name("ProgressLarge")
            //         .size3()
            //         .value(0.90)
            //         .build(),
            // );

            // Different value representations
            parent.spawn(Heading::h2("Value Representations").build());

            // Percentage (0-100)
            parent.spawn(Text::label("Percentage (65%)").build());
            parent.spawn(
                ProgressBuilder::new()
                    .name("ProgressPercentage")
                    .percentage(65.0)
                    .size2()
                    .build(),
            );

            // File download progress (bytes)
            parent.spawn(Text::label("File Download (750MB / 1GB)").build());
            parent.spawn(
                ProgressBuilder::new()
                    .name("ProgressFileDownload")
                    .progress(750.0, 1000.0)
                    .size2()
                    .label("Downloading large file...")
                    .build(),
            );

            // Custom range (1-10)
            parent.spawn(Text::label("Custom Range (7 / 10 steps)").build());
            parent.spawn(
                ProgressBuilder::new()
                    .name("ProgressSteps")
                    .value(7.0)
                    .max(10.0)
                    .size2()
                    .build(),
            );

            // Indeterminate progress section
            parent.spawn(Heading::h2("Indeterminate Progress").build());
            parent.spawn(Text::body("For when you don't know the total amount").build());

            // Small indeterminate
            parent.spawn(Text::label("Small Indeterminate").build());
            parent.spawn(
                ProgressBuilder::new()
                    .name("IndeterminateSmall")
                    .indeterminate()
                    .size1()
                    .build(),
            );

            // Medium indeterminate
            parent.spawn(Text::label("Medium Indeterminate").build());
            parent.spawn(
                ProgressBuilder::new()
                    .name("IndeterminateMedium")
                    .indeterminate()
                    .size2()
                    .build(),
            );

            // Large indeterminate
            // parent.spawn(Text::label("Large Indeterminate").build());
            // parent.spawn(
            //     ProgressBuilder::new()
            //         .name("IndeterminateLarge")
            //         .indeterminate()
            //         .size3()
            //         .build(),
            // );

            // // Color variants section
            // parent.spawn(Heading::h2("Color Variants").build());

            // // Accent color (default)
            // parent.spawn(Text::label("Accent Color (Default)").build());
            // parent.spawn(
            //     ProgressBuilder::new()
            //         .name("ProgressAccent")
            //         .value(0.80)
            //         .color(accent_palette())
            //         .size2()
            //         .build(),
            // );

            // // Blue color
            // parent.spawn(Text::label("Blue Color").build());
            // parent.spawn(
            //     ProgressBuilder::new()
            //         .name("ProgressBlue")
            //         .value(0.60)
            //         .color(if theme_mode() == ThemeMode::Dark {
            //             UiColorPalettes::dark_mode().blue
            //         } else {
            //             UiColorPalettes::light_mode().blue
            //         })
            //         .size2()
            //         .build(),
            // );

            // // Green color
            // parent.spawn(Text::label("Green Color").build());
            // parent.spawn(
            //     ProgressBuilder::new()
            //         .name("ProgressGreen")
            //         .value(0.95)
            //         .color(if theme_mode() == ThemeMode::Dark {
            //             UiColorPalettes::dark_mode().green
            //         } else {
            //             UiColorPalettes::light_mode().green
            //         })
            //         .size2()
            //         .build(),
            // );

            // // Red color (error state)
            // parent.spawn(Text::label("Red Color (Error State)").build());
            // parent.spawn(
            //     ProgressBuilder::new()
            //         .name("ProgressRed")
            //         .value(0.25)
            //         .color(if theme_mode() == ThemeMode::Dark {
            //             UiColorPalettes::dark_mode().red
            //         } else {
            //             UiColorPalettes::light_mode().red
            //         })
            //         .size2()
            //         .build(),
            // );

            // Simulated dynamic progress
            parent.spawn(Heading::h2("Dynamic Progress").build());
            parent.spawn(Text::body("This progress bar updates over time (simulation)").build());

            parent.spawn((
                ProgressBuilder::new()
                    .name("DynamicProgress")
                    .value(0.0)
                    .label("Loading simulation...")
                    .size2()
                    .build(),
                SimulatedProgress {
                    target: 1.0,
                    speed: 0.3, // Progress per second
                },
            ));

            // Practical usage examples
            parent.spawn(Heading::h2("Practical Usage Examples").build());

            // Loading screen simulation
            parent.spawn(Text::body("Loading Screen Example:").build());
            parent
                .spawn((
                    Node {
                        padding: UiRect::all(Val::Px(20.0)),
                        row_gap: Val::Px(10.0),
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
                    parent.spawn(Text::body("Loading application...").build());
                    parent.spawn(
                        ProgressBuilder::new()
                            .name("LoadingScreen")
                            .indeterminate()
                            .size2()
                            .build(),
                    );
                });
        });
}

// Component for simulating dynamic progress
#[derive(Component)]
struct SimulatedProgress {
    target: f32,
    speed: f32,
}

// System to simulate progress updates
fn simulate_progress(
    time: Res<Time>,
    mut query: Query<(&mut ProgressComponent, &SimulatedProgress)>,
) {
    for (mut progress, simulation) in query.iter_mut() {
        if progress.value < simulation.target {
            progress.value += simulation.speed * time.delta_secs();
            progress.value = progress.value.min(simulation.target);
        }
    }
}
