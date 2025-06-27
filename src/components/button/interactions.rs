use crate::assets::audio::{sound_effect, SfxAssets};
use bevy::prelude::*;
use bevy_picking::prelude::{Click, Out, Over, Pickable, Pointer, Pressed, Released};

use super::{
    core::{Button, ButtonState},
    events::ButtonClickEvent,
};

/// Marker component for text that should automatically update colors based on button state
/// Text with explicit colors (manual_color) will not have this component
#[derive(Component, Debug)]
pub struct ButtonManagedText;

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