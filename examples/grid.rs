use bevy::prelude::*;
use ui::{
    components::{text::Text, BoxComponent, GridComponent, GridTrackSize},
    plugin::ForgeUiPlugin,
    theme::typography::TextSize,
    utilities::{ui_root, ComponentBuilder},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ForgeUiPlugin)
        .add_systems(Startup, setup_ui)
        .run();
}

fn setup_ui(mut commands: Commands) {
    // Create camera
    commands.spawn(Camera2d);

    // Create UI root
    commands
        .spawn(ui_root("Grid Example"))
        .with_children(|parent| {
            // Title
            parent.spawn(
                Text::title("Grid Layout Examples")
                    .size(TextSize::X3l)
                    .build(),
            );

            // Demo 1: Simple 3-column grid
            parent
                .spawn(
                    GridComponent::columns("simple-grid", 3)
                        .gap(16.0)
                        .height(Val::Px(200.0))
                        .padding(Val::Px(20.0))
                        .background_color(Color::srgba(0.2, 0.2, 0.2, 0.5))
                        .radius(Val::Px(8.0))
                        .margin_y(Val::Px(10.0))
                        .build(),
                )
                .with_children(|parent| {
                    for i in 1..=6 {
                        parent
                            .spawn(
                                BoxComponent::new(&format!("item-{}", i))
                                    .panel()
                                    .height(Val::Px(60.0))
                                    .padding(Val::Px(8.0))
                                    .build(),
                            )
                            .with_children(|item| {
                                item.spawn(Text::body(&format!("Item {}", i)).build());
                            });
                    }
                });

            // Demo 2: Grid with different column sizes
            parent
                .spawn(
                    GridComponent::new("mixed-grid")
                        .columns_sizes(vec![
                            GridTrackSize::Fr(2.0),
                            GridTrackSize::Fr(1.0),
                            GridTrackSize::Fr(1.0),
                        ])
                        .gap(12.0)
                        .height(Val::Px(150.0))
                        .padding(Val::Px(20.0))
                        .background_color(Color::srgba(0.2, 0.3, 0.4, 0.5))
                        .radius(Val::Px(8.0))
                        .margin_y(Val::Px(10.0))
                        .build(),
                )
                .with_children(|parent| {
                    let items = ["Wide Item", "Item 2", "Item 3"];
                    for (i, label) in items.iter().enumerate() {
                        parent
                            .spawn(
                                BoxComponent::new(&format!("mixed-item-{}", i))
                                    .panel()
                                    .padding(Val::Px(8.0))
                                    .build(),
                            )
                            .with_children(|item| {
                                item.spawn(Text::body(*label).build());
                            });
                    }
                });

            // Demo 3: 3x2 Grid Layout
            parent
                .spawn(
                    GridComponent::layout("matrix-grid", 3, 2)
                        .gap(8.0)
                        .height(Val::Px(160.0))
                        .padding(Val::Px(20.0))
                        .background_color(Color::srgba(0.4, 0.2, 0.3, 0.5))
                        .radius(Val::Px(8.0))
                        .margin_y(Val::Px(10.0))
                        .build(),
                )
                .with_children(|parent| {
                    for row in 1..=2 {
                        for col in 1..=3 {
                            parent
                                .spawn(
                                    BoxComponent::new(&format!("matrix-item-{}-{}", row, col))
                                        .panel()
                                        .padding(Val::Px(8.0))
                                        .build(),
                                )
                                .with_children(|item| {
                                    item.spawn(Text::body(&format!("R{}C{}", row, col)).build());
                                });
                        }
                    }
                });
        });
}
