//! Button styling system for calculating colors, borders, and layout properties.
//!
//! This module handles all visual styling calculations for buttons based on their
//! variant, state, size, and theme configuration. It provides consistent styling
//! across different button types and interaction states.

use crate::theme::{
    color::{TextContrastLevel, UiColorPalette},
    layout::UiLayout,
};
use bevy::prelude::*;

use super::core::{Button, ButtonRadius, ButtonSize, ButtonState, ButtonVariant};

/// Complete styling information for a button in a specific state.
///
/// This struct contains all the visual properties needed to render a button,
/// including background color, border color, and text color. It is calculated
/// based on the button's variant, current state, and theme configuration.
#[derive(Debug, Clone)]
pub struct ButtonStyling {
    /// The background color for the button.
    pub background_color: BackgroundColor,
    /// The border color for the button (transparent for most variants).
    pub border_color: BorderColor,
    /// The text color for button content.
    pub text_color: TextColor,
}

impl Button {
    /// Calculates the complete styling for this button in the given state.
    ///
    /// This method takes into account the button's variant, disabled/loading status,
    /// and theme configuration to determine the appropriate colors and styling.
    ///
    /// # Parameters
    /// - `state`: The current interaction state of the button
    ///
    /// # Returns
    /// A `ButtonStyling` struct containing all visual properties for the button.
    ///
    /// # Example
    /// ```rust
    /// # use bevy::prelude::*;
    /// # use ui::components::button::core::{Button, ButtonState};
    /// # use ui::theme::color::UiColorPalette;
    /// 
    /// let button = Button {
    ///     // ... button configuration
    /// #   variant: Default::default(),
    /// #   size: Default::default(),
    /// #   radius: Default::default(),
    /// #   color: UiColorPalette::default(),
    /// #   disabled: false,
    /// #   loading: false,
    /// #   high_contrast: false,
    /// #   current_state: ButtonState::Normal,
    /// };
    /// let styling = button.get_styling(ButtonState::Hover);
    /// ```
    pub fn get_styling(&self, state: ButtonState) -> ButtonStyling {
        let current_state = if self.disabled {
            ButtonState::Disabled
        } else if self.loading {
            ButtonState::Loading
        } else {
            state
        };

        ButtonStyling {
            background_color: self.calculate_background_color(current_state),
            border_color: self.calculate_border_color(current_state),
            text_color: self.calculate_text_color(current_state),
        }
    }

    /// Calculates the background color for the button based on its variant and state.
    ///
    /// Different button variants have different color schemes:
    /// - Solid: Uses the primary color palette
    /// - Ghost: Uses subtle background colors
    /// - Soft/Outline: Uses base background colors
    ///
    /// # Parameters
    /// - `state`: The current button state
    ///
    /// # Returns
    /// The calculated background color with proper opacity for disabled states.
    fn calculate_background_color(&self, state: ButtonState) -> BackgroundColor {
        let base_color = match (self.variant, state) {
            (ButtonVariant::Solid, ButtonState::Normal) => self.color.solid,
            (ButtonVariant::Solid, ButtonState::Hover) => self.color.solid_hover,
            (ButtonVariant::Solid, ButtonState::Active) => self.color.bg_active,
            (ButtonVariant::Ghost, ButtonState::Normal) => self.color.bg_subtle,
            (ButtonVariant::Ghost, ButtonState::Hover) => self.color.bg_hover,
            (ButtonVariant::Ghost, ButtonState::Active) => self.color.bg_active,
            (ButtonVariant::Soft, ButtonState::Normal)
            | (ButtonVariant::Outline, ButtonState::Normal) => self.color.bg,
            (ButtonVariant::Soft, ButtonState::Hover)
            | (ButtonVariant::Outline, ButtonState::Hover) => self.color.bg_hover,
            (ButtonVariant::Soft, ButtonState::Active)
            | (ButtonVariant::Outline, ButtonState::Active) => self.color.bg_active,
            (_, ButtonState::Disabled) => match self.variant {
                ButtonVariant::Solid => self.color.solid,
                ButtonVariant::Ghost => self.color.base,
                ButtonVariant::Soft | ButtonVariant::Outline => self.color.bg_hover,
            },
            (_, ButtonState::Loading) => match self.variant {
                ButtonVariant::Solid => self.color.solid,
                ButtonVariant::Ghost => self.color.base,
                ButtonVariant::Soft | ButtonVariant::Outline => self.color.bg_hover,
            },
        };

        let mut bg_color = BackgroundColor(base_color);

        if state == ButtonState::Disabled {
            let srgba = bg_color.0.to_srgba();
            bg_color.0 = Color::srgba(srgba.red, srgba.green, srgba.blue, 0.6);
        }

        bg_color
    }

