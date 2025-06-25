//! Text styling trait system for consistent typography across UI components
//!
//! This module provides a comprehensive trait-based text styling system that
//! enables consistent typography configuration across all UI components. The
//! system offers both granular control and convenient presets for common
//! text styling patterns.
//!
//! # Core Components
//!
//! - **TextStyler**: Primary trait for individual text styling properties
//! - **TextStylePresets**: High-level presets for common text patterns
//! - **StyledTextBuilder**: Concrete implementation for direct text creation
//! - **Accessibility Integration**: Automatic contrast optimization and WCAG compliance
//!
//! # Design Philosophy
//!
//! The text styling system follows these principles:
//! - **Trait-Based**: Consistent API across all text-containing components
//! - **Accessibility-First**: Automatic contrast optimization and compliance
//! - **Preset-Driven**: Common patterns available as high-level presets
//! - **Flexible**: Fine-grained control when needed, sensible defaults otherwise
//! - **Performance**: Zero-cost abstractions with compile-time optimization
//!
//! # Usage Patterns
//!
//! ## Component Styling (Recommended)
//! ```rust
//! use your_crate::components::Button;
//! use your_crate::utilities::text_styling::TextStyler;
//!
//! // Style button text using trait methods
//! let styled_button = Button::new("Click Me")
//!     .text_weight(TextWeight::Bold)
//!     .text_size(TextSize::Lg)
//!     .text_high_contrast()
//!     .build();
//! ```
//!
//! ## Direct Text Creation
//! ```rust
//! use your_crate::utilities::text_styling::styled_text;
//!
//! // Create styled text directly
//! let heading = styled_text("Welcome")
//!     .as_heading_style()
//!     .text_center()
//!     .build();
//! ```
//!
//! ## Preset Usage
//! ```rust
//! // Use semantic presets for common patterns
//! let body_text = Button::new("Read More")
//!     .as_body_style();
//!
//! let caption = Label::new("Updated 2 hours ago")
//!     .as_caption_style();
//!
//! let code_block = Text::new("console.log('Hello')")
//!     .as_code_style();
//! ```
//!
//! # Accessibility Features
//!
//! - **Automatic Contrast**: Intelligent text color selection for readability
//! - **WCAG Compliance**: Built-in support for AA and AAA standards
//! - **Context Awareness**: Background-aware color optimization
//! - **Semantic Styling**: Meaningful text hierarchy for screen readers
//!
//! # Performance Characteristics
//!
//! - **Zero-Cost Traits**: Compile-time trait resolution with no runtime overhead
//! - **Builder Optimization**: Efficient builder pattern with minimal allocations
//! - **Cached Calculations**: Color contrast computed once and cached
//! - **Component Integration**: Seamless integration with existing component builders

use bevy::prelude::*;

use crate::{
    theme::{
        typography::{TextSize, TextWeight, FontFamily, TextVariant},
        color::{TextColor as TextColorEnum, TextContrastLevel},
    },
    utilities::ComponentBuilder,
};

/// Comprehensive text styling trait for UI components with text content
///
/// TextStyler provides a unified, fluent API for configuring text appearance
/// across all UI components that contain text. It offers fine-grained control
/// over typography, accessibility, and visual presentation while maintaining
/// consistency across the design system.
///
/// # Core Styling Methods
///
/// The trait is organized into logical groups of styling capabilities:
///
/// ## Typography Control
/// - Font size, weight, family, and style configuration
/// - Semantic size scales and weight hierarchy
/// - Support for custom font families and monospace text
///
/// ## Layout and Alignment
/// - Text alignment with accessibility considerations
/// - Convenient alignment shortcuts for common patterns
/// - Responsive alignment behavior
///
/// ## Accessibility and Contrast
/// - Automatic contrast optimization based on background context
/// - WCAG AA and AAA compliance levels
/// - Manual override options for specific design requirements
///
/// ## Color and Theming
/// - Semantic color variants with automatic theme integration
/// - Background-aware color selection
/// - Manual color specification when needed
///
/// # Implementation Pattern
///
/// All text-containing components should implement this trait:
///
/// ```rust
/// impl TextStyler for MyComponent {
///     fn text_size(mut self, size: TextSize) -> Self {
///         self.text_config.size = Some(size);
///         self
///     }
///     // ... implement other methods
/// }
/// ```
///
/// # Usage Examples
///
/// ```rust
/// // Typography configuration
/// component.text_size(TextSize::Lg)
///          .text_weight(TextWeight::Bold)
///          .text_family(FontFamily::Sans)
///          .text_italic();
///
/// // Accessibility configuration
/// component.text_high_contrast()
///          .text_on_background(Color::rgb(0.1, 0.1, 0.1))
///          .text_accessible();
///
/// // Layout configuration
/// component.text_center()
///          .text_align(JustifyText::Center);
///
/// // Color configuration
/// component.text_color(TextColor::Accent)
///          .text_auto_contrast();
/// ```
pub trait TextStyler {
    // =========================================================================
    // TYPOGRAPHY METHODS
    // =========================================================================

