use bevy::prelude::*;
use ui::{
    components::*,
    utilities::ui_root,
    plugin::ForgeUiPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ForgeUiPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, log_select_events)
        .run();
}

/// Log select events to verify they're working
fn log_select_events(
    mut select_change_events: EventReader<SelectChangeEvent>,
    mut select_open_events: EventReader<SelectOpenEvent>,
) {
    for event in select_change_events.read() {
        info!("✅ SelectChangeEvent: entity={:?}, value='{}', label='{}', previous={:?}",
              event.select_entity, event.selected_value, event.selected_label, event.previous_value);
    }
    
    for event in select_open_events.read() {
        info!("🔄 SelectOpenEvent: entity={:?}, open={}",
              event.select_entity, event.open);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // Create overlay layer for dropdowns
    commands.spawn(ui_root("overlay_layer"));

    commands
        .spawn(ui_root("select-example"))
        .with_children(|root| {
            root.spawn(
                Flex::new("main-container")
                    .column()
                    .gap(20.0)
                    .padding(Val::Px(20.0))
                    .align_center()
                    .justify_center()
                    .build(),
            )
            .with_children(|container| {
                container.spawn(
                    Heading::h2("Select Component Examples")
                        .build(),
                );

                // Basic Select with custom options
                container.spawn(
                    Select::new()
                        .placeholder("Choose an option...")
                        .option("red", "Red Color")
                        .option("green", "Green Color")
                        .option("blue", "Blue Color")
                        .width(Val::Px(200.0))
                        .build(),
                );

                // Select with pre-selected value (test Radix positioning)
                container.spawn(
                    Select::new()
                        .placeholder("Choose a fruit...")
                        .option("apple", "Apple")
                        .option("orange", "Orange")
                        .option("grape", "Grape")
                        .selected_value("orange")  // This should align with Orange in dropdown
                        .width(Val::Px(200.0))
                        .build(),
                );

                // Size variants
                container.spawn(
                    Select::new()
                        .placeholder("Small Select")
                        .size_1()
                        .width(Val::Px(200.0))
                        .build(),
                );

                container.spawn(
                    Select::new()
                        .placeholder("Default Select")
                        .size_2()
                        .width(Val::Px(200.0))
                        .build(),
                );

                container.spawn(
                    Select::new()
                        .placeholder("Large Select")
                        .size_3()
                        .width(Val::Px(200.0))
                        .build(),
                );

                // Style variants
                container.spawn(
                    Select::new()
                        .placeholder("Surface variant")
                        .surface()
                        .width(Val::Px(200.0))
                        .build(),
                );

                container.spawn(
                    Select::new()
                        .placeholder("Classic variant")
                        .classic()
                        .width(Val::Px(200.0))
                        .build(),
                );

                container.spawn(
                    Select::new()
                        .placeholder("Soft variant")
                        .soft()
                        .width(Val::Px(200.0))
                        .build(),
                );

                container.spawn(
                    Select::new()
                        .placeholder("Ghost variant")
                        .ghost()
                        .width(Val::Px(200.0))
                        .build(),
                );

                container.spawn(
                    Select::new()
                        .placeholder("Outline variant")
                        .outline()
                        .width(Val::Px(200.0))
                        .build(),
                );

                // Color variants
                container.spawn(
                    Select::new()
                        .placeholder("Blue Select")
                        .blue()
                        .width(Val::Px(200.0))
                        .build(),
                );

                container.spawn(
                    Select::new()
                        .placeholder("Green Select")
                        .green()
                        .width(Val::Px(200.0))
                        .build(),
                );

                container.spawn(
                    Select::new()
                        .placeholder("Red Select")
                        .red()
                        .width(Val::Px(200.0))
                        .build(),
                );

                // Disabled state
                container.spawn(
                    Select::new()
                        .placeholder("Disabled Select")
                        .disabled(true)
                        .width(Val::Px(200.0))
                        .build(),
                );
            });
        });
}