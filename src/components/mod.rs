use bevy::prelude::*;

pub mod box_component;
pub mod button;
pub mod card;
pub mod checkbox;
pub mod flex;
pub mod grid;
pub mod heading;
pub mod section;
pub mod switch;
pub mod text;

pub use box_component::*;
pub use button::*;
pub use card::*;
pub use checkbox::*;
pub use flex::*;
pub use grid::*;
pub use heading::*;
pub use section::*;
pub use switch::*;
pub use text::*;

pub struct ComponentsPlugin;

impl Plugin for ComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<button::ButtonClickEvent>()
            .add_event::<checkbox::CheckboxChangeEvent>()
            .add_event::<switch::SwitchChangeEvent>()
            .add_systems(
                Update,
                (
                    button::setup_button_interactions,
                    button::setup_spinner_textures,
                    button::animate_loading_spinners,
                    text::apply_text_fonts,
                    checkbox::handle_checkbox_interactions,
                    checkbox::spawn_checkmarks,
                    checkbox::update_checkmarks,
                    switch::setup_switch_interactions,
                    switch::spawn_switch_children,
                    switch::update_switch_styling,
                    crate::assets::icon::icon_interaction_system,
                ),
            );
    }
}
