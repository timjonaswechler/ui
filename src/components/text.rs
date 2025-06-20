use bevy::prelude::*;

use crate::{
    theme::{
        typography::{FontSize, TypographyAssets, SansVariant, SerifVariant, MonoVariant},
    },
    utilities::ComponentBuilder,
};

/// Text variant defining semantic meaning and default styling
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextVariant {
    #[default]
    Body,
    Caption,
    Label,
    Display,
    Title,
}

/// Text size variants for responsive typography
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextSize {
    Xs,
    Sm,
    #[default]
    Base,
    Lg,
    Xl,
    X2l,
    X3l,
    X4l,
    X5l,
    X6l,
    X7l,
    X8l,
    X9l,
}

/// Text weight variants for font weight control
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextWeight {
    Light,
    #[default]
    Regular,
    Medium,
    Bold,
}

/// Font family variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FontFamily {
    #[default]
    Sans,
    Serif,
    Mono,
}

/// Text color variants using theme colors
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum TextColor {
    #[default]
    Default,
    Muted,
    Accent,
    Error,
    Warning,
    Success,
    Custom(Color),
}

/// Builder for creating themed text components
#[derive(Debug, Clone)]
pub struct TextBuilder {
    content: String,
    variant: TextVariant,
    size: Option<TextSize>,
    weight: Option<TextWeight>,
    family: Option<FontFamily>,
    color: Option<TextColor>,
    italic: bool,
    align: Option<JustifyText>,
}

impl TextBuilder {
    /// Create a new text builder with content
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
        }
    }

    /// Set the text variant (semantic meaning)
    pub fn variant(mut self, variant: TextVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set the text size
    pub fn size(mut self, size: TextSize) -> Self {
        self.size = Some(size);
        self
    }

    /// Set the text weight
    pub fn weight(mut self, weight: TextWeight) -> Self {
        self.weight = Some(weight);
        self
    }

    /// Set the font family
    pub fn family(mut self, family: FontFamily) -> Self {
        self.family = Some(family);
        self
    }

    /// Set the text color
    pub fn color(mut self, color: TextColor) -> Self {
        self.color = Some(color);
        self
    }

    /// Make text italic
    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }

    /// Set text alignment
    pub fn align(mut self, align: JustifyText) -> Self {
        self.align = Some(align);
        self
    }

    /// Convenience method for center alignment
    pub fn center(self) -> Self {
        self.align(JustifyText::Center)
    }

    /// Convenience method for right alignment
    pub fn right(self) -> Self {
        self.align(JustifyText::Right)
    }

    /// Get the effective text size (variant default or explicit)
    fn effective_size(&self) -> TextSize {
        self.size.unwrap_or_else(|| match self.variant {
            TextVariant::Display => TextSize::X5l,
            TextVariant::Title => TextSize::X2l,
            TextVariant::Body => TextSize::Base,
            TextVariant::Label => TextSize::Sm,
            TextVariant::Caption => TextSize::Xs,
        })
    }

    /// Get the effective text weight (variant default or explicit)
    fn effective_weight(&self) -> TextWeight {
        self.weight.unwrap_or_else(|| match self.variant {
            TextVariant::Display => TextWeight::Bold,
            TextVariant::Title => TextWeight::Medium,
            TextVariant::Body => TextWeight::Regular,
            TextVariant::Label => TextWeight::Medium,
            TextVariant::Caption => TextWeight::Regular,
        })
    }

    /// Get the effective font family (variant default or explicit)
    fn effective_family(&self) -> FontFamily {
        self.family.unwrap_or_else(|| match self.variant {
            TextVariant::Display => FontFamily::Sans,
            TextVariant::Title => FontFamily::Sans,
            TextVariant::Body => FontFamily::Sans,
            TextVariant::Label => FontFamily::Sans,
            TextVariant::Caption => FontFamily::Sans,
        })
    }

    /// Get the effective text color (variant default or explicit)
    fn effective_color(&self) -> TextColor {
        self.color.unwrap_or_else(|| match self.variant {
            TextVariant::Display => TextColor::Default,
            TextVariant::Title => TextColor::Default,
            TextVariant::Body => TextColor::Default,
            TextVariant::Label => TextColor::Default,
            TextVariant::Caption => TextColor::Muted,
        })
    }

    /// Get font size in pixels from FontSize
    fn get_font_size(&self, font_size: &FontSize, size: TextSize) -> f32 {
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

    /// Get the appropriate font handle based on family, weight, and italic
    fn get_font_handle(&self, family: FontFamily, weight: TextWeight, italic: bool) -> Handle<Font> {
        // Try to get from global resources if available
        // This will be empty if assets aren't loaded yet, but will be updated by the system
        Handle::<Font>::default()
    }


    /// Convert TextColor to actual Color using theme
    fn map_color(&self, color: TextColor) -> Color {
        match color {
            TextColor::Default => Color::srgb(0.1, 0.1, 0.1), // Dark text
            TextColor::Muted => Color::srgb(0.5, 0.5, 0.5), // Muted gray
            TextColor::Accent => Color::srgb(0.2, 0.6, 0.9), // Blue accent
            TextColor::Error => Color::srgb(0.9, 0.2, 0.2), // Red
            TextColor::Warning => Color::srgb(0.9, 0.7, 0.2), // Orange
            TextColor::Success => Color::srgb(0.2, 0.8, 0.3), // Green
            TextColor::Custom(c) => c,
        }
    }
}

