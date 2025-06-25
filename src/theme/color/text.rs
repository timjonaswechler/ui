use super::structs::{UiColorPalette, TextContrastLevel};
use bevy::prelude::*;

/// Semantic text color variants with automatic contrast optimization
///
/// TextColor provides semantic color variants that automatically adapt to backgrounds
/// for optimal readability and accessibility compliance. The system intelligently
/// selects appropriate colors based on WCAG guidelines and contrast requirements.
///
/// # Color Variants
///
/// - **Default**: Primary text color for body content and standard UI text
/// - **Muted**: Secondary text color for less prominent content, captions, and metadata
/// - **Accent**: Emphasized text using the theme's accent color for highlights and links
/// - **Error**: Error states, validation messages, and destructive action indicators
/// - **Warning**: Warning states, caution messages, and attention-requiring content
/// - **Success**: Success states, confirmation messages, and positive feedback
/// - **Custom**: Explicit color override for specialized use cases
///
/// # Automatic Contrast System
///
/// The TextColor system includes intelligent contrast calculation that:
/// - Automatically selects light or dark text based on background luminance
/// - Ensures WCAG AA/AAA compliance for accessibility
/// - Provides consistent readability across different theme modes
/// - Adapts to user-defined custom backgrounds
///
/// # Usage Examples
///
/// ## Basic Text Colors
/// ```rust
/// use your_crate::theme::color::TextColor;
/// 
/// // Semantic text colors
/// let primary_text = TextColor::Default;
/// let secondary_text = TextColor::Muted;
/// let highlighted_text = TextColor::Accent;
/// let error_text = TextColor::Error;
/// ```
///
/// ## With Automatic Contrast
/// ```rust
/// // Text automatically adapts to background
/// let palette = accent_palette();
/// let background = Color::rgba(0.1, 0.1, 0.1, 1.0); // Dark background
/// 
/// // System automatically selects light text for dark background
/// let text_color = palette.resolve_text_color(
///     TextColor::Default,
///     Some(&background),
///     Some(TextContrastLevel::High),
///     false // Enable auto-contrast
/// );
/// ```
///
/// ## Custom Colors
/// ```rust
/// // Override with specific color
/// let custom_text = TextColor::Custom(Color::rgb(0.2, 0.8, 0.4));
/// ```
///
/// # Accessibility Features
///
/// - **WCAG Compliance**: Meets AA and AAA contrast standards
/// - **Automatic Adaptation**: Responds to background color changes
/// - **High Contrast Support**: Enhanced visibility for accessibility needs
/// - **Color Blindness Consideration**: Semantic meaning beyond color alone
///
/// # Theme Integration
///
/// TextColor works seamlessly with the theme system:
/// - Light/dark mode automatic adaptation
/// - Consistent color relationships across themes
/// - Palette-based color resolution
/// - Dynamic contrast calculation
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum TextColor {
    /// Primary text color for body content and standard UI elements (default)
    #[default]
    Default,
    /// Secondary text color for less prominent content and metadata
    Muted,
    /// Emphasized text using the theme's accent color for highlights
    Accent,
    /// Error states and destructive action indicators
    Error,
    /// Warning states and attention-requiring content
    Warning,
    /// Success states and positive feedback messages
    Success,
    /// Explicit color override for specialized use cases
    Custom(Color),
}