    /// Set the text size using semantic size scale
    ///
    /// Configures the font size using the theme's typography scale, ensuring
    /// consistent sizing across the design system and proper responsive behavior.
    ///
    /// # Parameters
    /// - `size`: Semantic text size from the typography scale
    ///
    /// # Examples
    /// ```rust
    /// component.text_size(TextSize::Lg);     // Large text for headings
    /// component.text_size(TextSize::Base);   // Standard body text size
    /// component.text_size(TextSize::Sm);     // Small text for captions
    /// ```
    fn text_size(self, size: TextSize) -> Self;
    
    /// Set the font weight for visual hierarchy
    ///
    /// Configures the font weight to establish visual hierarchy and emphasis.
    /// Different weights serve specific purposes in the design system.
    ///
    /// # Parameters
    /// - `weight`: Font weight from Light to ExtraBold
    ///
    /// # Examples
    /// ```rust
    /// component.text_weight(TextWeight::Bold);      // Headings and emphasis
    /// component.text_weight(TextWeight::Regular);   // Body text
    /// component.text_weight(TextWeight::Light);     // Subtle text
    /// ```
    fn text_weight(self, weight: TextWeight) -> Self;
    
    /// Set the font family for different text contexts
    ///
    /// Configures the font family based on content type and design requirements.
    /// Each family serves specific use cases and readability needs.
    ///
    /// # Parameters
    /// - `family`: Font family (Sans, Serif, Mono)
    ///
    /// # Examples
    /// ```rust
    /// component.text_family(FontFamily::Sans);   // UI text (default)
    /// component.text_family(FontFamily::Serif);  // Reading content
    /// component.text_family(FontFamily::Mono);   // Code and data
    /// ```
    fn text_family(self, family: FontFamily) -> Self;
    
    /// Apply italic styling for emphasis
    ///
    /// Adds italic styling to provide emphasis while maintaining readability.
    /// Useful for quotes, emphasis, and stylistic variation.
    ///
    /// # Examples
    /// ```rust
    /// component.text_italic();  // Emphasized text
    /// quote_text.text_italic(); // Quotation styling
    /// ```
    fn text_italic(self) -> Self;

    // =========================================================================
    // ALIGNMENT METHODS
    // =========================================================================
    
    /// Set text alignment with accessibility considerations
    ///
    /// Configures text alignment while considering readability and accessibility
    /// guidelines. Different alignments serve specific layout and content needs.
    ///
    /// # Parameters
    /// - `align`: Text alignment option
    ///
    /// # Examples
    /// ```rust
    /// component.text_align(JustifyText::Left);     // Default, most readable
    /// component.text_align(JustifyText::Center);   // Headings, symmetric layouts
    /// component.text_align(JustifyText::Right);    // Numeric data, special cases
    /// ```
    fn text_align(self, align: JustifyText) -> Self;
    
