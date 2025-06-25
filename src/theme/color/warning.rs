//! Warning Color Management for Forge UI
//!
//! This module provides specialized color management for warning states, caution
//! indicators, and attention-grabbing notifications in UI components. It offers
//! a carefully selected range of yellow/orange family colors optimized for
//! warning communication while maintaining excellent readability and accessibility.
//!
//! ## Key Features
//!
//! - **Semantic Warning Colors**: Yellow/amber family colors for effective warning communication
//! - **Global Warning Theming**: Consistent warning colors across all components
//! - **Runtime Customization**: Change warning color scheme during application runtime
//! - **Theme Integration**: Automatic light/dark mode color resolution
//! - **Accessibility Optimized**: High contrast ratios for warning visibility
//!
//! ## Warning Color Philosophy
//!
//! Warning colors serve important UX communication functions:
//! - **Clear Communication**: Users must understand the cautionary nature immediately
//! - **Non-Alarming**: Warnings should get attention without causing anxiety
//! - **Accessibility**: Colors must be visible to users with various visual abilities
//! - **Universal Recognition**: Yellow/amber are globally recognized warning colors
//! - **Contextual Appropriateness**: Different intensities for different warning levels
//!
//! ## Available Warning Colors
//!
//! Each warning color has specific characteristics and optimal use cases:
//!
//! ### Yellow (Default)
//! - **Character**: Bright, attention-getting, universally recognized
//! - **Use cases**: General warnings, form validation, important notices
//! - **Accessibility**: Excellent visibility and contrast in both themes
//! - **Psychology**: Caution without alarm, friendly attention-getting
//! - **Recognition**: Globally understood as "proceed with caution"
//!
//! ### Amber
//! - **Character**: Warm orange-yellow, more serious than pure yellow
//! - **Use cases**: System warnings, important updates, moderate urgency
//! - **Accessibility**: Strong contrast with professional appearance
//! - **Psychology**: More serious than yellow, suggests need for action
//! - **Recognition**: Associated with traffic lights and caution signals
//!
//! ## Usage Examples
//!
//! ### Basic Warning Styling
//! ```rust
//! use forge_ui::{warning_palette, set_warning_palette, WarningColor};
//!
//! // Get current warning color palette
//! let warning = warning_palette();
//! let warning_bg = warning.bg_subtle;     // Subtle warning background
//! let warning_border = warning.border;    // Warning border color
//! let warning_text = warning.text;        // Warning text color
//! let warning_solid = warning.solid;      // Prominent warning color
//! ```
//!
//! ### Component Integration
//! ```rust
//! // Alert component with warning styling
//! let warning_alert = Alert::new("Important Update")
//!     .color(warning_palette())    // Uses current warning colors
//!     .variant(AlertVariant::Warning)
//!     .build();
//!
//! // Button with warning emphasis
//! let warning_button = Button::new("Proceed with Caution")
//!     .color(warning_palette())
//!     .variant(ButtonVariant::Soft)
//!     .build();
//!
//! // Text with warning color
//! let warning_message = Text::new("Please review before continuing")
//!     .color(TextColor::Custom(warning_palette().text))
//!     .build();
//! ```
//!
//! ### Customizing Warning Colors
//! ```rust
//! // Set to amber for more serious warning tone
//! set_warning_palette(WarningColor::Amber);
//!
//! // Set to yellow for friendly, less urgent warnings
//! set_warning_palette(WarningColor::Yellow);
//! ```
//!
//! ## Design Guidelines
//!
//! ### When to Use Each Color
//!
//! - **Yellow**: General warnings, form validation, helpful notices, friendly cautions
//! - **Amber**: System warnings, important updates, moderate urgency, procedural cautions
//!
//! ### Warning vs Error vs Success
//!
//! - **Warning (Yellow/Amber)**: "Pay attention, but proceed" - caution, review needed
//! - **Error (Red family)**: "Stop, something is wrong" - problems, failures
//! - **Success (Green family)**: "Good, continue" - completion, success states
//!
//! ### Accessibility Best Practices
//!
//! - Never rely solely on color to communicate warnings
//! - Include warning icons or text labels
//! - Ensure sufficient contrast ratios (all warning colors meet WCAG AA)
//! - Test with colorblind users or simulation tools
//! - Consider cultural differences in color perception
//!
//! ## Implementation Notes
//!
//! - Thread-safe global state management with RwLock
//! - Automatic theme resolution for light/dark modes
//! - Fallback to Yellow ensures system reliability
//! - Seamless integration with component color systems
//! - Efficient lazy initialization and caching

