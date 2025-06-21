use super::{TextContrastLevel, UiColorPalette};
use bevy::prelude::*;

impl UiColorPalette {
    /// Berechnet die relative Helligkeit einer Farbe nach WCAG-Standard
    /// Rückgabe: Wert zwischen 0.0 (schwarz) und 1.0 (weiß)
    pub fn calculate_luminance(color: &Color) -> f32 {
        let srgba = color.to_srgba();

        // Gamma-Korrektur für sRGB
        let gamma_correct = |c: f32| {
            if c <= 0.03928 {
                c / 12.92
            } else {
                ((c + 0.055) / 1.055).powf(2.4)
            }
        };

        let r = gamma_correct(srgba.red);
        let g = gamma_correct(srgba.green);
        let b = gamma_correct(srgba.blue);

        // Relative Helligkeit nach WCAG 2.1
        0.2126 * r + 0.7152 * g + 0.0722 * b
    }

    /// Berechnet das Kontrastverhältnis zwischen zwei Farben
    /// Rückgabe: Wert zwischen 1.0 (kein Kontrast) und 21.0 (maximaler Kontrast)
    pub fn calculate_contrast_ratio(color1: &Color, color2: &Color) -> f32 {
        let lum1 = Self::calculate_luminance(color1);
        let lum2 = Self::calculate_luminance(color2);

        let lighter = lum1.max(lum2);
        let darker = lum1.min(lum2);

        (lighter + 0.05) / (darker + 0.05)
    }

    /// Wählt automatisch die optimale Textfarbe für einen gegebenen Hintergrund
    /// Berücksichtigt WCAG AA-Standard (4.5:1 Kontrastverhältnis)
    pub fn get_optimal_text_color(&self, background_color: &Color) -> Color {
        let white_contrast = Self::calculate_contrast_ratio(background_color, &Color::WHITE);
        let black_contrast = Self::calculate_contrast_ratio(background_color, &Color::BLACK);

        // Verwende text_contrast (dunkelste Farbe) oder base (hellste Farbe) der Palette
        let palette_dark_contrast =
            Self::calculate_contrast_ratio(background_color, &self.text_contrast);
        let palette_light_contrast = Self::calculate_contrast_ratio(background_color, &self.base);

        // Finde die beste Textfarbe mit höchstem Kontrast
        let candidates = vec![
            (Color::WHITE, white_contrast),
            (Color::BLACK, black_contrast),
            (self.text_contrast, palette_dark_contrast),
            (self.base, palette_light_contrast),
            (
                self.high_contrast,
                Self::calculate_contrast_ratio(background_color, &self.high_contrast),
            ),
        ];

        // Wähle die Farbe mit dem höchsten Kontrast, die WCAG AA erfüllt (≥4.5:1)
        let best_candidate = candidates
            .into_iter()
            .filter(|(_, contrast)| *contrast >= 4.5)
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .or_else(|| {
                // Fallback: Wähle die Farbe mit dem höchsten Kontrast, auch wenn sie WCAG nicht erfüllt
                vec![
                    (Color::WHITE, white_contrast),
                    (Color::BLACK, black_contrast),
                    (self.text_contrast, palette_dark_contrast),
                    (self.base, palette_light_contrast),
                ]
                .into_iter()
                .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            });

        best_candidate
            .map(|(color, _)| color)
            .unwrap_or(Color::BLACK)
    }

    /// Überprüft, ob eine Farbe als "hell" eingestuft wird
    /// Basiert auf der relativen Helligkeit (Schwellenwert: 0.5)
    pub fn is_light_color(color: &Color) -> bool {
        Self::calculate_luminance(color) > 0.5
    }

    /// Überprüft, ob eine Farbe als "dunkel" eingestuft wird
    pub fn is_dark_color(color: &Color) -> bool {
        !Self::is_light_color(color)
    }

    /// Gibt eine passende Textfarbe für verschiedene Kontraststufen zurück
    pub fn get_text_color_for_contrast_level(
        &self,
        background_color: &Color,
        level: TextContrastLevel,
    ) -> Color {
        match level {
            TextContrastLevel::Low => {
                // Geringerer Kontrast für sekundären Text
                if Self::is_light_color(background_color) {
                    self.text // Dunkler, aber nicht maximal
                } else {
                    self.bg_subtle // Heller, aber nicht maximal
                }
            }
            TextContrastLevel::Medium => {
                // Mittlerer Kontrast für normalen Text
                if Self::is_light_color(background_color) {
                    self.text_contrast // Dunkelste Farbe der Palette
                } else {
                    self.base // Hellste Farbe der Palette
                }
            }
            TextContrastLevel::High => {
                // Maximaler Kontrast für wichtigen Text
                self.get_optimal_text_color(background_color)
            }
            TextContrastLevel::Accessible => {
                // WCAG AAA-Standard (7:1 Kontrast)
                let white_contrast =
                    Self::calculate_contrast_ratio(background_color, &Color::WHITE);
                let black_contrast =
                    Self::calculate_contrast_ratio(background_color, &Color::BLACK);

                if white_contrast >= 7.0 && white_contrast >= black_contrast {
                    Color::WHITE
                } else if black_contrast >= 7.0 {
                    Color::BLACK
                } else {
                    // Fallback auf optimale Textfarbe wenn AAA nicht erreichbar
                    self.get_optimal_text_color(background_color)
                }
            }
        }
    }
}
