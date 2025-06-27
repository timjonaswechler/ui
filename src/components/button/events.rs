use bevy::prelude::*;

use super::core::ButtonVariant;

#[derive(Event, Debug, Clone)]
pub struct ButtonClickEvent {
    pub button_entity: Entity,
    pub button_variant: ButtonVariant,
}