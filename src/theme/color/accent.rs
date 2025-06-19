use super::{
    structs::{UiColorPalette, UiColorPalettes},
    theme_mode::{theme_mode, ThemeMode},
};
use once_cell::sync::Lazy;
use std::sync::RwLock;

/// Standard = Indigo-Palette. Jederzeit ersetzbar.
pub static ACCENT_PALETTE: Lazy<RwLock<UiColorPalette>> = Lazy::new(|| {
    RwLock::new(if theme_mode() == ThemeMode::Dark {
        UiColorPalettes::dark_mode().indigo
    } else {
        UiColorPalettes::light_mode().indigo
    })
});

pub fn accent_palette() -> UiColorPalette {
    ACCENT_PALETTE.read().unwrap().clone()
}

pub fn set_accent_palette(palette: UiColorPalette) {
    *ACCENT_PALETTE.write().unwrap() = palette;
}
