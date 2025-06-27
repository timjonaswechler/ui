use bevy::prelude::*;

#[derive(Component)]
pub struct SpinnerAnimation {
    pub rotation_speed: f32,
}

impl Default for SpinnerAnimation {
    fn default() -> Self {
        Self {
            rotation_speed: 360.0, // Grad pro Sekunde
        }
    }
}

// System für Loading-Animation (rotiert PNG-Spinner)
pub fn animate_loading_spinners(
    time: Res<Time>,
    mut spinners: Query<(&mut Transform, &SpinnerAnimation)>,
) {
    for (mut transform, spinner) in spinners.iter_mut() {
        let rotation_delta = spinner.rotation_speed * time.delta_secs();
        transform.rotation *= Quat::from_rotation_z(rotation_delta.to_radians());
    }
}

// System um Spinner-Texture zu laden
pub fn setup_spinner_textures(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    spinners: Query<Entity, (With<SpinnerAnimation>, Without<ImageNode>)>,
) {
    let spinner_texture: Handle<Image> = asset_server.load("texture/spinner_loading_icon.png");

    for entity in spinners.iter() {
        commands.entity(entity).insert((
            ImageNode::new(spinner_texture.clone()),
            Node {
                width: Val::Px(16.0),
                height: Val::Px(16.0),
                ..default()
            },
        ));
    }
}