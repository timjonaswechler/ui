use bevy::prelude::*;

#[derive(Debug, Clone)]
pub struct UiColorPalettes {
    pub white: UiColorPaletteBasic,
    pub black: UiColorPaletteBasic,
    pub gray: UiColorPalette,
    pub mauve: UiColorPalette,
    pub slate: UiColorPalette,
    pub sage: UiColorPalette,
    pub olive: UiColorPalette,
    pub sand: UiColorPalette,
    pub tomato: UiColorPalette,
    pub red: UiColorPalette,
    pub ruby: UiColorPalette,
    pub crimson: UiColorPalette,
    pub pink: UiColorPalette,
    pub plum: UiColorPalette,
    pub purple: UiColorPalette,
    pub violet: UiColorPalette,
    pub iris: UiColorPalette,
    pub indigo: UiColorPalette,
    pub blue: UiColorPalette,
    pub cyan: UiColorPalette,
    pub teal: UiColorPalette,
    pub jade: UiColorPalette,
    pub green: UiColorPalette,
    pub grass: UiColorPalette,
    pub bronze: UiColorPalette,
    pub gold: UiColorPalette,
    // pub brown: UiColorPalette,
    // pub orange: UiColorPalette,
}
pub struct UiColorPalettesData(pub UiColorPalettes);

#[derive(Debug, Clone, PartialEq)]
pub struct UiColorPalette {
    pub step01: Color,
    pub step02: Color,
    pub step03: Color,
    pub step04: Color,
    pub step05: Color,
    pub step06: Color,
    pub step07: Color,
    pub step08: Color,
    pub step09: Color,
    pub step10: Color,
    pub step11: Color,
    pub step12: Color,
    pub step01_a: Color,
    pub step02_a: Color,
    pub step03_a: Color,
    pub step04_a: Color,
    pub step05_a: Color,
    pub step06_a: Color,
    pub step07_a: Color,
    pub step08_a: Color,
    pub step09_a: Color,
    pub step10_a: Color,
    pub step11_a: Color,
    pub step12_a: Color,

    pub high_contrast: Color,
    pub surface: Color,
    pub indicator: Color,
    pub track: Color,
}
#[derive(Debug, Clone, PartialEq)]
pub struct UiColorPaletteBasic {
    pub step01: Color,
    pub step02: Color,
    pub step03: Color,
    pub step04: Color,
    pub step05: Color,
    pub step06: Color,
    pub step07: Color,
    pub step08: Color,
    pub step09: Color,
    pub step10: Color,
    pub step11: Color,
    pub step12: Color,
}

/// Verschiedene Kontraststufen für Textfarben
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextContrastLevel {
    /// Geringer Kontrast für sekundären Text (z.B. Beschreibungen)
    Low,
    /// Mittlerer Kontrast für normalen Text
    Medium,
    /// Hoher Kontrast für wichtigen Text (WCAG AA: ≥4.5:1)
    High,
    /// Barrierefreier Kontrast (WCAG AAA: ≥7:1)
    Accessible,
}