    /// Calculates the border color for the button based on its variant and state.
    ///
    /// Only outline buttons have visible borders. Other variants return transparent borders.
    ///
    /// # Parameters
    /// - `state`: The current button state
    ///
    /// # Returns
    /// The calculated border color (transparent for most variants).
    fn calculate_border_color(&self, state: ButtonState) -> BorderColor {
        match self.variant {
            ButtonVariant::Solid | ButtonVariant::Soft | ButtonVariant::Ghost => {
                BorderColor(Color::NONE)
            }
            ButtonVariant::Outline => match state {
                ButtonState::Normal | ButtonState::Active | ButtonState::Loading => {
                    BorderColor(self.color.border)
                }
                ButtonState::Hover => BorderColor(self.color.border_hover),
                ButtonState::Disabled => BorderColor(self.color.border),
            },
        }
    }

    /// Calculates the text color for the button based on its variant and state.
    ///
    /// Text colors are chosen to provide good contrast against the button's
    /// background color:
    /// - Solid buttons use contrast colors
    /// - Other variants use standard text colors
    ///
    /// # Parameters
    /// - `state`: The current button state
    ///
    /// # Returns
    /// The calculated text color with reduced opacity for disabled states.
    fn calculate_text_color(&self, state: ButtonState) -> TextColor {
        // Get the actual background color for this state
        let _background_color = match (self.variant, state) {
            (ButtonVariant::Solid, ButtonState::Normal) => self.color.solid,
            (ButtonVariant::Solid, ButtonState::Hover) => self.color.solid_hover,
            (ButtonVariant::Solid, ButtonState::Active) => self.color.bg_active,
            (ButtonVariant::Ghost, ButtonState::Normal) => self.color.bg_subtle,
            (ButtonVariant::Ghost, ButtonState::Hover) => self.color.bg_hover,
            (ButtonVariant::Ghost, ButtonState::Active) => self.color.bg_active,
            (ButtonVariant::Soft, ButtonState::Normal)
            | (ButtonVariant::Outline, ButtonState::Normal) => self.color.bg,
            (ButtonVariant::Soft, ButtonState::Hover)
            | (ButtonVariant::Outline, ButtonState::Hover) => self.color.bg_hover,
            (ButtonVariant::Soft, ButtonState::Active)
            | (ButtonVariant::Outline, ButtonState::Active) => self.color.bg_active,
            (_, ButtonState::Disabled) => match self.variant {
                ButtonVariant::Solid => self.color.solid,
                ButtonVariant::Ghost => self.color.base,
                ButtonVariant::Soft | ButtonVariant::Outline => self.color.bg_hover,
            },
            (_, ButtonState::Loading) => match self.variant {
                ButtonVariant::Solid => self.color.solid,
                ButtonVariant::Ghost => self.color.base,
                ButtonVariant::Soft | ButtonVariant::Outline => self.color.bg_hover,
            },
        };

        let _contrast_level = if self.high_contrast {
            TextContrastLevel::Accessible
        } else {
            TextContrastLevel::High
        };

        let mut text_color = match self.variant {
            ButtonVariant::Solid => {
                // Use text_contrast for solid buttons - this should be consistent
                self.color.text_contrast
            }
            ButtonVariant::Soft => {
                // Use normal text color for soft buttons
                self.color.text
            }
            ButtonVariant::Outline => {
                // Use normal text color for outline buttons
                self.color.text
            }
            ButtonVariant::Ghost => {
                // Use normal text color for ghost buttons
                self.color.text
            }
        };

        if state == ButtonState::Disabled {
            // Keep the same text color but reduce opacity for disabled state
            let srgba = text_color.to_srgba();
            text_color = Color::srgba(srgba.red, srgba.green, srgba.blue, 0.8);
        }

        TextColor(text_color)
    }
}

