//! Text Component for Forge UI
//!
//! The Text component provides a comprehensive typography system for Bevy applications,
//! following Radix UI design principles with enhanced accessibility features.
//! It offers semantic text variants, intelligent contrast optimization, and complete
//! theme integration for consistent typography across your application.
//!
//! ## Key Features
//!
//! - **Semantic Variants**: Display, Title, Body, Label, and Caption text types
//! - **Complete Typography Control**: Size, weight, family, style, and alignment
//! - **Intelligent Contrast**: Automatic color optimization based on background context
//! - **Accessibility Compliance**: WCAG AA/AAA contrast ratio support
//! - **Theme Integration**: Full integration with Radix UI color system
//! - **Font Management**: Automatic font loading and application system
//! - **Builder Pattern**: Fluent API for easy text configuration
//!
//! ## Text Variants
//!
//! - **Display**: Largest text for headings and heroes (bold, prominent)
//! - **Title**: Large text for section headers (medium weight)
//! - **Body**: Default text for content (regular weight)
//! - **Label**: Small text for UI labels (medium weight)
//! - **Caption**: Smallest text for metadata (muted color)
//!
//! ## Examples
//!
//! ### Basic Usage
//! ```rust
//! use forge_ui::Text;
//!
//! // Simple body text
//! let text = Text::body("Hello World").build();
//!
//! // Title with custom styling
//! let title = Text::title("Section Header")
//!     .size(TextSize::Lg)
//!     .weight(TextWeight::Bold)
//!     .center()
//!     .build();
//! ```
//!
//! ### Advanced Styling
//! ```rust
//! use forge_ui::{Text, TextColor, TextSize};
//!
//! // Custom styled text with manual color
//! let styled_text = Text::new("Custom Text")
//!     .size(TextSize::Xl)
//!     .weight(TextWeight::Medium)
//!     .color(TextColor::Accent)
//!     .italic()
//!     .build();
//!
//! // Code text with monospace font
//! let code = Text::code("console.log('hello');")
//!     .size(TextSize::Sm)
//!     .build();
//! ```
//!
//! ### Accessibility & Contrast
//! ```rust
//! use forge_ui::{Text, TextContrastLevel};
//!
//! // Automatic contrast optimization
//! let accessible_text = Text::accessible(
//!     "Important information",
//!     Color::srgb(0.2, 0.3, 0.8) // background color
//! ).build();
//!
//! // High contrast text for better visibility
//! let high_contrast = Text::body("High visibility text")
//!     .high_contrast()
//!     .build();
//! ```
//!
//! ## Contrast Intelligence
//!
//! The text system automatically calculates optimal text colors based on:
//! - Background color context
//! - Desired contrast level (High/Accessible)
//! - Color palette selection
//! - WCAG compliance requirements
//!
//! ## Font System Integration
//!
//! Text components automatically integrate with the font loading system:
//! - Fonts are loaded asynchronously via `FontAssets`
//! - The `apply_text_fonts` system handles font application
//! - Supports multiple font families: Sans, Serif, Mono
//! - Multiple weights: Light, Regular, Medium, Bold (+ Italic variants)

use bevy::prelude::*;

use crate::{
    theme::{
        color::{TextColor, TextContrastLevel},
        typography::{
            FontAssets, FontSize, FontFamily, TextSize, TextVariant, TextWeight,
            get_font_handle, get_font_size_pixels, get_effective_text_size,
            get_effective_text_weight, get_effective_font_family,
        },
    },
};

