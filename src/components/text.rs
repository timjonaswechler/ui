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
    utilities::ComponentBuilder,
};

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
    background_context: Option<Color>,
    contrast_level: Option<TextContrastLevel>,
    explicit_color_set: bool,
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
            background_context: None,
            contrast_level: Some(TextContrastLevel::High),
            explicit_color_set: false,
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

    /// Set the text color (disables automatic contrast optimization)
    pub fn color(mut self, color: TextColor) -> Self {
        self.color = Some(color);
        self.explicit_color_set = true;
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

    /// Set background context for intelligent contrast calculation
    pub fn on_background(mut self, background_color: Color) -> Self {
        self.background_context = Some(background_color);
        self
    }

    /// Set contrast level for accessibility
    pub fn contrast_level(mut self, level: TextContrastLevel) -> Self {
        self.contrast_level = Some(level);
        self
    }

    /// Convenience method for high contrast text (WCAG AA compliant)
    pub fn high_contrast(self) -> Self {
        self.contrast_level(TextContrastLevel::High)
    }

    /// Convenience method for accessible text (WCAG AAA compliant)
    pub fn accessible(self) -> Self {
        self.contrast_level(TextContrastLevel::Accessible)
    }

    /// Enable automatic background detection and contrast optimization
    pub fn auto_contrast(self) -> Self {
        self
    }

    /// Disable automatic contrast optimization (use explicit colors only)
    pub fn manual_color(mut self) -> Self {
        self.explicit_color_set = true;
        self
    }

    /// Convert TextColor to actual Color using intelligent contrast calculation
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

/// Marker component to store font configuration for text entities
#[derive(Component, Debug, Clone)]
pub struct TextFontInfo {
    pub family: FontFamily,
    pub weight: TextWeight,
}

/// Main Text component interface
pub struct Text;

impl Text {
    /// Create a new text component with automatic contrast optimization
    pub fn new(content: impl Into<String>) -> TextBuilder {
        TextBuilder::new(content)
    }

    /// Create a display text (largest, bold)
    pub fn display(content: impl Into<String>) -> TextBuilder {
        TextBuilder::new(content).variant(TextVariant::Display)
    }

    /// Create a title text (large, medium weight)
    pub fn title(content: impl Into<String>) -> TextBuilder {
        TextBuilder::new(content).variant(TextVariant::Title)
    }

    /// Create body text (default size and weight)
    pub fn body(content: impl Into<String>) -> TextBuilder {
        TextBuilder::new(content).variant(TextVariant::Body)
    }

    /// Create label text (small, medium weight)
    pub fn label(content: impl Into<String>) -> TextBuilder {
        TextBuilder::new(content).variant(TextVariant::Label)
    }

    /// Create caption text (smallest, muted)
    pub fn caption(content: impl Into<String>) -> TextBuilder {
        TextBuilder::new(content).variant(TextVariant::Caption)
    }

    /// Create italic text
    pub fn italic(content: impl Into<String>) -> TextBuilder {
        TextBuilder::new(content).italic()
    }

    /// Create monospace text (code)
    pub fn code(content: impl Into<String>) -> TextBuilder {
        TextBuilder::new(content).family(FontFamily::Mono)
    }

    /// Create text with automatic contrast optimization for given background
    pub fn on_background(content: impl Into<String>, background: Color) -> TextBuilder {
        TextBuilder::new(content)
            .on_background(background)
            .high_contrast()
    }

    /// Create accessible text (WCAG AAA compliant) for given background
    pub fn accessible(content: impl Into<String>, background: Color) -> TextBuilder {
        TextBuilder::new(content)
            .on_background(background)
            .accessible()
    }
}

/// System that applies fonts from FontAssets to text entities
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
        let font_handle = get_font_handle(&font_assets, font_info.family, font_info.weight);

        debug!(
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