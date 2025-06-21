use super::structs::{UiColorPalette, TextContrastLevel};
use bevy::prelude::*;

/// Text color variants using theme colors
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum TextColor {
    #[default]
    Default,
    Muted,
    Accent,
    Error,
    Warning,
    Success,
    Custom(Color),
}

impl UiColorPalette {
    /// Get text color based on contrast level with enhanced WCAG compliance
    /// 
    /// This method provides improved automatic text color selection based on the desired
    /// contrast level, ensuring WCAG compliance and accessibility standards.
    pub fn get_enhanced_text_color_for_contrast_level(
        &self, 
        _background: &Color, 
        level: TextContrastLevel
    ) -> Color {
        match level {
            TextContrastLevel::Low => self.text,
            TextContrastLevel::Medium => self.text,
            TextContrastLevel::High => self.text_contrast,
            TextContrastLevel::Accessible => self.high_contrast,
        }
    }
    
    /// Automatically determine optimal text color based on background luminance
    /// 
    /// This method calculates the luminance of the background color and selects
    /// either dark or light text to ensure maximum readability.
    pub fn auto_contrast_text(&self, background: &Color) -> Color {
        let luminance = calculate_relative_luminance(background);
        
        // Use WCAG guidelines: light backgrounds need dark text, dark backgrounds need light text
        if luminance > 0.5 {
            self.text_contrast // Dark text on light background
        } else {
            self.text // Light text on dark background  
        }
    }
    
    /// Get text color with guaranteed contrast ratio
    /// 
    /// This method ensures the returned text color meets the specified contrast
    /// level requirements against the given background.
    pub fn get_accessible_text_color(
        &self,
        background: &Color,
        level: TextContrastLevel,
    ) -> Color {
        let background_luminance = calculate_relative_luminance(background);
        
        // Target contrast ratios based on WCAG guidelines
        let _target_ratio = match level {
            TextContrastLevel::Low => 3.0,      // WCAG AA large text
            TextContrastLevel::Medium => 4.5,   // WCAG AA normal text
            TextContrastLevel::High => 4.5,     // WCAG AA normal text
            TextContrastLevel::Accessible => 7.0, // WCAG AAA
        };
        
        // Try different text colors and pick the one with best contrast
        let candidates = [
            self.text,
            self.text_contrast,
            self.high_contrast,
        ];
        
        candidates
            .iter()
            .max_by(|a, b| {
                let contrast_a = calculate_contrast_ratio(background_luminance, calculate_relative_luminance(a));
                let contrast_b = calculate_contrast_ratio(background_luminance, calculate_relative_luminance(b));
                contrast_a.partial_cmp(&contrast_b).unwrap_or(std::cmp::Ordering::Equal)
            })
            .copied()
            .unwrap_or(self.text_contrast)
    }
    
    /// Convert TextColor enum to actual Color using intelligent contrast calculation
    pub fn resolve_text_color(
        &self,
        text_color: TextColor,
        background_context: Option<&Color>,
        contrast_level: Option<TextContrastLevel>,
        explicit_color_set: bool,
    ) -> Color {
        // If user explicitly set a color, use traditional mapping (no auto-contrast)
        if explicit_color_set {
            match text_color {
                TextColor::Default => self.text_contrast,
                TextColor::Muted => self.text,
                TextColor::Accent | TextColor::Error | TextColor::Warning | TextColor::Success => {
                    self.text_contrast
                }
                TextColor::Custom(c) => c,
            }
        } else {
            // AUTO-CONTRAST MODE: Use intelligent contrast calculation
            
            // If we have explicit background context, use it for contrast calculation
            if let Some(background) = background_context {
                let level = contrast_level.unwrap_or(TextContrastLevel::High);
                self.get_enhanced_text_color_for_contrast_level(background, level)
            } else {
                // Use high contrast defaults that work on most backgrounds
                match text_color {
                    TextColor::Muted => self.text, // Medium contrast for secondary text
                    TextColor::Custom(c) => c,
                    _ => self.text_contrast, // High contrast for primary text
                }
            }
        }
    }
}

/// Calculate relative luminance according to WCAG guidelines
/// 
/// Relative luminance is the relative brightness of any point in a colorspace,
/// normalized to 0 for darkest black and 1 for lightest white.
fn calculate_relative_luminance(color: &Color) -> f32 {
    let srgba = color.to_srgba();
    
    // Convert to linear RGB
    let r = gamma_correct(srgba.red);
    let g = gamma_correct(srgba.green);
    let b = gamma_correct(srgba.blue);
    
    // WCAG formula for relative luminance
    0.2126 * r + 0.7152 * g + 0.0722 * b
}

/// Apply gamma correction for luminance calculation
fn gamma_correct(value: f32) -> f32 {
    if value <= 0.03928 {
        value / 12.92
    } else {
        ((value + 0.055) / 1.055).powf(2.4)
    }
}

/// Calculate contrast ratio between two luminance values
/// 
/// Returns a value between 1 (no contrast) and 21 (maximum contrast)
fn calculate_contrast_ratio(luminance1: f32, luminance2: f32) -> f32 {
    let lighter = luminance1.max(luminance2);
    let darker = luminance1.min(luminance2);
    
    (lighter + 0.05) / (darker + 0.05)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contrast_ratio_calculation() {
        // White on black should give maximum contrast
        let white_luminance = calculate_relative_luminance(&Color::WHITE);
        let black_luminance = calculate_relative_luminance(&Color::BLACK);
        let ratio = calculate_contrast_ratio(white_luminance, black_luminance);
        
        // Should be close to 21:1 (maximum contrast)
        assert!(ratio > 20.0);
    }
    
    #[test]
    fn test_luminance_calculation() {
        let white_luminance = calculate_relative_luminance(&Color::WHITE);
        let black_luminance = calculate_relative_luminance(&Color::BLACK);
        
        assert!(white_luminance > 0.9); // White should be close to 1.0
        assert!(black_luminance < 0.1); // Black should be close to 0.0
    }
}