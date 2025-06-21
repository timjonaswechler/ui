use once_cell::sync::Lazy;
use std::sync::RwLock;

use crate::theme::color::{structs::UiColorPalette, theme_mode};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WarningColor {
    Amber,
    Yellow,
}

impl Default for WarningColor {
    fn default() -> Self {
        Self::Yellow
    }
}

pub static WARNING_COLOR: Lazy<RwLock<WarningColor>> =
    Lazy::new(|| RwLock::new(WarningColor::default()));

pub fn warning_palette() -> UiColorPalette {
    let warning_color = *WARNING_COLOR.read().expect("WARNING_COLOR poisoned");
    let theme_palettes = theme_mode::theme();

    match warning_color {
        WarningColor::Yellow => theme_palettes.yellow,
        WarningColor::Amber => theme_palettes.amber,
    }
}

pub fn set_warning_palette(color: WarningColor) {
    *WARNING_COLOR.write().expect("WARNING_COLOR poisoned") = color;
}

pub fn get_warning_color() -> WarningColor {
    *WARNING_COLOR.read().expect("WARNING_COLOR poisoned")
}
