//! Error Color Management for Forge UI
//!
//! This module provides specialized color management for error states, warnings,
//! and critical notifications in UI components. It offers a curated selection
//! of red-family colors specifically chosen for their effectiveness in communicating
//! error conditions while maintaining accessibility and visual hierarchy.
//!
//! ## Key Features
//!
//! - **Semantic Error Colors**: Carefully selected red-family palettes for error communication
//! - **Global Error Theming**: Consistent error colors across all components
//! - **Runtime Customization**: Change error color scheme during application runtime
//! - **Theme Integration**: Automatic light/dark mode color resolution
//! - **Accessibility Focus**: All colors meet contrast requirements for error visibility
//!
//! ## Error Color Philosophy
//!
//! Error colors serve critical UX functions:
//! - **Immediate Recognition**: Users must instantly recognize error states
//! - **Accessibility Compliance**: Error colors must meet WCAG contrast requirements
//! - **Emotional Appropriateness**: Colors should convey appropriate urgency without causing panic
//! - **Consistency**: Error colors should be consistent across the entire application
//!
//! ## Available Error Colors
//!
//! Each error color has specific characteristics and use cases:
//!
//! ### Red (Default)
//! - **Character**: Classic, universally understood error color
//! - **Use cases**: Form validation, critical errors, deletion warnings
//! - **Accessibility**: Excellent contrast ratios in both light and dark modes
//! - **Psychology**: Authoritative but not aggressive
//!
//! ### Tomato
//! - **Character**: Vibrant, attention-grabbing red with orange undertones
//! - **Use cases**: Active error states, real-time validation feedback
//! - **Accessibility**: High visibility and contrast
//! - **Psychology**: Energetic, immediate attention
//!
//! ### Crimson
//! - **Character**: Deep, serious red for critical situations
//! - **Use cases**: Security warnings, destructive actions, system errors
//! - **Accessibility**: Strong contrast, highly visible
//! - **Psychology**: Serious, authoritative, critical
//!
//! ### Ruby
//! - **Character**: Rich, sophisticated red with purple undertones
//! - **Use cases**: Premium applications, elegant error states
//! - **Accessibility**: Good contrast with sophisticated appearance
//! - **Psychology**: Refined, less harsh than pure red
//!
//! ## Usage Examples
//!
//! ### Basic Error Styling
//! ```rust
//! use forge_ui::{error_palette, set_error_palette, ErrorColor};
//!
//! // Get current error color palette
//! let error_palette = error_palette();
//! let error_bg = error_palette.bg;           // Subtle error background
//! let error_border = error_palette.border;   // Error border color
//! let error_text = error_palette.text;       // Error text color
//! ```
//!
//! ### Component Integration
//! ```rust
//! // Button with error styling
//! let error_button = Button::new("Delete")
//!     .color(error_palette())    // Uses current error colors
//!     .variant(ButtonVariant::Solid)
//!     .build();
//!
//! // Text with error color
//! let error_message = Text::new("Invalid input")
//!     .color(TextColor::Custom(error_palette().text))
//!     .build();
//! ```
//!
//! ### Customizing Error Colors
//! ```rust
//! // Set different error color for brand consistency
//! set_error_palette(ErrorColor::Crimson);  // More serious tone
//! set_error_palette(ErrorColor::Tomato);   // More energetic tone
//! set_error_palette(ErrorColor::Ruby);     // More sophisticated tone
//!
//! // Reset to default
//! set_error_palette(ErrorColor::Red);
//! ```
//!
//! ## Design Guidelines
//!
//! ### When to Use Each Color
//! - **Red**: General errors, form validation, standard warnings
//! - **Tomato**: Active states, real-time feedback, urgent notifications
//! - **Crimson**: Critical errors, security issues, destructive actions
//! - **Ruby**: Premium applications, sophisticated error styling
//!
//! ### Accessibility Considerations
//! - All error colors meet WCAG AA contrast requirements
//! - Error states should never rely solely on color (use icons, text)
//! - Provide alternative indicators for colorblind users
//! - Ensure error messages are clearly readable
//!
//! ## Implementation Notes
//!
//! - Thread-safe global state management
//! - Automatic theme resolution (light/dark modes)
//! - Fallback to Red ensures system reliability
//! - Integration with component color systems

use once_cell::sync::Lazy;
use std::sync::RwLock;

use crate::theme::color::{structs::UiColorPalette, theme_mode};

