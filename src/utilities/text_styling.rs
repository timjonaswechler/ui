use bevy::prelude::*;

use crate::{
    theme::{
        typography::{TextSize, TextWeight, FontFamily, TextVariant},
        color::{TextColor as TextColorEnum, TextContrastLevel},
    },
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