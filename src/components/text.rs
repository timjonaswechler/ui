use bevy::prelude::*;

use crate::{
    theme::typography::{FontAssets, FontSize},
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

    /// Get direct font handle using global font assets (will be filled by system)
    fn get_direct_font_handle(&self, family: FontFamily, weight: TextWeight) -> Handle<Font> {
        // This will be updated by a system once FontAssets are loaded
        // For now return default, but we'll add a system to update these
        Handle::<Font>::default()
    }

    /// Convert TextColor to actual Color using theme-aware semantic palettes
    fn map_color(&self, color: TextColor) -> Color {
        use crate::theme::color::{
            accent_palette, error_palette, success_palette, theme, warning_palette,
        };

        match color {
            TextColor::Default => {
                // Use theme-aware gray palette for default text
                let gray_palette = theme().gray;
                gray_palette.text_contrast // High contrast text
            }
            TextColor::Muted => {
                // Use theme-aware gray palette for muted text
                let gray_palette = theme().gray;
                gray_palette.text // Medium contrast text
            }
            TextColor::Accent => {
                // Use dynamic accent palette
                accent_palette().text
            }
            TextColor::Error => {
                // Use red palette for errors
                error_palette().text
            }
            TextColor::Warning => {
                // Use warning palette (should be orange/yellow)
                warning_palette().text
            }
            TextColor::Success => {
                // Use success palette (green variants)
                success_palette().text
            }
            TextColor::Custom(c) => c,
        }
    }
}

impl ComponentBuilder for TextBuilder {
    type Output = (
        bevy::ui::widget::Text,
        TextFont,
        bevy::prelude::TextColor,
        TextLayout,
        Node,
        TextFontInfo,
    );

    fn build(self) -> Self::Output {
        let effective_size = self.effective_size();
        let effective_weight = self.effective_weight();
        let effective_family = self.effective_family();
        let effective_color = self.effective_color();

        let color = self.map_color(effective_color);
        let font_size = self.get_font_size(&FontSize::default(), effective_size);

        // Direct font selection based on family and weight
        let font_handle = self.get_direct_font_handle(effective_family, effective_weight);

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

/// Marker component to store font configuration for text entities
#[derive(Component, Debug, Clone)]
pub struct TextFontInfo {
    pub family: FontFamily,
    pub weight: TextWeight,
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

/// System that applies fonts directly from FontAssets to text entities
pub fn apply_text_fonts(
    mut commands: Commands,
    mut text_query: Query<(Entity, &TextFontInfo, &mut TextFont), With<TextFontInfo>>,
    font_assets: Option<Res<FontAssets>>,
) {
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

    for (entity, font_info, mut text_font) in text_query.iter_mut() {
        // Direct font selection from FontAssets
        let font_handle = match font_info.family {
            FontFamily::Sans => match font_info.weight {
                TextWeight::Light => font_assets.sans_light.clone(),
                TextWeight::Regular => font_assets.sans_regular.clone(),
                TextWeight::Medium => font_assets.sans_medium.clone(),
                TextWeight::Bold => font_assets.sans_bold.clone(),
            },
            FontFamily::Serif => match font_info.weight {
                TextWeight::Light | TextWeight::Regular | TextWeight::Medium => {
                    font_assets.serif_regular.clone()
                }
                TextWeight::Bold => font_assets.serif_bold.clone(),
            },
            FontFamily::Mono => match font_info.weight {
                TextWeight::Light | TextWeight::Regular | TextWeight::Medium => {
                    font_assets.mono_regular.clone()
                }
                TextWeight::Bold => font_assets.mono_bold.clone(),
            },
        };

        info!(
            "Applying font to entity {:?}: family={:?}, weight={:?}, font_handle={:?}",
            entity,
            font_info.family,
            font_info.weight,
            font_handle.id()
        );

        text_font.font = font_handle;

        // Remove the marker component to indicate font has been applied
        commands.entity(entity).remove::<TextFontInfo>();
    }
}
