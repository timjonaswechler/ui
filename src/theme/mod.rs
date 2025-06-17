use bevy::prelude::*;

pub mod color;

pub use color::*;

pub struct ThemePlugin;

impl Plugin for ThemePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RadixTheme>();
    }
}

#[derive(Resource, Default)]
pub struct RadixTheme {
    pub accent_color: AccentColor,
    pub gray_color: GrayColor,
    pub panel_background: PanelBackground,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AccentColor {
    #[default]
    Blue,
    Indigo,
    Violet,
    Purple,
    Plum,
    Pink,
    Red,
    Ruby,
    Crimson,
    Orange,
    Amber,
    Yellow,
    Lime,
    Green,
    Mint,
    Teal,
    Cyan,
    Sky,
    Iris,
    Jade,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GrayColor {
    #[default]
    Gray,
    Mauve,
    Slate,
    Sage,
    Olive,
    Sand,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PanelBackground {
    #[default]
    Translucent,
    Solid,
}