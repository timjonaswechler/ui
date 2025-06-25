//! Color Palette Structures for Forge UI
//!
//! This module defines the core color system structures that power Forge UI's
//! comprehensive theming capabilities. Based on Radix UI's color system,
//! it provides semantically meaningful color palettes with built-in accessibility
//! and design consistency.
//!
//! ## Key Features
//!
//! - **Complete Color System**: 32+ carefully crafted color palettes
//! - **Semantic Color Roles**: Background, border, text, and interaction states
//! - **Accessibility Built-in**: WCAG AA/AAA contrast ratios
//! - **Alpha Variants**: Transparent versions for layering and overlays
//! - **State Management**: Hover, active, and focus state colors
//! - **High Contrast Support**: Enhanced accessibility options
//!
//! ## Color Palette Philosophy
//!
//! Each color palette follows Radix UI's 12-step color scale:
//!
//! 1. **App background** - The main application background
//! 2. **Subtle background** - For subtle backgrounds and disabled states
//! 3. **UI element background** - For normal UI element backgrounds
//! 4. **Hovered UI element background** - For hovered UI elements
//! 5. **Active / Selected UI element background** - For active/selected elements
//! 6. **Subtle borders and separators** - For subtle borders
//! 7. **UI element border and focus rings** - For normal borders
//! 8. **Hovered UI element border** - For hovered borders
//! 9. **Solid backgrounds** - For solid, high-contrast backgrounds
//! 10. **Hovered solid backgrounds** - For hovered solid backgrounds
//! 11. **Low-contrast text** - For low-contrast text
//! 12. **High-contrast text** - For high-contrast text
//!
//! ## Usage Examples
//!
//! ```rust
//! use forge_ui::theme;
//!
//! // Access color palettes
//! let blue_palette = theme().blue;
//! let accent_color = blue_palette.solid;
//! let background = blue_palette.bg_subtle;
//!
//! // Use semantic roles
//! let button_bg = blue_palette.bg;          // Normal state
//! let button_hover = blue_palette.bg_hover; // Hover state
//! let button_border = blue_palette.border;  // Border color
//! let button_text = blue_palette.text;      // Text color
//!
//! // Alpha variants for overlays
//! let overlay_bg = blue_palette.bg_a;       // Semi-transparent background
//! let modal_border = blue_palette.border_a; // Semi-transparent border
//! ```

use bevy::prelude::*;

/// Complete collection of all available color palettes in the Forge UI system.
/// 
/// UiColorPalettes provides access to the full range of color options,
/// including neutral colors (grays), chromatic colors (blues, reds, etc.),
/// and special colors (white, black). Each palette contains semantic color
/// roles for consistent theming across components.
/// 
/// # Color Categories
/// 
/// ## Neutral Colors
/// - **Gray**: Primary neutral for most UI elements
/// - **Mauve**: Warm neutral with subtle purple undertones
/// - **Slate**: Cool neutral with blue undertones
/// - **Sage**: Natural neutral with green undertones
/// - **Olive**: Earthy neutral with olive undertones
/// - **Sand**: Warm neutral with beige undertones
/// 
/// ## Red Family
/// - **Tomato**: Vibrant red for notifications and alerts
/// - **Red**: Classic red for errors and warnings
/// - **Ruby**: Deep red for luxury and premium feels
/// - **Crimson**: Bold red for strong emphasis
/// 
/// ## Pink/Purple Family
/// - **Pink**: Playful pink for creative applications
/// - **Plum**: Rich purple-pink for sophisticated designs
/// - **Purple**: Classic purple for creativity and luxury
/// - **Violet**: Light purple for gentle emphasis
/// 
/// ## Blue Family
/// - **Iris**: Soft blue-purple for calm interfaces
/// - **Indigo**: Deep blue for professional applications
/// - **Blue**: Primary blue for most interactive elements
/// - **Cyan**: Bright blue-green for tech and modern designs
/// - **Sky**: Light blue for airy, spacious feels
/// 
/// ## Green Family
/// - **Teal**: Blue-green for balance and harmony
/// - **Jade**: Pure green for nature and growth
/// - **Green**: Success and positive actions
/// - **Grass**: Natural green for organic feels
/// - **Mint**: Light green for fresh, clean designs
/// - **Lime**: Bright green for energy and vitality
/// 
/// ## Warm Colors
/// - **Bronze**: Metallic brown for premium feels
/// - **Brown**: Earthy brown for natural, warm designs
/// - **Gold**: Luxury metallic for premium applications
/// - **Yellow**: Bright yellow for attention and energy
/// - **Amber**: Warm orange-yellow for warning states
/// - **Orange**: Vibrant orange for call-to-action elements
#[derive(Debug, Clone)]
pub struct UiColorPalettes {
    /// Pure white color palette for high contrast and clean designs
    pub white: UiColorPaletteBasic,
    /// Pure black color palette for maximum contrast and bold designs
    pub black: UiColorPaletteBasic,
    
