use bevy::prelude::*;
use ui::{
    assets::icons::{
        controllers::{
            GenericAtlases, GenericIcon, GenericIconId, KeyboardMouseAtlases, KeyboardMouseIcon,
            KeyboardMouseIconId, NintendoSwitchAtlases, NintendoSwitchIcon, NintendoSwitchIconId,
            PlayStationAtlases, PlayStationIcon, PlayStationIconId, XboxAtlases, XboxIcon,
            XboxIconId,
        },
        interface::{InterfaceAtlases, InterfaceIcon, InterfaceIconId},
        IconSize,
    },
    components::{heading::Heading, text::Text},
    plugin::{ForgeUiPlugin, UiState},
    theme::{
        color::{theme, TextColor as TextColorEnum},
        typography::{TextSize, TextWeight},
    },
    utilities::ui_root,
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
                                parent.spawn(
                                    GenericIcon::new(GenericIconId::Button)
                                        .size(IconSize::Small)
                                        .bundle(atlases),
                                );
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
                                parent.spawn(
                                    GenericIcon::new(GenericIconId::Button)
                                        .size(IconSize::Medium)
                                        .bundle(atlases),
                                );
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
                                parent.spawn(
                                    GenericIcon::new(GenericIconId::Button)
                                        .size(IconSize::Large)
                                        .bundle(atlases),
                                );
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
                                parent.spawn(
                                    GenericIcon::new(GenericIconId::Button)
                                        .size(IconSize::ExtraLarge)
                                        .bundle(atlases),
                                );
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
                            parent.spawn(GenericIcon::new(GenericIconId::Button).bundle(atlases));
                            parent.spawn(GenericIcon::new(GenericIconId::Joystick).bundle(atlases));
                            parent.spawn(GenericIcon::new(GenericIconId::Stick).bundle(atlases));
                        });
                    }

                    if let Some(atlases) = keyboad_mouse_atlas {
                        create_controller_card(parent, "Keyboard & Mouse", |parent| {
                            parent.spawn(
                                KeyboardMouseIcon::new(KeyboardMouseIconId::KeyboardW)
                                    .bundle(atlases),
                            );
                            parent.spawn(
                                KeyboardMouseIcon::new(KeyboardMouseIconId::KeyboardA)
                                    .bundle(atlases),
                            );
                            parent.spawn(
                                KeyboardMouseIcon::new(KeyboardMouseIconId::KeyboardS)
                                    .bundle(atlases),
                            );
                            parent.spawn(
                                KeyboardMouseIcon::new(KeyboardMouseIconId::KeyboardD)
                                    .bundle(atlases),
                            );
                            parent.spawn(
                                KeyboardMouseIcon::new(KeyboardMouseIconId::Mouse).bundle(atlases),
                            );
                        });
                    }
                    // Xbox Controls
                    if let Some(atlases) = xbox_atlases {
                        create_controller_card(parent, "Xbox Controls", |parent| {
                            parent.spawn(XboxIcon::new(XboxIconId::ButtonA).bundle(atlases));
                            parent.spawn(XboxIcon::new(XboxIconId::ButtonB).bundle(atlases));
                            parent.spawn(XboxIcon::new(XboxIconId::ButtonX).bundle(atlases));
                            parent.spawn(XboxIcon::new(XboxIconId::ButtonY).bundle(atlases));
                        });
                    }

                    // PlayStation Controls
                    if let Some(atlases) = playstation_atlases {
                        create_controller_card(parent, "PlayStation Controls", |parent| {
                            parent.spawn(
                                PlayStationIcon::new(PlayStationIconId::ButtonCross)
                                    .bundle(atlases),
                            );
                            parent.spawn(
                                PlayStationIcon::new(PlayStationIconId::ButtonCircle)
                                    .bundle(atlases),
                            );
                            parent.spawn(
                                PlayStationIcon::new(PlayStationIconId::ButtonSquare)
                                    .bundle(atlases),
                            );
                            parent.spawn(
                                PlayStationIcon::new(PlayStationIconId::ButtonTriangle)
                                    .bundle(atlases),
                            );
                        });
                    }

                    // Nintendo Switch Controls
                    if let Some(atlases) = switch_atlases {
                        create_controller_card(parent, "Nintendo Switch Controls", |parent| {
                            parent.spawn(
                                NintendoSwitchIcon::new(NintendoSwitchIconId::ButtonA)
                                    .bundle(atlases),
                            );
                            parent.spawn(
                                NintendoSwitchIcon::new(NintendoSwitchIconId::ButtonB)
                                    .bundle(atlases),
                            );
                            parent.spawn(
                                NintendoSwitchIcon::new(NintendoSwitchIconId::ButtonX)
                                    .bundle(atlases),
                            );
                            parent.spawn(
                                NintendoSwitchIcon::new(NintendoSwitchIconId::ButtonY)
                                    .bundle(atlases),
                            );
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
                        parent.spawn(InterfaceIcon::new(InterfaceIconId::Home).bundle(atlases));
                        parent.spawn(InterfaceIcon::new(InterfaceIconId::Gear).bundle(atlases));
                        parent.spawn(
                            InterfaceIcon::new(InterfaceIconId::ChevronDown).bundle(atlases),
                        );
                        parent.spawn(InterfaceIcon::new(InterfaceIconId::Avatar).bundle(atlases));
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
                        parent.spawn(GenericIcon::new(GenericIconId::Button).bundle(atlases));

                        // Red tint
                        parent.spawn(
                            GenericIcon::new(GenericIconId::Button)
                                .tint(theme().red.solid)
                                .bundle(atlases),
                        );

                        // Green tint
                        parent.spawn(
                            GenericIcon::new(GenericIconId::Button)
                                .tint(theme().green.solid)
                                .bundle(atlases),
                        );

                        // Blue tint
                        parent.spawn(
                            GenericIcon::new(GenericIconId::Button)
                                .tint(theme().indigo.solid)
                                .bundle(atlases),
                        );

                        // Purple tint
                        parent.spawn(
                            GenericIcon::new(GenericIconId::Button)
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
