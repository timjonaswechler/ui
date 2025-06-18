use bevy::prelude::*;
use super::{RadixTheme, ThemeMode, ColorScale};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SemanticTokens {
    pub background: Color,
    pub foreground: Color,
    pub muted: Color,
    pub muted_foreground: Color,
    pub popover: Color,
    pub popover_foreground: Color,
    pub card: Color,
    pub card_foreground: Color,
    pub border: Color,
    pub input: Color,
    pub primary: Color,
    pub primary_foreground: Color,
    pub secondary: Color,
    pub secondary_foreground: Color,
    pub accent: Color,
    pub accent_foreground: Color,
    pub destructive: Color,
    pub destructive_foreground: Color,
    pub ring: Color,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpacingScale {
    pub px: f32,
    pub xs: f32,
    pub sm: f32,
    pub md: f32,
    pub lg: f32,
    pub xl: f32,
    pub xl2: f32,
    pub xl3: f32,
    pub xl4: f32,
    pub xl5: f32,
    pub xl6: f32,
}

impl Default for SpacingScale {
    fn default() -> Self {
        Self {
            px: 1.0,
            xs: 2.0,
            sm: 4.0,
            md: 8.0,
            lg: 16.0,
            xl: 24.0,
            xl2: 32.0,
            xl3: 48.0,
            xl4: 64.0,
            xl5: 96.0,
            xl6: 128.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BorderRadiusScale {
    pub none: f32,
    pub sm: f32,
    pub md: f32,
    pub lg: f32,
    pub xl: f32,
    pub full: f32,
}

impl Default for BorderRadiusScale {
    fn default() -> Self {
        Self {
            none: 0.0,
            sm: 2.0,
            md: 4.0,
            lg: 8.0,
            xl: 12.0,
            full: 9999.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TypographyScale {
    pub xs: FontSizeData,
    pub sm: FontSizeData,
    pub base: FontSizeData,
    pub lg: FontSizeData,
    pub xl: FontSizeData,
    pub xl2: FontSizeData,
    pub xl3: FontSizeData,
    pub xl4: FontSizeData,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FontSizeData {
    pub size: f32,
    pub line_height: f32,
}

impl Default for TypographyScale {
    fn default() -> Self {
        Self {
            xs: FontSizeData { size: 12.0, line_height: 16.0 },
            sm: FontSizeData { size: 14.0, line_height: 20.0 },
            base: FontSizeData { size: 16.0, line_height: 24.0 },
            lg: FontSizeData { size: 18.0, line_height: 28.0 },
            xl: FontSizeData { size: 20.0, line_height: 28.0 },
            xl2: FontSizeData { size: 24.0, line_height: 32.0 },
            xl3: FontSizeData { size: 30.0, line_height: 36.0 },
            xl4: FontSizeData { size: 36.0, line_height: 40.0 },
        }
    }
}

impl SemanticTokens {
    pub fn from_theme(theme: &RadixTheme, mode: &ThemeMode) -> Self {
        let accent_scale = theme.get_accent_scale(mode);
        let gray_scale = theme.get_gray_scale(mode);

        match mode {
            ThemeMode::Light => Self::light_tokens(accent_scale, gray_scale),
            ThemeMode::Dark => Self::dark_tokens(accent_scale, gray_scale),
        }
    }

    fn light_tokens(accent: ColorScale, gray: ColorScale) -> Self {
        Self {
            background: Color::WHITE,
            foreground: gray.step_12,
            muted: gray.step_2,
            muted_foreground: gray.step_11,
            popover: Color::WHITE,
            popover_foreground: gray.step_12,
            card: Color::WHITE,
            card_foreground: gray.step_12,
            border: gray.step_6,
            input: gray.step_6,
            primary: accent.step_9,
            primary_foreground: Color::WHITE,
            secondary: gray.step_3,
            secondary_foreground: gray.step_11,
            accent: accent.step_4,
            accent_foreground: accent.step_11,
            destructive: Color::srgb(0.937, 0.267, 0.267), // Red-500 equivalent
            destructive_foreground: Color::WHITE,
            ring: accent.step_7,
        }
    }

    fn dark_tokens(accent: ColorScale, gray: ColorScale) -> Self {
        Self {
            background: gray.step_1,
            foreground: gray.step_12,
            muted: gray.step_3,
            muted_foreground: gray.step_11,
            popover: gray.step_2,
            popover_foreground: gray.step_12,
            card: gray.step_2,
            card_foreground: gray.step_12,
            border: gray.step_6,
            input: gray.step_6,
            primary: accent.step_9,
            primary_foreground: Color::WHITE,
            secondary: gray.step_3,
            secondary_foreground: gray.step_11,
            accent: accent.step_4,
            accent_foreground: accent.step_11,
            destructive: Color::srgb(0.937, 0.267, 0.267), // Red-500 equivalent
            destructive_foreground: Color::WHITE,
            ring: accent.step_7,
        }
    }
}

#[derive(Resource, Debug, Clone)]
pub struct ThemeTokens {
    pub semantic: SemanticTokens,
    pub spacing: SpacingScale,
    pub radius: BorderRadiusScale,
    pub typography: TypographyScale,
}

impl ThemeTokens {
    pub fn from_theme(theme: &RadixTheme, mode: &ThemeMode) -> Self {
        Self {
            semantic: SemanticTokens::from_theme(theme, mode),
            spacing: SpacingScale::default(),
            radius: BorderRadiusScale::default(),
            typography: TypographyScale::default(),
        }
    }
}