use bevy::prelude::*;
use super::{ThemeTokens, SemanticTokens, SpacingScale, BorderRadiusScale, TypographyScale};

pub trait ThemeAware {
    fn themed_background_color(&self, tokens: &SemanticTokens, variant: BackgroundVariant) -> BackgroundColor;
    fn themed_text_color(&self, tokens: &SemanticTokens, variant: TextVariant) -> TextColor;
    fn themed_border_color(&self, tokens: &SemanticTokens, variant: BorderVariant) -> BorderColor;
    fn themed_border_radius(&self, spacing: &BorderRadiusScale, size: RadiusSize) -> BorderRadius;
    fn themed_padding(&self, spacing: &SpacingScale, size: SpacingSize) -> UiRect;
    fn themed_margin(&self, spacing: &SpacingScale, size: SpacingSize) -> UiRect;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackgroundVariant {
    Default,
    Muted,
    Card,
    Popover,
    Primary,
    Secondary,
    Accent,
    Destructive,
    Transparent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextVariant {
    Default,
    Muted,
    Primary,
    Secondary,
    Accent,
    Destructive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BorderVariant {
    Default,
    Input,
    Ring,
    Muted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpacingSize {
    Px,
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
    Xl2,
    Xl3,
    Xl4,
    Xl5,
    Xl6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RadiusSize {
    None,
    Sm,
    Md,
    Lg,
    Xl,
    Full,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypographySize {
    Xs,
    Sm,
    Base,
    Lg,
    Xl,
    Xl2,
    Xl3,
    Xl4,
}

impl<T> ThemeAware for T {
    fn themed_background_color(&self, tokens: &SemanticTokens, variant: BackgroundVariant) -> BackgroundColor {
        let color = match variant {
            BackgroundVariant::Default => tokens.background,
            BackgroundVariant::Muted => tokens.muted,
            BackgroundVariant::Card => tokens.card,
            BackgroundVariant::Popover => tokens.popover,
            BackgroundVariant::Primary => tokens.primary,
            BackgroundVariant::Secondary => tokens.secondary,
            BackgroundVariant::Accent => tokens.accent,
            BackgroundVariant::Destructive => tokens.destructive,
            BackgroundVariant::Transparent => Color::NONE,
        };
        BackgroundColor(color)
    }

    fn themed_text_color(&self, tokens: &SemanticTokens, variant: TextVariant) -> TextColor {
        let color = match variant {
            TextVariant::Default => tokens.foreground,
            TextVariant::Muted => tokens.muted_foreground,
            TextVariant::Primary => tokens.primary_foreground,
            TextVariant::Secondary => tokens.secondary_foreground,
            TextVariant::Accent => tokens.accent_foreground,
            TextVariant::Destructive => tokens.destructive_foreground,
        };
        TextColor(color)
    }

    fn themed_border_color(&self, tokens: &SemanticTokens, variant: BorderVariant) -> BorderColor {
        let color = match variant {
            BorderVariant::Default => tokens.border,
            BorderVariant::Input => tokens.input,
            BorderVariant::Ring => tokens.ring,
            BorderVariant::Muted => tokens.muted,
        };
        BorderColor(color)
    }

    fn themed_border_radius(&self, radius: &BorderRadiusScale, size: RadiusSize) -> BorderRadius {
        let value = match size {
            RadiusSize::None => radius.none,
            RadiusSize::Sm => radius.sm,
            RadiusSize::Md => radius.md,
            RadiusSize::Lg => radius.lg,
            RadiusSize::Xl => radius.xl,
            RadiusSize::Full => radius.full,
        };
        BorderRadius::all(Val::Px(value))
    }

    fn themed_padding(&self, spacing: &SpacingScale, size: SpacingSize) -> UiRect {
        let value = spacing.get_value(size);
        UiRect::all(Val::Px(value))
    }

    fn themed_margin(&self, spacing: &SpacingScale, size: SpacingSize) -> UiRect {
        let value = spacing.get_value(size);
        UiRect::all(Val::Px(value))
    }
}

impl SpacingScale {
    pub fn get_value(&self, size: SpacingSize) -> f32 {
        match size {
            SpacingSize::Px => self.px,
            SpacingSize::Xs => self.xs,
            SpacingSize::Sm => self.sm,
            SpacingSize::Md => self.md,
            SpacingSize::Lg => self.lg,
            SpacingSize::Xl => self.xl,
            SpacingSize::Xl2 => self.xl2,
            SpacingSize::Xl3 => self.xl3,
            SpacingSize::Xl4 => self.xl4,
            SpacingSize::Xl5 => self.xl5,
            SpacingSize::Xl6 => self.xl6,
        }
    }
}

impl TypographyScale {
    pub fn get_font_size(&self, size: TypographySize) -> f32 {
        match size {
            TypographySize::Xs => self.xs.size,
            TypographySize::Sm => self.sm.size,
            TypographySize::Base => self.base.size,
            TypographySize::Lg => self.lg.size,
            TypographySize::Xl => self.xl.size,
            TypographySize::Xl2 => self.xl2.size,
            TypographySize::Xl3 => self.xl3.size,
            TypographySize::Xl4 => self.xl4.size,
        }
    }

    pub fn get_line_height(&self, size: TypographySize) -> f32 {
        match size {
            TypographySize::Xs => self.xs.line_height,
            TypographySize::Sm => self.sm.line_height,
            TypographySize::Base => self.base.line_height,
            TypographySize::Lg => self.lg.line_height,
            TypographySize::Xl => self.xl.line_height,
            TypographySize::Xl2 => self.xl2.line_height,
            TypographySize::Xl3 => self.xl3.line_height,
            TypographySize::Xl4 => self.xl4.line_height,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ThemedNode {
    pub base: Node,
    pub background: BackgroundColor,
    pub border: BorderColor,
    pub border_radius: BorderRadius,
}

impl ThemedNode {
    pub fn new(tokens: &ThemeTokens) -> Self {
        Self {
            base: Node::default(),
            background: BackgroundColor(tokens.semantic.background),
            border: BorderColor(tokens.semantic.border),
            border_radius: BorderRadius::all(Val::Px(tokens.radius.md)),
        }
    }

    pub fn with_background_variant(mut self, tokens: &ThemeTokens, variant: BackgroundVariant) -> Self {
        self.background = self.themed_background_color(&tokens.semantic, variant);
        self
    }

    pub fn with_border_variant(mut self, tokens: &ThemeTokens, variant: BorderVariant) -> Self {
        self.border = self.themed_border_color(&tokens.semantic, variant);
        self
    }

    pub fn with_radius_size(mut self, tokens: &ThemeTokens, size: RadiusSize) -> Self {
        self.border_radius = self.themed_border_radius(&tokens.radius, size);
        self
    }

    pub fn with_padding(mut self, tokens: &ThemeTokens, size: SpacingSize) -> Self {
        self.base.padding = self.themed_padding(&tokens.spacing, size);
        self
    }

    pub fn with_margin(mut self, tokens: &ThemeTokens, size: SpacingSize) -> Self {
        self.base.margin = self.themed_margin(&tokens.spacing, size);
        self
    }

    pub fn build(self) -> impl Bundle {
        (
            self.base,
            self.background,
            self.border,
            self.border_radius,
        )
    }
}

#[derive(Debug, Clone)]
pub struct ThemedText {
    pub text: Text,
    pub color: TextColor,
}

impl ThemedText {
    pub fn new(content: impl Into<String>, tokens: &ThemeTokens) -> Self {
        Self {
            text: Text::new(content),
            color: TextColor(tokens.semantic.foreground),
        }
    }

    pub fn with_variant(mut self, tokens: &ThemeTokens, variant: TextVariant) -> Self {
        self.color = self.themed_text_color(&tokens.semantic, variant);
        self
    }

    pub fn with_typography_size(mut self, tokens: &ThemeTokens, size: TypographySize) -> Self {
        let font_size = tokens.typography.get_font_size(size);
        // Note: In Bevy, text styling is more complex and may require TextStyle
        // This is a simplified version - you might need to expand this based on your needs
        self
    }

    pub fn build(self) -> impl Bundle {
        (self.text, self.color)
    }
}