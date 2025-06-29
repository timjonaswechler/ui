//! Grid styling system for visual configuration and theming.
//!
//! This module provides styling configuration types for controlling the visual
//! appearance of grid components, including background colors, borders, and
//! other visual properties that can override theme defaults.

use bevy::prelude::*;

/// Styling configuration for Grid component appearance.
/// 
/// GridStyling contains visual styling options that can override
/// theme-based defaults, providing fine control over grid appearance.
#[derive(Debug, Clone, Default)]
pub struct GridStyling {
    /// Alpha/opacity for background color (0.0 = transparent, 1.0 = opaque)
    pub background_alpha: f32,
    /// Border width in pixels (None = no border)
    pub border_width: Option<f32>,
    /// Explicit background color override (None = use theme)
    pub explicit_background: Option<Color>,
    /// Explicit border color override (None = use theme)
    pub explicit_border: Option<Color>,
}

/// Internal tracking for explicit color overrides.
/// 
/// This struct is used internally by the GridBuilder to track
/// when colors have been explicitly set, allowing proper precedence
/// over theme-based colors during the build process.
#[derive(Debug, Clone, Default)]
pub(super) struct ExplicitColors {
    /// Explicitly set background color
    pub background: Option<Color>,
    /// Explicitly set border color
    pub border: Option<Color>,
}