    // === Neutral Color Palettes ===
    /// Primary neutral gray - most versatile for general UI elements
    pub gray: UiColorPalette,
    /// Warm neutral with subtle purple undertones - sophisticated and elegant
    pub mauve: UiColorPalette,
    /// Cool neutral with blue undertones - modern and technical
    pub slate: UiColorPalette,
    /// Natural neutral with green undertones - calming and organic
    pub sage: UiColorPalette,
    /// Earthy neutral with olive undertones - natural and grounded
    pub olive: UiColorPalette,
    /// Warm neutral with beige undertones - comfortable and accessible
    pub sand: UiColorPalette,
    
    // === Red Family Palettes ===
    /// Vibrant red for notifications, alerts, and active states
    pub tomato: UiColorPalette,
    /// Classic red for errors, warnings, and critical actions
    pub red: UiColorPalette,
    /// Deep red for luxury, premium, and high-value content
    pub ruby: UiColorPalette,
    /// Bold red for strong emphasis and important calls-to-action
    pub crimson: UiColorPalette,
    
    // === Pink/Purple Family Palettes ===
    /// Playful pink for creative, fun, and expressive interfaces
    pub pink: UiColorPalette,
    /// Rich purple-pink for sophisticated and premium designs
    pub plum: UiColorPalette,
    /// Classic purple for creativity, luxury, and brand identity
    pub purple: UiColorPalette,
    /// Light purple for gentle emphasis and calm interfaces
    pub violet: UiColorPalette,
    
    // === Blue Family Palettes ===
    /// Soft blue-purple for calm, trustworthy interfaces
    pub iris: UiColorPalette,
    /// Deep blue for professional, corporate applications
    pub indigo: UiColorPalette,
    /// Primary blue for interactive elements and primary actions
    pub blue: UiColorPalette,
    /// Bright blue-green for tech, modern, and dynamic designs
    pub cyan: UiColorPalette,
    /// Light blue for airy, spacious, and open interfaces
    pub sky: UiColorPalette,
    
    // === Green Family Palettes ===
    /// Blue-green for balance, harmony, and professional interfaces
    pub teal: UiColorPalette,
    /// Pure green for nature, growth, and environmental themes
    pub jade: UiColorPalette,
    /// Success green for positive actions and confirmation states
    pub green: UiColorPalette,
    /// Natural green for organic, fresh, and environmental designs
    pub grass: UiColorPalette,
    /// Light green for fresh, clean, and minimalist interfaces
    pub mint: UiColorPalette,
    /// Bright green for energy, vitality, and attention-grabbing elements
    pub lime: UiColorPalette,
    
