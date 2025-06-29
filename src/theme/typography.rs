//! Typography System for Forge UI
//!
//! This module provides a comprehensive typography system for Bevy applications,
//! implementing a complete type scale, font management, and semantic text variants
//! based on modern web typography practices. The system offers responsive sizing,
//! semantic meaning, and excellent accessibility support.
//!
//! ## Key Features
//!
//! - **Semantic Text Variants**: Display, Title, Body, Label, and Caption with semantic meaning
//! - **Responsive Type Scale**: 13 size levels from Xs to X9l for comprehensive hierarchy
//! - **Multi-Font Family Support**: Sans, Serif, and Mono with proper weight variants
//! - **Smart Font Loading**: Efficient asset management with fallback handling
//! - **Theme Integration**: Seamless integration with color themes and spacing systems
//! - **Accessibility Focused**: WCAG-compliant sizing and contrast considerations
//!
//! ## Typography Philosophy
//!
//! The typography system follows modern design principles:
//! - **Semantic Structure**: Text variants carry semantic meaning, not just visual styling
//! - **Consistent Hierarchy**: Mathematical type scale ensures visual consistency
//! - **Readability First**: All sizing optimized for readability and accessibility
//! - **Performance Oriented**: Efficient font loading with smart asset management
//! - **Flexible System**: Easy customization while maintaining design consistency
//!
//! ## Text Variants and Semantic Meaning
//!
//! ### Display (X5l, Bold)
//! - **Purpose**: Large, attention-grabbing headlines
//! - **Use cases**: Hero sections, landing page titles, major announcements
//! - **Characteristics**: Maximum impact, strong visual hierarchy
//! - **Accessibility**: High contrast, clear readability at distance
//!
//! ### Title (X2l, Medium)
//! - **Purpose**: Section headers, page titles, important headings
//! - **Use cases**: Card titles, section dividers, navigation headers
//! - **Characteristics**: Clear hierarchy, professional appearance
//! - **Accessibility**: Optimal for screen readers and navigation
//!
//! ### Body (Base, Regular)
//! - **Purpose**: Primary content text, paragraphs, general information
//! - **Use cases**: Article content, descriptions, user-generated content
//! - **Characteristics**: Optimized for extended reading, neutral weight
//! - **Accessibility**: Maximum readability for long-form content
//!
//! ### Label (Sm, Medium)
//! - **Purpose**: Form labels, UI element labels, interactive text
//! - **Use cases**: Button text, input labels, navigation items
//! - **Characteristics**: Clear identification, slightly emphasized
//! - **Accessibility**: Clear association with interactive elements
//!
//! ### Caption (Xs, Regular)
//! - **Purpose**: Secondary information, metadata, supporting details
//! - **Use cases**: Image captions, footnotes, timestamps, help text
//! - **Characteristics**: Subtle, non-intrusive, supplementary
//! - **Accessibility**: Still readable while clearly secondary
//!
//! ## Type Scale System
//!
//! The type scale follows a mathematical progression optimized for digital interfaces:
//!
//! | Size | Multiplier | Pixels (16px base) | Use Cases |
//! |------|------------|-------------------|-----------|
//! | **Xs** | 0.75× | 12px | Captions, footnotes, metadata |
//! | **Sm** | 0.875× | 14px | Labels, small UI text |
//! | **Base** | 1.0× | 16px | Body text, primary content |
//! | **Lg** | 1.125× | 18px | Emphasized body text |
//! | **Xl** | 1.25× | 20px | Subheadings, lead text |
//! | **X2l** | 1.5× | 24px | Titles, section headers |
//! | **X3l** | 1.875× | 30px | Large headings |
//! | **X4l** | 2.25× | 36px | Major headings |
//! | **X5l** | 3.0× | 48px | Display text, hero titles |
//! | **X6l** | 3.75× | 60px | Extra large displays |
//! | **X7l** | 4.5× | 72px | Massive displays |
//! | **X8l** | 6.0× | 96px | Ultra large displays |
//! | **X9l** | 8.0× | 128px | Maximum impact displays |
//!
//! ## Font Families and Usage
//!
//! ### Sans (Roboto) - Default
//! - **Character**: Clean, modern, highly readable
//! - **Use cases**: UI text, body content, headings, general purpose
//! - **Weights**: Light, Regular, Medium, Bold
//! - **Accessibility**: Excellent screen readability, dyslexia-friendly
//!
//! ### Serif (Noto Serif)
//! - **Character**: Traditional, elegant, authoritative
//! - **Use cases**: Long-form content, formal documents, editorial content
//! - **Weights**: Regular, Bold
//! - **Accessibility**: Good for extended reading, traditional feel
//!
//! ### Mono (Roboto Mono)
//! - **Character**: Fixed-width, technical, code-oriented
//! - **Use cases**: Code blocks, technical content, data display
//! - **Weights**: Regular, Bold
//! - **Accessibility**: Clear character distinction, technical clarity
//!
//! ## Usage Examples
//!
//! ### Basic Text Creation
//! ```rust
//! use forge_ui::{Text, TextVariant, TextSize, TextWeight, FontFamily};
//!
//! // Semantic text variants with default sizing and weights
//! let display_text = Text::display("Welcome to Forge UI");
//! let title_text = Text::title("Getting Started");
//! let body_text = Text::body("This is the main content of your application.");
//! let label_text = Text::label("Username:");
//! let caption_text = Text::caption("Last updated 5 minutes ago");
//! ```
//!
//! ### Custom Typography Configuration
//! ```rust
//! // Override default variant settings
//! let custom_title = Text::title("Custom Title")
//!     .size(TextSize::X3l)              // Larger than default
//!     .weight(TextWeight::Bold)         // Bolder than default
//!     .family(FontFamily::Serif)        // Different font family
//!     .build();
//!
//! // Build from scratch with full control
//! let custom_text = Text::new("Custom Text")
//!     .variant(TextVariant::Body)
//!     .size(TextSize::Lg)
//!     .weight(TextWeight::Medium)
//!     .family(FontFamily::Mono)
//!     .build();
//! ```
//!
//! ### Responsive Typography
//! ```rust
//! // Typography that adapts to different contexts
//! let responsive_heading = Text::new("Responsive Heading")
//!     .variant(TextVariant::Title)
//!     .size(TextSize::X2l)              // Default for desktop
//!     // Size can be adjusted based on screen size in your systems
//!     .build();
//! ```
//!
//! ### Accessibility-Focused Typography
//! ```rust
//! // High contrast, readable text for accessibility
//! let accessible_text = Text::body("Important accessibility information")
//!     .weight(TextWeight::Medium)       // Slightly heavier for better contrast
//!     .size(TextSize::Lg)              // Larger for better readability
//!     .color(theme().text_contrast)     // High contrast color
//!     .build();
//! ```
//!
//! ## Advanced Typography Patterns
//!
//! ### Hierarchical Content Structure
//! ```rust
//! // Create clear content hierarchy
//! commands.spawn(Section::size_3("article").build())
//!     .with_children(|parent| {
//!         // Main headline
//!         parent.spawn(Text::display("Article Title").build());
//!
//!         // Subtitle
//!         parent.spawn(Text::title("Exploring Modern UI Design").build());
//!
//!         // Lead paragraph
//!         parent.spawn(Text::body("This article explores...")
//!             .size(TextSize::Lg)
//!             .weight(TextWeight::Medium)
//!             .build());
//!
//!         // Body content
//!         parent.spawn(Text::body("The main content continues...").build());
//!
//!         // Caption
//!         parent.spawn(Text::caption("Published on March 15, 2024").build());
//!     });
//! ```
//!
//! ### Code and Technical Content
//! ```rust
//! // Display code with appropriate typography
//! let code_block = Text::new("fn main() { println!(\"Hello, world!\"); }")
//!     .family(FontFamily::Mono)
//!     .size(TextSize::Sm)
//!     .background_color(theme().bg_subtle)
//!     .padding(16.0)
//!     .build();
//! ```
//!
//! ## Font Loading and Performance
//!
//! The typography system uses efficient font loading:
//!
//! ### Asset Management
//! - Fonts are loaded once at startup and cached
//! - Smart fallback handling ensures text always renders
//! - Memory-efficient asset sharing across components
//! - Support for hot reloading during development
//!
//! ### Performance Considerations
//! - Font assets are loaded asynchronously
//! - Text rendering uses Bevy's optimized text systems
//! - Minimal overhead for font variant switching
//! - Efficient glyph caching and reuse
//!
//! ## Accessibility Guidelines
//!
//! ### Size and Contrast
//! - Minimum 12px font size for all text (meets WCAG guidelines)
//! - High contrast colors for all text variants
//! - Scalable sizing system supports user preferences
//! - Clear visual hierarchy aids navigation
//!
//! ### Semantic Structure
//! - Text variants provide semantic meaning for screen readers
//! - Proper heading hierarchy for document structure
//! - Clear labels and descriptions for interactive elements
//! - Consistent typography patterns aid comprehension
//!
//! ### Responsive Design
//! - Typography scales appropriately for different screen sizes
//! - Touch-friendly sizing for interactive text elements
//! - Flexible line spacing and character spacing
//! - Adaptable layouts preserve readability
//!
//! ## Customization and Theming
//!
//! ### Global Typography Settings
//! ```rust
//! // Customize base font size and scaling
//! const CUSTOM_FONT_SIZE_BASE: f32 = 18.0;  // Larger base size
//! const CUSTOM_SCALING: f32 = 1.2;          // More dramatic scale
//! ```
//!
//! ### Font Family Customization
//! ```rust
//! // Add custom fonts to FontAssets
//! let custom_fonts = FontAssets {
//!     sans_regular: asset_server.load("fonts/CustomSans-Regular.ttf"),
//!     // ... other font variants
//! };
//! ```
//!
//! ## Integration with Theme System
//!
//! Typography integrates seamlessly with the color and spacing systems:
//! - Text colors automatically adapt to light/dark themes
//! - Spacing follows the same scale as typography
//! - Component integration maintains design consistency
//! - Theme switching preserves typography relationships