/// Builder for creating themed text components with advanced typography and accessibility features.
/// 
/// The TextBuilder provides a fluent API for configuring all aspects of text appearance,
/// including semantic variants, styling options, accessibility features, and intelligent
/// contrast optimization.
/// 
/// # Architecture
/// 
/// The builder separates concerns between:
/// - **Content**: The actual text to display
/// - **Semantics**: The role/purpose of the text (variant)
/// - **Styling**: Visual appearance (size, weight, family, etc.)
/// - **Accessibility**: Contrast and readability optimization
/// - **Layout**: Text alignment and positioning
/// 
/// # Example
/// ```rust
/// let text = TextBuilder::new("Hello World")
///     .variant(TextVariant::Title)
///     .size(TextSize::Lg)
///     .weight(TextWeight::Bold)
///     .color(TextColor::Accent)
///     .center()
///     .high_contrast()
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct TextBuilder {
    /// The text content to display
    content: String,
    /// Semantic variant that determines default styling
    variant: TextVariant,
    /// Override text size (if different from variant default)
    size: Option<TextSize>,
    /// Override text weight (if different from variant default)
    weight: Option<TextWeight>,
    /// Override font family (if different from variant default)
    family: Option<FontFamily>,
    /// Text color (enables manual color control)
    color: Option<TextColor>,
    /// Whether text should be italicized
    italic: bool,
    /// Text alignment within its container
    align: Option<JustifyText>,
    /// Background color for intelligent contrast calculation
    background_context: Option<Color>,
    /// Desired contrast level for accessibility
    contrast_level: Option<TextContrastLevel>,
    /// Whether color was explicitly set (disables auto-contrast)
    explicit_color_set: bool,
}

impl TextBuilder {
    /// Creates a new TextBuilder with the specified content.
    /// 
    /// The builder starts with sensible defaults:
    /// - Default variant (Body)
    /// - High contrast level for accessibility
    /// - Automatic contrast optimization enabled
    /// - Left text alignment
    /// 
    /// # Arguments
    /// * `content` - The text content to display
    /// 
    /// # Returns
    /// A new TextBuilder ready for customization
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

    /// Sets the semantic text variant.
    /// 
    /// Text variants define the role and default styling of text:
    /// - Display: Large, prominent text for headers
    /// - Title: Medium-large text for section titles
    /// - Body: Standard text for content
    /// - Label: Small text for UI labels
    /// - Caption: Tiny, muted text for metadata
    /// 
    /// # Arguments
    /// * `variant` - The TextVariant to apply
    pub fn variant(mut self, variant: TextVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Sets the text size, overriding the variant's default size.
    /// 
    /// # Arguments
    /// * `size` - The TextSize to use (Xs, Sm, Base, Lg, Xl, etc.)
    pub fn size(mut self, size: TextSize) -> Self {
        self.size = Some(size);
        self
    }

    /// Sets the text weight, overriding the variant's default weight.
    /// 
    /// # Arguments
    /// * `weight` - The TextWeight to use (Light, Regular, Medium, Bold)
    pub fn weight(mut self, weight: TextWeight) -> Self {
        self.weight = Some(weight);
        self
    }

    /// Sets the font family, overriding the variant's default family.
    /// 
    /// # Arguments
    /// * `family` - The FontFamily to use (Sans, Serif, Mono)
    pub fn family(mut self, family: FontFamily) -> Self {
        self.family = Some(family);
        self
    }

    /// Sets an explicit text color, disabling automatic contrast optimization.
    /// 
    /// When a color is explicitly set, the intelligent contrast system is disabled
    /// and the specified color is used exactly as provided.
    /// 
    /// # Arguments
    /// * `color` - The TextColor to use
    pub fn color(mut self, color: TextColor) -> Self {
        self.color = Some(color);
        self.explicit_color_set = true;
        self
    }

    /// Makes the text italic.
    /// 
    /// Note: Italic variants must be available in the font family.
    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }

    /// Sets the text alignment within its container.
    /// 
    /// # Arguments
    /// * `align` - The JustifyText alignment (Left, Center, Right)
    pub fn align(mut self, align: JustifyText) -> Self {
        self.align = Some(align);
        self
    }

    // === Convenience Methods for Alignment ===
    
    /// Convenience method for center text alignment.
    pub fn center(self) -> Self {
        self.align(JustifyText::Center)
    }

    /// Convenience method for right text alignment.
    pub fn right(self) -> Self {
        self.align(JustifyText::Right)
    }

    // === Background Context & Contrast Methods ===
    