/// Marker component to identify text that needs font updates when assets load
#[derive(Component, Debug, Clone)]
pub struct TextFontConfig {
    pub family: FontFamily,
    pub weight: TextWeight,
    pub size: TextSize,
    pub italic: bool,
}

impl ComponentBuilder for TextBuilder {
    type Output = (bevy::ui::widget::Text, TextFont, bevy::prelude::TextColor, TextLayout, Node, TextFontConfig);

    fn build(self) -> Self::Output {
        let effective_size = self.effective_size();
        let effective_weight = self.effective_weight();
        let effective_family = self.effective_family();
        let effective_color = self.effective_color();

        let color = self.map_color(effective_color);
        let font_size = self.get_font_size(&FontSize::default(), effective_size);
        let font_handle = self.get_font_handle(effective_family, effective_weight, self.italic);

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
            TextFontConfig {
                family: effective_family,
                weight: effective_weight,
                size: effective_size,
                italic: self.italic,
            },
        )
    }
}

/// Main Text component interface
pub struct Text;

impl Text {
    /// Create a new text component with content
    ///
    /// # Example
    /// ```rust
    /// use your_crate::components::Text;
    /// 
    /// let text = Text::new("Hello World").build();
    /// ```
    pub fn new(content: impl Into<String>) -> TextBuilder {
        TextBuilder::new(content)
    }

    /// Create a display text (largest, bold)
    ///
    /// # Example
    /// ```rust
    /// let display = Text::display("Main Title").build();
    /// ```
    pub fn display(content: impl Into<String>) -> TextBuilder {
        TextBuilder::new(content).variant(TextVariant::Display)
    }

    /// Create a title text (large, medium weight)
    ///
    /// # Example  
    /// ```rust
    /// let title = Text::title("Section Title").build();
    /// ```
    pub fn title(content: impl Into<String>) -> TextBuilder {
        TextBuilder::new(content).variant(TextVariant::Title)
    }

    /// Create body text (default size and weight)
    ///
    /// # Example
    /// ```rust
    /// let body = Text::body("This is body text.").build();
    /// ```
    pub fn body(content: impl Into<String>) -> TextBuilder {
        TextBuilder::new(content).variant(TextVariant::Body)
    }

    /// Create label text (small, medium weight)
    ///
    /// # Example
    /// ```rust
    /// let label = Text::label("Field Label").build();
    /// ```
    pub fn label(content: impl Into<String>) -> TextBuilder {
        TextBuilder::new(content).variant(TextVariant::Label)
    }

    /// Create caption text (smallest, muted)
    ///
    /// # Example
    /// ```rust
    /// let caption = Text::caption("Helper text").build();
    /// ```
    pub fn caption(content: impl Into<String>) -> TextBuilder {
        TextBuilder::new(content).variant(TextVariant::Caption)
    }

    /// Create italic text
    ///
    /// # Example
    /// ```rust
    /// let italic = Text::italic("Emphasized text").build();
    /// ```
    pub fn italic(content: impl Into<String>) -> TextBuilder {
        TextBuilder::new(content).italic()
    }

    /// Create monospace text (code)
    ///
    /// # Example
    /// ```rust
    /// let code = Text::code("fn main() {}").build();
    /// ```
    pub fn code(content: impl Into<String>) -> TextBuilder {
        TextBuilder::new(content).family(FontFamily::Mono)
    }
}