use crate::plugin::{FONT_SIZE_BASE, SCALING};
use bevy::prelude::*;

/// Text variant that defines semantic meaning and establishes default styling for different text purposes.
///
/// TextVariant provides semantic categorization for text content, similar to HTML semantic elements.
/// Each variant has carefully chosen default size, weight, and usage patterns optimized for its
/// specific communication purpose and accessibility requirements.
///
/// # Design Philosophy
///
/// Text variants follow semantic meaning over visual styling:
/// - Each variant represents a specific content purpose
/// - Default styling is optimized for that purpose
/// - Visual hierarchy is established through semantic importance
/// - Accessibility is built into the semantic structure
///
/// # Variant Selection Guidelines
///
/// Choose variants based on content purpose, not desired appearance:
/// - **Display**: For maximum visual impact and attention-getting
/// - **Title**: For section organization and content hierarchy
/// - **Body**: For primary readable content and information
/// - **Label**: For UI element identification and interaction
/// - **Caption**: For supplementary and supporting information
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextVariant {
    /// Primary readable content with optimal settings for extended reading.
    ///
    /// **Default Settings**: Base size (16px), Regular weight, Sans family
    /// **Purpose**: Main content text, paragraphs, articles, descriptions
    /// **Characteristics**: Neutral, readable, optimized for comprehension
    /// **Accessibility**: Maximum readability for long-form content
    #[default]
    Body,

    /// Secondary information and metadata with subtle, non-intrusive styling.
    ///
    /// **Default Settings**: Xs size (12px), Regular weight, Sans family
    /// **Purpose**: Image captions, footnotes, timestamps, help text
    /// **Characteristics**: Small but readable, clearly secondary
    /// **Accessibility**: Still meets minimum size requirements while being subtle
    Caption,

    /// UI element labels and interactive text with clear identification.
    ///
    /// **Default Settings**: Sm size (14px), Medium weight, Sans family
    /// **Purpose**: Button text, form labels, navigation items, UI controls
    /// **Characteristics**: Clear, slightly emphasized, action-oriented
    /// **Accessibility**: Clearly associates with interactive elements
    Label,

    /// Large, attention-grabbing headlines for maximum visual impact.
    ///
    /// **Default Settings**: X5l size (48px), Bold weight, Sans family
    /// **Purpose**: Hero sections, landing pages, major announcements
    /// **Characteristics**: Maximum impact, strong visual hierarchy
    /// **Accessibility**: High contrast, readable at distance
    Display,

    /// Section headers and page titles for content organization.
    ///
    /// **Default Settings**: X2l size (24px), Medium weight, Sans family
    /// **Purpose**: Card titles, section dividers, page headers
    /// **Characteristics**: Clear hierarchy, professional appearance
    /// **Accessibility**: Optimal for screen readers and document structure
    Title,
}