use once_cell::sync::Lazy;
use std::sync::RwLock;

use crate::theme::color::{structs::UiColorPalette, theme_mode};

/// Available warning color options for communicating caution and attention states.
/// 
/// WarningColor provides carefully selected yellow/amber family colors that are
/// optimized for warning communication. These colors are chosen for their ability
/// to get attention without causing alarm, while maintaining excellent accessibility
/// and universal recognition as warning indicators.
/// 
/// # Color Selection Criteria
/// 
/// Each warning color is evaluated based on:
/// - **Visibility**: Must be easily seen and recognized
/// - **Psychology**: Appropriate emotional response for warning states
/// - **Accessibility**: Meets or exceeds WCAG contrast requirements
/// - **Universal Recognition**: Globally understood warning associations
/// - **Non-Alarming**: Gets attention without causing anxiety
/// 
/// # Usage Guidelines
/// 
/// Choose warning colors based on the severity and context:
/// - **General Warnings**: Yellow for friendly, non-urgent cautions
/// - **System Warnings**: Amber for more serious, action-required warnings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WarningColor {
    /// Warm orange-yellow for serious warnings and system alerts.
    /// 
    /// **Characteristics**: More serious than yellow, suggests urgency
    /// **Best for**: System warnings, important updates, moderate urgency situations
    /// **Psychology**: Professional seriousness, calls for action
    /// **Recognition**: Associated with traffic amber lights, industrial warnings
    Amber,
    
    /// Bright yellow - the classic, friendly warning color.
    /// 
    /// **Characteristics**: Bright, attention-getting, non-threatening
    /// **Best for**: Form validation, helpful notices, general cautions
    /// **Psychology**: Friendly attention, caution without alarm
    /// **Recognition**: Universally understood "proceed with caution" signal
    Yellow,
}

impl Default for WarningColor {
    /// Returns Yellow as the default warning color.
    /// 
    /// Yellow is chosen as the default because it:
    /// - Is universally recognized as a warning/caution indicator
    /// - Provides excellent visibility without being alarming
    /// - Works exceptionally well in both light and dark themes
    /// - Has friendly, non-threatening psychological associations
    /// - Offers optimal accessibility across different visual conditions
    /// - Strikes the right balance between attention-getting and non-urgent
    fn default() -> Self {
        Self::Yellow
    }
}

/// Global warning color state with thread-safe access.
/// 
/// This static variable maintains the current warning color choice used throughout
/// the application. It's initialized with the default Yellow color and can be
/// changed at runtime to customize the warning appearance globally.
/// 
/// # Thread Safety
/// 
/// Protected by RwLock for safe concurrent access:
/// - Multiple readers can access simultaneously for optimal performance
/// - Exclusive writer access for color updates
/// - Safe usage across multiple threads and Bevy systems
/// 
/// # Initialization
/// 
/// Uses `once_cell::Lazy` for efficient lazy initialization,
/// ensuring the warning color is only set up when first accessed.
/// This provides better startup performance and memory usage.
pub static WARNING_COLOR: Lazy<RwLock<WarningColor>> =
    Lazy::new(|| RwLock::new(WarningColor::default()));

/// Gets the current warning color palette with full theme integration.
/// 
/// This function returns a complete UiColorPalette for the currently selected
/// warning color, automatically resolved for the current theme mode (light/dark).
/// The palette contains all semantic color roles needed for comprehensive
/// warning styling across UI components.
/// 
/// # Returns
/// 
/// A complete `UiColorPalette` containing:
/// - **Background colors**: Subtle warning backgrounds, hover states, active states
/// - **Border colors**: Warning borders, focus rings, dividers
/// - **Accent colors**: Solid warning backgrounds, prominent warning elements
/// - **Text colors**: Warning text, high-contrast warning text
/// - **Alpha variants**: Semi-transparent versions for overlays and layering
/// 
/// # Theme Resolution
/// 
/// The palette is automatically resolved for the current theme:
/// - **Light mode**: Darker, more saturated warning colors on light backgrounds
/// - **Dark mode**: Lighter, more vibrant warning colors on dark backgrounds
/// 
/// # Performance
/// 
/// Uses a read lock for efficient concurrent access. The warning color
/// selection is cached, so repeated calls have minimal overhead.
/// 
/// # Example Usage
/// ```rust
/// let warning_palette = warning_palette();
/// 
/// // Different warning styling options
/// let warning_bg = warning_palette.bg_subtle;   // Subtle warning background
/// let warning_border = warning_palette.border;  // Warning border color
/// let warning_solid = warning_palette.solid;    // Prominent warning color
/// let warning_text = warning_palette.text;      // Warning text color
/// 
/// // Alpha variants for overlays
/// let warning_overlay = warning_palette.bg_a;   // Semi-transparent warning background
/// let warning_accent = warning_palette.solid_a; // Semi-transparent warning accent
/// ```
/// 
/// # Panic Safety
/// 
/// Uses `expect()` for lock access - will panic only if the lock is poisoned,
/// which indicates a serious threading issue that should halt execution.
pub fn warning_palette() -> UiColorPalette {
    let warning_color = *WARNING_COLOR.read().expect("WARNING_COLOR poisoned");
    let theme_palettes = theme_mode::theme();

    match warning_color {
        WarningColor::Yellow => theme_palettes.yellow,
        WarningColor::Amber => theme_palettes.amber,
    }
}