    /// Sets the background color context for intelligent contrast calculation.
    /// 
    /// When provided, the text system will automatically choose text colors
    /// that provide sufficient contrast against this background, following
    /// WCAG accessibility guidelines.
    /// 
    /// # Arguments
    /// * `background_color` - The background color to optimize contrast against
    pub fn on_background(mut self, background_color: Color) -> Self {
        self.background_context = Some(background_color);
        self
    }

    /// Sets the desired contrast level for accessibility compliance.
    /// 
    /// # Arguments
    /// * `level` - The TextContrastLevel (High for WCAG AA, Accessible for WCAG AAA)
    pub fn contrast_level(mut self, level: TextContrastLevel) -> Self {
        self.contrast_level = Some(level);
        self
    }

    /// Sets high contrast mode for WCAG AA compliance.
    /// 
    /// Ensures text meets WCAG AA guidelines for color contrast ratios.
    pub fn high_contrast(self) -> Self {
        self.contrast_level(TextContrastLevel::High)
    }

    /// Sets accessible contrast mode for WCAG AAA compliance.
    /// 
    /// Ensures text meets the highest WCAG AAA guidelines for accessibility.
    pub fn accessible(self) -> Self {
        self.contrast_level(TextContrastLevel::Accessible)
    }

    /// Enables automatic background detection and contrast optimization.
    /// 
    /// This is the default behavior - the system will automatically
    /// optimize text colors for readability.
    pub fn auto_contrast(self) -> Self {
        self
    }

    /// Disables automatic contrast optimization.
    /// 
    /// Forces the system to use explicit colors only, without automatic
    /// contrast adjustments based on background context.
    pub fn manual_color(mut self) -> Self {
        self.explicit_color_set = true;
        self
    }

    /// Converts TextColor to actual Color using intelligent contrast calculation.
    /// 
    /// This method implements the core logic for automatic contrast optimization,
    /// taking into account:
    /// - The specified text color type
    /// - Background context (if provided)
    /// - Desired contrast level
    /// - Whether explicit color control is enabled
    /// 
    /// # Arguments
    /// * `color` - The TextColor to resolve
    /// 
    /// # Returns
    /// A Color optimized for the current context and contrast requirements
    fn map_color(&self, color: TextColor) -> Color {
        use crate::theme::color::{
            accent_palette, error_palette, success_palette, theme, warning_palette,
        };

        // Get the appropriate palette for color resolution
        let palette = match color {
            TextColor::Default | TextColor::Muted => theme().gray,
            TextColor::Accent => accent_palette(),
            TextColor::Error => error_palette(),
            TextColor::Warning => warning_palette(),
            TextColor::Success => success_palette(),
            TextColor::Custom(c) => return c,
        };

        // Use the new resolve_text_color method from UiColorPalette
        palette.resolve_text_color(
            color,
            self.background_context.as_ref(),
            self.contrast_level,
            self.explicit_color_set,
        )
    }
}

impl TextBuilder {
    /// Builds the text configuration into a Bevy Bundle.
    /// 
    /// This method performs the complex logic of:
    /// 1. Resolving effective styling based on variant and overrides
    /// 2. Calculating optimal colors using contrast intelligence
    /// 3. Creating appropriate Bevy components
    /// 4. Setting up font information for the font loading system
    /// 
    /// # Returns
    /// A tuple of Bevy components ready for entity spawning
    /// 
    /// The returned bundle includes all necessary Bevy components for a fully functional text entity:
    /// - Text: The text content widget
    /// - TextFont: Font and size information
    /// - TextColor: Resolved color
    /// - TextLayout: Alignment and layout
    /// - Node: UI layout node
    /// - TextFontInfo: Font metadata for the font system
    pub fn build(self) -> impl Bundle {
        let effective_size = get_effective_text_size(self.variant, self.size);
        let effective_weight = get_effective_text_weight(self.variant, self.weight);
        let effective_family = get_effective_font_family(self.variant, self.family);
        let effective_color = self.color.unwrap_or_else(|| match self.variant {
            TextVariant::Display => TextColor::Default,
            TextVariant::Title => TextColor::Default,
            TextVariant::Body => TextColor::Default,
            TextVariant::Label => TextColor::Default,
            TextVariant::Caption => TextColor::Muted,
        });

        let color = self.map_color(effective_color);
        let font_size = get_font_size_pixels(&FontSize::default(), effective_size);

        // Font handle will be updated by the apply_text_fonts system
        let font_handle = Handle::<Font>::default();

        (
            bevy::ui::widget::Text::new(self.content),
            TextFont {
                font: font_handle,
                font_size,
                ..default()
            },
            bevy::prelude::TextColor(color),
            TextLayout::new_with_justify(self.align.unwrap_or(JustifyText::Left)),
            Node::default(),
            TextFontInfo {
                family: effective_family,
                weight: effective_weight,
            },
        )
    }
}