/// Text size variants implementing a mathematical type scale for responsive typography.
///
/// TextSize provides a comprehensive sizing system based on a mathematical progression
/// that ensures consistent visual hierarchy and optimal readability across different
/// screen sizes and use cases. The scale follows modern typography best practices
/// with careful attention to accessibility requirements.
///
/// # Scale Philosophy
///
/// The type scale is designed around:
/// - **Mathematical consistency**: Proportional relationships between sizes
/// - **Readability optimization**: Each size tested for optimal reading experience
/// - **Accessibility compliance**: All sizes meet WCAG minimum requirements
/// - **Responsive flexibility**: Works across different screen sizes and contexts
/// - **Semantic alignment**: Sizes align with content importance and hierarchy
///
/// # Usage Guidelines
///
/// - **Xs-Sm**: Supporting content, captions, metadata
/// - **Base-Lg**: Primary content, body text, standard UI text
/// - **Xl-X2l**: Headings, titles, emphasized content
/// - **X3l-X5l**: Major headings, display text, hero content
/// - **X6l-X9l**: Massive displays, special occasions, maximum impact
///
/// # Responsive Considerations
///
/// Larger sizes (X6l+) should be used carefully on mobile devices.
/// Consider reducing by 1-2 levels for optimal mobile experience.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextSize {
    /// Extra small text (12px) for captions and metadata.
    ///
    /// **Use cases**: Image captions, footnotes, copyright text, timestamps
    /// **Accessibility**: Minimum readable size, still meets WCAG guidelines
    /// **Responsive**: Works well on all screen sizes
    Xs,

    /// Small text (14px) for labels and secondary UI elements.
    ///
    /// **Use cases**: Form labels, button text, navigation items, helper text
    /// **Accessibility**: Comfortable reading size for UI elements
    /// **Responsive**: Ideal for mobile UI components
    Sm,

    /// Base text size (16px) - the foundation of the type scale.
    ///
    /// **Use cases**: Body text, paragraphs, primary content, default text
    /// **Accessibility**: Optimal size for extended reading and comprehension
    /// **Responsive**: Works perfectly across all devices and contexts
    #[default]
    Base,

    /// Large text (18px) for emphasized body content.
    ///
    /// **Use cases**: Lead paragraphs, emphasized content, large UI text
    /// **Accessibility**: Enhanced readability for important content
    /// **Responsive**: Excellent for both desktop and mobile
    Lg,

    /// Extra large text (20px) for subheadings and prominent content.
    ///
    /// **Use cases**: Subheadings, callouts, prominent UI elements
    /// **Accessibility**: Clear hierarchy marker, easy to scan
    /// **Responsive**: Good for desktop, consider Lg on mobile
    Xl,

    /// Double extra large (24px) for titles and section headers.
    ///
    /// **Use cases**: Section titles, card headers, page titles
    /// **Accessibility**: Clear content organization for screen readers
    /// **Responsive**: May need adjustment on smaller screens
    X2l,

    /// Triple extra large (30px) for major headings.
    ///
    /// **Use cases**: Page headers, major section dividers
    /// **Accessibility**: Strong visual hierarchy, clear navigation landmarks
    /// **Responsive**: Consider X2l or Xl on mobile devices
    X3l,

    /// Quadruple extra large (36px) for prominent headings.
    ///
    /// **Use cases**: Major headings, feature announcements
    /// **Accessibility**: High-impact content, clear importance signaling
    /// **Responsive**: Reduce on mobile for better space utilization
    X4l,

    /// Quintuple extra large (48px) for display text and hero content.
    ///
    /// **Use cases**: Hero sections, landing page titles, display headlines
    /// **Accessibility**: Maximum impact, readable at distance
    /// **Responsive**: Definitely reduce on mobile (consider X3l or X4l)
    X5l,

    /// Sextuple extra large (60px) for extra large displays.
    ///
    /// **Use cases**: Special announcements, marketing headers
    /// **Accessibility**: Very high impact, use sparingly
    /// **Responsive**: Mobile: use X4l or smaller for practical layouts
    X6l,

    /// Septuple extra large (72px) for massive displays.
    ///
    /// **Use cases**: Event announcements, special occasions
    /// **Accessibility**: Extreme impact, ensure sufficient contrast
    /// **Responsive**: Mobile: use X3l or X4l for usability
    X7l,

    /// Octuple extra large (96px) for ultra large displays.
    ///
    /// **Use cases**: Conference displays, large presentations
    /// **Accessibility**: Maximum visibility, use with high contrast
    /// **Responsive**: Desktop only - use much smaller sizes on mobile
    X8l,

    /// Nonuple extra large (128px) for maximum impact displays.
    ///
    /// **Use cases**: Large venue displays, maximum impact situations
    /// **Accessibility**: Extreme visibility, ensure accessibility compliance
    /// **Responsive**: Large desktop only - significantly reduce on all other devices
    X9l,
}

/// Text weight variants providing semantic font weight control with accessibility considerations.
///
/// TextWeight offers carefully selected font weight options that balance visual hierarchy
/// with readability and accessibility. Each weight is chosen for optimal rendering
/// across different screen types and reading conditions.
///
/// # Weight Selection Criteria
///
/// Each weight is evaluated based on:
/// - **Readability**: Clarity at different sizes and screen types
/// - **Contrast**: Sufficient contrast against backgrounds
/// - **Hierarchy**: Clear differentiation between weight levels
/// - **Accessibility**: Compliance with visual accessibility standards
/// - **Performance**: Efficient rendering and font loading
///
/// # Usage Guidelines
///
/// - **Light**: Use sparingly, only for large text with good contrast
/// - **Regular**: Primary choice for body text and general content
/// - **Medium**: Emphasis without aggressive boldness, ideal for labels
/// - **Bold**: Strong emphasis, headings, important information
///
/// # Accessibility Considerations
///
/// Light weights should be used carefully with sufficient contrast ratios.
/// Bold weights provide excellent accessibility for emphasis and hierarchy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextWeight {
    /// Light font weight for subtle, refined text styling.
    ///
    /// **Use cases**: Large headings with good contrast, design accents
    /// **Accessibility**: Use only with high contrast and larger sizes
    /// **Best practices**: Ensure sufficient contrast ratio, avoid on small text
    /// **Availability**: Sans family only (Roboto Light)
    Light,

    /// Regular font weight - the foundation for readable text.
    ///
    /// **Use cases**: Body text, paragraphs, general content, default text
    /// **Accessibility**: Optimal readability for extended reading
    /// **Best practices**: Primary choice for most text content
    /// **Availability**: All font families (Sans, Serif, Mono)
    #[default]
    Regular,

    /// Medium font weight for gentle emphasis and UI elements.
    ///
    /// **Use cases**: Labels, subheadings, emphasized text, UI controls
    /// **Accessibility**: Good balance of emphasis and readability
    /// **Best practices**: Ideal for labels and moderate emphasis
    /// **Availability**: Sans family only (Roboto Medium)
    Medium,

    /// Bold font weight for strong emphasis and clear hierarchy.
    ///
    /// **Use cases**: Headings, important information, strong emphasis
    /// **Accessibility**: Excellent for creating clear visual hierarchy
    /// **Best practices**: Use for headings and important information
    /// **Availability**: All font families (Sans, Serif, Mono)
    Bold,
}