    /// Center align text for symmetric layouts
    ///
    /// Convenience method for center alignment, commonly used for headings,
    /// buttons, and symmetric layout elements.
    ///
    /// # Examples
    /// ```rust
    /// heading.text_center();        // Centered headings
    /// button.text_center();         // Balanced button text
    /// modal_title.text_center();    // Symmetric modal titles
    /// ```
    fn text_center(self) -> Self;
    
    /// Right align text for data and special layouts
    ///
    /// Convenience method for right alignment, useful for numeric data,
    /// timestamps, and specific layout requirements.
    ///
    /// # Examples
    /// ```rust
    /// price_display.text_right();   // Numeric alignment
    /// timestamp.text_right();       // Metadata positioning
    /// table_numbers.text_right();   // Data consistency
    /// ```
    fn text_right(self) -> Self;

    // =========================================================================
    // ACCESSIBILITY AND CONTRAST METHODS
    // =========================================================================
    
    /// Set background context for intelligent contrast calculation
    ///
    /// Provides background color information to enable automatic contrast
    /// optimization, ensuring optimal text readability against the background.
    ///
    /// # Parameters
    /// - `background_color`: The background color this text will appear on
    ///
    /// # Examples
    /// ```rust
    /// let dark_bg = Color::rgb(0.1, 0.1, 0.1);
    /// component.text_on_background(dark_bg);  // Optimizes for dark background
    ///
    /// let blue_bg = Color::rgb(0.2, 0.4, 0.8);
    /// text.text_on_background(blue_bg);       // Adapts to colored background
    /// ```
    fn text_on_background(self, background_color: Color) -> Self;
    
    /// Set specific contrast level for accessibility compliance
    ///
    /// Configures the desired contrast level for accessibility standards,
    /// allowing fine-grained control over text readability requirements.
    ///
    /// # Parameters
    /// - `level`: Contrast level (Low, Medium, High, Accessible)
    ///
    /// # Examples
    /// ```rust
    /// component.text_contrast_level(TextContrastLevel::High);       // WCAG AA
    /// component.text_contrast_level(TextContrastLevel::Accessible); // WCAG AAA
    /// ```
    fn text_contrast_level(self, level: TextContrastLevel) -> Self;
    
    /// Enable high contrast text for WCAG AA compliance
    ///
    /// Convenience method to ensure text meets WCAG AA contrast standards
    /// (4.5:1 ratio for normal text). Recommended for most UI text.
    ///
    /// # Examples
    /// ```rust
    /// body_text.text_high_contrast();    // Accessible body text
    /// button_text.text_high_contrast();  // Clear button labels
    /// form_labels.text_high_contrast();  // Readable form elements
    /// ```
    fn text_high_contrast(self) -> Self;
    
    /// Enable accessible text for WCAG AAA compliance
    ///
    /// Convenience method to ensure text meets WCAG AAA contrast standards
    /// (7:1 ratio). Recommended for critical information and enhanced accessibility.
    ///
    /// # Examples
    /// ```rust
    /// error_text.text_accessible();      // Critical error messages
    /// legal_text.text_accessible();      // Important legal information
    /// alert_text.text_accessible();      // High-priority alerts
    /// ```
    fn text_accessible(self) -> Self;
    
    /// Enable automatic contrast optimization based on background
    ///
    /// Activates intelligent contrast calculation that automatically selects
    /// the best text color for optimal readability against backgrounds.
    ///
    /// # Examples
    /// ```rust
    /// dynamic_text.text_auto_contrast();     // Adapts to any background
    /// theme_text.text_auto_contrast();       // Works with theme switching
    /// overlay_text.text_auto_contrast();     // Optimizes for overlay content
    /// ```
    fn text_auto_contrast(self) -> Self;
    
    /// Disable automatic contrast optimization for manual color control
    ///
    /// Disables automatic contrast calculation, allowing explicit color
    /// specification without automatic adjustments.
    ///
    /// # Examples
    /// ```rust
    /// brand_text.text_manual_color()         // Preserve exact brand colors
    ///           .text_color(TextColor::Custom(BRAND_BLUE));
    ///
    /// decorative_text.text_manual_color();   // Maintain design intent
    /// ```
    fn text_manual_color(self) -> Self;

