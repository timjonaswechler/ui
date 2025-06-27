//! Button interaction systems and components for handling user input.
//!
//! This module manages button interactions including hover, click, press, and release
//! events. It handles visual state changes, sound effects, and text color management
//! for button components.

use crate::assets::audio::{sound_effect, SfxAssets};
use bevy::prelude::*;
use bevy_picking::prelude::{Click, Out, Over, Pickable, Pointer, Pressed, Released};

use super::{
    core::{Button, ButtonState},
    events::ButtonClickEvent,
};

/// Marker component for text that should automatically update colors based on button state.
///
/// Text entities with this component will have their colors managed automatically
/// by the button interaction system. Text with explicit colors (manual_color)
/// will not have this component and will maintain their custom colors.
///
/// This component is typically added to text entities that are children of buttons
/// and should change color based on the button's current state (normal, hover, active, etc.).
#[derive(Component, Debug)]
pub struct ButtonManagedText;

/// System that sets up interaction observers for newly added buttons.
///
/// This system runs when new buttons are added to the world and attaches
/// the necessary event observers for handling user interactions. It also
/// applies the initial styling to ensure buttons appear correctly.
///
/// # Parameters
/// - `commands`: Commands for modifying entities
/// - `buttons`: Query for newly added button entities
/// - `button_query`: Query for button component data
/// - `bg_colors`: Query for background color components
/// - `text_colors`: Query for text color components
/// - `children_query`: Query for child entities
/// - `managed_text_query`: Query for managed text components
pub fn setup_button_interactions(
    mut commands: Commands,
    buttons: Query<Entity, Added<Button>>,
    button_query: Query<&Button>,
    mut bg_colors: Query<&mut BackgroundColor>,
    mut text_colors: Query<&mut TextColor>,
    children_query: Query<&Children>,
    managed_text_query: Query<&ButtonManagedText>,
) {
    for entity in &buttons {
        commands
            .entity(entity)
            .observe(on_button_click)
            .observe(on_button_hover)
            .observe(on_button_hover_out)
            .observe(on_button_pressed)
            .observe(on_button_released);

        // Immediately apply correct styling to ensure consistency
        if let Ok(button) = button_query.get(entity) {
            apply_button_styling(
                entity,
                button,
                ButtonState::Normal,
                &mut bg_colors,
                &mut text_colors,
                &children_query,
                &managed_text_query,
            );
        }
    }
}

/// Event handler for button click interactions.
///
/// This function is called when a button receives a click event. It validates
/// that the button is not disabled or loading, plays a sound effect, and
/// sends a `ButtonClickEvent` for other systems to handle.
///
/// # Parameters
/// - `trigger`: The click event trigger
/// - `buttons`: Query for button components
/// - `events`: Event writer for button click events
/// - `commands`: Commands for spawning sound effects
/// - `sfx_assets`: Sound effect assets
fn on_button_click(
    trigger: Trigger<Pointer<Click>>,
    buttons: Query<&Button>,
    mut events: EventWriter<ButtonClickEvent>,
    mut commands: Commands,
    sfx_assets: Res<SfxAssets>,
) {
    let entity = trigger.target();
    if let Ok(button) = buttons.get(entity) {
        // Don't trigger if disabled or loading
        if button.disabled || button.loading {
            return;
        }

        info!("Button clicked! Variant: {:?}", button.variant);

        // Play tap sound effect
        commands.spawn(sound_effect(sfx_assets.tap.clone()));

        // Send custom event
        events.write(ButtonClickEvent {
            button_entity: entity,
            button_variant: button.variant,
        });
    }
}

/// Event handler for button hover (mouse over) interactions.
///
/// This function is called when the mouse cursor enters a button's area.
/// It updates the button's visual state to the hover state if the button
/// is not disabled or loading.
///
/// # Parameters
/// - `trigger`: The hover event trigger
/// - `buttons`: Query for mutable button components
/// - `bg_colors`: Query for background color components
/// - `text_colors`: Query for text color components
/// - `children_query`: Query for child entities
/// - `managed_text_query`: Query for managed text components
fn on_button_hover(
    trigger: Trigger<Pointer<Over>>,
    mut buttons: Query<&mut Button>,
    mut bg_colors: Query<&mut BackgroundColor>,
    mut text_colors: Query<&mut TextColor>,
    children_query: Query<&Children>,
    managed_text_query: Query<&ButtonManagedText>,
) {
    let entity = trigger.target();
    if let Ok(mut button) = buttons.get_mut(entity) {
        if button.disabled || button.loading {
            return;
        }

        button.current_state = ButtonState::Hover;
        apply_button_styling(
            entity,
            &button,
            ButtonState::Hover,
            &mut bg_colors,
            &mut text_colors,
            &children_query,
            &managed_text_query,
        );
    }
}

/// Event handler for button hover out (mouse leave) interactions.
///
/// This function is called when the mouse cursor leaves a button's area.
/// It updates the button's visual state back to the normal state.
///
/// # Parameters
/// - `trigger`: The hover out event trigger
/// - `buttons`: Query for mutable button components
/// - `bg_colors`: Query for background color components
/// - `text_colors`: Query for text color components
/// - `children_query`: Query for child entities
/// - `managed_text_query`: Query for managed text components
fn on_button_hover_out(
    trigger: Trigger<Pointer<Out>>,
    mut buttons: Query<&mut Button>,
    mut bg_colors: Query<&mut BackgroundColor>,
    mut text_colors: Query<&mut TextColor>,
    children_query: Query<&Children>,
    managed_text_query: Query<&ButtonManagedText>,
) {
    let entity = trigger.target();
    if let Ok(mut button) = buttons.get_mut(entity) {
        button.current_state = ButtonState::Normal;
        apply_button_styling(
            entity,
            &button,
            ButtonState::Normal,
            &mut bg_colors,
            &mut text_colors,
            &children_query,
            &managed_text_query,
        );
    }
}