    // === Warm Color Palettes ===
    /// Metallic brown for premium, luxury, and sophisticated designs
    pub bronze: UiColorPalette,
    /// Earthy brown for natural, warm, and comfortable interfaces
    pub brown: UiColorPalette,
    /// Luxury metallic for premium, high-value, and exclusive content
    pub gold: UiColorPalette,
    /// Bright yellow for attention, energy, and optimistic interfaces
    pub yellow: UiColorPalette,
    /// Warm orange-yellow for warning states and caution indicators
    pub amber: UiColorPalette,
    /// Vibrant orange for call-to-action and energetic elements
    pub orange: UiColorPalette,
}
/// Enumeration of all available color palette names for programmatic access.
/// 
/// This enum provides a way to reference color palettes by name, useful for
/// configuration systems, user preferences, or dynamic theme switching.
/// Each variant corresponds to a palette in UiColorPalettes.
/// 
/// # Usage
/// ```rust
/// use forge_ui::UiColorPalettesName;
/// 
/// let palette_name = UiColorPalettesName::Blue;
/// let theme_color = match palette_name {
///     UiColorPalettesName::Blue => theme().blue,
///     UiColorPalettesName::Red => theme().red,
///     // ... other variants
/// };
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UiColorPalettesName {
    /// Primary neutral gray palette
    Gray,
    /// Warm neutral with purple undertones
    Mauve,
    /// Cool neutral with blue undertones
    Slate,
    /// Natural neutral with green undertones
    Sage,
    /// Earthy neutral with olive undertones
    Olive,
    /// Warm neutral with beige undertones
    Sand,
    /// Vibrant red for notifications
    Tomato,
    /// Classic red for errors
    Red,
    /// Deep red for luxury
    Ruby,
    /// Bold red for emphasis
    Crimson,
    /// Playful pink for creativity
    Pink,
    /// Rich purple-pink for sophistication
    Plum,
    /// Classic purple for creativity
    Purple,
    /// Light purple for calm interfaces
    Violet,
    /// Soft blue-purple for trust
    Iris,
    /// Deep blue for professional use
    Indigo,
    /// Primary blue for interactions
    Blue,
    /// Bright blue-green for modern designs
    Cyan,
    /// Blue-green for balance
    Teal,
    /// Pure green for nature
    Jade,
    /// Success green for positive actions
    Green,
    /// Natural green for organic designs
    Grass,
    /// Metallic brown for premium feels
    Bronze,
    /// Earthy brown for warmth
    Brown,
    /// Luxury metallic for exclusivity
    Gold,
    /// Light blue for spacious feels
    Sky,
    /// Light green for freshness
    Mint,
    /// Bright green for energy
    Lime,
    /// Bright yellow for attention
    Yellow,
    /// Warm orange-yellow for warnings
    Amber,
    /// Vibrant orange for call-to-action
    Orange,
}
/// Resource wrapper for the complete color palette collection.
/// 
/// This newtype wrapper allows UiColorPalettes to be used as a Bevy resource,
/// providing global access to all color palettes throughout the application.
/// The palettes are typically loaded during app initialization and remain
/// available for the entire application lifecycle.
/// 
/// # Usage as Bevy Resource
/// ```rust
/// // In your app setup
/// app.insert_resource(UiColorPalettesData(load_color_palettes()));
/// 
/// // In systems
/// fn my_system(palettes: Res<UiColorPalettesData>) {
///     let blue_palette = &palettes.0.blue;
///     // Use the palette...
/// }
/// ```
pub struct UiColorPalettesData(pub UiColorPalettes);

