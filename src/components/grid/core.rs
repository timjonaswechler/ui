//! Core Grid component and basic types.
//!
//! This module contains the main `GridComponent` struct and its fundamental
//! implementation. The component provides comprehensive CSS Grid layout
//! capabilities with full theme integration.

use crate::{
    components::box_component::{RadiusLevel, SpacingLevel},
    theme::{
        color::{accent_palette, UiColorPalette},
        layout::UiLayout,
    },
};
use bevy::prelude::*;

use super::{styling::GridStyling, tracks::{GridTrack, GridGap}};

/// Core Grid component that defines a CSS Grid layout container.
/// 
/// This component provides comprehensive grid layout capabilities using the CSS Grid model,
/// with full integration into the Forge UI theme system. It supports complex two-dimensional
/// layouts with flexible track sizing, gap control, and automatic styling.
/// 
/// # Grid Model
/// 
/// The grid operates on the standard CSS Grid model:
/// - **Grid Container**: The element with `display: grid` (this component)
/// - **Grid Items**: Child elements placed within the grid
/// - **Grid Lines**: Invisible lines that divide the grid
/// - **Grid Tracks**: Rows and columns between grid lines
/// - **Grid Areas**: Rectangular regions bounded by grid lines
/// 
/// # Track Definition
/// 
/// Tracks can be defined using various sizing methods:
/// - Template tracks: Explicitly defined track sizes
/// - Auto tracks: Implicitly created tracks for overflow content
/// - Repeat patterns: Repeated track definitions
/// - Mixed sizing: Combination of different track types
#[derive(Component, Debug, Clone)]
pub struct GridComponent {
    /// Grid template columns definition (explicit column tracks)
    pub template_columns: GridTrack,
    /// Grid template rows definition (explicit row tracks)
    pub template_rows: GridTrack,
    /// Auto columns configuration for implicit tracks
    pub auto_columns: Option<GridTrack>,
    /// Auto rows configuration for implicit tracks
    pub auto_rows: Option<GridTrack>,
    /// Gap configuration between grid items
    pub gap: GridGap,
    /// Color palette for theme-based styling
    pub color_palette: UiColorPalette,
    /// Additional styling configuration
    pub styling_config: GridStyling,
}

impl Default for GridComponent {
    /// Creates a default Grid component with sensible defaults:
    /// - No explicit grid tracks (content will create implicit tracks)
    /// - No gaps between grid items
    /// - Accent color palette for theming
    /// - Default styling configuration
    fn default() -> Self {
        Self {
            template_columns: GridTrack::none(),
            template_rows: GridTrack::none(),
            auto_columns: None,
            auto_rows: None,
            gap: GridGap::None,
            color_palette: accent_palette(),
            styling_config: GridStyling::default(),
        }
    }
}