    // =========================================================================
    // COLOR METHODS
    // =========================================================================
    
    /// Set explicit text color with semantic meaning
    ///
    /// Configures the text color using semantic color variants that automatically
    /// integrate with the theme system and provide appropriate meaning.
    ///
    /// # Parameters
    /// - `color`: Semantic text color variant
    ///
    /// # Examples
    /// ```rust
    /// component.text_color(TextColorEnum::Default);  // Primary text color
    /// component.text_color(TextColorEnum::Muted);    // Secondary text color
    /// component.text_color(TextColorEnum::Accent);   // Brand/highlight color
    /// component.text_color(TextColorEnum::Error);    // Error state color
    /// component.text_color(TextColorEnum::Success);  // Success state color
    /// ```
    fn text_color(self, color: TextColorEnum) -> Self;
}

/// High-level text styling presets for common typography patterns
///
/// TextStylePresets provides semantic, high-level styling presets that
/// implement common typography patterns used throughout UI design. These
/// presets combine multiple styling properties to create cohesive,
/// accessible text presentations for specific use cases.
///
/// # Design Philosophy
///
/// Presets are designed with:
/// - **Semantic Meaning**: Each preset has clear purpose and context
/// - **Accessibility First**: All presets meet or exceed WCAG guidelines
/// - **Consistency**: Uniform styling across similar content types
/// - **Flexibility**: Presets can be further customized after application
///
/// # Available Presets
///
/// - **heading_style**: Large, bold text for primary headings
/// - **body_style**: Standard text for content and paragraphs
/// - **caption_style**: Small, muted text for metadata and descriptions
/// - **label_style**: Medium weight text for form labels and UI elements
/// - **code_style**: Monospace text for code blocks and data
/// - **emphasis_style**: Italic, weighted text for emphasis
/// - **accessible_style**: High-contrast text for critical information
///
/// # Usage Patterns
///
/// ```rust
/// // Apply preset then customize if needed
/// let custom_heading = component
///     .as_heading_style()              // Apply heading preset
///     .text_size(TextSize::Xl)         // Customize size
///     .text_center();                  // Add center alignment
///
/// // Use presets for consistent component styling
/// let form_label = Label::new("Email Address")
///     .as_label_style();               // Standard label styling
///
/// let error_message = Text::new("Invalid email format")
///     .as_accessible_style()           // High contrast for errors
///     .text_color(TextColor::Error);   // Add error semantics
/// ```
///
/// # Customization Strategy
///
/// Presets provide a starting point that can be refined:
/// 1. **Apply Preset**: Start with semantic preset for base styling
/// 2. **Customize Properties**: Override specific properties as needed
/// 3. **Add Context**: Apply background context and accessibility settings
/// 4. **Finalize**: Build the final styled component
///
/// # Accessibility Integration
///
/// All presets include accessibility considerations:
/// - **Appropriate Contrast**: Automatic contrast level selection
/// - **Readable Sizes**: Minimum sizes for accessibility compliance
/// - **Semantic Hierarchy**: Clear visual hierarchy for screen readers
/// - **Color Independence**: Meaning not dependent solely on color
pub trait TextStylePresets: TextStyler + Sized {
    /// Apply heading styling for primary headings and section titles
    ///
    /// Configures text with heading-appropriate styling: bold weight, large size,
    /// and high contrast for accessibility. Suitable for H1-H3 headings and
    /// primary section titles.
    ///
    /// # Styling Applied
    /// - **Weight**: Bold for strong visual hierarchy
    /// - **Size**: Large for prominence and importance
    /// - **Contrast**: High contrast for accessibility compliance
    ///
    /// # Examples
    /// ```rust
    /// let page_title = Text::new("Welcome to Our App")
    ///     .as_heading_style();
    ///
    /// let section_header = Heading::new("User Settings")
    ///     .as_heading_style()
    ///     .text_center();
    /// ```
    fn as_heading_style(self) -> Self {
        self.text_weight(TextWeight::Bold)
            .text_size(TextSize::Lg)
            .text_high_contrast()
    }
    