/// Complete color palette following Radix UI's 12-step color scale.
/// 
/// UiColorPalette represents a single color family (e.g., blue, red, green)
/// with all semantic color roles needed for comprehensive UI theming.
/// Each palette includes both opaque and alpha (semi-transparent) variants
/// for maximum flexibility in layering and composition.
/// 
/// # Color Roles
/// 
/// ## Background Colors
/// - `base`: Main application background
/// - `bg_subtle`: Subtle background for disabled states
/// - `bg`: Normal UI element background
/// - `bg_hover`: Hovered UI element background
/// - `bg_active`: Active/selected UI element background
/// 
/// ## Border Colors
/// - `line`: Subtle borders and separators
/// - `border`: Normal UI element borders and focus rings
/// - `border_hover`: Hovered UI element borders
/// 
/// ## Accent Colors
/// - `solid`: Solid, high-contrast backgrounds
/// - `solid_hover`: Hovered solid backgrounds
/// 
/// ## Text Colors
/// - `text`: Low-contrast text (secondary content)
/// - `text_contrast`: High-contrast text (primary content)
/// 
/// ## Special Colors
/// - `high_contrast`: Maximum contrast for accessibility
/// - `surface`: Surface-level background
/// - `indicator`: Small indicator elements (dots, badges)
/// - `track`: Track elements (sliders, progress bars)
/// 
/// # Alpha Variants
/// 
/// Every color has an alpha variant (suffix `_a`) that provides the same
/// semantic meaning but with transparency for layering effects.
/// 
/// # Example Usage
/// ```rust
/// let blue = theme().blue;
/// 
/// // Button styling
/// let button_bg = blue.bg;           // Normal state
/// let button_hover = blue.bg_hover;  // Hover state
/// let button_border = blue.border;   // Border
/// let button_text = blue.text;       // Text color
/// 
/// // Overlay styling
/// let modal_bg = blue.bg_a;          // Semi-transparent background
/// let overlay_border = blue.border_a; // Semi-transparent border
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct UiColorPalette {
    // === Opaque Background Colors ===
    /// Step 1: App background - The main application background color
    pub base: Color,
    /// Step 2: Subtle background - For subtle backgrounds and disabled states
    pub bg_subtle: Color,
    /// Step 3: UI element background - Normal background for UI components
    pub bg: Color,
    /// Step 4: Hovered UI element background - Background when hovering
    pub bg_hover: Color,
    /// Step 5: Active/selected UI element background - For active or selected states
    pub bg_active: Color,
    
    // === Opaque Border Colors ===
    /// Step 6: Subtle borders and separators - For fine lines and subtle divisions
    pub line: Color,
    /// Step 7: UI element border and focus rings - Standard border color
    pub border: Color,
    /// Step 8: Hovered UI element border - Border color when hovering
    pub border_hover: Color,
    
    // === Opaque Accent Colors ===
    /// Step 9: Solid backgrounds - High-contrast, solid background color
    pub solid: Color,
    /// Step 10: Hovered solid backgrounds - Solid background when hovering
    pub solid_hover: Color,
    
    // === Opaque Text Colors ===
    /// Step 11: Low-contrast text - For secondary text and descriptions
    pub text: Color,
    /// Step 12: High-contrast text - For primary text and important content
    pub text_contrast: Color,
    
    // === Alpha (Semi-transparent) Variants ===
    /// Alpha variant of base - Semi-transparent app background
    pub base_a: Color,
    /// Alpha variant of bg_subtle - Semi-transparent subtle background
    pub bg_subtle_a: Color,
    /// Alpha variant of bg - Semi-transparent UI element background
    pub bg_a: Color,
    /// Alpha variant of bg_hover - Semi-transparent hovered background
    pub bg_hover_a: Color,
    /// Alpha variant of bg_active - Semi-transparent active background
    pub bg_active_a: Color,
    /// Alpha variant of line - Semi-transparent subtle borders
    pub line_a: Color,
    /// Alpha variant of border - Semi-transparent borders
    pub border_a: Color,
    /// Alpha variant of border_hover - Semi-transparent hovered borders
    pub border_hover_a: Color,
    /// Alpha variant of solid - Semi-transparent solid backgrounds
    pub solid_a: Color,
    /// Alpha variant of solid_hover - Semi-transparent hovered solid backgrounds
    pub solid_hover_a: Color,
    /// Alpha variant of text - Semi-transparent text
    pub text_a: Color,
    /// Alpha variant of text_contrast - Semi-transparent high-contrast text
    pub text_contrast_a: Color,

    // === Special Purpose Colors ===
    /// Maximum contrast color for accessibility and emphasis
    pub high_contrast: Color,
    /// Surface-level background color for elevated elements
    pub surface: Color,
    /// Small indicator elements like dots, badges, and status markers
    pub indicator: Color,
    /// Track elements for sliders, progress bars, and similar components
    pub track: Color,
}
/// Basic color palette for fundamental colors (white, black).
/// 
/// UiColorPaletteBasic provides a simplified color palette structure
/// for colors that don't need the full 12-step scale or alpha variants.
/// This is typically used for pure white and pure black palettes
/// where the semantic roles are more straightforward.
/// 
/// # Use Cases
/// - Pure white palette for high contrast themes
/// - Pure black palette for dark themes or maximum contrast
/// - Special palettes where alpha variants aren't needed
/// 
/// # Comparison with UiColorPalette
/// Unlike the full UiColorPalette, this basic version:
/// - Has no alpha variants (all colors are opaque)
/// - Has no special purpose colors (high_contrast, surface, etc.)
/// - Focuses on core semantic roles only
#[derive(Debug, Clone, PartialEq)]
pub struct UiColorPaletteBasic {
    /// App background - The main application background color
    pub base: Color,
    /// Subtle background - For subtle backgrounds and disabled states  
    pub bg_subtle: Color,
    /// UI element background - Normal background for UI components
    pub bg: Color,
    /// Hovered UI element background - Background when hovering
    pub bg_hover: Color,
    /// Active/selected UI element background - For active or selected states
    pub bg_active: Color,
    /// Subtle borders and separators - For fine lines and subtle divisions
    pub line: Color,
    /// UI element border and focus rings - Standard border color
    pub border: Color,
    /// Hovered UI element border - Border color when hovering
    pub border_hover: Color,
    /// Solid backgrounds - High-contrast, solid background color
    pub solid: Color,
    /// Hovered solid backgrounds - Solid background when hovering
    pub solid_hover: Color,
    /// Low-contrast text - For secondary text and descriptions
    pub text: Color,
    /// High-contrast text - For primary text and important content
    pub text_contrast: Color,
}