/// Event handler for button press interactions.
///
/// This function is called when a button is pressed down (mouse button down).
/// It updates the button's visual state to the active state if the button
/// is not disabled or loading.
///
/// # Parameters
/// - `trigger`: The press event trigger
/// - `buttons`: Query for mutable button components
/// - `bg_colors`: Query for background color components
/// - `text_colors`: Query for text color components
/// - `children_query`: Query for child entities
/// - `managed_text_query`: Query for managed text components
fn on_button_pressed(
    trigger: Trigger<Pointer<Pressed>>,
    mut buttons: Query<&mut Button>,
    mut bg_colors: Query<&mut BackgroundColor>,
    mut text_colors: Query<&mut TextColor>,
    children_query: Query<&Children>,
    managed_text_query: Query<&ButtonManagedText>,
) {
    let entity = trigger.target();
    if let Ok(mut button) = buttons.get_mut(entity) {
        if button.disabled || button.loading {
            return;
        }

        button.current_state = ButtonState::Active;
        apply_button_styling(
            entity,
            &button,
            ButtonState::Active,
            &mut bg_colors,
            &mut text_colors,
            &children_query,
            &managed_text_query,
        );
    }
}

/// Event handler for button release interactions.
///
/// This function is called when a button is released (mouse button up).
/// It updates the button's visual state back to the hover state if the button
/// is not disabled or loading.
///
/// # Parameters
/// - `trigger`: The release event trigger
/// - `buttons`: Query for mutable button components
/// - `bg_colors`: Query for background color components
/// - `text_colors`: Query for text color components
/// - `children_query`: Query for child entities
/// - `managed_text_query`: Query for managed text components
fn on_button_released(
    trigger: Trigger<Pointer<Released>>,
    mut buttons: Query<&mut Button>,
    mut bg_colors: Query<&mut BackgroundColor>,
    mut text_colors: Query<&mut TextColor>,
    children_query: Query<&Children>,
    managed_text_query: Query<&ButtonManagedText>,
) {
    let entity = trigger.target();
    if let Ok(mut button) = buttons.get_mut(entity) {
        if button.disabled || button.loading {
            return;
        }

        button.current_state = ButtonState::Hover;
        apply_button_styling(
            entity,
            &button,
            ButtonState::Hover,
            &mut bg_colors,
            &mut text_colors,
            &children_query,
            &managed_text_query,
        );
    }
}

/// Applies visual styling to a button based on its current state.
///
/// This function updates the button's background color, text color, and
/// any managed text children to match the styling for the given state.
/// It recursively updates text colors in child entities that have the
/// `ButtonManagedText` component.
///
/// # Parameters
/// - `entity`: The button entity to style
/// - `button`: The button component data
/// - `state`: The current button state to style for
/// - `bg_colors`: Query for background color components
/// - `text_colors`: Query for text color components
/// - `children_query`: Query for child entities
/// - `managed_text_query`: Query for managed text components
fn apply_button_styling(
    entity: Entity,
    button: &Button,
    state: ButtonState,
    bg_colors: &mut Query<&mut BackgroundColor>,
    text_colors: &mut Query<&mut TextColor>,
    children_query: &Query<&Children>,
    managed_text_query: &Query<&ButtonManagedText>,
) {
    use super::styling::ButtonStyling;
    
    let styling = button.get_styling(state);

    // Update button background color
    if let Ok(mut bg_color) = bg_colors.get_mut(entity) {
        *bg_color = styling.background_color;
    }

    // Update direct text color (fallback for old button style)
    if let Ok(mut text_color) = text_colors.get_mut(entity) {
        *text_color = styling.text_color;
    }

    // Update text colors in children (for TextBuilder-created text components)
    if let Ok(children) = children_query.get(entity) {
        for child in children.iter() {
            update_text_colors_recursive(
                &child,
                &styling.text_color,
                text_colors,
                children_query,
                managed_text_query,
            );
        }
    }
}

/// Recursively updates text colors for managed text entities.
///
/// This function traverses the entity hierarchy and updates text colors
/// for any entities that have both `TextColor` and `ButtonManagedText`
/// components. It preserves custom text colors by only updating entities
/// with the managed text marker.
///
/// # Parameters
/// - `entity`: The entity to check and update
/// - `new_color`: The new text color to apply
/// - `text_colors`: Query for text color components
/// - `children_query`: Query for child entities
/// - `managed_text_query`: Query for managed text components
fn update_text_colors_recursive(
    entity: &Entity,
    new_color: &TextColor,
    text_colors: &mut Query<&mut TextColor>,
    children_query: &Query<&Children>,
    managed_text_query: &Query<&ButtonManagedText>,
) {
    // Update text color only if this entity has both TextColor and ButtonManagedText components
    if managed_text_query.get(*entity).is_ok() {
        if let Ok(mut text_color) = text_colors.get_mut(*entity) {
            *text_color = *new_color;
        }
    }

    // Recursively check children
    if let Ok(children) = children_query.get(*entity) {
        for child in children.iter() {
            update_text_colors_recursive(
                &child,
                new_color,
                text_colors,
                children_query,
                managed_text_query,
            );
        }
    }
}