use bevy::prelude::*;

use ui::{
    assets::{
        icons::{
            controllers::{
                generic, generic::Generic, keyboard_mouse, keyboard_mouse::KeyboardMouse,
                playstation, playstation::Playstation, xbox::Xbox,
            },
            interface::{AArrowDown, Anchor},
        },
        Interface,
    },
    components::{heading::Heading, text::Text},
    plugin::{ForgeUiPlugin, UiState},
    theme::{
        color::{theme, warning_palette, TextColor as TextColorEnum},
        typography::{FontSize, TextSize, TextWeight},
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

fn setup_icon_demo(mut commands: Commands) {
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
            create_size_demo_section(parent );

            // Controller Icons Section
            create_controller_section(
                parent
            );

            // Interface Icons Section
            create_interface_section(parent);

            // Color Tinting Demo
            create_tinting_section(parent);
        });
    });
}

fn create_size_demo_section(parent: &mut ChildSpawnerCommands) {
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
                            parent.spawn(Generic::new(generic::Button).build());
                            parent.spawn(
                                Text::caption("Small (16px)")
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
                            parent.spawn(Generic::new(generic::Button).build());
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
                            parent.spawn(Generic::new(generic::Button).size(TextSize::Lg).build());
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
                            parent.spawn(Generic::new(generic::Button).size(TextSize::Xl).build());
                            parent.spawn(
                                Text::caption("Extra Large (64px)")
                                    .size(TextSize::Xs)
                                    .color(TextColorEnum::Muted)
                                    .build(),
                            );
                        });
                });
        });
}

fn create_controller_section(parent: &mut ChildSpawnerCommands) {
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

                    create_controller_card(parent, "Generic Controls", |parent| {
                        parent.spawn(Generic::new(generic::Button).build());
                        parent.spawn(Generic::new(generic::Stick).build());
                        parent.spawn(Generic::new(generic::Joystick).build());
                    });

                    create_controller_card(parent, "Keyboard & Mouse", |parent| {
                        parent.spawn(KeyboardMouse::new(keyboard_mouse::KeyboardW).build());
                        parent.spawn(KeyboardMouse::new(keyboard_mouse::Mouse).build());
                    });

                    // Xbox Controls

                    // create_controller_card(parent, "Xbox Controls", |parent| {
                    //     parent.spawn(Xbox::new(xbox::ButtonA).build());
                    //     parent.spawn(Xbox::new(xbox::ButtonB).build());
                    //     parent.spawn(Xbox::new(xbox::ButtonX).build());
                    //     parent.spawn(Xbox::new(xbox::ButtonY).build());
                    // });

                    // // PlayStation Controls

                    create_controller_card(parent, "PlayStation Controls", |parent| {
                        parent.spawn(
                            Playstation::new(playstation::ButtonCross)
                                .size(TextSize::X6l)
                                .build(),
                        );
                        parent.spawn(Playstation::new(playstation::ButtonCircleOutline).build());
                        parent.spawn(Playstation::new(playstation::ButtonSquare).build());
                        parent.spawn(Playstation::new(playstation::ButtonSquareOutline).build());
                        parent.spawn(Playstation::new(playstation::ButtonTriangle).build());
                    });

                    // create_controller_card(parent, "Nintendo Switch Controls", |parent| {
                    //     parent.spawn(
                    //         NintendoSwitchIcon::new(NintendoSwitchIconId::ButtonA).bundle(atlases),
                    //     );
                    //     parent.spawn(
                    //         NintendoSwitchIcon::new(NintendoSwitchIconId::ButtonB).bundle(atlases),
                    //     );
                    //     parent.spawn(
                    //         NintendoSwitchIcon::new(NintendoSwitchIconId::ButtonX).bundle(atlases),
                    //     );
                    //     parent.spawn(
                    //         NintendoSwitchIcon::new(NintendoSwitchIconId::ButtonY).bundle(atlases),
                    //     );
                    // });
                });
        });
}

fn create_interface_section(parent: &mut ChildSpawnerCommands) {
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

            parent
                .spawn((Node {
                    display: Display::Grid,
                    grid_template_columns: vec![RepeatedGridTrack::fr(1, 1.0); 6],
                    column_gap: Val::Px(16.0),
                    row_gap: Val::Px(16.0),
                    padding: UiRect::all(Val::Px(16.0)),
                    ..default()
                },))
                .with_children(|parent| {
                    // Using the new font-based icon API with individual icon structs
                    parent.spawn(Interface::new(AArrowDown).build());
                    parent.spawn(
                        Interface::new(Anchor)
                            .size(TextSize::X8l)
                            .color(TextColorEnum::Custom(theme().amber.track))
                            .build(),
                    );
                });
        });
}

fn create_tinting_section(parent: &mut ChildSpawnerCommands) {
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

            parent
                .spawn((Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(16.0),
                    padding: UiRect::all(Val::Px(16.0)),
                    ..default()
                },))
                .with_children(|parent| {
                    // White (default)
                    parent.spawn(Generic::new(generic::Button).build());

                    // Red tint
                    parent.spawn(
                        Generic::new(generic::Button)
                            .color(TextColorEnum::Accent)
                            .build(),
                    );

                    // Green tint
                    parent.spawn(
                        Generic::new(generic::Button)
                            .color(TextColorEnum::Success)
                            .build(),
                    );

                    // Blue tint
                    parent.spawn(
                        Generic::new(generic::Button)
                            .color(TextColorEnum::Warning)
                            .build(),
                    );

                    // Purple tint
                    parent.spawn(
                        Generic::new(generic::Button)
                            .color(TextColorEnum::Custom(theme().pink.solid_hover_a))
                            .build(),
                    );
                });
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
