use bevy::prelude::*;
use ui::{
    assets::icons::{
        controllers::{
            GenericAtlases, KeyboardMouseAtlases, NintendoSwitchAtlases, PlayStationAtlases,
            XboxAtlases,
        },
        interface::InterfaceAtlases,
        GenericButton, GenericJoystick, GenericStick, InterfaceAvatar, InterfaceChevronDown,
        InterfaceGear, InterfaceHome, KeyboardA, KeyboardD, KeyboardS, KeyboardW, Mouse,
        PlayStationCircle, PlayStationCross, PlayStationSquare, PlayStationTriangle, SwitchButtonA,
        SwitchButtonB, SwitchButtonX, SwitchButtonY, XboxA, XboxB, XboxX, XboxY,
    },
    components::{heading::Heading, text::Text},
    plugin::{ForgeUiPlugin, UiState},
    theme::{
        color::{theme, TextColor as TextColorEnum},
        typography::{TextSize, TextWeight},
    },
    utilities::{ui_root, ComponentBuilder},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ForgeUiPlugin)
        .add_systems(OnEnter(UiState::Ready), (setup_icon_demo).chain())
        .run();
}

fn setup_icon_demo(
    mut commands: Commands,
    generic_atlases: Option<Res<GenericAtlases>>,
    keyboard_mouse_atlases: Option<Res<KeyboardMouseAtlases>>,
    xbox_atlases: Option<Res<XboxAtlases>>,
    playstation_atlases: Option<Res<PlayStationAtlases>>,
    switch_atlases: Option<Res<NintendoSwitchAtlases>>,
    interface_atlases: Option<Res<InterfaceAtlases>>,
) {
    // Camera
    commands.spawn(Camera2d);

    // UI Root
    commands.spawn(ui_root("Icons_Root")).with_children(|parent| {
        // Main container with scrolling
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(32.0)),
                row_gap: Val::Px(32.0),
                overflow: Overflow::clip_y(),
                ..default()
            },
        )).with_children(|parent| {
            // Title
            parent.spawn(
                Heading::h1("Icon System Demo")
                    .size(TextSize::X6l)
                    .weight(TextWeight::Bold)
                    .build()
            );

            parent.spawn(
                Text::body("Comprehensive showcase of the icon system with multiple categories and sizes")
                    .color(TextColorEnum::Muted)
                    .size(TextSize::Lg)
                    .build()
            );

            // Icon Size Demo Section
            create_size_demo_section(parent, generic_atlases.as_deref());

            // Controller Icons Section
            create_controller_section(
                parent,
                generic_atlases.as_deref(),
                keyboard_mouse_atlases.as_deref(),
                xbox_atlases.as_deref(),
                playstation_atlases.as_deref(),
                switch_atlases.as_deref(),
            );

            // Interface Icons Section
            create_interface_section(parent, interface_atlases.as_deref());

            // Color Tinting Demo
            create_tinting_section(parent, generic_atlases.as_deref());
        });
    });
}

