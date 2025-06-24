use bevy::{color::palettes::css::*, prelude::*, winit::WinitSettings};
use ui::{
    components::{SwitchComponent, SwitchChangeEvent},
    plugin::ForgeUiPlugin,
    theme::color::{accent_palette, error_palette, success_palette, warning_palette},
    utilities::{ui_root::UIRoot, ComponentBuilder},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ForgeUiPlugin)
        .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, setup)
        .add_systems(Update, handle_switch_events)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        UIRoot::new("main"),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            padding: UiRect::all(Val::Px(20.0)),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        BackgroundColor(GRAY.into()),
    )).with_children(|parent| {
        // Title
        parent
            .spawn(Node {
                width: Val::Percent(100.0),
                height: Val::Px(80.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::bottom(Val::Px(20.0)),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn((
                    Text::new("Switch Component Examples"),
                    TextColor(BLACK.into()),
                    TextFont {
                        font_size: 32.0,
                        ..default()
                    },
                ));
            });

        // Main content container
        parent
            .spawn(Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Px(30.0),
                ..default()
            })
            .with_children(|parent| {
                // Sizes section
                create_section(parent, "Sizes", |parent| {
                    parent
                        .spawn(Node {
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Px(20.0),
                            align_items: AlignItems::Center,
                            ..default()
                        })
                        .with_children(|parent| {
                            // Size 1
                            create_labeled_switch(
                                parent,
                                "Size 1 (16px)",
                                SwitchComponent::new("size1").size_1().build(),
                            );

                            // Size 2 (default)
                            create_labeled_switch(
                                parent,
                                "Size 2 (20px)",
                                SwitchComponent::new("size2").size_2().checked().build(),
                            );

                            // Size 3
                            create_labeled_switch(
                                parent,
                                "Size 3 (24px)",
                                SwitchComponent::new("size3").size_3().build(),
                            );
                        });
                });

                // Variants section
                create_section(parent, "Variants", |parent| {
                    parent
                        .spawn(Node {
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Px(20.0),
                            align_items: AlignItems::Center,
                            ..default()
                        })
                        .with_children(|parent| {
                            // Surface (default)
                            create_labeled_switch(
                                parent,
                                "Surface",
                                SwitchComponent::new("surface").surface().checked().build(),
                            );

                            // Classic
                            create_labeled_switch(
                                parent,
                                "Classic",
                                SwitchComponent::new("classic").classic().checked().build(),
                            );

                            // Soft
                            create_labeled_switch(
                                parent,
                                "Soft",
                                SwitchComponent::new("soft").soft().checked().build(),
                            );
                        });
                });

                // Colors section
                create_section(parent, "Colors", |parent| {
                    parent
                        .spawn(Node {
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Px(20.0),
                            align_items: AlignItems::Center,
                            ..default()
                        })
                        .with_children(|parent| {
                            // Accent (default)
                            create_labeled_switch(
                                parent,
                                "Accent",
                                SwitchComponent::new("accent")
                                    .color(accent_palette())
                                    .checked()
                                    .build(),
                            );

                            // Success (Green)
                            create_labeled_switch(
                                parent,
                                "Success",
                                SwitchComponent::new("success")
                                    .color(success_palette())
                                    .checked()
                                    .build(),
                            );

                            // Warning (Orange)
                            create_labeled_switch(
                                parent,
                                "Warning",
                                SwitchComponent::new("warning")
                                    .color(warning_palette())
                                    .checked()
                                    .build(),
                            );

                            // Error (Red)
                            create_labeled_switch(
                                parent,
                                "Error",
                                SwitchComponent::new("error")
                                    .color(error_palette())
                                    .checked()
                                    .build(),
                            );
                        });
                });

                // States section
                create_section(parent, "States", |parent| {
                    parent
                        .spawn(Node {
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Px(20.0),
                            align_items: AlignItems::Center,
                            ..default()
                        })
                        .with_children(|parent| {
                            // Normal unchecked
                            create_labeled_switch(
                                parent,
                                "Unchecked",
                                SwitchComponent::new("unchecked").build(),
                            );

                            // Normal checked
                            create_labeled_switch(
                                parent,
                                "Checked",
                                SwitchComponent::new("checked").checked().build(),
                            );

                            // Disabled unchecked
                            create_labeled_switch(
                                parent,
                                "Disabled",
                                SwitchComponent::new("disabled_unchecked").disabled().build(),
                            );

                            // Disabled checked
                            create_labeled_switch(
                                parent,
                                "Disabled Checked",
                                SwitchComponent::new("disabled_checked")
                                    .checked()
                                    .disabled()
                                    .build(),
                            );
                        });
                });

                // High contrast section
                create_section(parent, "High Contrast", |parent| {
                    parent
                        .spawn(Node {
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Px(20.0),
                            align_items: AlignItems::Center,
                            ..default()
                        })
                        .with_children(|parent| {
                            // Normal contrast
                            create_labeled_switch(
                                parent,
                                "Normal",
                                SwitchComponent::new("normal_contrast").checked().build(),
                            );

                            // High contrast
                            create_labeled_switch(
                                parent,
                                "High Contrast",
                                SwitchComponent::new("high_contrast")
                                    .checked()
                                    .high_contrast()
                                    .build(),
                            );
                        });
                });

                // Interactive demo section
                create_section(parent, "Interactive Demo", |parent| {
                    parent
                        .spawn(Node {
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(15.0),
                            align_items: AlignItems::Center,
                            ..default()
                        })
                        .with_children(|parent| {
                            // Demo switches with different configurations
                            create_demo_row(parent, "Notifications", "demo_notifications");
                            create_demo_row(parent, "Auto-save", "demo_autosave");
                            create_demo_row(parent, "Dark mode", "demo_darkmode");
                            create_demo_row(parent, "Enable sound", "demo_sound");
                        });
                });
            });
    });
}

fn create_section<F>(parent: &mut ChildBuilder, title: &str, content_fn: F)
where
    F: FnOnce(&mut ChildBuilder),
{
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(15.0),
            align_items: AlignItems::Center,
            width: Val::Percent(100.0),
            ..default()
        })
        .with_children(|parent| {
            // Section title
            parent.spawn((
                Text::new(title),
                TextColor(DARK_GRAY.into()),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
            ));

            // Section content
            content_fn(parent);
        });
}

fn create_labeled_switch(
    parent: &mut ChildBuilder,
    label: &str,
    switch_bundle: impl Bundle,
) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(8.0),
            align_items: AlignItems::Center,
            ..default()
        })
        .with_children(|parent| {
            // Switch
            parent.spawn(switch_bundle);

            // Label
            parent.spawn((
                Text::new(label),
                TextColor(DARK_GRAY.into()),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
            ));
        });
}

fn create_demo_row(parent: &mut ChildBuilder, label: &str, switch_name: &str) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(15.0),
            align_items: AlignItems::Center,
            width: Val::Px(250.0),
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        })
        .with_children(|parent| {
            // Label
            parent.spawn((
                Text::new(label),
                TextColor(BLACK.into()),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
            ));

            // Switch
            parent.spawn(SwitchComponent::new(switch_name).build());
        });
}

fn handle_switch_events(mut switch_events: EventReader<SwitchChangeEvent>) {
    for event in switch_events.read() {
        println!(
            "Switch toggled! Entity: {:?}, Checked: {}, Size: {:?}",
            event.switch_entity, event.checked, event.size
        );
    }
}