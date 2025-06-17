use bevy::prelude::*;

pub mod button;

pub use button::*;

pub struct ComponentsPlugin;

impl Plugin for ComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, button::setup_button_interactions);
    }
}