fn create_size_demo_section(
    parent: &mut ChildSpawnerCommands,
    generic_atlases: Option<&GenericAtlases>,
) {
    parent
        .spawn((Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(16.0),
            ..default()
        },))
        .with_children(|parent| {
            // Section title
            parent.spawn(
                Heading::h2("Icon Sizes")
                    .size(TextSize::X3l)
                    .weight(TextWeight::Medium)
                    .build(),
            );

            if let Some(atlases) = generic_atlases {
                // Size demonstration row
                parent
                    .spawn((Node {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(24.0),
                        padding: UiRect::all(Val::Px(16.0)),
                        ..default()
                    },))
                    .with_children(|parent| {
                        // Small (16px)
                        parent
                            .spawn((Node {
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                row_gap: Val::Px(8.0),
                                ..default()
                            },))
                            .with_children(|parent| {
                                parent.spawn(GenericButton::small().bundle(atlases));
                                parent.spawn(
                                    Text::caption("Small (16px)")
                                        .size(TextSize::Xs)
                                        .color(TextColorEnum::Muted)
                                        .build(),
                                );
                            });

                        // Medium (24px) - Default
                        parent
                            .spawn((Node {
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                row_gap: Val::Px(8.0),
                                ..default()
                            },))
                            .with_children(|parent| {
                                parent.spawn(GenericButton::medium().bundle(atlases));
                                parent.spawn(
                                    Text::caption("Medium (24px)")
                                        .size(TextSize::Xs)
                                        .color(TextColorEnum::Muted)
                                        .build(),
                                );
                            });

                        // Large (32px)
                        parent
                            .spawn((Node {
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                row_gap: Val::Px(8.0),
                                ..default()
                            },))
                            .with_children(|parent| {
                                parent.spawn(GenericButton::large().bundle(atlases));
                                parent.spawn(
                                    Text::caption("Large (32px)")
                                        .size(TextSize::Xs)
                                        .color(TextColorEnum::Muted)
                                        .build(),
                                );
                            });

                        // Extra Large (64px)
                        parent
                            .spawn((Node {
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                row_gap: Val::Px(8.0),
                                ..default()
                            },))
                            .with_children(|parent| {
                                parent.spawn(GenericButton::extra_large().bundle(atlases));
                                parent.spawn(
                                    Text::caption("Extra Large (64px)")
                                        .size(TextSize::Xs)
                                        .color(TextColorEnum::Muted)
                                        .build(),
                                );
                            });
                    });
            } else {
                parent.spawn(
                    Text::body("Loading icon atlases...")
                        .color(TextColorEnum::Muted)
                        .build(),
                );
            }
        });
}

fn create_controller_section(
    parent: &mut ChildSpawnerCommands,
    generic_atlases: Option<&GenericAtlases>,
    keyboad_mouse_atlas: Option<&KeyboardMouseAtlases>,
    xbox_atlases: Option<&XboxAtlases>,
    playstation_atlases: Option<&PlayStationAtlases>,
    switch_atlases: Option<&NintendoSwitchAtlases>,
) {
    parent
        .spawn((Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(16.0),
            ..default()
        },))
        .with_children(|parent| {
            // Section title
            parent.spawn(
                Heading::h2("Controller Icons")
                    .size(TextSize::X3l)
                    .weight(TextWeight::Medium)
                    .build(),
            );

            // Grid for different controller types
            parent
                .spawn((Node {
                    display: Display::Grid,
                    grid_template_columns: vec![RepeatedGridTrack::fr(1, 1.0); 2],
                    column_gap: Val::Px(32.0),
                    row_gap: Val::Px(24.0),
                    ..default()
                },))
                .with_children(|parent| {
                    // Generic Controls
                    if let Some(atlases) = generic_atlases {
                        create_controller_card(parent, "Generic Controls", |parent| {
                            parent.spawn(GenericButton::new().bundle(atlases));
                            parent.spawn(GenericJoystick::new().bundle(atlases));
                            parent.spawn(GenericStick::new().bundle(atlases));
                        });
                    }

                    if let Some(atlases) = keyboad_mouse_atlas {
                        create_controller_card(parent, "Keyboard & Mouse", |parent| {
                            parent.spawn(KeyboardW::new().bundle(atlases));
                            parent.spawn(KeyboardA::new().bundle(atlases));
                            parent.spawn(KeyboardS::new().bundle(atlases));
                            parent.spawn(KeyboardD::new().bundle(atlases));
                            parent.spawn(Mouse::new().bundle(atlases));
                        });
                    }
                    // Xbox Controls
                    if let Some(atlases) = xbox_atlases {
                        create_controller_card(parent, "Xbox Controls", |parent| {
                            parent.spawn(XboxA::new().bundle(atlases));
                            parent.spawn(XboxB::new().bundle(atlases));
                            parent.spawn(XboxX::new().bundle(atlases));
                            parent.spawn(XboxY::new().bundle(atlases));
                        });
                    }

                    // PlayStation Controls
                    if let Some(atlases) = playstation_atlases {
                        create_controller_card(parent, "PlayStation Controls", |parent| {
                            parent.spawn(PlayStationCross::new().bundle(atlases));
                            parent.spawn(PlayStationCircle::new().bundle(atlases));
                            parent.spawn(PlayStationSquare::new().bundle(atlases));
                            parent.spawn(PlayStationTriangle::new().bundle(atlases));
                        });
                    }

                    // Nintendo Switch Controls
                    if let Some(atlases) = switch_atlases {
                        create_controller_card(parent, "Nintendo Switch Controls", |parent| {
                            parent.spawn(SwitchButtonA::new().bundle(atlases));
                            parent.spawn(SwitchButtonB::new().bundle(atlases));
                            parent.spawn(SwitchButtonX::new().bundle(atlases));
                            parent.spawn(SwitchButtonY::new().bundle(atlases));
                        });
                    }
                });
        });
}

