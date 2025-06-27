use bevy::prelude::*;

pub mod badge;
pub mod box_component;
pub mod button;
pub mod card;
pub mod checkbox;
pub mod flex;
pub mod grid;
pub mod heading;
pub mod hover_card;
pub mod progress;
pub mod radio;
pub mod section;
pub mod select;
pub mod separator;
pub mod slider;
pub mod switch;
pub mod tabs;
pub mod text;
pub mod toggle;

pub use badge::*;
pub use box_component::*;
pub use button::*;
pub use card::*;
pub use checkbox::*;
pub use flex::*;
pub use grid::*;
pub use heading::*;
pub use hover_card::*;
pub use progress::*;
pub use radio::*;
pub use section::*;
pub use select::*;
pub use separator::*;
pub use slider::*;
pub use switch::*;
pub use tabs::*;
pub use text::*;
pub use toggle::*;

pub struct ComponentsPlugin;

impl Plugin for ComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<button::ButtonClickEvent>()
            .add_event::<checkbox::CheckboxChangeEvent>()
            .add_event::<hover_card::HoverCardOpenEvent>()
            .add_event::<hover_card::HoverCardCloseEvent>()
            .add_event::<radio::RadioChangeEvent>()
            .add_event::<radio::RadioGroupValueChangeEvent>()
            .add_event::<select::SelectOpenEvent>()
            .add_event::<select::SelectChangeEvent>()
            .add_event::<slider::SliderValueChangeEvent>()
            .add_event::<slider::SliderValueCommitEvent>()
            .add_event::<switch::SwitchChangeEvent>()
            .add_event::<toggle::ToggleChangeEvent>()
            .add_systems(
                Update,
                (
                    button::setup_button_interactions,
                    button::setup_spinner_textures,
                    button::animate_loading_spinners,
                    text::apply_text_fonts,
                    hover_card::hover_card_interaction_system,
                    hover_card::hover_card_state_system,
                    checkbox::handle_checkbox_interactions,
                    checkbox::spawn_checkmarks,
                    checkbox::update_checkmarks,
                    radio::handle_radio_interactions,
                ),
            )
            .add_systems(
                Update,
                (
                    radio::update_radio_groups,
                    radio::spawn_radio_indicators,
                    radio::update_radio_indicators,
                    radio::setup_radio_interactions,
                    radio::link_radios_to_groups,
                ),
            )
            .add_systems(
                Update,
                (
                    select::setup_select_interactions,
                    select::update_select_trigger_text,
                ),
            )
            .add_systems(Update, (switch::setup_switch_interactions,))
            .add_systems(
                Update,
                (switch::spawn_switch_children, switch::update_switch_styling),
            )
            .add_systems(
                Update,
                (
                    toggle::setup_toggle_interactions,
                    toggle::spawn_toggle_children,
                    toggle::update_toggle_styling,
                ),
            )
            .add_systems(
                Update,
                (
                    progress::setup_progress_components,
                    progress::animate_indeterminate_progress,
                    progress::update_progress_values,
                    hover_card::hover_card_positioning_system,
                    hover_card::hover_card_portal_system,
                    hover_card::hover_card_keyboard_system,
                    hover_card::hover_card_animation_system,
                    select::position_select_dropdowns,
                    select::handle_click_outside_select,
                    slider::handle_slider_drag,
                    slider::handle_track_click,
                    slider::update_slider_visuals,
                    tabs::handle_trigger_clicks,
                    tabs::style_active_triggers,
                    // Icon interaction system removed - handled by individual icon systems
                ),
            );
    }
}