/// Font family variants providing different typographic personalities and use cases.
///
/// FontFamily offers three carefully selected font families that cover all major
/// typography needs in modern applications. Each family is chosen for its readability,
/// accessibility, and broad device compatibility.
///
/// # Font Selection Criteria
///
/// Each font family is evaluated based on:
/// - **Readability**: Clarity across different sizes and screen types
/// - **Accessibility**: Support for users with reading difficulties
/// - **Performance**: Efficient loading and rendering
/// - **Compatibility**: Wide device and browser support
/// - **Character Support**: Comprehensive Unicode and international support
///
/// # Design Philosophy
///
/// - **Sans**: Clean, modern, universal - the primary choice for UI and content
/// - **Serif**: Traditional, authoritative, readable - for formal and long-form content
/// - **Mono**: Technical, precise, code-oriented - for data and technical content
///
/// # Accessibility Features
///
/// All font families are chosen for:
/// - Clear character distinction (important for dyslexia)
/// - Good contrast and readability
/// - Support for screen readers
/// - Compliance with accessibility guidelines
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FontFamily {
    /// Sans-serif font family (Roboto) - clean, modern, highly readable.
    ///
    /// **Character**: Clean, modern, friendly, highly legible
    /// **Use cases**: UI text, body content, headings, general purpose text
    /// **Weights**: Light, Regular, Medium, Bold
    /// **Accessibility**: Excellent screen readability, dyslexia-friendly design
    /// **Performance**: Optimized for screen rendering, good hinting
    /// **Best for**: Default choice for most text content and UI elements
    #[default]
    Sans,

    /// Serif font family (Noto Serif) - traditional, elegant, authoritative.
    ///
    /// **Character**: Traditional, elegant, authoritative, formal
    /// **Use cases**: Long-form content, articles, formal documents, editorial content
    /// **Weights**: Regular, Bold (Medium/Light fallback to Regular)
    /// **Accessibility**: Good for extended reading, traditional readability
    /// **Performance**: Optimized for text rendering, good character spacing
    /// **Best for**: Formal content, articles, academic text, printed-style content
    Serif,

    /// Monospace font family (Roboto Mono) - fixed-width, technical, code-oriented.
    ///
    /// **Character**: Fixed-width, technical, precise, code-friendly
    /// **Use cases**: Code blocks, technical content, data display, terminal-style text
    /// **Weights**: Regular, Bold (Medium/Light fallback to Regular)
    /// **Accessibility**: Clear character distinction, excellent for code
    /// **Performance**: Optimized for code display, consistent character width
    /// **Best for**: Code, data tables, technical documentation, monospace layouts
    Mono,

    InterfaceFont,
    GenericController,
    KeyboardMouse,
    NintendoSwitch,
    NintendoSwitch2,
    PlayStation,
    Xbox,
    SteamController,
    SteamDeck,
    Gamecube,
    Wii,
    WiiU,
    Touch,
    Indicators,
}
/// Font size configuration structure implementing the complete type scale system.
///
/// FontSize provides the concrete pixel values for all text sizes in the typography system.
/// The structure implements a mathematical progression that ensures consistent visual hierarchy
/// and optimal readability across different use cases and screen sizes.
///
/// # Scale Mathematics
///
/// The scale uses base multipliers to create proportional relationships:
/// - Base size: 16px (optimal for screen reading)
/// - Scaling factor: Applied globally for responsive adjustments
/// - Mathematical progression: Each level increases proportionally
///
/// # Configuration Philosophy
///
/// - **Accessibility first**: All sizes meet WCAG minimum requirements
/// - **Screen optimized**: Values tested across different screen densities
/// - **Scalable design**: Global scaling factor supports user preferences
/// - **Performance aware**: Efficient calculation and caching
///
/// # Customization
///
/// Modify `FONT_SIZE_BASE` and `SCALING` constants in the plugin to adjust
/// the entire type scale proportionally while maintaining relationships.
#[derive(Debug, Clone)]
pub struct FontSize {
    /// Extra small size (12px default) for captions and metadata
    pub xs: f32,
    /// Small size (14px default) for labels and UI elements
    pub sm: f32,
    /// Base size (16px default) - foundation of the type scale
    pub base: f32,
    /// Large size (18px default) for emphasized body text
    pub lg: f32,
    /// Extra large size (20px default) for subheadings
    pub xl: f32,
    /// Double extra large (24px default) for titles
    pub x2l: f32,
    /// Triple extra large (30px default) for major headings
    pub x3l: f32,
    /// Quadruple extra large (36px default) for prominent headings
    pub x4l: f32,
    /// Quintuple extra large (48px default) for display text
    pub x5l: f32,
    /// Sextuple extra large (60px default) for large displays
    pub x6l: f32,
    /// Septuple extra large (72px default) for massive displays
    pub x7l: f32,
    /// Octuple extra large (96px default) for ultra large displays
    pub x8l: f32,
    /// Nonuple extra large (128px default) for maximum impact
    pub x9l: f32,
}

impl Default for FontSize {
    /// Creates the default font size configuration with mathematically calculated values.
    ///
    /// The default implementation creates a complete type scale based on:
    /// - **Base size**: 16px (`FONT_SIZE_BASE`) - optimal for screen reading
    /// - **Global scaling**: Applied via `SCALING` constant for responsive adjustments
    /// - **Mathematical progression**: Carefully chosen multipliers for optimal hierarchy
    ///
    /// # Scale Multipliers
    ///
    /// The multipliers are chosen for:
    /// - **Visual distinction**: Each level provides clear hierarchy
    /// - **Practical usage**: Tested across real-world content scenarios
    /// - **Accessibility**: All sizes meet minimum readability requirements
    /// - **Performance**: Efficient calculation and memory usage
    ///
    /// # Responsive Behavior
    ///
    /// The global `SCALING` factor allows the entire type scale to be adjusted
    /// proportionally for different screen sizes or user preferences while
    /// maintaining the mathematical relationships between sizes.
    fn default() -> Self {
        Self {
            // 0.75× base - minimum readable size for captions
            xs: 0.75 * FONT_SIZE_BASE * SCALING,
            // 0.875× base - comfortable size for UI labels
            sm: 0.875 * FONT_SIZE_BASE * SCALING,
            // 1.0× base - foundation size optimized for body text
            base: 1.0 * FONT_SIZE_BASE * SCALING,
            // 1.125× base - subtle emphasis for lead text
            lg: 1.125 * FONT_SIZE_BASE * SCALING,
            // 1.25× base - clear step up for subheadings
            xl: 1.25 * FONT_SIZE_BASE * SCALING,
            // 1.5× base - significant increase for titles
            x2l: 1.5 * FONT_SIZE_BASE * SCALING,
            // 1.875× base - major heading size
            x3l: 1.875 * FONT_SIZE_BASE * SCALING,
            // 2.25× base - prominent heading size
            x4l: 2.25 * FONT_SIZE_BASE * SCALING,
            // 3.0× base - display text for hero content
            x5l: 3.0 * FONT_SIZE_BASE * SCALING,
            // 3.75× base - large display text
            x6l: 3.75 * FONT_SIZE_BASE * SCALING,
            // 4.5× base - massive display text
            x7l: 4.5 * FONT_SIZE_BASE * SCALING,
            // 6.0× base - ultra large display text
            x8l: 6.0 * FONT_SIZE_BASE * SCALING,
            // 8.0× base - maximum impact display text
            x9l: 8.0 * FONT_SIZE_BASE * SCALING,
        }
    }
}

