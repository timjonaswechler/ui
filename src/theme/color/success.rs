//! Success Color Management for Forge UI
//!
//! This module provides specialized color management for success states, positive
//! feedback, and completion indicators in UI components. It offers a carefully
//! curated selection of green-family colors optimized for success communication
//! while maintaining excellent accessibility and positive psychological associations.
//!
//! ## Key Features
//!
//! - **Semantic Success Colors**: Green-family colors for effective positive feedback
//! - **Global Success Theming**: Consistent success colors across all components
//! - **Runtime Customization**: Change success color scheme during application runtime
//! - **Theme Integration**: Automatic light/dark mode color resolution
//! - **Accessibility Optimized**: High contrast ratios for clear positive feedback
//!
//! ## Success Color Philosophy
//!
//! Success colors serve crucial UX communication functions:
//! - **Positive Reinforcement**: Users need clear confirmation of successful actions
//! - **Progress Indication**: Show completion and achievement states effectively
//! - **Trust Building**: Successful interactions build user confidence and trust
//! - **Universal Recognition**: Green is globally recognized for success and "go"
//! - **Emotional Response**: Colors should evoke positive, accomplished feelings
//!
//! ## Available Success Colors
//!
//! Each success color has distinct characteristics and optimal use cases:
//!
//! ### Green (Default)
//! - **Character**: Classic, universally recognized success color
//! - **Use cases**: Form submissions, successful operations, positive confirmations
//! - **Accessibility**: Excellent contrast and visibility across all contexts
//! - **Psychology**: Satisfaction, accomplishment, reliability, "go" signal
//! - **Recognition**: Global standard for success, completion, and positive states
//!
//! ### Teal
//! - **Character**: Professional blue-green, sophisticated and balanced
//! - **Use cases**: Business applications, professional success states, balanced feedback
//! - **Accessibility**: Strong contrast with modern, professional appearance
//! - **Psychology**: Balance, professionalism, trustworthy success
//! - **Recognition**: Associated with reliability and professional accomplishment
//!
//! ### Jade
//! - **Character**: Pure, vibrant green with natural associations
//! - **Use cases**: Environmental apps, health applications, natural/organic themes
//! - **Accessibility**: Clear visibility with natural, appealing aesthetics
//! - **Psychology**: Growth, nature, health, pure success
//! - **Recognition**: Connected to nature, growth, and organic success
//!
//! ### Grass
//! - **Character**: Fresh, natural green evoking growth and vitality
//! - **Use cases**: Outdoor applications, growth metrics, fresh/new successes
//! - **Accessibility**: Good contrast with energetic, fresh appearance
//! - **Psychology**: Energy, freshness, new growth, dynamic success
//! - **Recognition**: Associated with new beginnings and fresh achievements
//!
//! ## Usage Examples
//!
//! ### Basic Success Styling
//! ```rust
//! use forge_ui::{success_palette, set_success_palette, SuccessColor};
//!
//! // Get current success color palette
//! let success = success_palette();
//! let success_bg = success.bg_subtle;     // Subtle success background
//! let success_border = success.border;    // Success border color
//! let success_text = success.text;        // Success text color
//! let success_solid = success.solid;      // Prominent success color
//! ```
//!
//! ### Component Integration
//! ```rust
//! // Success alert component
//! let success_alert = Alert::new("Operation completed successfully!")
//!     .color(success_palette())    // Uses current success colors
//!     .variant(AlertVariant::Success)
//!     .icon("check-circle")
//!     .build();
//!
//! // Success button for positive actions
//! let success_button = Button::new("Save Changes")
//!     .color(success_palette())
//!     .variant(ButtonVariant::Solid)
//!     .build();
//!
//! // Progress indicator showing completion
//! let progress_complete = ProgressBar::new(100)
//!     .color(success_palette())
//!     .show_checkmark(true)
//!     .build();
//! ```
//!
//! ### Customizing Success Colors
//! ```rust
//! // Set to teal for professional, balanced success feedback
//! set_success_palette(SuccessColor::Teal);
//!
//! // Set to jade for natural, environmental theme
//! set_success_palette(SuccessColor::Jade);
//!
//! // Set to grass for fresh, energetic success feedback
//! set_success_palette(SuccessColor::Grass);
//!
//! // Reset to classic green
//! set_success_palette(SuccessColor::Green);
//! ```
//!
//! ## Design Guidelines
//!
//! ### When to Use Each Color
//!
//! - **Green**: Universal success, form completions, standard positive feedback
//! - **Teal**: Professional applications, business success states, balanced feedback
//! - **Jade**: Environmental themes, health apps, natural/organic success states
//! - **Grass**: Growth indicators, fresh achievements, energetic positive feedback
//!
//! ### Success Communication Best Practices
//!
//! - **Immediate Feedback**: Success states should be immediately visible
//! - **Clear Messaging**: Combine color with text and/or icons for clarity
//! - **Appropriate Duration**: Success feedback should be visible long enough to register
//! - **Consistent Application**: Use the same success color throughout the app
//! - **Accessibility**: Ensure success states are perceivable by all users
//!
//! ### Success vs Warning vs Error
//!
//! - **Success (Green family)**: "Great, completed successfully" - achievements, completions
//! - **Warning (Yellow/Amber)**: "Pay attention, but proceed" - cautions, reviews needed
//! - **Error (Red family)**: "Stop, something is wrong" - problems, failures
//!
//! ## Accessibility Considerations
//!
//! - All success colors meet WCAG AA contrast requirements
//! - Never rely solely on color to communicate success
//! - Include success icons (checkmarks, thumbs up) alongside color
//! - Provide text confirmations of successful actions
//! - Test with colorblind users or simulation tools
//! - Consider animation or other feedback mechanisms
//!
//! ## Implementation Notes
//!
//! - Thread-safe global state management using RwLock
//! - Automatic theme resolution for light/dark modes
//! - Fallback to Green ensures reliable success communication
//! - Seamless integration with all UI component color systems
//! - Efficient lazy initialization and performance optimization