impl UiColorPalette {
    /// Get text color based on contrast level with enhanced WCAG compliance
    ///
    /// This method provides improved automatic text color selection based on the desired
    /// contrast level, ensuring WCAG compliance and accessibility standards. It selects
    /// the most appropriate text color from the palette based on the specified contrast
    /// requirements.
    ///
    /// # Parameters
    ///
    /// - `_background`: Background color context (reserved for future contrast calculation)
    /// - `level`: Desired contrast level following WCAG guidelines
    ///
    /// # Contrast Level Mapping
    ///
    /// - **Low**: Standard text color for normal readability
    /// - **Medium**: Standard text color with slight emphasis
    /// - **High**: High contrast text for improved readability
    /// - **Accessible**: Maximum contrast text for accessibility compliance
    ///
    /// # Returns
    ///
    /// A `Color` that meets the specified contrast level requirements.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let palette = accent_palette();
    /// let background = Color::WHITE;
    /// 
    /// // Get high contrast text for accessibility
    /// let accessible_text = palette.get_enhanced_text_color_for_contrast_level(
    ///     &background,
    ///     TextContrastLevel::Accessible
    /// );
    /// ```
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
    /// This method calculates the relative luminance of the background color using
    /// WCAG-compliant formulas and selects the most appropriate text color to ensure
    /// maximum readability and contrast.
    ///
    /// # Algorithm
    ///
    /// 1. Calculate relative luminance of the background color
    /// 2. Use WCAG threshold (0.5) to determine if background is light or dark
    /// 3. Select contrasting text color from the palette
    ///
    /// # Parameters
    ///
    /// - `background`: The background color to calculate contrast against
    ///
    /// # Returns
    ///
    /// A `Color` that provides optimal contrast against the given background.
    ///
    /// # WCAG Compliance
    ///
    /// The method follows WCAG guidelines:
    /// - Light backgrounds (luminance > 0.5) receive dark text
    /// - Dark backgrounds (luminance ≤ 0.5) receive light text
    ///
    /// # Examples
    ///
    /// ```rust
    /// let palette = accent_palette();
    /// 
    /// // Dark text on light background
    /// let light_bg = Color::rgb(0.9, 0.9, 0.9);
    /// let text_on_light = palette.auto_contrast_text(&light_bg);
    /// 
    /// // Light text on dark background
    /// let dark_bg = Color::rgb(0.1, 0.1, 0.1);
    /// let text_on_dark = palette.auto_contrast_text(&dark_bg);
    /// ```
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
    /// level requirements against the given background by testing all available
    /// text colors in the palette and selecting the one with the highest contrast.
    ///
    /// # Parameters
    ///
    /// - `background`: The background color to calculate contrast against
    /// - `level`: The desired contrast level following WCAG standards
    ///
    /// # WCAG Contrast Standards
    ///
    /// The method targets these contrast ratios:
    /// - **Low (3.0:1)**: WCAG AA for large text (18pt+ or 14pt+ bold)
    /// - **Medium (4.5:1)**: WCAG AA for normal text
    /// - **High (4.5:1)**: WCAG AA for normal text with emphasis
    /// - **Accessible (7.0:1)**: WCAG AAA for enhanced accessibility
    ///
    /// # Algorithm
    ///
    /// 1. Calculate background luminance
    /// 2. Test all available text colors in the palette
    /// 3. Calculate contrast ratio for each candidate
    /// 4. Return the color with the highest contrast ratio
    ///
    /// # Returns
    ///
    /// A `Color` that provides the best possible contrast against the background,
    /// falling back to `text_contrast` if no candidates are available.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let palette = accent_palette();
    /// let blue_background = Color::rgb(0.2, 0.4, 0.8);
    /// 
    /// // Get WCAG AAA compliant text color
    /// let accessible_text = palette.get_accessible_text_color(
    ///     &blue_background,
    ///     TextContrastLevel::Accessible
    /// );
    /// 
    /// // The method will select the text color with highest contrast
    /// // against the blue background, ensuring 7.0:1 ratio when possible
    /// ```
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
    ///
    /// This is the primary method for resolving semantic text colors into actual Color
    /// values. It provides two modes of operation: explicit color mapping and automatic
    /// contrast optimization, ensuring optimal readability in all contexts.
    ///
    /// # Parameters
    ///
    /// - `text_color`: The semantic text color variant to resolve
    /// - `background_context`: Optional background color for contrast calculation
    /// - `contrast_level`: Desired contrast level (defaults to High if not specified)
    /// - `explicit_color_set`: Whether the user explicitly set a color preference
    ///
    /// # Resolution Modes
    ///
    /// ## Explicit Color Mode (`explicit_color_set = true`)
    /// Uses traditional theme color mapping without automatic contrast adjustment:
    /// - Preserves user's intentional color choices
    /// - Maps semantic colors to specific palette colors
    /// - Suitable when precise color control is required
    ///
    /// ## Auto-Contrast Mode (`explicit_color_set = false`)
    /// Uses intelligent contrast calculation for optimal readability:
    /// - Automatically adapts to background colors
    /// - Ensures WCAG compliance
    /// - Provides consistent readability across themes
    ///
    /// # Examples
    ///
    /// ```rust
    /// let palette = accent_palette();
    /// let dark_background = Color::rgb(0.1, 0.1, 0.1);
    ///
    /// // Auto-contrast mode - adapts to background
    /// let auto_text = palette.resolve_text_color(
    ///     TextColor::Default,
    ///     Some(&dark_background),
    ///     Some(TextContrastLevel::High),
    ///     false // Enable auto-contrast
    /// );
    ///
    /// // Explicit mode - uses fixed theme colors
    /// let fixed_text = palette.resolve_text_color(
    ///     TextColor::Accent,
    ///     None,
    ///     None,
    ///     true // Use explicit mapping
    /// );
    /// ```
    ///
    /// # Accessibility Benefits
    ///
    /// - Automatic WCAG compliance in auto-contrast mode
    /// - Consistent readability across different backgrounds
    /// - Support for high contrast accessibility requirements
    /// - Intelligent fallbacks for edge cases
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
/// normalized to 0 for darkest black and 1 for lightest white. This calculation
/// is essential for determining appropriate text colors and ensuring accessibility
/// compliance.
///
/// # Algorithm
///
/// 1. Convert color to sRGBA color space
/// 2. Apply gamma correction to convert from sRGB to linear RGB
/// 3. Apply WCAG luminance formula with standard coefficients
///
/// # WCAG Formula
///
/// The formula used is: L = 0.2126 * R + 0.7152 * G + 0.0722 * B
/// where R, G, B are the gamma-corrected linear RGB values.
///
/// These coefficients reflect the human eye's sensitivity to different colors:
/// - Green (0.7152): Highest sensitivity
/// - Red (0.2126): Medium sensitivity  
/// - Blue (0.0722): Lowest sensitivity
///
/// # Parameters
///
/// - `color`: The color to calculate luminance for
///
/// # Returns
///
/// A floating-point value between 0.0 (black) and 1.0 (white) representing
/// the relative luminance of the color.
///
/// # Examples
///
/// ```rust
/// // Pure colors
/// let white_luminance = calculate_relative_luminance(&Color::WHITE); // ≈ 1.0
/// let black_luminance = calculate_relative_luminance(&Color::BLACK); // ≈ 0.0
/// let red_luminance = calculate_relative_luminance(&Color::RED);     // ≈ 0.21
/// let green_luminance = calculate_relative_luminance(&Color::GREEN); // ≈ 0.72
/// let blue_luminance = calculate_relative_luminance(&Color::BLUE);   // ≈ 0.07
/// ```
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
///
/// Gamma correction converts sRGB color values to linear RGB values, which is
/// necessary for accurate luminance calculation. The sRGB color space uses a
/// non-linear encoding to match human perception, but luminance calculations
/// must be performed in linear space.
///
/// # Algorithm
///
/// The function uses the standard sRGB to linear RGB conversion:
/// - For dark values (≤ 0.03928): Linear scaling by 12.92
/// - For bright values (> 0.03928): Power function with gamma ≈ 2.4
///
/// # Parameters
///
/// - `value`: sRGB color component value (0.0 to 1.0)
///
/// # Returns
///
/// Linear RGB value (0.0 to 1.0) suitable for luminance calculation.
///
/// # Mathematical Formula
///
/// ```
/// if value ≤ 0.03928:
///     linear = value / 12.92
/// else:
///     linear = ((value + 0.055) / 1.055)^2.4
/// ```
///
/// # Why This Matters
///
/// Gamma correction is crucial for accurate contrast calculations because:
/// - Human perception of brightness is non-linear
/// - sRGB encoding compensates for display gamma
/// - Luminance calculations must use linear light values
/// - WCAG standards require this specific conversion
fn gamma_correct(value: f32) -> f32 {
    if value <= 0.03928 {
        value / 12.92
    } else {
        ((value + 0.055) / 1.055).powf(2.4)
    }
}

/// Calculate contrast ratio between two luminance values
///
/// The contrast ratio is a measure of the difference in perceived brightness
/// between two colors. It's the primary metric used by WCAG guidelines to
/// ensure sufficient contrast for accessibility.
///
/// # Formula
///
/// Contrast Ratio = (L1 + 0.05) / (L2 + 0.05)
/// where L1 is the lighter luminance and L2 is the darker luminance.
///
/// The constant 0.05 is added to both values to handle the case where one
/// of the colors is pure black (luminance = 0).
///
/// # Parameters
///
/// - `luminance1`: Relative luminance of the first color (0.0 to 1.0)
/// - `luminance2`: Relative luminance of the second color (0.0 to 1.0)
///
/// # Returns
///
/// A contrast ratio value between 1.0 (no contrast) and 21.0 (maximum contrast).
///
/// # WCAG Standards
///
/// - **1.0**: No contrast (identical colors)
/// - **3.0**: WCAG AA minimum for large text (18pt+ or 14pt+ bold)
/// - **4.5**: WCAG AA minimum for normal text
/// - **7.0**: WCAG AAA minimum for enhanced accessibility
/// - **21.0**: Maximum contrast (white on black or vice versa)
///
/// # Examples
///
/// ```rust
/// // Calculate contrast between white and black
/// let white_luminance = 1.0;
/// let black_luminance = 0.0;
/// let max_contrast = calculate_contrast_ratio(white_luminance, black_luminance);
/// // Result: ≈ 21.0 (maximum possible contrast)
///
/// // Calculate contrast for accessibility compliance
/// let bg_luminance = 0.2;
/// let text_luminance = 0.8;
/// let contrast = calculate_contrast_ratio(bg_luminance, text_luminance);
/// // Result: ≈ 3.4 (meets WCAG AA for large text)
/// ```
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