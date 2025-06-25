//! Accent Color Management for Forge UI
//!
//! This module provides global accent color management functionality, allowing
//! applications to dynamically change their primary accent color while maintaining
//! consistency across all UI components. The accent color system supports both
//! light and dark themes with automatic theme-aware color resolution.
//!
//! ## Key Features
//!
//! - **Global Accent Color**: Single source of truth for the primary accent color
//! - **Theme Awareness**: Automatic light/dark mode color resolution
//! - **Runtime Switching**: Change accent colors during application runtime
//! - **Thread Safety**: Safe access from multiple threads using RwLock
//! - **Fallback Support**: Graceful handling of invalid or missing color palettes
//!
//! ## Default Accent Color
//!
//! The system defaults to the **Indigo** palette, which provides:
//! - Professional and trustworthy appearance
//! - Excellent accessibility and contrast ratios
//! - Wide compatibility across different UI contexts
//! - Neutral appeal suitable for most applications
//!
//! ## Usage Examples
//!
//! ### Basic Usage
//! ```rust
//! use forge_ui::{accent_palette, set_accent_palette, UiColorPalettesName};
//!
//! // Get current accent color palette
//! let accent = accent_palette();
//! let primary_color = accent.solid;
//! let background_color = accent.bg;
//!
//! // Change to blue accent
//! set_accent_palette(Some(UiColorPalettesName::Blue));
//!
//! // Reset to default (indigo)
//! set_accent_palette(None);
//! ```
//!
//! ### Theme-Aware Usage
//! ```rust
//! // The accent palette automatically adapts to light/dark theme
//! let accent = accent_palette();
//! 
//! // In light mode: lighter, more vibrant colors
//! // In dark mode: darker, more muted colors
//! let button_color = accent.solid;  // Always appropriate for current theme
//! ```
//!
//! ### Component Integration
//! ```rust
//! // Components automatically use the current accent color
//! let button = Button::new("Primary Action")
//!     .color(accent_palette())  // Uses current accent
//!     .build();
//!
//! // Or use the accent color directly in styling
//! let custom_style = Style {
//!     background_color: accent_palette().solid,
//!     border_color: accent_palette().border,
//!     // ...
//! };
//! ```
//!
//! ## Color Palette Options
//!
//! The system supports all available color palettes:
//!
//! ### Neutral Options
//! - **Gray**: Subtle, professional appearance
//! - **Mauve**: Warm, sophisticated feel
//! - **Slate**: Cool, modern aesthetic
//!
//! ### Colorful Options
//! - **Blue**: Classic, trustworthy (good alternative to Indigo)
//! - **Green**: Growth, success, environmental themes
//! - **Red**: Energy, urgency, important actions
//! - **Purple**: Creativity, luxury, premium feel
//! - **Orange**: Energetic, friendly, call-to-action
//!
//! ## Implementation Notes
//!
//! - Uses `once_cell::Lazy` for efficient initialization
//! - Thread-safe access via `RwLock` for concurrent applications
//! - Automatic theme resolution prevents color mismatches
//! - Fallback to Indigo ensures the system never fails

use super::{
    structs::{UiColorPalette, UiColorPalettes, UiColorPalettesName},
    theme_mode::{theme_mode, ThemeMode},
};
use once_cell::sync::Lazy;
use std::sync::RwLock;

/// Global accent color palette with thread-safe access.
/// 
/// This static variable holds the current accent color palette used throughout
/// the application. It's initialized with the Indigo palette as a sensible default
/// that works well in most contexts and provides excellent accessibility.
/// 
/// # Thread Safety
/// 
/// The accent palette is protected by an RwLock, allowing:
/// - Multiple concurrent readers for high-performance access
/// - Exclusive writer access for palette updates
/// - Safe usage across multiple threads and systems
/// 
/// # Theme Awareness
/// 
/// The palette automatically resolves to the appropriate light or dark variant
/// based on the current theme mode, ensuring visual consistency.
/// 
/// # Default Choice: Indigo
/// 
/// Indigo is chosen as the default because it:
/// - Provides professional, trustworthy appearance
/// - Offers excellent contrast ratios for accessibility
/// - Works well across different UI contexts
/// - Has broad appeal and neutral associations
pub static ACCENT_PALETTE: Lazy<RwLock<UiColorPalette>> = Lazy::new(|| {
    RwLock::new(if theme_mode() == ThemeMode::Dark {
        UiColorPalettes::dark_mode().indigo
    } else {
        UiColorPalettes::light_mode().indigo
    })
});

