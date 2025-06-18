use bevy::prelude::*;

pub mod color;
pub mod tokens;
pub mod utils;

pub use color::*;
pub use tokens::*;
pub use utils::*;

pub struct ThemePlugin;

impl Plugin for ThemePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RadixTheme>()
           .init_resource::<ThemeMode>()
           .init_resource::<ThemeTokens>()
           .add_systems(Update, update_theme_tokens.run_if(resource_changed::<ThemeMode>));
    }
}

#[derive(Resource)]
pub struct RadixTheme {
    pub accent_color: AccentColor,
    pub gray_color: GrayColor,
    pub panel_background: PanelBackground,
    pub light_colors: UiColorPalettesData,
    pub dark_colors: UiColorPalettesData,
}

impl Default for RadixTheme {
    fn default() -> Self {
        Self {
            accent_color: AccentColor::default(),
            gray_color: GrayColor::default(),
            panel_background: PanelBackground::default(),
            light_colors: UiColorPalettesData::default_light(),
            dark_colors: UiColorPalettesData::default_dark(),
        }
    }
}

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ThemeMode {
    #[default]
    Light,
    Dark,
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

impl RadixTheme {
    pub fn get_current_colors(&self, mode: &ThemeMode) -> &UiColorPalettesData {
        match mode {
            ThemeMode::Light => &self.light_colors,
            ThemeMode::Dark => &self.dark_colors,
        }
    }

    pub fn get_accent_palette(&self, mode: &ThemeMode) -> &UiColorPaletteData {
        let colors = self.get_current_colors(mode);
        match self.accent_color {
            AccentColor::Blue => &colors.blue,
            AccentColor::Indigo => &colors.indigo,
            AccentColor::Violet => &colors.violet,
            AccentColor::Purple => &colors.purple,
            AccentColor::Plum => &colors.plum,
            AccentColor::Pink => &colors.pink,
            AccentColor::Red => &colors.red,
            AccentColor::Ruby => &colors.ruby,
            AccentColor::Crimson => &colors.crimson,
            AccentColor::Orange => &colors.orange,
            AccentColor::Amber => &colors.amber,
            AccentColor::Yellow => &colors.yellow,
            AccentColor::Lime => &colors.lime,
            AccentColor::Green => &colors.green,
            AccentColor::Mint => &colors.mint,
            AccentColor::Teal => &colors.teal,
            AccentColor::Cyan => &colors.cyan,
            AccentColor::Sky => &colors.sky,
            AccentColor::Iris => &colors.iris,
            AccentColor::Jade => &colors.jade,
        }
    }

    pub fn get_gray_palette(&self, mode: &ThemeMode) -> &UiColorPaletteData {
        let colors = self.get_current_colors(mode);
        match self.gray_color {
            GrayColor::Gray => &colors.gray,
            GrayColor::Mauve => &colors.mauve,
            GrayColor::Slate => &colors.slate,
            GrayColor::Sage => &colors.sage,
            GrayColor::Olive => &colors.olive,
            GrayColor::Sand => &colors.sand,
        }
    }

    pub fn get_accent_scale(&self, mode: &ThemeMode) -> ColorScale {
        self.get_accent_palette(mode).to_color_scale()
    }

    pub fn get_gray_scale(&self, mode: &ThemeMode) -> ColorScale {
        self.get_gray_palette(mode).to_color_scale()
    }
}

impl AccentColor {
    pub fn as_str(&self) -> &'static str {
        match self {
            AccentColor::Blue => "blue",
            AccentColor::Indigo => "indigo",
            AccentColor::Violet => "violet",
            AccentColor::Purple => "purple",
            AccentColor::Plum => "plum",
            AccentColor::Pink => "pink",
            AccentColor::Red => "red",
            AccentColor::Ruby => "ruby",
            AccentColor::Crimson => "crimson",
            AccentColor::Orange => "orange",
            AccentColor::Amber => "amber",
            AccentColor::Yellow => "yellow",
            AccentColor::Lime => "lime",
            AccentColor::Green => "green",
            AccentColor::Mint => "mint",
            AccentColor::Teal => "teal",
            AccentColor::Cyan => "cyan",
            AccentColor::Sky => "sky",
            AccentColor::Iris => "iris",
            AccentColor::Jade => "jade",
        }
    }
}

impl GrayColor {
    pub fn as_str(&self) -> &'static str {
        match self {
            GrayColor::Gray => "gray",
            GrayColor::Mauve => "mauve",
            GrayColor::Slate => "slate",
            GrayColor::Sage => "sage",
            GrayColor::Olive => "olive",
            GrayColor::Sand => "sand",
        }
    }
}

impl Default for ThemeTokens {
    fn default() -> Self {
        let theme = RadixTheme::default();
        let mode = ThemeMode::default();
        Self::from_theme(&theme, &mode)
    }
}

pub fn update_theme_tokens(
    theme: Res<RadixTheme>,
    mode: Res<ThemeMode>,
    mut tokens: ResMut<ThemeTokens>,
) {
    *tokens = ThemeTokens::from_theme(&theme, &mode);
}

pub fn toggle_theme_mode(mut mode: ResMut<ThemeMode>) {
    *mode = match *mode {
        ThemeMode::Light => ThemeMode::Dark,
        ThemeMode::Dark => ThemeMode::Light,
    };
}