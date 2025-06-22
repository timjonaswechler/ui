use bevy::prelude::*;

pub mod box_component;
pub mod button;
pub mod heading;
pub mod text;

pub use box_component::*;
pub use button::*;
pub use heading::*;
pub use text::*;

pub struct ComponentsPlugin;

impl Plugin for ComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<button::ButtonClickEvent>()
            .add_systems(
                Update,
                (
                    button::setup_button_interactions,
                    button::setup_spinner_textures,
                    button::animate_loading_spinners,
                    text::apply_text_fonts,
                ),
            );
    }
}
