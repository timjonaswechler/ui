// crates/forge_ui/src/theme/layout.rs
use crate::plugin::{FONT_SIZE_BASE, SCALING, SPACING_FACTOR};
use bevy::reflect::Reflect;

#[derive(Debug, Clone, Reflect)]
pub struct UiLayout {
    pub padding: UiSpacing,
    pub margin: UiSpacing,
    pub gap: UiSpacing,
    pub radius: UiRadius,
    pub border: UiSpacing,
}

#[derive(Debug, Clone, Reflect, Default)]
pub struct UiSpacing {
    pub xs: f32,
    pub sm: f32,
    pub base: f32,
    pub lg: f32,
    pub xl: f32,
    pub x2l: f32,
    pub x3l: f32,
    pub x4l: f32,
    pub x5l: f32,
}

#[derive(Debug, Clone, Reflect, Default)]
pub struct UiRadius {
    pub none: f32,
    pub xs: f32,
    pub sm: f32,
    pub base: f32,
    pub lg: f32,
    pub xl: f32,
    pub x2l: f32,
    pub x3l: f32,
    pub x4l: f32,
    pub full: f32,
}

impl Default for UiLayout {
    fn default() -> Self {
        UiLayout {
            padding: UiSpacing {
                xs: 1.0 * SPACING_FACTOR * SCALING,
                sm: 2.0 * SPACING_FACTOR * SCALING,
                base: 3.0 * SPACING_FACTOR * SCALING,
                lg: 4.0 * SPACING_FACTOR * SCALING,
                xl: 5.0 * SPACING_FACTOR * SCALING,
                x2l: 6.0 * SPACING_FACTOR * SCALING,
                x3l: 7.0 * SPACING_FACTOR * SCALING,
                x4l: 8.0 * SPACING_FACTOR * SCALING,
                x5l: 9.0 * SPACING_FACTOR * SCALING,
            },
            margin: UiSpacing {
                xs: 1.0 * SPACING_FACTOR * SCALING,
                sm: 2.0 * SPACING_FACTOR * SCALING,
                base: 3.0 * SPACING_FACTOR * SCALING,
                lg: 4.0 * SPACING_FACTOR * SCALING,
                xl: 5.0 * SPACING_FACTOR * SCALING,
                x2l: 6.0 * SPACING_FACTOR * SCALING,
                x3l: 7.0 * SPACING_FACTOR * SCALING,
                x4l: 8.0 * SPACING_FACTOR * SCALING,
                x5l: 9.0 * SPACING_FACTOR * SCALING,
            },
            gap: UiSpacing {
                xs: 1.0 * SPACING_FACTOR * SCALING,
                sm: 2.0 * SPACING_FACTOR * SCALING,
                base: 3.0 * SPACING_FACTOR * SCALING,
                lg: 4.0 * SPACING_FACTOR * SCALING,
                xl: 5.0 * SPACING_FACTOR * SCALING,
                x2l: 6.0 * SPACING_FACTOR * SCALING,
                x3l: 7.0 * SPACING_FACTOR * SCALING,
                x4l: 8.0 * SPACING_FACTOR * SCALING,
                x5l: 9.0 * SPACING_FACTOR * SCALING,
            },
            radius: UiRadius {
                none: 0.0,
                xs: 0.125 * FONT_SIZE_BASE * SCALING,
                sm: 0.25 * FONT_SIZE_BASE * SCALING,
                base: 0.375 * FONT_SIZE_BASE * SCALING,
                lg: 0.5 * FONT_SIZE_BASE * SCALING,
                xl: 0.75 * FONT_SIZE_BASE * SCALING,
                x2l: 1.0 * FONT_SIZE_BASE * SCALING,
                x3l: 1.5 * FONT_SIZE_BASE * SCALING,
                x4l: 2.0 * FONT_SIZE_BASE * SCALING,
                full: f32::MAX,
            },
            border: UiSpacing {
                xs: 1.0,
                sm: 2.0,
                base: 3.0,
                lg: 5.0,
                xl: 7.0,
                x2l: 9.0,
                x3l: 12.0,
                x4l: 15.0,
                x5l: 19.0,
            },
        }
    }
}
