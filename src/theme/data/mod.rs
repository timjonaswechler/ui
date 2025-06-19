// src/theme/data/mod.rs
mod color;
mod layout;
mod typography;

pub use color::*;
pub use layout::*;
pub use typography::*;

use bevy::asset::Asset;
use bevy::reflect::Reflect;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Asset, Reflect)]
#[serde(default)]
pub struct UiThemeData {
    pub font: UiTypographyData,
    pub layout: UiLayoutData,
    pub color: UiColorPalettesData,
    pub accent: UiColorPaletteData,
    pub accent_a: UiColorPaletteData,
    pub gray_accent: UiColorPaletteData,
    pub gray_accent_a: UiColorPaletteData,
}

impl Default for UiThemeData {
    fn default() -> Self {
        UiThemeData {
            font: UiTypographyData::default(),
            layout: UiLayoutData::default(),
            color: UiColorPalettesData::default_light(),
            accent: UiColorPalettesData::default_light().blue,
            accent_a: UiColorPalettesData::default_light().blue_a,
            gray_accent: UiColorPalettesData::default_light().gray,
            gray_accent_a: UiColorPalettesData::default_light().gray_a,
        }
    }
}
