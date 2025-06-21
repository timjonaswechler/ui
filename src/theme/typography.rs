// z.B. in crates/forge_ui/src/theme/typography.rs
use crate::plugin::{FONT_SIZE_BASE, SCALING};

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

/// Simple font assets resource for direct font loading
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
