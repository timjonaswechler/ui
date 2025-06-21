use once_cell::sync::Lazy;
use std::sync::RwLock;

use crate::theme::color::{structs::UiColorPalette, theme_mode};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorColor {
    Tomato,
    Red,
    Crimson,
    Ruby,
}

impl Default for ErrorColor {
    fn default() -> Self {
        Self::Red
    }
}

pub static ERROR_COLOR: Lazy<RwLock<ErrorColor>> = Lazy::new(|| RwLock::new(ErrorColor::default()));

pub fn error_palette() -> UiColorPalette {
    let error_color = *ERROR_COLOR.read().expect("ERROR_COLOR poisoned");
    let theme_palettes = theme_mode::theme();

    match error_color {
        ErrorColor::Tomato => theme_palettes.tomato,
        ErrorColor::Red => theme_palettes.red,
        ErrorColor::Crimson => theme_palettes.crimson,
        ErrorColor::Ruby => theme_palettes.ruby,
    }
}

pub fn set_error_palette(color: ErrorColor) {
    *ERROR_COLOR.write().expect("ERROR_COLOR poisoned") = color;
}

pub fn get_error_color() -> ErrorColor {
    *ERROR_COLOR.read().expect("ERROR_COLOR poisoned")
}
