use bevy::prelude::*;

use crate::{
    components::text::{TextBuilder, TextSize, TextWeight, FontFamily, TextColor as TextColorEnum},
    theme::color::TextContrastLevel,
    utilities::ComponentBuilder,
};

/// Common trait for components that contain text and support advanced text styling
/// 
/// This trait provides a unified API for text styling across different UI components
/// like buttons, headings, labels, etc.
pub trait TextStyler {
    /// Set the text size
    fn text_size(self, size: TextSize) -> Self;
    
    /// Set the text weight
    fn text_weight(self, weight: TextWeight) -> Self;
    
    /// Set the font family
    fn text_family(self, family: FontFamily) -> Self;
    
    /// Make text italic
    fn text_italic(self) -> Self;
    
    /// Set text alignment
    fn text_align(self, align: JustifyText) -> Self;
    
    /// Center align text
    fn text_center(self) -> Self;
    
    /// Right align text
    fn text_right(self) -> Self;
    
    /// Set background context for intelligent contrast calculation
    fn text_on_background(self, background_color: Color) -> Self;
    
    /// Set contrast level for accessibility
    fn text_contrast_level(self, level: TextContrastLevel) -> Self;
    
    /// Enable high contrast text (WCAG AA compliant)
    fn text_high_contrast(self) -> Self;
    
    /// Enable accessible text (WCAG AAA compliant)
    fn text_accessible(self) -> Self;
    
    /// Enable automatic contrast optimization
    fn text_auto_contrast(self) -> Self;
    
    /// Disable automatic contrast optimization (use explicit colors only)
    fn text_manual_color(self) -> Self;
    
    /// Set explicit text color
    fn text_color(self, color: TextColorEnum) -> Self;
}

/// Convenience trait for quick text styling presets
pub trait TextStylePresets: TextStyler + Sized {
    /// Apply heading-like styling (larger, bold, high contrast)
    fn as_heading_style(self) -> Self {
        self.text_weight(TextWeight::Bold)
            .text_size(TextSize::Lg)
            .text_high_contrast()
    }
    
    /// Apply body text styling (normal size and weight)
    fn as_body_style(self) -> Self {
        self.text_weight(TextWeight::Regular)
            .text_size(TextSize::Base)
    }
    
    /// Apply caption styling (smaller, muted)
    fn as_caption_style(self) -> Self {
        self.text_weight(TextWeight::Regular)
            .text_size(TextSize::Sm)
            .text_color(TextColorEnum::Muted)
    }
    
    /// Apply label styling (medium weight, small)
    fn as_label_style(self) -> Self {
        self.text_weight(TextWeight::Medium)
            .text_size(TextSize::Sm)
    }
    
    /// Apply code styling (monospace, slightly smaller)
    fn as_code_style(self) -> Self {
        self.text_family(FontFamily::Mono)
            .text_size(TextSize::Sm)
    }
    
    /// Apply emphasis styling (italic, medium weight)
    fn as_emphasis_style(self) -> Self {
        self.text_italic()
            .text_weight(TextWeight::Medium)
    }
    
    /// Apply accessible styling for critical text
    fn as_accessible_style(self) -> Self {
        self.text_accessible()
            .text_weight(TextWeight::Medium)
    }
}

// Blanket implementation for all TextStyler implementors
impl<T: TextStyler + Sized> TextStylePresets for T {}

/// Helper function to create a text builder with common styling
pub fn styled_text(content: impl Into<String>) -> StyledTextBuilder {
    StyledTextBuilder {
        builder: crate::components::text::Text::new(content),
    }
}

/// Wrapper around TextBuilder that implements TextStyler
pub struct StyledTextBuilder {
    builder: TextBuilder,
}

impl StyledTextBuilder {
    /// Build the final text component
    pub fn build(self) -> impl Bundle {
        self.builder.build()
    }
    
    /// Get the underlying TextBuilder
    pub fn into_text_builder(self) -> TextBuilder {
        self.builder
    }
}

impl TextStyler for StyledTextBuilder {
    fn text_size(mut self, size: TextSize) -> Self {
        self.builder = self.builder.size(size);
        self
    }
    
    fn text_weight(mut self, weight: TextWeight) -> Self {
        self.builder = self.builder.weight(weight);
        self
    }
    
    fn text_family(mut self, family: FontFamily) -> Self {
        self.builder = self.builder.family(family);
        self
    }
    
    fn text_italic(mut self) -> Self {
        self.builder = self.builder.italic();
        self
    }
    
    fn text_align(mut self, align: JustifyText) -> Self {
        self.builder = self.builder.align(align);
        self
    }
    
    fn text_center(mut self) -> Self {
        self.builder = self.builder.center();
        self
    }
    
    fn text_right(mut self) -> Self {
        self.builder = self.builder.right();
        self
    }
    
    fn text_on_background(mut self, background_color: Color) -> Self {
        self.builder = self.builder.on_background(background_color);
        self
    }
    
    fn text_contrast_level(mut self, level: TextContrastLevel) -> Self {
        self.builder = self.builder.contrast_level(level);
        self
    }
    
    fn text_high_contrast(mut self) -> Self {
        self.builder = self.builder.high_contrast();
        self
    }
    
    fn text_accessible(mut self) -> Self {
        self.builder = self.builder.accessible();
        self
    }
    
    fn text_auto_contrast(mut self) -> Self {
        self.builder = self.builder.auto_contrast();
        self
    }
    
    fn text_manual_color(mut self) -> Self {
        self.builder = self.builder.manual_color();
        self
    }
    
    fn text_color(mut self, color: TextColorEnum) -> Self {
        self.builder = self.builder.color(color);
        self
    }
}