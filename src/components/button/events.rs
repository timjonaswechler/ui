//! Button event definitions for handling user interactions.
//!
//! This module defines custom events that are triggered when users interact
//! with button components, allowing for decoupled event handling throughout
//! the application.

use bevy::prelude::*;

use super::core::ButtonVariant;

/// Event triggered when a button is clicked by the user.
///
/// This event is sent whenever a user successfully clicks a button that is
/// not disabled or in a loading state. It contains information about which
/// button was clicked and its variant type.
///
/// # Usage
/// ```rust
/// # use bevy::prelude::*;
/// # use ui::components::button::events::ButtonClickEvent;
/// # use ui::components::button::core::ButtonVariant;
/// 
/// fn handle_button_clicks(mut events: EventReader<ButtonClickEvent>) {
///     for event in events.read() {
///         match event.button_variant {
///             ButtonVariant::Solid => println!("Solid button clicked!"),
///             ButtonVariant::Outline => println!("Outline button clicked!"),
///             _ => println!("Other button clicked!"),
///         }
///     }
/// }
/// ```
#[derive(Event, Debug, Clone)]
pub struct ButtonClickEvent {
    /// The entity ID of the button that was clicked.
    pub button_entity: Entity,
    /// The variant type of the button that was clicked.
    pub button_variant: ButtonVariant,
}