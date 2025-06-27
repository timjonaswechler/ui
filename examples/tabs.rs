use bevy::prelude::*;
use ui::{
    components::{TabsBuilder, text::Text},
    plugin::{ForgeUiPlugin, UiState},
    theme::color::theme,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ForgeUiPlugin)
        .add_systems(OnEnter(UiState::Ready), setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    let root = commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(20.0),
            padding: UiRect::all(Val::Px(24.0)),
            ..default()
        })
        .id();

    let tab1_content = commands
        .spawn((
            Node {
                width: Val::Px(300.0),
                height: Val::Px(150.0),
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(theme().gray.bg),
        ))
        .with_children(|parent| {
            parent.spawn(Text::body("This is the content for Tab 1.").build());
        })
        .id();

    let tab2_content = commands
        .spawn((
            Node {
                width: Val::Px(300.0),
                height: Val::Px(150.0),
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(theme().gray.bg),
        ))
        .with_children(|parent| {
            parent.spawn(Text::body("This is the content for Tab 2.").build());
        })
        .id();

    let tab3_content = commands
        .spawn((
            Node {
                width: Val::Px(300.0),
                height: Val::Px(150.0),
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(theme().gray.bg),
        ))
        .with_children(|parent| {
            parent.spawn(Text::body("This is the content for Tab 3.").build());
        })
        .id();

    let tabs = TabsBuilder::new()
        .triggers(vec!["Tab 1", "Tab 2", "Tab 3"])
        .contents(vec![tab1_content, tab2_content, tab3_content])
        .build(&mut commands);

    commands.entity(root).add_child(tabs);
}