/// Sets the global warning color for the entire application.
/// 
/// This function changes the warning color used by all components globally.
/// The change is immediate and affects all new warning styling, but existing
/// components may need to be refreshed to reflect the new color choice.
/// 
/// # Arguments
/// 
/// * `color` - The WarningColor variant to use for all warning states
/// 
/// # Color Characteristics
/// 
/// Choose based on your application's warning needs:
/// 
/// - **Yellow**: Friendly, non-threatening warnings for general use
///   - Form validation messages
///   - Helpful notices and tips
///   - Non-urgent cautions
///   - User guidance and suggestions
/// 
/// - **Amber**: More serious warnings requiring attention
///   - System status warnings
///   - Important updates or changes
///   - Moderate urgency situations
///   - Procedural cautions
/// 
/// # Thread Safety
/// 
/// This function acquires a write lock, ensuring exclusive access during
/// the color change. This prevents race conditions but may briefly block
/// other threads accessing the warning palette.
/// 
/// # Example Usage
/// ```rust
/// use forge_ui::{set_warning_palette, WarningColor};
/// 
/// // Set to amber for more serious, urgent warning tone
/// set_warning_palette(WarningColor::Amber);
/// 
/// // Set to yellow for friendly, helpful warning tone
/// set_warning_palette(WarningColor::Yellow);
/// ```
/// 
/// # Design Considerations
/// 
/// When choosing a warning color, consider:
/// - **Application context**: Is it casual, professional, or critical?
/// - **Warning severity**: Are warnings generally urgent or informational?
/// - **User experience**: Do you want friendly guidance or serious cautions?
/// - **Brand consistency**: Does the color align with your brand personality?
/// 
/// # Panic Safety
/// 
/// Uses `expect()` for lock access - will panic only if the lock is poisoned,
/// indicating a serious threading issue that should halt execution.
pub fn set_warning_palette(color: WarningColor) {
    *WARNING_COLOR.write().expect("WARNING_COLOR poisoned") = color;
}

/// Gets the currently selected warning color variant.
/// 
/// This function returns the WarningColor enum variant currently being used
/// for warning styling, without resolving it to a full color palette.
/// Useful for conditional logic, settings persistence, UI controls that
/// display the current warning color choice, or analytics.
/// 
/// # Returns
/// 
/// The current `WarningColor` variant (Yellow or Amber)
/// 
/// # Performance
/// 
/// Very fast operation using a read lock. Multiple threads can call
/// this simultaneously without blocking each other, making it suitable
/// for frequent polling or real-time UI updates.
/// 
/// # Example Usage
/// ```rust
/// use forge_ui::{get_warning_color, WarningColor};
/// 
/// let current_color = get_warning_color();
/// 
/// // Conditional logic based on warning color
/// match current_color {
///     WarningColor::Yellow => {
///         println!("Using friendly yellow warnings");
///         // Maybe show additional helpful hints
///     },
///     WarningColor::Amber => {
///         println!("Using serious amber warnings");
///         // Maybe show more structured warning information
///     },
/// }
/// 
/// // Settings persistence
/// let warning_setting = match current_color {
///     WarningColor::Yellow => "yellow",
///     WarningColor::Amber => "amber",
/// };
/// save_user_preference("warning_color", warning_setting);
/// 
/// // UI state for settings panel
/// let is_yellow_selected = matches!(current_color, WarningColor::Yellow);
/// let is_amber_selected = matches!(current_color, WarningColor::Amber);
/// ```
/// 
/// # Panic Safety
/// 
/// Uses `expect()` for lock access - will panic only if the lock is poisoned,
/// indicating a serious threading issue that should halt execution.
pub fn get_warning_color() -> WarningColor {
    *WARNING_COLOR.read().expect("WARNING_COLOR poisoned")
}
