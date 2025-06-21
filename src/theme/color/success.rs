use once_cell::sync::Lazy;
use std::sync::RwLock;

use crate::theme::color::{structs::UiColorPalette, theme_mode};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SuccessColor {
    Green,
    Teal,
    Jade,
    Grass,
}

impl Default for SuccessColor {
    fn default() -> Self {
        Self::Green
    }
}

pub static SUCCESS_COLOR: Lazy<RwLock<SuccessColor>> = Lazy::new(|| RwLock::new(SuccessColor::default()));

pub fn success_palette() -> UiColorPalette {
    let success_color = *SUCCESS_COLOR.read().expect("SUCCESS_COLOR poisoned");
    let theme_palettes = theme_mode::theme();
    
    match success_color {
        SuccessColor::Green => theme_palettes.green,
        SuccessColor::Teal => theme_palettes.teal,
        SuccessColor::Jade => theme_palettes.jade,
        SuccessColor::Grass => theme_palettes.grass,
    }
}

pub fn set_success_palette(color: SuccessColor) {
    *SUCCESS_COLOR.write().expect("SUCCESS_COLOR poisoned") = color;
}

pub fn get_success_color() -> SuccessColor {
    *SUCCESS_COLOR.read().expect("SUCCESS_COLOR poisoned")
}