/// Available error color options for communicating error states.
/// 
/// ErrorColor provides a curated selection of red-family colors specifically
/// chosen for their effectiveness in error communication. Each color has
/// distinct characteristics and psychological associations suitable for
/// different types of error scenarios.
/// 
/// # Color Selection Criteria
/// 
/// Each color is chosen based on:
/// - **Accessibility**: Meets WCAG contrast requirements
/// - **Psychology**: Appropriate emotional response for error states
/// - **Versatility**: Works across different UI contexts
/// - **Brand Compatibility**: Suitable for various brand aesthetics
/// 
/// # Usage Guidelines
/// 
/// Choose error colors based on your application's needs:
/// - **Conservative**: Use Red for universal recognition
/// - **Energetic**: Use Tomato for dynamic, active feedback
/// - **Critical**: Use Crimson for serious, important errors
/// - **Sophisticated**: Use Ruby for premium, elegant applications
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorColor {
    /// Vibrant red with orange undertones for active error states.
    /// 
    /// **Characteristics**: Energetic, attention-grabbing, dynamic
    /// **Best for**: Real-time validation, active error states, notifications
    /// **Psychology**: Immediate attention, urgency without panic
    Tomato,
    
    /// Classic red - the universal error color.
    /// 
    /// **Characteristics**: Familiar, authoritative, well-understood
    /// **Best for**: Form validation, general errors, standard warnings
    /// **Psychology**: Clear communication, professional, reliable
    Red,
    
    /// Deep red for critical and serious error conditions.
    /// 
    /// **Characteristics**: Serious, authoritative, high-impact
    /// **Best for**: Security warnings, destructive actions, critical system errors
    /// **Psychology**: Gravity, importance, requires immediate attention
    Crimson,
    
    /// Sophisticated red with purple undertones for premium applications.
    /// 
    /// **Characteristics**: Refined, elegant, less harsh than pure red
    /// **Best for**: Premium applications, sophisticated error styling
    /// **Psychology**: Professional, refined, approachable authority
    Ruby,
}

impl Default for ErrorColor {
    /// Returns Red as the default error color.
    /// 
    /// Red is chosen as the default because it:
    /// - Is universally recognized as an error indicator
    /// - Provides excellent accessibility across all contexts
    /// - Works well in both light and dark themes
    /// - Has broad cultural acceptance for error communication
    /// - Offers good balance between visibility and professionalism
    fn default() -> Self {
        Self::Red
    }
}

/// Global error color state with thread-safe access.
/// 
/// This static variable maintains the current error color choice used throughout
/// the application. It's initialized with the default Red color and can be
/// changed at runtime to customize the error appearance globally.
/// 
/// # Thread Safety
/// 
/// Protected by RwLock for safe concurrent access:
/// - Multiple readers can access simultaneously for high performance
/// - Exclusive writer access for color updates
/// - Safe usage across multiple threads and Bevy systems
/// 
/// # Initialization
/// 
/// Uses `once_cell::Lazy` for efficient lazy initialization,
/// ensuring the error color is only set up when first accessed.
pub static ERROR_COLOR: Lazy<RwLock<ErrorColor>> = Lazy::new(|| RwLock::new(ErrorColor::default()));

/// Gets the current error color palette with full theme integration.
/// 
/// This function returns a complete UiColorPalette for the currently selected
/// error color, automatically resolved for the current theme mode (light/dark).
/// The palette contains all semantic color roles needed for comprehensive
/// error styling across UI components.
/// 
/// # Returns
/// 
/// A complete `UiColorPalette` containing:
/// - **Background colors**: Subtle error backgrounds, hover states
/// - **Border colors**: Error borders, focus rings, dividers
/// - **Accent colors**: Solid error backgrounds, prominent error elements
/// - **Text colors**: Error text, high-contrast error text
/// - **Alpha variants**: Semi-transparent versions for overlays
/// 
/// # Theme Resolution
/// 
/// The palette is automatically resolved for the current theme:
/// - **Light mode**: Darker, more saturated error colors on light backgrounds
/// - **Dark mode**: Lighter, more muted error colors on dark backgrounds
/// 
/// # Performance
/// 
/// Uses a read lock for efficient concurrent access. The error color
/// selection is cached, so repeated calls are very fast.
/// 
/// # Example Usage
/// ```rust
/// let error_palette = error_palette();
/// 
/// // Different error styling options
/// let error_bg = error_palette.bg_subtle;     // Subtle error background
/// let error_border = error_palette.border;    // Error border color
/// let error_solid = error_palette.solid;      // Prominent error color
/// let error_text = error_palette.text;        // Error text color
/// 
/// // Alpha variants for overlays
/// let error_overlay = error_palette.bg_a;     // Semi-transparent error background
/// ```
/// 
/// # Panic Safety
/// 
/// Uses `expect()` for lock access - will panic only if the lock is poisoned,
/// which indicates a serious threading issue that should halt execution.
pub fn error_palette() -> UiColorPalette {
    let error_color = *ERROR_COLOR.read().expect("ERROR_COLOR poisoned");
    let theme_palettes = theme_mode::theme();

    match error_color {
        ErrorColor::Tomato => theme_palettes.tomato,
        ErrorColor::Red => theme_palettes.red,
        ErrorColor::Crimson => theme_palettes.crimson,
        ErrorColor::Ruby => theme_palettes.ruby,
    }
}

