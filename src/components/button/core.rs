use crate::theme::color::{accent_palette, UiColorPalette};
use bevy::prelude::*;

/// Represents the current interactive state of a button component.
///
/// Button states are used to determine the visual appearance and behavior
/// of buttons based on user interactions and system conditions.
///
/// # States
/// * `Normal` - Default state when button is ready for interaction
/// * `Hover` - Mouse cursor is over the button
/// * `Active` - Button is being pressed or activated
/// * `Disabled` - Button is disabled and cannot be interacted with
/// * `Loading` - Button is in a loading state, showing a loading indicator
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonState {
    /// Default state when button is ready for interaction
    Normal,
    /// Mouse cursor is over the button
    Hover,
    /// Button is being pressed or activated
    Active,
    /// Button is disabled and cannot be interacted with
    Disabled,
    /// Button is in a loading state, showing a loading indicator
    Loading,
}

/// Core Button component that defines the appearance and behavior of interactive button elements.
///
/// Buttons are fundamental UI components that allow users to trigger actions or navigate
/// through an application. They support multiple visual variants, sizes, states, and
/// can be extensively customized through the theme system.
///
/// # Fields
///
/// * `variant` - Visual style variant (Solid, Soft, Outline, Ghost)
/// * `size` - Size variant affecting padding, text size, and overall dimensions
/// * `color` - Color palette from the theme system
/// * `high_contrast` - Enhanced contrast for better accessibility
/// * `radius` - Border radius configuration
/// * `loading` - Whether the button is in a loading state
/// * `disabled` - Whether the button is disabled
/// * `current_state` - Current interactive state of the button
///
/// # Examples
///
/// ```rust
/// use ui::components::button::{Button, ButtonVariant, ButtonSize};
///
/// // Create a basic button with default settings
/// let button = Button::default();
///
/// // Create a customized button
/// let custom_button = Button {
///     variant: ButtonVariant::Solid,
///     size: ButtonSize::Large,
///     high_contrast: true,
///     loading: false,
///     disabled: false,
///     ..Default::default()
/// };
/// ```
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
    /// Creates a default Button with sensible defaults:
    /// - Solid variant for prominent appearance
    /// - Default (medium) size for general use
    /// - Accent color for primary actions
    /// - Normal contrast for standard visibility
    /// - Base radius for modern appearance
    /// - Not loading or disabled
    /// - Normal state
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

/// Defines the visual style variant of a button.
///
/// Buttons can have different visual styles which affect their background,
/// border, and text colors. Each variant provides a different level of
/// visual prominence and serves different purposes in the UI hierarchy.
///
/// # Variants
/// * `Solid` - Full background color with high contrast text (default) - Use for primary actions
/// * `Soft` - Light background color with darker text - Use for secondary actions
/// * `Outline` - Transparent background with border and dark text - Use for tertiary actions
/// * `Ghost` - Transparent background with no border - Use for subtle actions
///
/// # Examples
///
/// ```rust
/// use ui::components::button::ButtonVariant;
///
/// let primary_variant = ButtonVariant::Solid;      // Most prominent
/// let secondary_variant = ButtonVariant::Soft;     // Less prominent
/// let tertiary_variant = ButtonVariant::Outline;   // Subtle with border
/// let minimal_variant = ButtonVariant::Ghost;      // Minimal visual impact
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Component)]
pub enum ButtonVariant {
    /// Full background color with high contrast text (default) - Use for primary actions
    #[default]
    Solid,
    /// Light background color with darker text - Use for secondary actions
    Soft,
    /// Transparent background with border and dark text - Use for tertiary actions
    Outline,
    /// Transparent background with no border - Use for subtle actions
    Ghost,
}

/// Defines the size variant of a button.
///
/// Buttons can have different size presets that affect their padding,
/// text size, and overall dimensions. Size choice should be based on
/// the importance of the action and available space.
///
/// # Variants
/// * `Small` - Compact size for space-constrained areas (8px/2px padding, SM text)
/// * `Default` - Standard size for most scenarios (12px/6px padding, Base text)
/// * `Large` - Expanded size for emphasis or improved touch targets (16px/8px padding, LG text)
///
/// # Usage Guidelines
///
/// - Use `Small` for secondary actions in compact interfaces
/// - Use `Default` for most button interactions
/// - Use `Large` for primary call-to-action buttons or touch interfaces
///
/// # Examples
///
/// ```rust
/// use ui::components::button::ButtonSize;
///
/// let compact_button = ButtonSize::Small;    // For toolbars, cards
/// let standard_button = ButtonSize::Default; // General use
/// let prominent_button = ButtonSize::Large;  // CTAs, mobile
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Component)]
pub enum ButtonSize {
    /// Compact size for space-constrained areas
    Small,
    /// Standard size for most scenarios (default)
    #[default]
    Default,
    /// Expanded size for emphasis or improved touch targets
    Large,
}

/// Defines the border radius options for button appearance.
///
/// Controls the roundness of button corners, from sharp rectangular
/// buttons to completely rounded pill-shaped buttons. The choice affects
/// the visual style and can convey different personality traits.
///
/// # Variants
/// * `None` - Sharp corners (0px radius) - Modern, technical feel
/// * `ExtraSmall` - Minimal rounding (2px) - Subtle softness
/// * `Small` - Light rounding (4px) - Friendly appearance
/// * `Base` - Standard rounding (6px) - Balanced, modern look
/// * `Large` - Pronounced rounding (8px) - Soft, approachable
/// * `ExtraLarge` - Heavy rounding (12px) - Playful, rounded
/// * `Extra2Large` - Very heavy rounding (16px) - Bold, distinct
/// * `Extra3Large` - Extreme rounding (20px) - Highly stylized
/// * `Extra4Large` - Maximum rounding (24px) - Maximum style
/// * `Full` - Pill shape (9999px radius) - Completely rounded ends
///
/// # Examples
///
/// ```rust
/// use ui::components::button::ButtonRadius;
///
/// let sharp_button = ButtonRadius::None;        // Technical interfaces
/// let modern_button = ButtonRadius::Base;       // Most applications
/// let friendly_button = ButtonRadius::Large;    // Approachable design
/// let pill_button = ButtonRadius::Full;         // Distinctive style
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonRadius {
    /// Sharp corners (0px radius)
    None,
    /// Minimal rounding (2px)
    ExtraSmall,
    /// Light rounding (4px)
    Small,
    /// Standard rounding (6px)
    Base,
    /// Pronounced rounding (8px)
    Large,
    /// Heavy rounding (12px)
    ExtraLarge,
    /// Very heavy rounding (16px)
    Extra2Large,
    /// Extreme rounding (20px)
    Extra3Large,
    /// Maximum rounding (24px)
    Extra4Large,
    /// Pill shape (9999px radius)
    Full,
}