/// Gets the current accent color palette.
/// 
/// This function provides read-only access to the global accent color palette.
/// The returned palette is automatically resolved for the current theme mode
/// (light or dark) and contains all semantic color roles needed for UI theming.
/// 
/// # Returns
/// 
/// A complete `UiColorPalette` with theme-appropriate colors for:
/// - Background colors (base, bg, bg_hover, bg_active, etc.)
/// - Border colors (line, border, border_hover)
/// - Accent colors (solid, solid_hover)
/// - Text colors (text, text_contrast)
/// - Alpha variants for transparency effects
/// 
/// # Performance
/// 
/// This function uses a read lock, allowing multiple concurrent accesses
/// without blocking. The palette is cloned to ensure thread safety and
/// prevent lock contention.
/// 
/// # Example
/// ```rust
/// let accent = accent_palette();
/// 
/// // Use different color roles
/// let primary_button = accent.solid;      // High-contrast accent color
/// let subtle_bg = accent.bg_subtle;       // Subtle background
/// let border_color = accent.border;       // Standard border
/// let hover_state = accent.bg_hover;      // Hover background
/// ```
pub fn accent_palette() -> UiColorPalette {
    ACCENT_PALETTE.read().unwrap().clone()
}

/// Sets the global accent color palette.
/// 
/// This function allows runtime customization of the application's accent color.
/// The change is immediately effective for all new components and can be used
/// to implement user preferences, branding changes, or dynamic theming.
/// 
/// # Arguments
/// 
/// * `palette` - The color palette to use as the new accent color.
///   - `Some(palette_name)` - Use the specified color palette
///   - `None` - Reset to the default Indigo palette
/// 
/// # Theme Awareness
/// 
/// The function automatically resolves the appropriate light or dark variant
/// of the specified palette based on the current theme mode. This ensures
/// visual consistency regardless of the theme setting.
/// 
/// # Supported Palettes
/// 
/// All color palettes from `UiColorPalettesName` are supported, including:
/// - Neutral colors (Gray, Mauve, Slate, Sage, Olive, Sand)
/// - Red family (Tomato, Red, Ruby, Crimson)
/// - Pink/Purple family (Pink, Plum, Purple, Violet, Iris)
/// - Blue family (Indigo, Blue, Cyan)
/// - Green family (Teal, Jade, Green)
/// - Additional colors (and others...)
/// 
/// # Fallback Behavior
/// 
/// If an unrecognized palette or `None` is provided, the system falls back
/// to the default Indigo palette, ensuring the application never enters
/// an invalid state.
/// 
/// # Thread Safety
/// 
/// This function acquires a write lock, ensuring exclusive access during
/// the palette update. This prevents race conditions but may briefly block
/// other threads accessing the accent palette.
/// 
/// # Examples
/// 
/// ```rust
/// use forge_ui::{set_accent_palette, UiColorPalettesName};
/// 
/// // Set accent to blue for a trustworthy, professional look
/// set_accent_palette(Some(UiColorPalettesName::Blue));
/// 
/// // Set accent to green for success/environmental themes
/// set_accent_palette(Some(UiColorPalettesName::Green));
/// 
/// // Set accent to red for urgent/important actions
/// set_accent_palette(Some(UiColorPalettesName::Red));
/// 
/// // Reset to default indigo
/// set_accent_palette(None);
/// ```
/// 
/// # Performance Considerations
/// 
/// - Palette changes are immediate but may require UI refresh
/// - Consider batching multiple theming changes together
/// - The write lock briefly blocks all palette access
pub fn set_accent_palette(palette: Option<UiColorPalettesName>) {
    *ACCENT_PALETTE.write().unwrap() = if theme_mode() == ThemeMode::Dark {
        match palette {
            // === Neutral Color Palettes ===
            Some(UiColorPalettesName::Gray) => UiColorPalettes::dark_mode().gray,
            Some(UiColorPalettesName::Mauve) => UiColorPalettes::dark_mode().mauve,
            Some(UiColorPalettesName::Slate) => UiColorPalettes::dark_mode().slate,
            Some(UiColorPalettesName::Sage) => UiColorPalettes::dark_mode().sage,
            Some(UiColorPalettesName::Olive) => UiColorPalettes::dark_mode().olive,
            Some(UiColorPalettesName::Sand) => UiColorPalettes::dark_mode().sand,
            
            // === Red Family Palettes ===
            Some(UiColorPalettesName::Tomato) => UiColorPalettes::dark_mode().tomato,
            Some(UiColorPalettesName::Red) => UiColorPalettes::dark_mode().red,
            Some(UiColorPalettesName::Ruby) => UiColorPalettes::dark_mode().ruby,
            Some(UiColorPalettesName::Crimson) => UiColorPalettes::dark_mode().crimson,
            
            // === Pink/Purple Family Palettes ===
            Some(UiColorPalettesName::Pink) => UiColorPalettes::dark_mode().pink,
            Some(UiColorPalettesName::Plum) => UiColorPalettes::dark_mode().plum,
            Some(UiColorPalettesName::Purple) => UiColorPalettes::dark_mode().purple,
            Some(UiColorPalettesName::Violet) => UiColorPalettes::dark_mode().violet,
            Some(UiColorPalettesName::Iris) => UiColorPalettes::dark_mode().iris,
            
            // === Blue Family Palettes ===
            Some(UiColorPalettesName::Indigo) => UiColorPalettes::dark_mode().indigo,
            Some(UiColorPalettesName::Blue) => UiColorPalettes::dark_mode().blue,
            Some(UiColorPalettesName::Cyan) => UiColorPalettes::dark_mode().cyan,
            
            // === Green Family Palettes ===
            Some(UiColorPalettesName::Teal) => UiColorPalettes::dark_mode().teal,
            Some(UiColorPalettesName::Jade) => UiColorPalettes::dark_mode().jade,
            Some(UiColorPalettesName::Green) => UiColorPalettes::dark_mode().green,
            
            // === Fallback to Default ===
            _ => {
                // Default to indigo for unrecognized palettes or None
                UiColorPalettes::dark_mode().indigo
            }
        }
    } else {
        match palette {
            // === Neutral Color Palettes ===
            Some(UiColorPalettesName::Gray) => UiColorPalettes::light_mode().gray,
            Some(UiColorPalettesName::Mauve) => UiColorPalettes::light_mode().mauve,
            Some(UiColorPalettesName::Slate) => UiColorPalettes::light_mode().slate,
            Some(UiColorPalettesName::Sage) => UiColorPalettes::light_mode().sage,
            Some(UiColorPalettesName::Olive) => UiColorPalettes::light_mode().olive,
            Some(UiColorPalettesName::Sand) => UiColorPalettes::light_mode().sand,
            
            // === Red Family Palettes ===
            Some(UiColorPalettesName::Tomato) => UiColorPalettes::light_mode().tomato,
            Some(UiColorPalettesName::Red) => UiColorPalettes::light_mode().red,
            Some(UiColorPalettesName::Ruby) => UiColorPalettes::light_mode().ruby,
            Some(UiColorPalettesName::Crimson) => UiColorPalettes::light_mode().crimson,
            
            // === Pink/Purple Family Palettes ===
            Some(UiColorPalettesName::Pink) => UiColorPalettes::light_mode().pink,
            Some(UiColorPalettesName::Plum) => UiColorPalettes::light_mode().plum,
            Some(UiColorPalettesName::Purple) => UiColorPalettes::light_mode().purple,
            Some(UiColorPalettesName::Violet) => UiColorPalettes::light_mode().violet,
            Some(UiColorPalettesName::Iris) => UiColorPalettes::light_mode().iris,
            
            // === Blue Family Palettes ===
            Some(UiColorPalettesName::Indigo) => UiColorPalettes::light_mode().indigo,
            Some(UiColorPalettesName::Blue) => UiColorPalettes::light_mode().blue,
            Some(UiColorPalettesName::Cyan) => UiColorPalettes::light_mode().cyan,
            
            // === Green Family Palettes ===
            Some(UiColorPalettesName::Teal) => UiColorPalettes::light_mode().teal,
            Some(UiColorPalettesName::Jade) => UiColorPalettes::light_mode().jade,
            Some(UiColorPalettesName::Green) => UiColorPalettes::light_mode().green,
            
            // === Fallback to Default ===
            _ => {
                // Default to indigo for unrecognized palettes or None
                UiColorPalettes::light_mode().indigo
            }
        }
    }
}