use once_cell::sync::Lazy;
use std::sync::RwLock;

use crate::theme::color::{structs::UiColorPalette, theme_mode};

/// Available success color options for communicating positive outcomes and achievements.
/// 
/// SuccessColor provides a carefully curated selection of green-family colors that are
/// optimized for success communication. These colors are chosen for their ability to
/// provide clear positive feedback, build user confidence, and maintain excellent
/// accessibility while evoking appropriate psychological responses.
/// 
/// # Color Selection Criteria
/// 
/// Each success color is evaluated based on:
/// - **Positive Psychology**: Evokes feelings of accomplishment and satisfaction
/// - **Universal Recognition**: Globally understood success associations
/// - **Accessibility**: Meets or exceeds WCAG contrast requirements
/// - **Brand Flexibility**: Suitable for various application aesthetics
/// - **Contextual Appropriateness**: Works across different success scenarios
/// 
/// # Usage Guidelines
/// 
/// Choose success colors based on your application's character and use cases:
/// - **Universal Applications**: Green for broad compatibility and recognition
/// - **Professional Contexts**: Teal for business and professional applications
/// - **Natural/Health Themes**: Jade for environmental and wellness applications
/// - **Dynamic/Fresh Contexts**: Grass for energetic and growth-oriented applications
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SuccessColor {
    /// Classic green - the universal success color.
    /// 
    /// **Characteristics**: Familiar, reliable, universally understood
    /// **Best for**: Form submissions, general success states, positive confirmations
    /// **Psychology**: Accomplishment, reliability, "go" signal
    /// **Recognition**: Global standard for success and positive outcomes
    Green,
    
    /// Professional blue-green for balanced, sophisticated success feedback.
    /// 
    /// **Characteristics**: Professional, balanced, sophisticated
    /// **Best for**: Business applications, professional success states, corporate environments
    /// **Psychology**: Balance, professionalism, trustworthy success
    /// **Recognition**: Associated with reliability and professional accomplishment
    Teal,
    
    /// Pure, vibrant green with natural associations.
    /// 
    /// **Characteristics**: Pure, natural, vibrant, health-associated
    /// **Best for**: Environmental apps, health applications, natural/organic themes
    /// **Psychology**: Growth, nature, health, pure achievement
    /// **Recognition**: Connected to nature, wellness, and organic success
    Jade,
    
    /// Fresh, natural green evoking growth and vitality.
    /// 
    /// **Characteristics**: Fresh, energetic, natural, growth-oriented
    /// **Best for**: Growth metrics, fresh achievements, outdoor/fitness applications
    /// **Psychology**: Energy, freshness, new growth, dynamic progress
    /// **Recognition**: Associated with new beginnings and fresh accomplishments
    Grass,
}