/// Unified font assets resource providing efficient font loading and management.
///
/// FontAssets serves as the central resource for all font assets in the application,
/// providing direct access to font handles for the complete typography system.
/// This structure replaces older font management approaches with a simplified,
/// performance-oriented design.
///
/// # Design Principles
///
/// - **Centralized management**: Single source of truth for all font assets
/// - **Performance optimized**: Direct handle access without lookup overhead
/// - **Memory efficient**: Shared font handles prevent duplicate loading
/// - **Type safe**: Compile-time guarantees for font availability
/// - **Asset hot reloading**: Supports development-time font updates
///
/// # Font Loading Strategy
///
/// Fonts are loaded once at startup and cached for the application lifetime:
/// 1. **Startup loading**: All fonts loaded during application initialization
/// 2. **Handle caching**: Font handles stored in this resource for reuse
/// 3. **Efficient access**: Direct handle access without runtime lookups
/// 4. **Memory sharing**: Multiple components share the same font handles
///
/// # Fallback Behavior
///
/// When specific weight variants are not available:
/// - **Serif family**: Medium/Light weights fallback to Regular
/// - **Mono family**: Medium/Light weights fallback to Regular
/// - **Sans family**: All weights are available (Light, Regular, Medium, Bold)
///
/// # Font Quality and Performance
///
/// All included fonts are:
/// - **Screen optimized**: Excellent rendering at different screen densities
/// - **Unicode comprehensive**: Support for international characters
/// - **Performance tested**: Efficient loading and rendering characteristics
/// - **Accessibility friendly**: Clear character distinction and readability
#[derive(Resource, Debug, Clone)]
pub struct FontAssets {
    // === Sans-Serif Family (Roboto) ===
    // Complete weight range for maximum design flexibility
    /// Roboto Light - subtle, refined styling for large text
    pub sans_light: Handle<Font>,
    /// Roboto Regular - primary font for body text and general content
    pub sans_regular: Handle<Font>,
    /// Roboto Medium - gentle emphasis for labels and UI elements
    pub sans_medium: Handle<Font>,
    /// Roboto Bold - strong emphasis for headings and important text
    pub sans_bold: Handle<Font>,

    // === Serif Family (Noto Serif) ===
    // Traditional typography for formal and long-form content
    /// Noto Serif Regular - elegant serif for body text and articles
    pub serif_regular: Handle<Font>,
    /// Noto Serif Bold - serif emphasis for headings and strong text
    pub serif_bold: Handle<Font>,

    // === Monospace Family (Roboto Mono) ===
    // Fixed-width fonts for code and technical content
    /// Roboto Mono Regular - primary monospace for code and data
    pub mono_regular: Handle<Font>,
    /// Roboto Mono Bold - monospace emphasis for code headings
    pub mono_bold: Handle<Font>,

    // === Icons ===
    pub interface: Handle<Font>,
    pub generic_controller: Handle<Font>,
    pub keyboard_mouse: Handle<Font>,
    pub nintendo_switch: Handle<Font>,
    pub nintendo_switch_2: Handle<Font>,
    pub playstation: Handle<Font>,
    pub xbox: Handle<Font>,
    pub steam_controller: Handle<Font>,
    pub steam_deck: Handle<Font>,
    pub gamecube: Handle<Font>,
    pub wii: Handle<Font>,
    pub wii_u: Handle<Font>,
    pub touch: Handle<Font>,
    pub indicators: Handle<Font>,
}