/// Sets the global error color for the entire application.
/// 
/// This function changes the error color used by all components globally.
/// The change is immediate and affects all new error styling, but existing
/// components may need to be refreshed to reflect the new color.
/// 
/// # Arguments
/// 
/// * `color` - The ErrorColor variant to use for all error states
/// 
/// # Color Characteristics
/// 
/// Each color option has specific use cases:
/// 
/// - **Tomato**: Vibrant, energetic - good for active feedback
/// - **Red**: Classic, professional - good for general errors
/// - **Crimson**: Serious, critical - good for important warnings
/// - **Ruby**: Sophisticated, elegant - good for premium applications
/// 
/// # Thread Safety
/// 
/// This function acquires a write lock, ensuring exclusive access during
/// the color change. This prevents race conditions but may briefly block
/// other threads accessing the error palette.
/// 
/// # Example Usage
/// ```rust
/// use forge_ui::{set_error_palette, ErrorColor};
/// 
/// // Set to vibrant tomato for active, energetic error feedback
/// set_error_palette(ErrorColor::Tomato);
/// 
/// // Set to crimson for serious, critical error scenarios
/// set_error_palette(ErrorColor::Crimson);
/// 
/// // Set to ruby for sophisticated, premium application styling
/// set_error_palette(ErrorColor::Ruby);
/// 
/// // Reset to classic red for universal compatibility
/// set_error_palette(ErrorColor::Red);
/// ```
/// 
/// # Design Considerations
/// 
/// When choosing an error color, consider:
/// - **Brand identity**: Does the color align with your brand?
/// - **User psychology**: What emotional response do you want?
/// - **Application context**: Is it casual, professional, or critical?
/// - **Accessibility**: All options meet WCAG standards, but test with users
/// 
/// # Panic Safety
/// 
/// Uses `expect()` for lock access - will panic only if the lock is poisoned,
/// indicating a serious threading issue that should halt execution.
pub fn set_error_palette(color: ErrorColor) {
    *ERROR_COLOR.write().expect("ERROR_COLOR poisoned") = color;
}

/// Gets the currently selected error color variant.
/// 
/// This function returns the ErrorColor enum variant currently being used
/// for error styling, without resolving it to a full color palette.
/// Useful for conditional logic, settings persistence, or UI controls
/// that display the current error color choice.
/// 
/// # Returns
/// 
/// The current `ErrorColor` variant (Tomato, Red, Crimson, or Ruby)
/// 
/// # Performance
/// 
/// Very fast operation using a read lock. Multiple threads can call
/// this simultaneously without blocking each other.
/// 
/// # Example Usage
/// ```rust
/// use forge_ui::{get_error_color, ErrorColor};
/// 
/// let current_color = get_error_color();
/// 
/// // Conditional logic based on error color
/// match current_color {
///     ErrorColor::Red => println!("Using classic red errors"),
///     ErrorColor::Tomato => println!("Using vibrant tomato errors"),
///     ErrorColor::Crimson => println!("Using serious crimson errors"),
///     ErrorColor::Ruby => println!("Using sophisticated ruby errors"),
/// }
/// 
/// // Settings persistence
/// let error_setting = match current_color {
///     ErrorColor::Red => "red",
///     ErrorColor::Tomato => "tomato",
///     ErrorColor::Crimson => "crimson",
///     ErrorColor::Ruby => "ruby",
/// };
/// save_user_preference("error_color", error_setting);
/// ```
/// 
/// # Panic Safety
/// 
/// Uses `expect()` for lock access - will panic only if the lock is poisoned,
/// indicating a serious threading issue that should halt execution.
pub fn get_error_color() -> ErrorColor {
    *ERROR_COLOR.read().expect("ERROR_COLOR poisoned")
}