/// System that initializes and updates text fonts when TypographyAssets are available
pub fn update_text_fonts(
    mut text_query: Query<(&TextFontConfig, &mut TextFont)>,
    typography_assets: Option<Res<TypographyAssets>>,
    sans_variant: Option<Res<SansVariant>>,
    serif_variant: Option<Res<SerifVariant>>,
    mono_variant: Option<Res<MonoVariant>>,
) {
    let Some(typography) = typography_assets else {
        return;
    };

    // Check if we have any text to update
    let text_count = text_query.iter().count();
    if text_count == 0 {
        return;
    }

    // Update all text components - either on asset change or first time setup
    let should_update = typography.is_changed() || 
        sans_variant.as_ref().map_or(false, |v| v.is_changed()) ||
        serif_variant.as_ref().map_or(false, |v| v.is_changed()) ||
        mono_variant.as_ref().map_or(false, |v| v.is_changed()) ||
        text_query.iter().any(|(_, font)| font.font == Handle::<Font>::default());

    if should_update {
        info!("Updating {} text components with typography assets", text_count);
        
        for (config, mut text_font) in text_query.iter_mut() {
            // Get the correct font handle with italic support
            let font_handle = get_font_handle_with_italic(
                config.family,
                config.weight,
                config.italic,
                &typography.families,
                sans_variant.as_deref(),
                serif_variant.as_deref(),
                mono_variant.as_deref(),
            );
            
            // Update font size
            let font_size = match config.size {
                TextSize::Xs => typography.size.xs,
                TextSize::Sm => typography.size.sm,
                TextSize::Base => typography.size.base,
                TextSize::Lg => typography.size.lg,
                TextSize::Xl => typography.size.xl,
                TextSize::X2l => typography.size.x2l,
                TextSize::X3l => typography.size.x3l,
                TextSize::X4l => typography.size.x4l,
                TextSize::X5l => typography.size.x5l,
                TextSize::X6l => typography.size.x6l,
                TextSize::X7l => typography.size.x7l,
                TextSize::X8l => typography.size.x8l,
                TextSize::X9l => typography.size.x9l,
            };
            
            text_font.font = font_handle;
            text_font.font_size = font_size;
        }
    }
}

/// Helper function to get the correct font handle with italic support
fn get_font_handle_with_italic(
    family: FontFamily,
    weight: TextWeight,
    italic: bool,
    basic_families: &crate::theme::typography::FontFamilies,
    sans_variant: Option<&SansVariant>,
    serif_variant: Option<&SerifVariant>,
    mono_variant: Option<&MonoVariant>,
) -> Handle<Font> {
    match family {
        FontFamily::Sans => {
            if let Some(sans) = sans_variant {
                match (weight, italic) {
                    (TextWeight::Light, false) => sans.sans_light.clone(),
                    (TextWeight::Light, true) => sans.sans_light_italic.clone(),
                    (TextWeight::Regular, false) => sans.sans_regular.clone(),
                    (TextWeight::Regular, true) => sans.sans_regular_italic.clone(),
                    (TextWeight::Medium, false) => sans.sans_medium.clone(),
                    (TextWeight::Medium, true) => sans.sans_medium_italic.clone(),
                    (TextWeight::Bold, false) => sans.sans_bold.clone(),
                    (TextWeight::Bold, true) => sans.sans_bold_italic.clone(),
                }
            } else {
                // Fallback to basic families
                match weight {
                    TextWeight::Light => basic_families.sans_light.clone(),
                    TextWeight::Regular => basic_families.sans_regular.clone(),
                    TextWeight::Medium => basic_families.sans_medium.clone(),
                    TextWeight::Bold => basic_families.sans_bold.clone(),
                }
            }
        },
        FontFamily::Serif => {
            if let Some(serif) = serif_variant {
                match (weight, italic) {
                    (TextWeight::Light, false) => serif.serif_light.clone(),
                    (TextWeight::Light, true) => serif.serif_light_italic.clone(),
                    (TextWeight::Regular, false) => serif.serif_regular.clone(),
                    (TextWeight::Regular, true) => serif.serif_regular_italic.clone(),
                    (TextWeight::Medium, false) => serif.serif_medium.clone(),
                    (TextWeight::Medium, true) => serif.serif_medium_italic.clone(),
                    (TextWeight::Bold, false) => serif.serif_bold.clone(),
                    (TextWeight::Bold, true) => serif.serif_bold_italic.clone(),
                }
            } else {
                // Fallback to basic families
                match weight {
                    TextWeight::Regular | TextWeight::Light | TextWeight::Medium => basic_families.serif_regular.clone(),
                    TextWeight::Bold => basic_families.serif_bold.clone(),
                }
            }
        },
        FontFamily::Mono => {
            if let Some(mono) = mono_variant {
                match (weight, italic) {
                    (TextWeight::Light, false) => mono.mono_light.clone(),
                    (TextWeight::Light, true) => mono.mono_light_italic.clone(),
                    (TextWeight::Regular, false) => mono.mono_regular.clone(),
                    (TextWeight::Regular, true) => mono.mono_regular_italic.clone(),
                    (TextWeight::Medium, false) => mono.mono_medium.clone(),
                    (TextWeight::Medium, true) => mono.mono_medium_italic.clone(),
                    (TextWeight::Bold, false) => mono.mono_bold.clone(),
                    (TextWeight::Bold, true) => mono.mono_bold_italic.clone(),
                }
            } else {
                // Fallback to basic families
                match weight {
                    TextWeight::Regular | TextWeight::Light | TextWeight::Medium => basic_families.mono_regular.clone(),
                    TextWeight::Bold => basic_families.mono_bold.clone(),
                }
            }
        },
    }
}