impl Default for SuccessColor {
    /// Returns Green as the default success color.
    /// 
    /// Green is chosen as the default because it:
    /// - Is universally recognized as a success/positive indicator across all cultures
    /// - Provides excellent accessibility and contrast in both light and dark themes
    /// - Has strong psychological associations with accomplishment and "go" signals
    /// - Works effectively across diverse application types and contexts
    /// - Offers optimal balance between visibility and positive emotional response
    /// - Maintains broad compatibility with various brand aesthetics and design systems
    fn default() -> Self {
        Self::Green
    }
}

/// Global success color state with thread-safe access.
/// 
/// This static variable maintains the current success color choice used throughout
/// the application. It's initialized with the default Green color and can be
/// changed at runtime to customize the success appearance globally across all
/// components and UI elements.
/// 
/// # Thread Safety
/// 
/// Protected by RwLock for safe concurrent access:
/// - Multiple readers can access simultaneously for optimal performance
/// - Exclusive writer access for color updates without race conditions
/// - Safe usage across multiple threads and Bevy systems
/// 
/// # Initialization
/// 
/// Uses `once_cell::Lazy` for efficient lazy initialization,
/// ensuring the success color is only set up when first accessed.
/// This approach provides better startup performance and memory efficiency.
pub static SUCCESS_COLOR: Lazy<RwLock<SuccessColor>> = Lazy::new(|| RwLock::new(SuccessColor::default()));

/// Gets the current success color palette with full theme integration.
/// 
/// This function returns a complete UiColorPalette for the currently selected
/// success color, automatically resolved for the current theme mode (light/dark).
/// The palette contains all semantic color roles needed for comprehensive
/// success styling across UI components.
/// 
/// # Returns
/// 
/// A complete `UiColorPalette` containing:
/// - **Background colors**: Subtle success backgrounds, hover states, active states
/// - **Border colors**: Success borders, focus rings, dividers
/// - **Accent colors**: Solid success backgrounds, prominent success elements
/// - **Text colors**: Success text, high-contrast success text
/// - **Alpha variants**: Semi-transparent versions for overlays and layering effects
/// 
/// # Theme Resolution
/// 
/// The palette is automatically resolved for the current theme:
/// - **Light mode**: Darker, more saturated success colors on light backgrounds
/// - **Dark mode**: Lighter, more vibrant success colors on dark backgrounds
/// 
/// # Performance
/// 
/// Uses a read lock for efficient concurrent access. The success color
/// selection is cached, so repeated calls have minimal performance overhead.
/// 
/// # Example Usage
/// ```rust
/// let success_palette = success_palette();
/// 
/// // Different success styling options
/// let success_bg = success_palette.bg_subtle;   // Subtle success background
/// let success_border = success_palette.border;  // Success border color
/// let success_solid = success_palette.solid;    // Prominent success color
/// let success_text = success_palette.text;      // Success text color
/// 
/// // Alpha variants for overlays and effects
/// let success_overlay = success_palette.bg_a;   // Semi-transparent success background
/// let success_accent = success_palette.solid_a; // Semi-transparent success accent
/// 
/// // High contrast for accessibility
/// let success_contrast = success_palette.text_contrast; // Maximum contrast success text
/// ```
/// 
/// # Panic Safety
/// 
/// Uses `expect()` for lock access - will panic only if the lock is poisoned,
/// which indicates a serious threading issue that should halt execution.
pub fn success_palette() -> UiColorPalette {
    let success_color = *SUCCESS_COLOR.read().expect("SUCCESS_COLOR poisoned");
    let theme_palettes = theme_mode::theme();
    
    match success_color {
        SuccessColor::Green => theme_palettes.green,
        SuccessColor::Teal => theme_palettes.teal,
        SuccessColor::Jade => theme_palettes.jade,
        SuccessColor::Grass => theme_palettes.grass,
    }
}

