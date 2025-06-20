// z.B. in crates/forge_ui/src/theme/typography.rs
use crate::plugin::{FONT_SIZE_BASE, SCALING};
use bevy::asset::Asset;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
/// Die zentrale Ressource, die sowohl die geladenen Schriftart-Assets
/// als auch die Konfiguration für Schriftgrößen enthält.
///
/// Diese Struktur wird von `bevy_asset_loader` befüllt und dann als
/// Bevy-Ressource in die Welt eingefügt.
#[derive(AssetCollection, Resource, Debug, Clone)]
pub struct TypographyAssets {
    pub size: FontSize,
    pub families: FontFamilies,
}
/// Datenstruktur nur für die Definition der Schriftgrößen.
/// Identisch zu Ihrer `UiFontSizeData`.
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

/// Enthält die Handles für die verschiedenen Schriftart-Familien (sans, serif, mono).
/// Dies ist eine verschachtelte AssetCollection.
#[derive(AssetCollection, Resource, Debug, Clone, Default)]
pub struct FontFamilies {
    /// Das Standard-Fallback-Font.
    #[asset(path = "fonts/Roboto-Regular.ttf")]
    pub default: Handle<Font>,

    /// Sans fonts
    #[asset(path = "fonts/Roboto-Light.ttf")]
    pub sans_light: Handle<Font>,
    #[asset(path = "fonts/Roboto-Regular.ttf")]
    pub sans_regular: Handle<Font>,
    #[asset(path = "fonts/Roboto-Medium.ttf")]
    pub sans_medium: Handle<Font>,
    #[asset(path = "fonts/Roboto-Bold.ttf")]
    pub sans_bold: Handle<Font>,
    
    /// Serif fonts
    #[asset(path = "fonts/NotoSerif-Regular.ttf")]
    pub serif_regular: Handle<Font>,
    #[asset(path = "fonts/NotoSerif-Bold.ttf")]
    pub serif_bold: Handle<Font>,
    
    /// Mono fonts
    #[asset(path = "fonts/RobotoMono-Regular.ttf")]
    pub mono_regular: Handle<Font>,
    #[asset(path = "fonts/RobotoMono-Bold.ttf")]
    pub mono_bold: Handle<Font>,
}

#[derive(AssetCollection, Resource, Asset, TypePath, Debug, Clone)]
pub struct SansVariant {
    #[asset(path = "fonts/Roboto-Light.ttf")]
    pub sans_light: Handle<bevy::prelude::Font>,
    #[asset(path = "fonts/Roboto-LightItalic.ttf")]
    pub sans_light_italic: Handle<bevy::prelude::Font>,
    #[asset(path = "fonts/Roboto-Regular.ttf")]
    pub sans_regular: Handle<bevy::prelude::Font>,
    #[asset(path = "fonts/Roboto-RegularItalic.ttf")]
    pub sans_regular_italic: Handle<bevy::prelude::Font>,
    #[asset(path = "fonts/Roboto-Medium.ttf")]
    pub sans_medium: Handle<bevy::prelude::Font>,
    #[asset(path = "fonts/Roboto-MediumItalic.ttf")]
    pub sans_medium_italic: Handle<bevy::prelude::Font>,
    #[asset(path = "fonts/Roboto-Bold.ttf")]
    pub sans_bold: Handle<bevy::prelude::Font>,
    #[asset(path = "fonts/Roboto-BoldItalic.ttf")]
    pub sans_bold_italic: Handle<bevy::prelude::Font>,
}

#[derive(AssetCollection, Resource, Asset, TypePath, Debug, Clone)]
pub struct SerifVariant {
    #[asset(path = "fonts/NotoSerif-Light.ttf")]
    pub serif_light: Handle<bevy::prelude::Font>,
    #[asset(path = "fonts/NotoSerif-LightItalic.ttf")]
    pub serif_light_italic: Handle<bevy::prelude::Font>,
    #[asset(path = "fonts/NotoSerif-Regular.ttf")]
    pub serif_regular: Handle<bevy::prelude::Font>,
    #[asset(path = "fonts/NotoSerif-RegularItalic.ttf")]
    pub serif_regular_italic: Handle<bevy::prelude::Font>,
    #[asset(path = "fonts/NotoSerif-Medium.ttf")]
    pub serif_medium: Handle<bevy::prelude::Font>,
    #[asset(path = "fonts/NotoSerif-MediumItalic.ttf")]
    pub serif_medium_italic: Handle<bevy::prelude::Font>,
    #[asset(path = "fonts/NotoSerif-Bold.ttf")]
    pub serif_bold: Handle<bevy::prelude::Font>,
    #[asset(path = "fonts/NotoSerif-BoldItalic.ttf")]
    pub serif_bold_italic: Handle<bevy::prelude::Font>,
}

#[derive(AssetCollection, Resource, Asset, TypePath, Debug, Clone)]
pub struct MonoVariant {
    #[asset(path = "fonts/RobotoMono-Light.ttf")]
    pub mono_light: Handle<bevy::prelude::Font>,
    #[asset(path = "fonts/RobotoMono-LightItalic.ttf")]
    pub mono_light_italic: Handle<bevy::prelude::Font>,
    #[asset(path = "fonts/RobotoMono-Regular.ttf")]
    pub mono_regular: Handle<bevy::prelude::Font>,
    #[asset(path = "fonts/RobotoMono-RegularItalic.ttf")]
    pub mono_regular_italic: Handle<bevy::prelude::Font>,
    #[asset(path = "fonts/RobotoMono-Medium.ttf")]
    pub mono_medium: Handle<bevy::prelude::Font>,
    #[asset(path = "fonts/RobotoMono-MediumItalic.ttf")]
    pub mono_medium_italic: Handle<bevy::prelude::Font>,
    #[asset(path = "fonts/RobotoMono-Bold.ttf")]
    pub mono_bold: Handle<bevy::prelude::Font>,
    #[asset(path = "fonts/RobotoMono-BoldItalic.ttf")]
    pub mono_bold_italic: Handle<bevy::prelude::Font>,
}