/// Text contrast levels for accessibility compliance and visual hierarchy.
/// 
/// TextContrastLevel defines different levels of text contrast for creating
/// proper visual hierarchy while maintaining accessibility standards.
/// Each level corresponds to specific WCAG (Web Content Accessibility Guidelines)
/// requirements and use cases.
/// 
/// # Accessibility Standards
/// 
/// The contrast levels are designed to meet WCAG requirements:
/// - **WCAG AA**: Minimum accessibility standard (4.5:1 contrast ratio)
/// - **WCAG AAA**: Enhanced accessibility standard (7:1 contrast ratio)
/// 
/// # Usage Guidelines
/// 
/// - **Low**: Use sparingly for truly secondary information
/// - **Medium**: Standard text that needs to be readable but not prominent
/// - **High**: Important text that must be easily readable (meets WCAG AA)
/// - **Accessible**: Critical text requiring maximum accessibility (meets WCAG AAA)
/// 
/// # Example Usage
/// ```rust
/// use forge_ui::TextContrastLevel;
/// 
/// // Different text hierarchy levels
/// let title_contrast = TextContrastLevel::Accessible;  // Maximum visibility
/// let body_contrast = TextContrastLevel::High;         // Standard readability
/// let caption_contrast = TextContrastLevel::Medium;    // Secondary information
/// let disabled_contrast = TextContrastLevel::Low;      // Minimal visibility
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextContrastLevel {
    /// Low contrast for secondary, less important text.
    /// 
    /// **Use cases**: Helper text, placeholders, disabled text
    /// **Accessibility**: Below WCAG AA standards (use carefully)
    /// **Visual weight**: Minimal, de-emphasized
    Low,
    
    /// Medium contrast for regular, readable text.
    /// 
    /// **Use cases**: Body text, descriptions, secondary headings
    /// **Accessibility**: Approaches WCAG AA standards
    /// **Visual weight**: Normal, readable but not prominent
    Medium,
    
    /// High contrast for important, easily readable text.
    /// 
    /// **Use cases**: Primary headings, important body text, navigation
    /// **Accessibility**: Meets WCAG AA standards (≥4.5:1 contrast ratio)
    /// **Visual weight**: Strong, clearly visible
    High,
    
    /// Maximum contrast for critical accessibility and prominence.
    /// 
    /// **Use cases**: Critical information, error messages, primary CTAs
    /// **Accessibility**: Meets WCAG AAA standards (≥7:1 contrast ratio)
    /// **Visual weight**: Maximum, highest priority content
    Accessible,
}