/// Retrieves the appropriate font handle based on family and weight combination.
///
/// This function provides intelligent font selection with automatic fallback handling
/// for weight variants that are not available in all font families. It ensures
/// that text always renders with an appropriate font while maintaining the intended
/// visual hierarchy.
///
/// # Arguments
///
/// * `assets` - Reference to the FontAssets resource containing all font handles
/// * `family` - The desired font family (Sans, Serif, or Mono)
/// * `weight` - The desired font weight (Light, Regular, Medium, or Bold)
///
/// # Returns
///
/// A cloned Handle<Font> for the requested font, with automatic fallback to
/// available weights when specific combinations are not available.
///
/// # Fallback Behavior
///
/// **Sans family (Roboto)**: All weights available, no fallback needed
/// - Light → Roboto Light
/// - Regular → Roboto Regular
/// - Medium → Roboto Medium
/// - Bold → Roboto Bold
///
/// **Serif family (Noto Serif)**: Limited weights with intelligent fallback
/// - Light → Noto Serif Regular (fallback for readability)
/// - Regular → Noto Serif Regular
/// - Medium → Noto Serif Regular (fallback for consistency)
/// - Bold → Noto Serif Bold
///
/// **Mono family (Roboto Mono)**: Limited weights with intelligent fallback
/// - Light → Roboto Mono Regular (fallback for code clarity)
/// - Regular → Roboto Mono Regular
/// - Medium → Roboto Mono Regular (fallback for consistency)
/// - Bold → Roboto Mono Bold
///
/// # Performance
///
/// Font handles are cloned (which is efficient for Bevy handles) rather than
/// referenced to avoid lifetime complications in component systems. The
/// underlying font data is shared, so cloning handles has minimal overhead.
///
/// # Example Usage
/// ```rust
/// let font_assets = world.resource::<FontAssets>();
///
/// // Get a bold sans-serif font
/// let heading_font = get_font_handle(font_assets, FontFamily::Sans, TextWeight::Bold);
///
/// // Get a regular monospace font (for code)
/// let code_font = get_font_handle(font_assets, FontFamily::Mono, TextWeight::Regular);
///
/// // Get a medium serif font (falls back to regular)
/// let serif_font = get_font_handle(font_assets, FontFamily::Serif, TextWeight::Medium);
/// ```
pub fn get_font_handle(
    assets: &FontAssets,
    family: FontFamily,
    weight: TextWeight,
) -> Handle<Font> {
    match family {
        FontFamily::Sans => {
            // Sans family has all weight variants available
            match weight {
                TextWeight::Light => assets.sans_light.clone(),
                TextWeight::Regular => assets.sans_regular.clone(),
                TextWeight::Medium => assets.sans_medium.clone(),
                TextWeight::Bold => assets.sans_bold.clone(),
            }
        }
        FontFamily::Serif => {
            // Serif family has limited weights, fallback to Regular for missing variants
            match weight {
                TextWeight::Light | TextWeight::Regular | TextWeight::Medium => {
                    assets.serif_regular.clone()
                }
                TextWeight::Bold => assets.serif_bold.clone(),
            }
        }
        FontFamily::Mono => {
            // Mono family has limited weights, fallback to Regular for missing variants
            match weight {
                TextWeight::Light | TextWeight::Regular | TextWeight::Medium => {
                    assets.mono_regular.clone()
                }
                TextWeight::Bold => assets.mono_bold.clone(),
            }
        }
        FontFamily::InterfaceFont => match weight {
            TextWeight::Bold | TextWeight::Light | TextWeight::Medium | TextWeight::Regular => {
                assets.interface.clone()
            }
        },
        FontFamily::GenericController => match weight {
            TextWeight::Bold | TextWeight::Light | TextWeight::Medium | TextWeight::Regular => {
                assets.generic_controller.clone()
            }
        },
        FontFamily::KeyboardMouse => match weight {
            TextWeight::Bold | TextWeight::Light | TextWeight::Medium | TextWeight::Regular => {
                assets.keyboard_mouse.clone()
            }
        },
        FontFamily::NintendoSwitch => match weight {
            TextWeight::Bold | TextWeight::Light | TextWeight::Medium | TextWeight::Regular => {
                assets.nintendo_switch.clone()
            }
        },
        FontFamily::NintendoSwitch2 => match weight {
            TextWeight::Bold | TextWeight::Light | TextWeight::Medium | TextWeight::Regular => {
                assets.nintendo_switch_2.clone()
            }
        },
        FontFamily::PlayStation => match weight {
            TextWeight::Bold | TextWeight::Light | TextWeight::Medium | TextWeight::Regular => {
                assets.playstation.clone()
            }
        },
        FontFamily::Xbox => match weight {
            TextWeight::Bold | TextWeight::Light | TextWeight::Medium | TextWeight::Regular => {
                assets.xbox.clone()
            }
        },
        FontFamily::SteamController => match weight {
            TextWeight::Bold | TextWeight::Light | TextWeight::Medium | TextWeight::Regular => {
                assets.steam_controller.clone()
            }
        },
        FontFamily::SteamDeck => match weight {
            TextWeight::Bold | TextWeight::Light | TextWeight::Medium | TextWeight::Regular => {
                assets.steam_deck.clone()
            }
        },
        FontFamily::Gamecube => match weight {
            TextWeight::Bold | TextWeight::Light | TextWeight::Medium | TextWeight::Regular => {
                assets.gamecube.clone()
            }
        },
        FontFamily::Wii => match weight {
            TextWeight::Bold | TextWeight::Light | TextWeight::Medium | TextWeight::Regular => {
                assets.wii.clone()
            }
        },
        FontFamily::WiiU => match weight {
            TextWeight::Bold | TextWeight::Light | TextWeight::Medium | TextWeight::Regular => {
                assets.wii_u.clone()
            }
        },
        FontFamily::Touch => match weight {
            TextWeight::Bold | TextWeight::Light | TextWeight::Medium | TextWeight::Regular => {
                assets.touch.clone()
            }
        },
        FontFamily::Indicators => match weight {
            TextWeight::Bold | TextWeight::Light | TextWeight::Medium | TextWeight::Regular => {
                assets.indicators.clone()
            }
        },
    }
}

/// Retrieves the exact pixel value for a given text size from the font size configuration.
///
/// This function provides efficient access to the calculated font sizes in pixels,
/// allowing components to get precise sizing for text rendering. The function ensures
/// type safety and eliminates magic numbers throughout the codebase.
///
/// # Arguments
///
/// * `font_size` - Reference to the FontSize configuration containing all calculated sizes
/// * `size` - The TextSize variant for which to retrieve the pixel value
///
/// # Returns
///
/// The calculated font size in pixels as f32, ready for use in Bevy text components.
///
/// # Performance
///
/// This is a simple match operation with no allocations or complex calculations.
/// The pixel values are pre-calculated in the FontSize structure, making this
/// function very efficient for repeated calls.
///
/// # Usage in Components
///
/// ```rust
/// // Get the default font size configuration
/// let font_config = FontSize::default();
///
/// // Get pixel size for body text
/// let body_size = get_font_size_pixels(&font_config, TextSize::Base);
///
/// // Get pixel size for a large heading
/// let heading_size = get_font_size_pixels(&font_config, TextSize::X3l);
///
/// // Use in Bevy text style
/// let text_style = TextStyle {
///     font_size: body_size,
///     // ... other properties
/// };
/// ```
///
/// # Integration with Theme System
///
/// This function is typically used internally by text components to convert
/// semantic size tokens (TextSize) into concrete pixel values for rendering.
/// It bridges the gap between the design system and Bevy's rendering requirements.
pub fn get_font_size_pixels(font_size: &FontSize, size: TextSize) -> f32 {
    match size {
        TextSize::Xs => font_size.xs,
        TextSize::Sm => font_size.sm,
        TextSize::Base => font_size.base,
        TextSize::Lg => font_size.lg,
        TextSize::Xl => font_size.xl,
        TextSize::X2l => font_size.x2l,
        TextSize::X3l => font_size.x3l,
        TextSize::X4l => font_size.x4l,
        TextSize::X5l => font_size.x5l,
        TextSize::X6l => font_size.x6l,
        TextSize::X7l => font_size.x7l,
        TextSize::X8l => font_size.x8l,
        TextSize::X9l => font_size.x9l,
    }
}