/// Marker component that stores font configuration for text entities.
/// 
/// This component is used by the font loading system to apply the correct
/// fonts to text entities after font assets have been loaded. It acts as
/// a bridge between the text configuration and the actual font handles.
/// 
/// The component is automatically removed after fonts are applied by the
/// `apply_text_fonts` system.
#[derive(Component, Debug, Clone)]
pub struct TextFontInfo {
    /// The font family to use (Sans, Serif, Mono)
    pub family: FontFamily,
    /// The font weight to use (Light, Regular, Medium, Bold)
    pub weight: TextWeight,
}

/// Main Text component interface providing semantic text creation methods.
/// 
/// The Text struct serves as the primary entry point for creating text components
/// with semantic meaning. It provides convenient factory methods for common text
/// types while maintaining full access to the underlying TextBuilder for advanced
/// customization.
/// 
/// # Design Philosophy
/// 
/// This interface follows the principle of "semantic first, styling second":
/// 1. Choose the appropriate semantic method (display, title, body, etc.)
/// 2. Customize styling as needed using the returned TextBuilder
/// 3. Build the final component with `.build()`
/// 
/// # Example Usage
/// ```rust
/// // Semantic text creation
/// let heading = Text::display("Welcome").center().build();
/// let content = Text::body("Lorem ipsum...").build();
/// let metadata = Text::caption("Last updated: today").build();
/// 
/// // Specialized text types
/// let code = Text::code("fn main() {}").build();
/// let accessible = Text::accessible("Important", background_color).build();
/// ```
pub struct Text;

impl Text {
    // === Core Factory Methods ===
    
    /// Creates a new text component with automatic contrast optimization.
    /// 
    /// This is the base method that provides full control over text configuration.
    /// Use this when you need custom styling that doesn't fit the semantic variants.
    /// 
    /// # Arguments
    /// * `content` - The text content to display
    /// 
    /// # Returns
    /// A TextBuilder ready for customization
    pub fn new(content: impl Into<String>) -> TextBuilder {
        TextBuilder::new(content)
    }

    // === Semantic Text Variants ===
    
    /// Creates display text - the largest, most prominent text style.
    /// 
    /// Use for main headings, hero text, or primary page titles.
    /// Typically bold and significantly larger than other text.
    /// 
    /// # Arguments
    /// * `content` - The text content to display
    pub fn display(content: impl Into<String>) -> TextBuilder {
        TextBuilder::new(content).variant(TextVariant::Display)
    }

    /// Creates title text - large text for section headers.
    /// 
    /// Use for section titles, card headers, or secondary headings.
    /// Medium weight with larger size than body text.
    /// 
    /// # Arguments
    /// * `content` - The text content to display
    pub fn title(content: impl Into<String>) -> TextBuilder {
        TextBuilder::new(content).variant(TextVariant::Title)
    }

    /// Creates body text - the default text style for content.
    /// 
    /// Use for paragraphs, descriptions, and general content.
    /// Regular weight with standard size for optimal readability.
    /// 
    /// # Arguments
    /// * `content` - The text content to display
    pub fn body(content: impl Into<String>) -> TextBuilder {
        TextBuilder::new(content).variant(TextVariant::Body)
    }

    /// Creates label text - small text for UI elements.
    /// 
    /// Use for form labels, button text, navigation items.
    /// Medium weight with smaller size for UI density.
    /// 
    /// # Arguments
    /// * `content` - The text content to display
    pub fn label(content: impl Into<String>) -> TextBuilder {
        TextBuilder::new(content).variant(TextVariant::Label)
    }