fn create_interface_section(
    parent: &mut ChildSpawnerCommands,
    interface_atlases: Option<&InterfaceAtlases>,
) {
    parent
        .spawn((Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(16.0),
            ..default()
        },))
        .with_children(|parent| {
            // Section title
            parent.spawn(
                Heading::h2("Interface Icons")
                    .size(TextSize::X3l)
                    .weight(TextWeight::Medium)
                    .build(),
            );

            if let Some(atlases) = interface_atlases {
                parent
                    .spawn((Node {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(16.0),
                        padding: UiRect::all(Val::Px(16.0)),
                        ..default()
                    },))
                    .with_children(|parent| {
                        parent.spawn(InterfaceHome::new().bundle(atlases));
                        parent.spawn(InterfaceGear::new().bundle(atlases));
                        parent.spawn(InterfaceChevronDown::new().bundle(atlases));
                        parent.spawn(InterfaceAvatar::new().bundle(atlases));
                    });
            } else {
                parent.spawn(
                    Text::body("Loading interface icons...")
                        .color(TextColorEnum::Muted)
                        .build(),
                );
            }
        });
}

fn create_tinting_section(
    parent: &mut ChildSpawnerCommands,
    generic_atlases: Option<&GenericAtlases>,
) {
    parent
        .spawn((Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(16.0),
            ..default()
        },))
        .with_children(|parent| {
            // Section title
            parent.spawn(
                Heading::h2("Color Tinting")
                    .size(TextSize::X3l)
                    .weight(TextWeight::Medium)
                    .build(),
            );

            if let Some(atlases) = generic_atlases {
                parent
                    .spawn((Node {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(16.0),
                        padding: UiRect::all(Val::Px(16.0)),
                        ..default()
                    },))
                    .with_children(|parent| {
                        // White (default)
                        parent.spawn(GenericButton::new().bundle(atlases));

                        // Red tint
                        parent.spawn(GenericButton::new().tint(theme().red.solid).bundle(atlases));

                        // Green tint
                        parent.spawn(
                            GenericButton::new()
                                .tint(theme().green.solid)
                                .bundle(atlases),
                        );

                        // Blue tint
                        parent.spawn(
                            GenericButton::new()
                                .tint(theme().indigo.solid)
                                .bundle(atlases),
                        );

                        // Purple tint
                        parent.spawn(
                            GenericButton::new()
                                .tint(theme().purple.solid)
                                .bundle(atlases),
                        );
                    });
            }
        });
}

fn create_controller_card<F>(parent: &mut ChildSpawnerCommands, title: &str, spawn_icons: F)
where
    F: FnOnce(&mut ChildSpawnerCommands),
{
    parent
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(16.0)),
                border: UiRect::all(Val::Px(1.0)),
                row_gap: Val::Px(12.0),
                ..default()
            },
            BorderColor(theme().gray.border),
            BackgroundColor(theme().gray.surface),
            BorderRadius::all(Val::Px(8.0)),
        ))
        .with_children(|parent| {
            // Card title
            parent.spawn(
                Heading::h3(title)
                    .size(TextSize::Lg)
                    .weight(TextWeight::Medium)
                    .build(),
            );

            // Icons grid
            parent
                .spawn((Node {
                    display: Display::Grid,
                    grid_template_columns: vec![RepeatedGridTrack::fr(1, 1.0); 4],
                    column_gap: Val::Px(12.0),
                    row_gap: Val::Px(12.0),
                    ..default()
                },))
                .with_children(spawn_icons);
        });
}
