use crate::theme::color::{accent_palette, UiColorPalette};
use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonState {
    Normal,
    Hover,
    Active,
    Disabled,
    Loading,
}

#[derive(Component, Debug, Clone)]
pub struct Button {
    pub variant: ButtonVariant,
    pub size: ButtonSize,
    pub color: UiColorPalette,
    pub high_contrast: bool,
    pub radius: ButtonRadius,
    pub loading: bool,
    pub disabled: bool,
    pub current_state: ButtonState,
}

impl Default for Button {
    fn default() -> Self {
        Self {
            variant: ButtonVariant::Solid,
            size: ButtonSize::Default,
            color: accent_palette(),
            high_contrast: false,
            radius: ButtonRadius::Base,
            loading: false,
            disabled: false,
            current_state: ButtonState::Normal,
        }
    }
}

/// Defines the visual style variant of a button
///
/// Buttons can have different visual styles which affect their background,
/// border, and text colors. Each variant provides a different level of
/// visual prominence and purpose.
///
/// # Variants
/// * `Solid` - Full background color with high contrast text (default)
/// * `Soft` - Light background color with darker text
/// * `Outline` - Transparent background with border and dark text
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Component)]
pub enum ButtonVariant {
    /// Full background color with high contrast text (default)
    #[default]
    Solid,
    /// Light background color with darker text
    Soft,
    /// Transparent background with border and dark text
    Outline,
    Ghost,
}

/// Defines the size variant of a button
///
/// Buttons can have different size presets that affect their padding,
/// text size, and overall dimensions.
///
/// # Variants
/// * `Default` - Standard size for most scenarios (default)
/// * `Small` - Compact size for space-constrained areas
/// * `Large` - Expanded size for emphasis or improved touch targets
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Component)]
pub enum ButtonSize {
    /// Standard size for most scenarios (default)
    #[default]
    Default,
    /// Compact size for space-constrained areas
    Small,
    /// Expanded size for emphasis or improved touch targets
    Large,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonRadius {
    None,
    ExtraSmall,
    Small,
    Base,
    Large,
    ExtraLarge,
    Extra2Large,
    Extra3Large,
    Extra4Large,
    Full,
}