/// Sets the global success color for the entire application.
/// 
/// This function changes the success color used by all components globally.
/// The change is immediate and affects all new success styling, but existing
/// components may need to be refreshed to reflect the new color choice.
/// 
/// # Arguments
/// 
/// * `color` - The SuccessColor variant to use for all success states
/// 
/// # Color Characteristics and Use Cases
/// 
/// Choose based on your application's context and brand:
/// 
/// - **Green**: Universal success color for broad compatibility
///   - Form submissions and validations
///   - General positive confirmations
///   - Standard success operations
///   - Cross-cultural applications
/// 
/// - **Teal**: Professional, balanced success feedback
///   - Business and corporate applications
///   - Professional dashboards and tools
///   - Balanced, sophisticated success states
///   - Financial and data applications
/// 
/// - **Jade**: Natural, pure success associations
///   - Environmental and sustainability apps
///   - Health and wellness applications
///   - Natural/organic product interfaces
///   - Growth and nature-focused applications
/// 
/// - **Grass**: Fresh, energetic success feedback
///   - Fitness and outdoor applications
///   - Growth metrics and progress tracking
///   - Fresh achievements and new milestones
///   - Dynamic, energy-focused interfaces
/// 
/// # Thread Safety
/// 
/// This function acquires a write lock, ensuring exclusive access during
/// the color change. This prevents race conditions but may briefly block
/// other threads accessing the success palette.
/// 
/// # Example Usage
/// ```rust
/// use forge_ui::{set_success_palette, SuccessColor};
/// 
/// // Set to teal for professional, business applications
/// set_success_palette(SuccessColor::Teal);
/// 
/// // Set to jade for environmental or health applications
/// set_success_palette(SuccessColor::Jade);
/// 
/// // Set to grass for fitness or growth-focused applications
/// set_success_palette(SuccessColor::Grass);
/// 
/// // Reset to universal green for broad compatibility
/// set_success_palette(SuccessColor::Green);
/// ```
/// 
/// # Design Considerations
/// 
/// When choosing a success color, consider:
/// - **Application domain**: What type of successes are you celebrating?
/// - **Brand personality**: Formal, natural, energetic, or balanced?
/// - **User expectations**: What success feedback do users expect?
/// - **Cultural context**: Will the app be used internationally?
/// - **Accessibility needs**: All colors meet WCAG standards, but consider user testing
/// 
/// # Panic Safety
/// 
/// Uses `expect()` for lock access - will panic only if the lock is poisoned,
/// indicating a serious threading issue that should halt execution.
pub fn set_success_palette(color: SuccessColor) {
    *SUCCESS_COLOR.write().expect("SUCCESS_COLOR poisoned") = color;
}

/// Gets the currently selected success color variant.
/// 
/// This function returns the SuccessColor enum variant currently being used
/// for success styling, without resolving it to a full color palette.
/// Useful for conditional logic, settings persistence, UI controls that
/// display the current success color choice, or application analytics.
/// 
/// # Returns
/// 
/// The current `SuccessColor` variant (Green, Teal, Jade, or Grass)
/// 
/// # Performance
/// 
/// Very fast operation using a read lock. Multiple threads can call
/// this simultaneously without blocking each other, making it suitable
/// for frequent polling, real-time UI updates, or performance-critical code.
/// 
/// # Example Usage
/// ```rust
/// use forge_ui::{get_success_color, SuccessColor};
/// 
/// let current_color = get_success_color();
/// 
/// // Conditional logic based on success color
/// match current_color {
///     SuccessColor::Green => {
///         println!("Using universal green success feedback");
///         // Maybe show standard success icons
///     },
///     SuccessColor::Teal => {
///         println!("Using professional teal success feedback");
///         // Maybe show business-oriented success metrics
///     },
///     SuccessColor::Jade => {
///         println!("Using natural jade success feedback");
///         // Maybe show growth or health-related success indicators
///     },
///     SuccessColor::Grass => {
///         println!("Using fresh grass success feedback");
///         // Maybe show dynamic progress or achievement animations
///     },
/// }
/// 
/// // Settings persistence for user preferences
/// let success_setting = match current_color {
///     SuccessColor::Green => "green",
///     SuccessColor::Teal => "teal",
///     SuccessColor::Jade => "jade",
///     SuccessColor::Grass => "grass",
/// };
/// save_user_preference("success_color", success_setting);
/// 
/// // UI state for settings or customization panels
/// let is_green_selected = matches!(current_color, SuccessColor::Green);
/// let is_teal_selected = matches!(current_color, SuccessColor::Teal);
/// let is_jade_selected = matches!(current_color, SuccessColor::Jade);
/// let is_grass_selected = matches!(current_color, SuccessColor::Grass);
/// 
/// // Analytics or telemetry
/// track_color_usage("success_color", current_color);
/// ```
/// 
/// # Panic Safety
/// 
/// Uses `expect()` for lock access - will panic only if the lock is poisoned,
/// indicating a serious threading issue that should halt execution.
pub fn get_success_color() -> SuccessColor {
    *SUCCESS_COLOR.read().expect("SUCCESS_COLOR poisoned")
}