    /// Apply body text styling for paragraphs and standard content
    ///
    /// Configures text with optimal styling for reading: regular weight and
    /// standard size. This is the baseline styling for most text content.
    ///
    /// # Styling Applied
    /// - **Weight**: Regular for optimal readability
    /// - **Size**: Base size for comfortable reading
    ///
    /// # Examples
    /// ```rust
    /// let paragraph = Text::new("This is body text content...")
    ///     .as_body_style();
    ///
    /// let description = Label::new("Enter your email address")
    ///     .as_body_style();
    /// ```
    fn as_body_style(self) -> Self {
        self.text_weight(TextWeight::Regular)
            .text_size(TextSize::Base)
    }
    
    /// Apply caption styling for metadata and secondary information
    ///
    /// Configures text for secondary information: smaller size and muted color.
    /// Suitable for timestamps, descriptions, helper text, and metadata.
    ///
    /// # Styling Applied
    /// - **Weight**: Regular for readability at small size
    /// - **Size**: Small to indicate secondary importance
    /// - **Color**: Muted to reduce visual prominence
    ///
    /// # Examples
    /// ```rust
    /// let timestamp = Text::new("Updated 2 hours ago")
    ///     .as_caption_style();
    ///
    /// let helper_text = Text::new("This field is optional")
    ///     .as_caption_style();
    /// ```
    fn as_caption_style(self) -> Self {
        self.text_weight(TextWeight::Regular)
            .text_size(TextSize::Sm)
            .text_color(TextColorEnum::Muted)
    }
    
    /// Apply label styling for form labels and UI element labels
    ///
    /// Configures text for form labels and UI element identification: medium
    /// weight for emphasis and small size for efficiency.
    ///
    /// # Styling Applied
    /// - **Weight**: Medium for clear association and emphasis
    /// - **Size**: Small for efficient space usage
    ///
    /// # Examples
    /// ```rust
    /// let form_label = Label::new("Email Address")
    ///     .as_label_style();
    ///
    /// let button_label = Text::new("Save Changes")
    ///     .as_label_style();
    /// ```
    fn as_label_style(self) -> Self {
        self.text_weight(TextWeight::Medium)
            .text_size(TextSize::Sm)
    }
    
    /// Apply code styling for code blocks and monospace content
    ///
    /// Configures text for code display: monospace font and appropriate size.
    /// Suitable for code blocks, inline code, data tables, and technical content.
    ///
    /// # Styling Applied
    /// - **Family**: Monospace for character alignment and code readability
    /// - **Size**: Small for information density
    ///
    /// # Examples
    /// ```rust
    /// let code_block = Text::new("console.log('Hello World');")
    ///     .as_code_style();
    ///
    /// let file_path = Text::new("/usr/local/bin/app")
    ///     .as_code_style();
    /// ```
    fn as_code_style(self) -> Self {
        self.text_family(FontFamily::Mono)
            .text_size(TextSize::Sm)
    }
    
    /// Apply emphasis styling for highlighted or stressed content
    ///
    /// Configures text for emphasis: italic style and medium weight.
    /// Suitable for quotes, stressed words, and content that needs attention.
    ///
    /// # Styling Applied
    /// - **Style**: Italic for visual emphasis
    /// - **Weight**: Medium for subtle strengthening
    ///
    /// # Examples
    /// ```rust
    /// let quote = Text::new("Design is not just what it looks like...")
    ///     .as_emphasis_style();
    ///
    /// let stressed_word = Text::new("important")
    ///     .as_emphasis_style();
    /// ```
    fn as_emphasis_style(self) -> Self {
        self.text_italic()
            .text_weight(TextWeight::Medium)
    }
    
