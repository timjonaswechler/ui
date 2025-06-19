// crates/forge_ui/src/theme/runtime/mod.rs
use bevy::{prelude::*, reflect::TypePath};

use crate::assets::FontAssets;
use crate::plugin::UiConfig;
use crate::theme::data::UiThemeData;

mod color;
mod layout;
mod typography;

pub(crate) use color::*;
pub(crate) use layout::*;
pub(crate) use typography::*;

#[derive(Debug, Clone, Asset, TypePath, Resource)]
pub struct UiTheme {
    pub font: UiTypography,
    pub layout: UiLayout,
    pub color: UiColorPalettes,
    pub accent: UiColorPalette,
    pub accent_a: UiColorPalette,
    pub gray_accent: UiColorPalette,
    pub gray_accent_a: UiColorPalette,
}

impl UiTheme {
    /// Baut aus den rohen Theme-Daten und der Config das Runtime-Theme
    pub fn build_from_data(
        font_assets: Res<FontAssets>,
        data: &UiThemeData,
        config: &UiConfig,
    ) -> Self {
        let font = typography::build(&font_assets, &data.font, config);
        let layout = layout::build(&data.layout, config);
        let color = color::build_palettes(&data.color);
        let accent = color::build_palette(&data.accent);
        let accent_a = color::build_palette(&data.accent_a);
        let gray_accent = color::build_palette(&data.gray_accent);
        let gray_accent_a = color::build_palette(&data.gray_accent_a);

        UiTheme {
            font,
            layout,
            color,
            accent,
            accent_a,
            gray_accent,
            gray_accent_a,
        }
    }
}
