use bevy::prelude::*;
use ui::{
    components::{
        checkbox::{Checkbox, CheckboxChangeEvent},
        text::Text,
        BoxComponent, FlexComponent,
    },
    plugin::ForgeUiPlugin,
    theme::{color::accent_palette, typography::TextSize},
    utilities::{ui_root, ComponentBuilder},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ForgeUiPlugin)
        .add_systems(Startup, setup_ui)
        .add_systems(Update, handle_checkbox_events)
        .run();
}

fn setup_ui(mut commands: Commands) {
    // Create camera
    commands.spawn(Camera2d);

    // Create UI root
    commands
        .spawn(ui_root("Checkbox Example"))
        .with_children(|parent| {
            // Title
            parent.spawn(
                Text::title("Checkbox Component Examples")
                    .size(TextSize::X3l)
                    .build(),
            );

            // Section spacing
            parent.spawn(
                BoxComponent::new("spacing-1")
                    .height(Val::Px(32.0))
                    .build(),
            );

            // Size Examples Section
            parent.spawn(
                Text::title("Checkbox Sizes")
                    .size(TextSize::Xl)
                    .build(),
            );

            parent
                .spawn(
                    FlexComponent::row("sizes-demo")
                        .gap(24.0)
                        .align_center()
                        .margin_y(Val::Px(16.0))
                        .build(),
                )
                .with_children(|row| {
                    // Size 1 - Compact
                    row.spawn(
                        FlexComponent::column("size1-demo")
                            .gap(8.0)
                            .align_center()
                            .build(),
                    )
                    .with_children(|col| {
                        col.spawn(Checkbox::size_1("size1-checkbox").build());
                        col.spawn(
                            Text::body("Size 1 (16px)")
                                .size(TextSize::Sm)
                                .build(),
                        );
                    });

                    // Size 2 - Default
                    row.spawn(
                        FlexComponent::column("size2-demo")
                            .gap(8.0)
                            .align_center()
                            .build(),
                    )
                    .with_children(|col| {
                        col.spawn(Checkbox::size_2("size2-checkbox").build());
                        col.spawn(
                            Text::body("Size 2 (20px)")
                                .size(TextSize::Sm)
                                .build(),
                        );
                    });

                    // Size 3 - Large
                    row.spawn(
                        FlexComponent::column("size3-demo")
                            .gap(8.0)
                            .align_center()
                            .build(),
                    )
                    .with_children(|col| {
                        col.spawn(Checkbox::size_3("size3-checkbox").build());
                        col.spawn(
                            Text::body("Size 3 (24px)")
                                .size(TextSize::Sm)
                                .build(),
                        );
                    });
                });

            // Section spacing
            parent.spawn(
                BoxComponent::new("spacing-2")
                    .height(Val::Px(32.0))
                    .build(),
            );

            // State Examples Section
            parent.spawn(
                Text::title("Checkbox States")
                    .size(TextSize::Xl)
                    .build(),
            );

            parent
                .spawn(
                    FlexComponent::column("states-demo")
                        .gap(16.0)
                        .margin_y(Val::Px(16.0))
                        .build(),
                )
                .with_children(|col| {
                    // Unchecked checkbox with label
                    col.spawn(
                        FlexComponent::row("unchecked-demo")
                            .gap(12.0)
                            .align_center()
                            .build(),
                    )
                    .with_children(|row| {
                        row.spawn(Checkbox::new("unchecked-checkbox").build());
                        row.spawn(
                            Text::body("Unchecked - Click to toggle")
                                .build(),
                        );
                    });

                    // Checked checkbox with label
                    col.spawn(
                        FlexComponent::row("checked-demo")
                            .gap(12.0)
                            .align_center()
                            .build(),
                    )
                    .with_children(|row| {
                        row.spawn(Checkbox::checked("checked-checkbox").build());
                        row.spawn(
                            Text::body("Initially checked - Click to toggle")
                                .build(),
                        );
                    });

                    // Disabled unchecked checkbox
                    col.spawn(
                        FlexComponent::row("disabled-unchecked-demo")
                            .gap(12.0)
                            .align_center()
                            .build(),
                    )
                    .with_children(|row| {
                        row.spawn(Checkbox::new("disabled-unchecked").disabled().build());
                        row.spawn(
                            Text::body("Disabled (unchecked)")
                                .build(),
                        );
                    });

                    // Disabled checked checkbox
                    col.spawn(
                        FlexComponent::row("disabled-checked-demo")
                            .gap(12.0)
                            .align_center()
                            .build(),
                    )
                    .with_children(|row| {
                        row.spawn(Checkbox::checked("disabled-checked").disabled().build());
                        row.spawn(
                            Text::body("Disabled (checked)")
                                .build(),
                        );
                    });
                });

            // Section spacing
            parent.spawn(
                BoxComponent::new("spacing-3")
                    .height(Val::Px(32.0))
                    .build(),
            );

            // Form Example Section
            parent.spawn(
                Text::title("Form Example")
                    .size(TextSize::Xl)
                    .build(),
            );

            parent
                .spawn(
                    BoxComponent::new("form-demo")
                        .panel()
                        .padding(Val::Px(20.0))
                        .margin_y(Val::Px(16.0))
                        .build(),
                )
                .with_children(|form| {
                    form.spawn(
                        Text::title("Settings")
                            .size(TextSize::Lg)
                            .build(),
                    );

                    form.spawn(
                        BoxComponent::new("form-spacing")
                            .height(Val::Px(16.0))
                            .build(),
                    );

                    // Settings options
                    let settings_options = [
                        ("notifications", "Enable notifications"),
                        ("auto-save", "Auto-save documents"),
                        ("dark-mode", "Use dark theme"),
                        ("analytics", "Share usage analytics"),
                        ("updates", "Check for updates automatically"),
                    ];

                    for (id, label) in settings_options {
                        form.spawn(
                            FlexComponent::row(&format!("{}-row", id))
                                .gap(12.0)
                                .align_center()
                                .margin_y(Val::Px(8.0))
                                .build(),
                        )
                        .with_children(|row| {
                            row.spawn(Checkbox::new(id).build());
                            row.spawn(Text::body(label).build());
                        });
                    }
                });

            // Section spacing
            parent.spawn(
                BoxComponent::new("spacing-4")
                    .height(Val::Px(32.0))
                    .build(),
            );

            // Color Variants Section
            parent.spawn(
                Text::title("Color Variants")
                    .size(TextSize::Xl)
                    .build(),
            );

            parent
                .spawn(
                    FlexComponent::row("colors-demo")
                        .gap(24.0)
                        .align_center()
                        .margin_y(Val::Px(16.0))
                        .build(),
                )
                .with_children(|row| {
                    row.spawn(
                        FlexComponent::column("accent-demo")
                            .gap(8.0)
                            .align_center()
                            .build(),
                    )
                    .with_children(|col| {
                        col.spawn(
                            Checkbox::checked("accent-checkbox")
                                .accent()
                                .build(),
                        );
                        col.spawn(
                            Text::body("Accent")
                                .size(TextSize::Sm)
                                .build(),
                        );
                    });

                    // Custom color examples could be added here when color system is extended
                });

            // Instructions
            parent.spawn(
                BoxComponent::new("spacing-5")
                    .height(Val::Px(32.0))
                    .build(),
            );

            parent.spawn(
                Text::body("Click any checkbox to toggle its state. Check the console for change events.")
                    .size(TextSize::Sm)
                    .build(),
            );
        });
}

fn handle_checkbox_events(mut checkbox_events: EventReader<CheckboxChangeEvent>) {
    for event in checkbox_events.read() {
        println!(
            "✅ Checkbox {:?} changed to: {} (size: {:?})",
            event.checkbox_entity, event.checked, event.size
        );
    }
}