    /// Creates caption text - the smallest text for metadata.
    /// 
    /// Use for timestamps, footnotes, auxiliary information.
    /// Smallest size with muted color for subtle presentation.
    /// 
    /// # Arguments
    /// * `content` - The text content to display
    pub fn caption(content: impl Into<String>) -> TextBuilder {
        TextBuilder::new(content).variant(TextVariant::Caption)
    }

    // === Specialized Text Types ===
    
    /// Creates italic text with the default variant.
    /// 
    /// Convenience method for creating emphasized text.
    /// 
    /// # Arguments
    /// * `content` - The text content to display
    pub fn italic(content: impl Into<String>) -> TextBuilder {
        TextBuilder::new(content).italic()
    }

    /// Creates monospace text suitable for code display.
    /// 
    /// Uses the Mono font family for consistent character spacing.
    /// Ideal for code snippets, file paths, or technical content.
    /// 
    /// # Arguments
    /// * `content` - The text content to display
    pub fn code(content: impl Into<String>) -> TextBuilder {
        TextBuilder::new(content).family(FontFamily::Mono)
    }

    // === Accessibility-Focused Methods ===
    
    /// Creates text with automatic contrast optimization for a given background.
    /// 
    /// Automatically calculates the best text color to ensure high contrast
    /// against the specified background color, meeting WCAG AA guidelines.
    /// 
    /// # Arguments
    /// * `content` - The text content to display
    /// * `background` - The background color to optimize against
    pub fn on_background(content: impl Into<String>, background: Color) -> TextBuilder {
        TextBuilder::new(content)
            .on_background(background)
            .high_contrast()
    }

    /// Creates accessible text meeting WCAG AAA compliance.
    /// 
    /// Ensures the highest level of color contrast for maximum accessibility.
    /// Use when accessibility is critical or for users with visual impairments.
    /// 
    /// # Arguments
    /// * `content` - The text content to display
    /// * `background` - The background color to optimize against
    pub fn accessible(content: impl Into<String>, background: Color) -> TextBuilder {
        TextBuilder::new(content)
            .on_background(background)
            .accessible()
    }
}

/// System that applies fonts from FontAssets to text entities.
/// 
/// This system is responsible for the deferred font loading process:
/// 1. Queries all text entities with TextFontInfo markers
/// 2. Resolves font handles from the FontAssets resource
/// 3. Updates TextFont components with proper font handles
/// 4. Removes TextFontInfo markers to indicate completion
/// 
/// The system runs continuously but only processes entities that haven't
/// had their fonts applied yet (those with TextFontInfo components).
/// 
/// # Arguments
/// * `commands` - For removing marker components
/// * `text_query` - Query for text entities needing font application
/// * `font_assets` - Resource containing loaded font handles
pub fn apply_text_fonts(
    mut commands: Commands,
    mut text_query: Query<(Entity, &TextFontInfo, &mut TextFont), With<TextFontInfo>>,
    font_assets: Option<Res<FontAssets>>,
) {
    // Wait for font assets to be loaded
    let Some(font_assets) = font_assets else {
        return;
    };

    let text_count = text_query.iter().count();
    if text_count == 0 {
        return;
    }

    info!(
        "Applying fonts to {} text components using FontAssets",
        text_count
    );

    // Process each text entity that needs font application
    for (entity, font_info, mut text_font) in text_query.iter_mut() {
        // Get the appropriate font handle based on family and weight
        let font_handle = get_font_handle(&font_assets, font_info.family, font_info.weight);

        debug!(
            "Applying font to entity {:?}: family={:?}, weight={:?}, font_handle={:?}",
            entity,
            font_info.family,
            font_info.weight,
            font_handle.id()
        );

        // Update the text font with the resolved handle
        text_font.font = font_handle;

        // Remove the marker component to indicate font has been applied
        // This prevents the entity from being processed again
        commands.entity(entity).remove::<TextFontInfo>();
    }
}