    /// Apply accessible styling for critical and important information
    ///
    /// Configures text with maximum accessibility: WCAG AAA compliance and
    /// medium weight for enhanced readability. Use for error messages,
    /// legal text, and critical information.
    ///
    /// # Styling Applied
    /// - **Contrast**: WCAG AAA level (7:1 ratio) for maximum accessibility
    /// - **Weight**: Medium for enhanced readability
    ///
    /// # Examples
    /// ```rust
    /// let error_message = Text::new("Invalid password format")
    ///     .as_accessible_style()
    ///     .text_color(TextColor::Error);
    ///
    /// let legal_notice = Text::new("By continuing, you agree to our terms")
    ///     .as_accessible_style();
    /// ```
    fn as_accessible_style(self) -> Self {
        self.text_accessible()
            .text_weight(TextWeight::Medium)
    }
}

// Blanket implementation for all TextStyler implementors
impl<T: TextStyler + Sized> TextStylePresets for T {}

/// Helper function to create a text builder with common styling
pub fn styled_text(content: impl Into<String>) -> StyledTextBuilder {
    StyledTextBuilder::new(content)
}

/// Wrapper around TextBuilder that implements TextStyler
pub struct StyledTextBuilder {
    content: String,
    variant: TextVariant,
    size: Option<TextSize>,
    weight: Option<TextWeight>,
    family: Option<FontFamily>,
    color: Option<TextColorEnum>,
    italic: bool,
    align: Option<JustifyText>,
    background_context: Option<Color>,
    contrast_level: Option<TextContrastLevel>,
    explicit_color_set: bool,
}

impl StyledTextBuilder {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            variant: TextVariant::default(),
            size: None,
            weight: None,
            family: None,
            color: None,
            italic: false,
            align: None,
            background_context: None,
            contrast_level: Some(TextContrastLevel::High),
            explicit_color_set: false,
        }
    }
    /// Build the final text component
    pub fn build(self) -> impl Bundle {
        use crate::components::text::Text;
        
        let mut builder = Text::new(self.content)
            .variant(self.variant);
            
        if let Some(size) = self.size {
            builder = builder.size(size);
        }
        if let Some(weight) = self.weight {
            builder = builder.weight(weight);
        }
        if let Some(family) = self.family {
            builder = builder.family(family);
        }
        if let Some(color) = self.color {
            builder = builder.color(color);
        }
        if self.italic {
            builder = builder.italic();
        }
        if let Some(align) = self.align {
            builder = builder.align(align);
        }
        if let Some(bg) = self.background_context {
            builder = builder.on_background(bg);
        }
        if let Some(level) = self.contrast_level {
            builder = builder.contrast_level(level);
        }
        if self.explicit_color_set {
            builder = builder.manual_color();
        }
        
        builder.build()
    }
}

impl TextStyler for StyledTextBuilder {
    fn text_size(mut self, size: TextSize) -> Self {
        self.size = Some(size);
        self
    }
    
    fn text_weight(mut self, weight: TextWeight) -> Self {
        self.weight = Some(weight);
        self
    }
    
    fn text_family(mut self, family: FontFamily) -> Self {
        self.family = Some(family);
        self
    }
    
    fn text_italic(mut self) -> Self {
        self.italic = true;
        self
    }
    
    fn text_align(mut self, align: JustifyText) -> Self {
        self.align = Some(align);
        self
    }
    
    fn text_center(self) -> Self {
        self.text_align(JustifyText::Center)
    }
    
    fn text_right(self) -> Self {
        self.text_align(JustifyText::Right)
    }
    
    fn text_on_background(mut self, background_color: Color) -> Self {
        self.background_context = Some(background_color);
        self
    }
    
    fn text_contrast_level(mut self, level: TextContrastLevel) -> Self {
        self.contrast_level = Some(level);
        self
    }
    
    fn text_high_contrast(self) -> Self {
        self.text_contrast_level(TextContrastLevel::High)
    }
    
    fn text_accessible(self) -> Self {
        self.text_contrast_level(TextContrastLevel::Accessible)
    }
    
    fn text_auto_contrast(mut self) -> Self {
        self.explicit_color_set = false;
        self
    }
    
    fn text_manual_color(mut self) -> Self {
        self.explicit_color_set = true;
        self
    }
    
    fn text_color(mut self, color: TextColorEnum) -> Self {
        self.color = Some(color);
        self.explicit_color_set = true;
        self
    }
}