use super::{
    structs::{UiColorPalette, UiColorPalettes, UiColorPalettesName},
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

pub fn set_accent_palette(palette: Option<UiColorPalettesName>) {
    *ACCENT_PALETTE.write().unwrap() = if theme_mode() == ThemeMode::Dark {
        match palette {
            Some(UiColorPalettesName::Gray) => UiColorPalettes::dark_mode().gray,
            Some(UiColorPalettesName::Mauve) => UiColorPalettes::dark_mode().mauve,
            Some(UiColorPalettesName::Slate) => UiColorPalettes::dark_mode().slate,
            Some(UiColorPalettesName::Sage) => UiColorPalettes::dark_mode().sage,
            Some(UiColorPalettesName::Olive) => UiColorPalettes::dark_mode().olive,
            Some(UiColorPalettesName::Sand) => UiColorPalettes::dark_mode().sand,
            Some(UiColorPalettesName::Tomato) => UiColorPalettes::dark_mode().tomato,
            Some(UiColorPalettesName::Red) => UiColorPalettes::dark_mode().red,
            Some(UiColorPalettesName::Ruby) => UiColorPalettes::dark_mode().ruby,
            Some(UiColorPalettesName::Crimson) => UiColorPalettes::dark_mode().crimson,
            Some(UiColorPalettesName::Pink) => UiColorPalettes::dark_mode().pink,
            Some(UiColorPalettesName::Plum) => UiColorPalettes::dark_mode().plum,
            Some(UiColorPalettesName::Purple) => UiColorPalettes::dark_mode().purple,
            Some(UiColorPalettesName::Violet) => UiColorPalettes::dark_mode().violet,
            Some(UiColorPalettesName::Iris) => UiColorPalettes::dark_mode().iris,
            Some(UiColorPalettesName::Indigo) => UiColorPalettes::dark_mode().indigo,
            Some(UiColorPalettesName::Blue) => UiColorPalettes::dark_mode().blue,
            Some(UiColorPalettesName::Cyan) => UiColorPalettes::dark_mode().cyan,
            Some(UiColorPalettesName::Teal) => UiColorPalettes::dark_mode().teal,
            Some(UiColorPalettesName::Jade) => UiColorPalettes::dark_mode().jade,
            Some(UiColorPalettesName::Green) => UiColorPalettes::dark_mode().green,
            _ => {
                // Default to indigo if the palette is not recognized
                UiColorPalettes::dark_mode().indigo
            }
        }
    } else {
        match palette {
            Some(UiColorPalettesName::Gray) => UiColorPalettes::light_mode().gray,
            Some(UiColorPalettesName::Mauve) => UiColorPalettes::light_mode().mauve,
            Some(UiColorPalettesName::Slate) => UiColorPalettes::light_mode().slate,
            Some(UiColorPalettesName::Sage) => UiColorPalettes::light_mode().sage,
            Some(UiColorPalettesName::Olive) => UiColorPalettes::light_mode().olive,
            Some(UiColorPalettesName::Sand) => UiColorPalettes::light_mode().sand,
            Some(UiColorPalettesName::Tomato) => UiColorPalettes::light_mode().tomato,
            Some(UiColorPalettesName::Red) => UiColorPalettes::light_mode().red,
            Some(UiColorPalettesName::Ruby) => UiColorPalettes::light_mode().ruby,
            Some(UiColorPalettesName::Crimson) => UiColorPalettes::light_mode().crimson,
            Some(UiColorPalettesName::Pink) => UiColorPalettes::light_mode().pink,
            Some(UiColorPalettesName::Plum) => UiColorPalettes::light_mode().plum,
            Some(UiColorPalettesName::Purple) => UiColorPalettes::light_mode().purple,
            Some(UiColorPalettesName::Violet) => UiColorPalettes::light_mode().violet,
            Some(UiColorPalettesName::Iris) => UiColorPalettes::light_mode().iris,
            Some(UiColorPalettesName::Indigo) => UiColorPalettes::light_mode().indigo,
            Some(UiColorPalettesName::Blue) => UiColorPalettes::light_mode().blue,
            Some(UiColorPalettesName::Cyan) => UiColorPalettes::light_mode().cyan,
            Some(UiColorPalettesName::Teal) => UiColorPalettes::light_mode().teal,
            Some(UiColorPalettesName::Jade) => UiColorPalettes::light_mode().jade,
            Some(UiColorPalettesName::Green) => UiColorPalettes::light_mode().green,
            _ => {
                // Default to indigo if the palette is not recognized
                UiColorPalettes::light_mode().indigo
            }
        }
    }
}
