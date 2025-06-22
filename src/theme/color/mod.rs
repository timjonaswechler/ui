mod accent;
mod dark_theme;
mod error;
mod functions;
mod light_theme;
mod structs;
mod success;
pub mod text;
pub mod theme_mode;
mod warning;

pub use accent::*;
pub use error::*;
pub use structs::*;
pub use success::*;
pub use text::*;
pub use theme_mode::*;
pub use warning::*;

// Convenience palette functions
pub fn blue_palette() -> UiColorPalette {
    theme_mode::theme().blue
}

pub fn green_palette() -> UiColorPalette {
    success_palette()
}

pub fn red_palette() -> UiColorPalette {
    error_palette()
}
