use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn camera
    commands.spawn(Camera2d);
    
    // Add a background colored rectangle (spawn first so it's behind text)
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            ..default()
        },
        BackgroundColor(Color::srgb(0.1, 0.1, 0.3)),
    ));
    
    // Spawn a simple text UI element (spawn later so it's in front)
    commands.spawn((
        Text::new("Hello, Bevy UI!"),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(50.0),
            left: Val::Px(50.0),
            ..default()
        },
        TextFont {
            font: asset_server.load("fonts/Roboto-Regular.ttf"),
            font_size: 40.0,
            ..default()
        },
        TextColor(Color::WHITE),
    ));
}