/// Determines the effective text size by applying variant defaults or explicit overrides.
///
/// This function implements the core logic of the semantic typography system by providing
/// appropriate default sizes for each text variant while allowing explicit overrides.
/// It ensures that text variants have meaningful default sizing that can be customized
/// when needed.
///
/// # Arguments
///
/// * `variant` - The semantic text variant that determines the default size
/// * `explicit_size` - Optional explicit size override that takes precedence over defaults
///
/// # Returns
///
/// The resolved TextSize to use, either the explicit override or the variant's default.
///
/// # Variant Default Sizes
///
/// Each variant has carefully chosen default sizes based on its semantic purpose:
///
/// - **Display** → X5l (48px) - Large, attention-grabbing headlines
/// - **Title** → X2l (24px) - Section headers and page titles
/// - **Body** → Base (16px) - Primary readable content
/// - **Label** → Sm (14px) - UI labels and interactive text
/// - **Caption** → Xs (12px) - Secondary information and metadata
///
/// # Design Philosophy
///
/// The default sizes are chosen based on:
/// - **Semantic meaning**: Size reflects content importance and purpose
/// - **Readability**: Optimal sizes for each content type
/// - **Visual hierarchy**: Clear differentiation between content levels
/// - **Accessibility**: All sizes meet minimum readability requirements
///
/// # Usage Patterns
///
/// ```rust
/// // Use variant default (Body → Base size)
/// let body_size = get_effective_text_size(TextVariant::Body, None);
///
/// // Override variant default (Body → Large size for emphasis)
/// let emphasized_body = get_effective_text_size(TextVariant::Body, Some(TextSize::Lg));
///
/// // Title with custom size (Title → Extra large instead of default X2l)
/// let large_title = get_effective_text_size(TextVariant::Title, Some(TextSize::X3l));
/// ```
///
/// # Integration with Builder Pattern
///
/// This function is typically used in text component builders to resolve the final
/// size based on semantic intent and explicit customization:
///
/// ```rust
/// // In a text component builder
/// let final_size = get_effective_text_size(self.variant, self.explicit_size);
/// let pixel_size = get_font_size_pixels(&font_config, final_size);
/// ```
pub fn get_effective_text_size(variant: TextVariant, explicit_size: Option<TextSize>) -> TextSize {
    explicit_size.unwrap_or_else(|| match variant {
        TextVariant::Display => TextSize::X5l, // 48px - Maximum impact for hero content
        TextVariant::Title => TextSize::X2l,   // 24px - Clear hierarchy for section headers
        TextVariant::Body => TextSize::Base,   // 16px - Optimal for extended reading
        TextVariant::Label => TextSize::Sm,    // 14px - Appropriate for UI elements
        TextVariant::Caption => TextSize::Xs,  // 12px - Subtle for secondary information
    })
}

/// Determines the effective text weight by applying variant defaults or explicit overrides.
///
/// This function implements semantic weight selection for text variants, ensuring that
/// each variant has appropriate default weight that reflects its content importance
/// and visual hierarchy requirements.
///
/// # Arguments
///
/// * `variant` - The semantic text variant that determines the default weight
/// * `explicit_weight` - Optional explicit weight override that takes precedence over defaults
///
/// # Returns
///
/// The resolved TextWeight to use, either the explicit override or the variant's default.
///
/// # Variant Default Weights
///
/// Each variant has carefully chosen default weights based on its semantic purpose:
///
/// - **Display** → Bold - Maximum impact for attention-grabbing headlines
/// - **Title** → Medium - Balanced emphasis for clear hierarchy without aggression
/// - **Body** → Regular - Optimal readability for extended reading
/// - **Label** → Medium - Slight emphasis for clear UI element identification
/// - **Caption** → Regular - Subtle weight appropriate for secondary information
///
/// # Weight Selection Rationale
///
/// **Bold for Display**: Creates maximum visual impact while maintaining readability
/// **Medium for Titles/Labels**: Provides clear hierarchy without being overly heavy
/// **Regular for Body/Caption**: Ensures optimal readability for content consumption
///
/// # Accessibility Considerations
///
/// - All default weights ensure sufficient contrast and readability
/// - Bold weights provide clear visual hierarchy for screen readers
/// - Regular weights optimize extended reading comfort
/// - Medium weights balance emphasis with approachability
///
/// # Usage Examples
///
/// ```rust
/// // Use variant default (Body → Regular weight)
/// let body_weight = get_effective_text_weight(TextVariant::Body, None);
///
/// // Override for emphasis (Body → Bold for important information)
/// let important_body = get_effective_text_weight(TextVariant::Body, Some(TextWeight::Bold));
///
/// // Title with custom weight (Title → Bold instead of default Medium)
/// let bold_title = get_effective_text_weight(TextVariant::Title, Some(TextWeight::Bold));
/// ```
///
/// # Performance
///
/// Simple match operation with no allocations, suitable for frequent use
/// in component building and text styling systems.
pub fn get_effective_text_weight(
    variant: TextVariant,
    explicit_weight: Option<TextWeight>,
) -> TextWeight {
    explicit_weight.unwrap_or_else(|| match variant {
        TextVariant::Display => TextWeight::Bold, // Maximum impact for hero content
        TextVariant::Title => TextWeight::Medium, // Balanced emphasis for headers
        TextVariant::Body => TextWeight::Regular, // Optimal for extended reading
        TextVariant::Label => TextWeight::Medium, // Clear identification for UI
        TextVariant::Caption => TextWeight::Regular, // Subtle for secondary info
    })
}

/// Determines the effective font family by applying variant defaults or explicit overrides.
///
/// This function implements semantic font family selection for text variants. Currently,
/// all variants default to Sans for optimal screen readability and UI consistency,
/// but the system supports explicit overrides for specialized content.
///
/// # Arguments
///
/// * `variant` - The semantic text variant (used for potential future family differentiation)
/// * `explicit_family` - Optional explicit family override that takes precedence over defaults
///
/// # Returns
///
/// The resolved FontFamily to use, either the explicit override or the variant's default.
///
/// # Variant Default Families
///
/// Currently, all variants default to Sans (Roboto) for:
/// - **Consistency**: Uniform appearance across all UI elements
/// - **Readability**: Optimal screen readability for all content types
/// - **Performance**: Single font family reduces loading overhead
/// - **Accessibility**: Sans-serif fonts are generally more accessible
///
/// Future versions may differentiate families by variant:
/// - Display/Title: Sans for modern, clean appearance
/// - Body: Potential Serif option for long-form reading
/// - Label: Sans for UI clarity
/// - Caption: Sans for consistency
///
/// # Override Scenarios
///
/// Explicit family overrides are useful for:
/// - **Code content**: Use Mono for technical text
/// - **Formal content**: Use Serif for traditional, authoritative text
/// - **Brand consistency**: Match specific brand typography requirements
/// - **Content differentiation**: Distinguish special content areas
///
/// # Usage Examples
///
/// ```rust
/// // Use variant default (all variants → Sans currently)
/// let ui_family = get_effective_font_family(TextVariant::Label, None);
///
/// // Override for code content (any variant → Mono)
/// let code_family = get_effective_font_family(TextVariant::Body, Some(FontFamily::Mono));
///
/// // Override for formal content (any variant → Serif)
/// let formal_family = get_effective_font_family(TextVariant::Body, Some(FontFamily::Serif));
/// ```
///
/// # Design Philosophy
///
/// The current Sans-for-all approach prioritizes:
/// - **Simplicity**: Easier to maintain consistent appearance
/// - **Performance**: Fewer font assets to load and manage
/// - **Accessibility**: Sans-serif generally more screen-readable
/// - **Flexibility**: Easy to override when needed
///
/// # Future Evolution
///
/// This function provides the foundation for more sophisticated font family
/// selection based on variant semantics, content type, or theme preferences.
pub fn get_effective_font_family(
    variant: TextVariant,
    explicit_family: Option<FontFamily>,
) -> FontFamily {
    explicit_family.unwrap_or_else(|| match variant {
        // All variants currently default to Sans for consistency and readability
        // Future versions may differentiate based on semantic meaning
        TextVariant::Display => FontFamily::Sans, // Clean, modern for maximum impact
        TextVariant::Title => FontFamily::Sans,   // Consistent with UI theme
        TextVariant::Body => FontFamily::Sans,    // Optimal screen readability
        TextVariant::Label => FontFamily::Sans,   // Clear UI element identification
        TextVariant::Caption => FontFamily::Sans, // Consistent secondary styling
    })
}

