use bevy::prelude::*;

pub mod button;

pub use button::*;

pub struct ComponentsPlugin;

impl Plugin for ComponentsPlugin {
    fn build(&self, _app: &mut App) {
        // Komponenten-Plugins werden hier hinzugefügt
    }
}