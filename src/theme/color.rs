use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColorScale {
    pub step_1: Color,
    pub step_2: Color,
    pub step_3: Color,
    pub step_4: Color,
    pub step_5: Color,
    pub step_6: Color,
    pub step_7: Color,
    pub step_8: Color,
    pub step_9: Color,
    pub step_10: Color,
    pub step_11: Color,
    pub step_12: Color,
    pub surface: Color,
    pub indicator: Color,
    pub track: Color,
    pub contrast: Color,
}

impl ColorScale {
    pub fn backgrounds(&self) -> [Color; 2] {
        [self.step_1, self.step_2]
    }
    
    pub fn interactive_components(&self) -> [Color; 3] {
        [self.step_3, self.step_4, self.step_5]
    }
    
    pub fn borders_and_separators(&self) -> [Color; 3] {
        [self.step_6, self.step_7, self.step_8]
    }
    
    pub fn solid_colors(&self) -> [Color; 2] {
        [self.step_9, self.step_10]
    }
    
    pub fn accessible_text(&self) -> [Color; 2] {
        [self.step_11, self.step_12]
    }
}

pub fn indigo_scale() -> ColorScale {
    ColorScale {
        step_1: Color::srgb(0.99, 0.99, 1.0),
        step_2: Color::srgb(0.96, 0.97, 1.0),
        step_3: Color::srgb(0.91, 0.94, 1.0),
        step_4: Color::srgb(0.85, 0.89, 1.0),
        step_5: Color::srgb(0.78, 0.84, 1.0),
        step_6: Color::srgb(0.70, 0.78, 0.99),
        step_7: Color::srgb(0.62, 0.71, 0.98),
        step_8: Color::srgb(0.52, 0.63, 0.96),
        step_9: Color::srgb(0.26, 0.46, 0.96),
        step_10: Color::srgb(0.25, 0.43, 0.88),
        step_11: Color::srgb(0.24, 0.39, 0.78),
        step_12: Color::srgb(0.09, 0.15, 0.35),
        surface: Color::srgb(0.99, 0.99, 1.0),
        indicator: Color::srgb(0.26, 0.46, 0.96),
        track: Color::srgb(0.70, 0.78, 0.99),
        contrast: Color::srgb(1.0, 1.0, 1.0),
    }
}

pub fn gray_scale() -> ColorScale {
    ColorScale {
        step_1: Color::srgb(0.98, 0.98, 0.98),
        step_2: Color::srgb(0.96, 0.96, 0.96),
        step_3: Color::srgb(0.93, 0.93, 0.93),
        step_4: Color::srgb(0.89, 0.89, 0.89),
        step_5: Color::srgb(0.85, 0.85, 0.85),
        step_6: Color::srgb(0.80, 0.80, 0.80),
        step_7: Color::srgb(0.74, 0.74, 0.74),
        step_8: Color::srgb(0.65, 0.65, 0.65),
        step_9: Color::srgb(0.55, 0.55, 0.55),
        step_10: Color::srgb(0.51, 0.51, 0.51),
        step_11: Color::srgb(0.44, 0.44, 0.44),
        step_12: Color::srgb(0.09, 0.09, 0.09),
        surface: Color::srgb(0.98, 0.98, 0.98),
        indicator: Color::srgb(0.55, 0.55, 0.55),
        track: Color::srgb(0.80, 0.80, 0.80),
        contrast: Color::srgb(1.0, 1.0, 1.0),
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ThemeColors {
    pub accent: ColorScale,
    pub gray: ColorScale,
    pub background: Color,
    pub panel_solid: Color,
    pub panel_translucent: Color,
    pub surface: Color,
    pub overlay: Color,
}