/// Startup system that loads all font assets and initializes the typography system.
///
/// This system is responsible for loading all font files from the assets directory
/// and creating the FontAssets resource that provides access to font handles throughout
/// the application. It should be run early in the startup sequence to ensure fonts
/// are available when UI components are created.
///
/// # Asset Loading Strategy
///
/// The system loads fonts using Bevy's asset server, which provides:
/// - **Asynchronous loading**: Fonts load in the background without blocking
/// - **Asset hot reloading**: Font updates during development are automatically detected
/// - **Memory efficiency**: Font data is shared across all uses
/// - **Format support**: Supports TTF and other common font formats
///
/// # Font File Organization
///
/// Fonts are expected to be organized in the `assets/fonts/` directory:
///
/// ```
/// assets/
/// └── fonts/
///     ├── Roboto-Light.ttf       # Sans Light
///     ├── Roboto-Regular.ttf     # Sans Regular
///     ├── Roboto-Medium.ttf      # Sans Medium
///     ├── Roboto-Bold.ttf        # Sans Bold
///     ├── NotoSerif-Regular.ttf  # Serif Regular
///     ├── NotoSerif-Bold.ttf     # Serif Bold
///     ├── RobotoMono-Regular.ttf # Mono Regular
///     └── RobotoMono-Bold.ttf    # Mono Bold
/// ```
///
/// # System Integration
///
/// This system should be added to the Startup schedule:
///
/// ```rust
/// app.add_systems(Startup, load_font_assets);
/// ```
///
/// # Error Handling
///
/// The asset server handles missing files gracefully:
/// - Missing fonts will be logged as warnings
/// - Text components will fall back to default system fonts
/// - Application continues to function with reduced typography features
///
/// # Performance Considerations
///
/// - Font loading is asynchronous and non-blocking
/// - Font handles are lightweight and efficient to clone
/// - Actual font data is loaded and cached by the asset server
/// - All text components share the same font handles for memory efficiency
///
/// # Development vs Production
///
/// **Development**: Hot reloading allows font updates without restarting
/// **Production**: Fonts are embedded in the application bundle for reliability
///
/// # Customization
///
/// To use different fonts:
/// 1. Replace font files in the `assets/fonts/` directory
/// 2. Update the file paths in this function
/// 3. Ensure font licenses allow distribution with your application
pub fn load_font_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Loading font assets for typography system...");

    let font_assets = FontAssets {
        // === Sans-Serif Family (Roboto) ===
        // Complete weight range for maximum design flexibility
        sans_light: asset_server.load("fonts/Roboto-Light.ttf"),
        sans_regular: asset_server.load("fonts/Roboto-Regular.ttf"),
        sans_medium: asset_server.load("fonts/Roboto-Medium.ttf"),
        sans_bold: asset_server.load("fonts/Roboto-Bold.ttf"),

        // === Serif Family (Noto Serif) ===
        // Traditional typography for formal content
        serif_regular: asset_server.load("fonts/NotoSerif-Regular.ttf"),
        serif_bold: asset_server.load("fonts/NotoSerif-Bold.ttf"),

        // === Monospace Family (Roboto Mono) ===
        // Fixed-width fonts for code and technical content
        mono_regular: asset_server.load("fonts/RobotoMono-Regular.ttf"),
        mono_bold: asset_server.load("fonts/RobotoMono-Bold.ttf"),
        interface: asset_server.load("ui/icons/interface/lucide.ttf"),
        generic_controller: asset_server
            .load("ui/icons/controllers/generic/kenney_input_generic.otf"),
        keyboard_mouse: asset_server
            .load("ui/icons/controllers/keyboard_mouse/kenney_input_keyboard_&_mouse.ttf"),
        nintendo_switch: asset_server
            .load("ui/icons/controllers/nintendo_switch/kenney_input_nintendo_switch.ttf"),
        nintendo_switch_2: asset_server
            .load("ui/icons/controllers/nintendo_switch_2/kenney_input_nintendo_switch_2.ttf"),
        playstation: asset_server
            .load("ui/icons/controllers/playstation_series/kenney_input_playstation_series.ttf"),
        xbox: asset_server.load("ui/icons/controllers/xbox_series/kenney_input_xbox_series.ttf"),
        steam_controller: asset_server
            .load("ui/icons/controllers/steam_controller/kenney_input_steam_controller.ttf"),
        steam_deck: asset_server
            .load("ui/icons/controllers/steam_deck/kenney_input_steam_deck.ttf"),
        gamecube: asset_server
            .load("ui/icons/controllers/nintendo_gamecube/kenney_input_nintendo_gamecube.ttf"),
        wii: asset_server.load("ui/icons/controllers/nintendo_wii/kenney_input_nintendo_wii.ttf"),
        wii_u: asset_server
            .load("ui/icons/controllers/nintendo_wiiu/kenney_input_nintendo_wii_u.ttf"),
        touch: asset_server.load("ui/icons/controllers/touch/kenney_input_touch.ttf"),
        indicators: asset_server.load("ui/icons/controllers/indicators/kenney_input_flairs.ttf"),
    };

    commands.insert_resource(font_assets);
    info!("Font assets resource created and typography system initialized");
}