/// Calculates the layout style properties for a button.
///
/// This function determines padding, alignment, and border properties
/// based on the button's size configuration. It ensures consistent
/// spacing and alignment across different button sizes.
///
/// # Parameters
/// - `button`: The button to calculate styling for
///
/// # Returns
/// A `Node` component with the appropriate layout properties.
///
/// # Example
/// ```rust
/// # use bevy::prelude::*;
/// # use ui::components::button::core::Button;
/// # use ui::components::button::styling::calculate_button_style;
/// # use ui::theme::color::UiColorPalette;
/// 
/// let button = Button {
///     // ... button configuration
/// #   variant: Default::default(),
/// #   size: Default::default(),
/// #   radius: Default::default(),
/// #   color: UiColorPalette::default(),
/// #   disabled: false,
/// #   loading: false,
/// #   high_contrast: false,
/// #   current_state: Default::default(),
/// };
/// let node_style = calculate_button_style(&button);
/// ```
pub fn calculate_button_style(button: &Button) -> Node {
    let padding = UiRect::axes(
        Val::Px(match button.size {
            ButtonSize::Default => UiLayout::default().padding.base + 2.0,
            ButtonSize::Small => UiLayout::default().padding.sm + 2.0,
            ButtonSize::Large => UiLayout::default().padding.lg + 8.0,
        }),
        Val::Px(match button.size {
            ButtonSize::Default => UiLayout::default().padding.base,
            ButtonSize::Small => UiLayout::default().padding.sm,
            ButtonSize::Large => UiLayout::default().padding.lg,
        }),
    );

    Node {
        padding,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        border: UiRect::all(Val::Px(1.0)),
        ..default()
    }
}

/// Calculates the border radius properties for a button.
///
/// This function converts a `ButtonRadius` enum value into the appropriate
/// `BorderRadius` component with consistent corner rounding based on the
/// theme's radius configuration.
///
/// # Parameters
/// - `radius`: The button radius configuration
///
/// # Returns
/// A `BorderRadius` component with uniform corner rounding.
///
/// # Example
/// ```rust
/// # use ui::components::button::core::ButtonRadius;
/// # use ui::components::button::styling::calculate_border_radius;
/// 
/// let border_radius = calculate_border_radius(ButtonRadius::Base);
/// ```
pub fn calculate_border_radius(radius: ButtonRadius) -> BorderRadius {
    let border_radius = match radius {
        ButtonRadius::None => Val::Px(0.0),
        ButtonRadius::ExtraSmall => Val::Px(UiLayout::default().radius.xs),
        ButtonRadius::Small => Val::Px(UiLayout::default().radius.sm),
        ButtonRadius::Base => Val::Px(UiLayout::default().radius.base),
        ButtonRadius::Large => Val::Px(UiLayout::default().radius.lg),
        ButtonRadius::ExtraLarge => Val::Px(UiLayout::default().radius.xl),
        ButtonRadius::Extra2Large => Val::Px(UiLayout::default().radius.x2l),
        ButtonRadius::Extra3Large => Val::Px(UiLayout::default().radius.x3l),
        ButtonRadius::Extra4Large => Val::Px(UiLayout::default().radius.x4l),
        ButtonRadius::Full => Val::Px(UiLayout::default().radius.full),
    };
    
    BorderRadius {
        top_left: border_radius,
        top_right: border_radius,
        bottom_left: border_radius,
        bottom_right: border_radius,
    }
}