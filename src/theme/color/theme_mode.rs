use once_cell::sync::Lazy;
use std::sync::RwLock;

use crate::theme::color::UiColorPalettes;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    Light,
    Dark,
}

pub static THEME_MODE: Lazy<RwLock<ThemeMode>> = Lazy::new(|| RwLock::new(ThemeMode::Light));

pub fn theme_mode() -> ThemeMode {
    *THEME_MODE.read().expect("THEME_MODE poisoned")
}

pub fn set_theme_mode(mode: ThemeMode) {
    *THEME_MODE.write().expect("THEME_MODE poisoned") = mode;
}
pub fn toggle_theme_mode() {
    let mut mode = THEME_MODE.write().expect("THEME_MODE poisoned");
    *mode = match *mode {
        ThemeMode::Light => ThemeMode::Dark,
        ThemeMode::Dark => ThemeMode::Light,
    };
}
pub fn is_dark_mode() -> bool {
    matches!(
        *THEME_MODE.read().expect("THEME_MODE poisoned"),
        ThemeMode::Dark
    )
}
pub fn is_light_mode() -> bool {
    matches!(
        *THEME_MODE.read().expect("THEME_MODE poisoned"),
        ThemeMode::Light
    )
}
pub fn set_light_mode() {
    set_theme_mode(ThemeMode::Light);
}
pub fn set_dark_mode() {
    set_theme_mode(ThemeMode::Dark);
}
pub fn toggle_theme() {
    toggle_theme_mode();
}
pub fn theme() -> UiColorPalettes {
    if is_dark_mode() {
        UiColorPalettes::dark_mode()
    } else {
        UiColorPalettes::light_mode()
    }
}
