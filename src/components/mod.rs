use bevy::prelude::*;

pub mod button;

pub use button::*;

pub struct ComponentsPlugin;

impl Plugin for ComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<button::ButtonClickEvent>().add_systems(
            Update,
            (
                button::setup_button_interactions,
                button::setup_spinner_textures,
                button::animate_loading_spinners,
            ),
        );
    }
}
