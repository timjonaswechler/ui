use crate::plugin::{FONT_SIZE_BASE, SCALING};
use bevy::prelude::*;

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
/// Font size configuration structure
#[derive(Debug, Clone)]
pub struct FontSize {
    pub xs: f32,
    pub sm: f32,
    pub base: f32,
    pub lg: f32,
    pub xl: f32,
    pub x2l: f32,
    pub x3l: f32,
    pub x4l: f32,
    pub x5l: f32,
    pub x6l: f32,
    pub x7l: f32,
    pub x8l: f32,
    pub x9l: f32,
}

// Implementierung von `Default` für die gesamte Theme-Struktur.
// Die Asset-Handles werden von bevy_asset_loader befüllt, wir müssen hier nur
// die Standardwerte für die Konfigurationsdaten (wie Schriftgrößen) festlegen.
impl Default for FontSize {
    fn default() -> Self {
        Self {
            xs: 0.75 * FONT_SIZE_BASE * SCALING,
            sm: 0.875 * FONT_SIZE_BASE * SCALING,
            base: 1.0 * FONT_SIZE_BASE * SCALING,
            lg: 1.125 * FONT_SIZE_BASE * SCALING,
            xl: 1.25 * FONT_SIZE_BASE * SCALING,
            x2l: 1.5 * FONT_SIZE_BASE * SCALING,
            x3l: 1.875 * FONT_SIZE_BASE * SCALING,
            x4l: 2.25 * FONT_SIZE_BASE * SCALING,
            x5l: 3.0 * FONT_SIZE_BASE * SCALING,
            x6l: 3.75 * FONT_SIZE_BASE * SCALING,
            x7l: 4.5 * FONT_SIZE_BASE * SCALING,
            x8l: 6.0 * FONT_SIZE_BASE * SCALING,
            x9l: 8.0 * FONT_SIZE_BASE * SCALING,
        }
    }
}


/// Unified font assets resource for direct font loading
/// This replaces the old FontFamilies struct for simplified asset management
#[derive(Resource, Debug, Clone)]
pub struct FontAssets {
    // Sans Familie
    pub sans_light: Handle<Font>,
    pub sans_regular: Handle<Font>,
    pub sans_medium: Handle<Font>,
    pub sans_bold: Handle<Font>,

    // Serif Familie
    pub serif_regular: Handle<Font>,
    pub serif_bold: Handle<Font>,

    // Mono Familie
    pub mono_regular: Handle<Font>,
    pub mono_bold: Handle<Font>,
}

/// Get font handle based on family and weight
pub fn get_font_handle(assets: &FontAssets, family: FontFamily, weight: TextWeight) -> Handle<Font> {
    match family {
        FontFamily::Sans => match weight {
            TextWeight::Light => assets.sans_light.clone(),
            TextWeight::Regular => assets.sans_regular.clone(),
            TextWeight::Medium => assets.sans_medium.clone(),
            TextWeight::Bold => assets.sans_bold.clone(),
        },
        FontFamily::Serif => match weight {
            TextWeight::Light | TextWeight::Regular | TextWeight::Medium => {
                assets.serif_regular.clone()
            }
            TextWeight::Bold => assets.serif_bold.clone(),
        },
        FontFamily::Mono => match weight {
            TextWeight::Light | TextWeight::Regular | TextWeight::Medium => {
                assets.mono_regular.clone()
            }
            TextWeight::Bold => assets.mono_bold.clone(),
        },
    }
}

/// Get font size in pixels from FontSize enum
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

/// Get effective text size based on variant defaults
pub fn get_effective_text_size(variant: TextVariant, explicit_size: Option<TextSize>) -> TextSize {
    explicit_size.unwrap_or_else(|| match variant {
        TextVariant::Display => TextSize::X5l,
        TextVariant::Title => TextSize::X2l,
        TextVariant::Body => TextSize::Base,
        TextVariant::Label => TextSize::Sm,
        TextVariant::Caption => TextSize::Xs,
    })
}

/// Get effective text weight based on variant defaults
pub fn get_effective_text_weight(variant: TextVariant, explicit_weight: Option<TextWeight>) -> TextWeight {
    explicit_weight.unwrap_or_else(|| match variant {
        TextVariant::Display => TextWeight::Bold,
        TextVariant::Title => TextWeight::Medium,
        TextVariant::Body => TextWeight::Regular,
        TextVariant::Label => TextWeight::Medium,
        TextVariant::Caption => TextWeight::Regular,
    })
}

/// Get effective font family based on variant defaults
pub fn get_effective_font_family(variant: TextVariant, explicit_family: Option<FontFamily>) -> FontFamily {
    explicit_family.unwrap_or_else(|| match variant {
        TextVariant::Display => FontFamily::Sans,
        TextVariant::Title => FontFamily::Sans,
        TextVariant::Body => FontFamily::Sans,
        TextVariant::Label => FontFamily::Sans,
        TextVariant::Caption => FontFamily::Sans,
    })
}

/// Startup system that loads all font assets directly using the asset server
pub fn load_font_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Loading font assets...");

    let font_assets = FontAssets {
        // Sans Familie laden
        sans_light: asset_server.load("fonts/Roboto-Light.ttf"),
        sans_regular: asset_server.load("fonts/Roboto-Regular.ttf"),
        sans_medium: asset_server.load("fonts/Roboto-Medium.ttf"),
        sans_bold: asset_server.load("fonts/Roboto-Bold.ttf"),

        // Serif Familie laden
        serif_regular: asset_server.load("fonts/NotoSerif-Regular.ttf"),
        serif_bold: asset_server.load("fonts/NotoSerif-Bold.ttf"),

        // Mono Familie laden
        mono_regular: asset_server.load("fonts/RobotoMono-Regular.ttf"),
        mono_bold: asset_server.load("fonts/RobotoMono-Bold.ttf"),
    };

    commands.insert_resource(font_assets);
    info!("Font assets resource created and inserted");
}
