//! Button animation components and systems for loading spinners.
//!
//! This module provides loading spinner animations for buttons in the loading state.
//! It handles rotation animation and texture loading for spinner icons.

use bevy::prelude::*;

/// Component that defines a loading spinner animation for buttons.
///
/// The spinner rotates continuously when the button is in a loading state,
/// providing visual feedback to the user that an operation is in progress.
///
/// # Example
/// ```rust
/// # use bevy::prelude::*;
/// # use ui::components::button::animations::SpinnerAnimation;
/// 
/// fn spawn_loading_button(mut commands: Commands) {
///     commands.spawn((
///         SpinnerAnimation {
///             rotation_speed: 180.0, // Half speed rotation
///         },
///         // ... other button components
///     ));
/// }
/// ```
#[derive(Component)]
pub struct SpinnerAnimation {
    /// Rotation speed in degrees per second.
    /// Default is 360.0 (one full rotation per second).
    pub rotation_speed: f32,
}

impl Default for SpinnerAnimation {
    fn default() -> Self {
        Self {
            rotation_speed: 360.0, // Degrees per second
        }
    }
}

/// System that animates loading spinners by rotating them continuously.
///
/// This system runs every frame and updates the rotation of all entities
/// with both a `Transform` and `SpinnerAnimation` component. The rotation
/// is applied around the Z-axis (clockwise when viewed from above).
///
/// # Parameters
/// - `time`: Resource providing delta time for frame-rate independent animation
/// - `spinners`: Query for all entities with spinner animations
pub fn animate_loading_spinners(
    time: Res<Time>,
    mut spinners: Query<(&mut Transform, &SpinnerAnimation)>,
) {
    for (mut transform, spinner) in spinners.iter_mut() {
        let rotation_delta = spinner.rotation_speed * time.delta_secs();
        transform.rotation *= Quat::from_rotation_z(rotation_delta.to_radians());
    }
}

/// System that loads and applies spinner textures to spinner entities.
///
/// This system runs once to set up the visual appearance of spinner animations.
/// It loads the spinner texture asset and applies it to all entities that have
/// a `SpinnerAnimation` component but don't yet have an `ImageNode`.
///
/// The spinner texture is loaded from "texture/spinner_loading_icon.png" and
/// displayed as a 16x16 pixel image.
///
/// # Parameters
/// - `commands`: Commands for spawning and modifying entities
/// - `asset_server`: Resource for loading assets
/// - `spinners`: Query for spinner entities without textures
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