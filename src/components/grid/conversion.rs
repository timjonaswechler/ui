//! Grid conversion utilities and build system.
//!
//! This module handles the conversion from high-level grid configuration
//! to Bevy's low-level UI components. It includes the build method and
//! utility functions for track size conversion.

use crate::theme::layout::UiLayout;
use bevy::prelude::*;
use bevy::ui::{
    GridPlacement, MaxTrackSizingFunction, MinTrackSizingFunction, RepeatedGridTrack,
};
use bevy_picking::prelude::Pickable;

use super::{
    builder::GridBuilder,
    tracks::{GridGap, GridTrack, GridTrackSize},
};

impl GridBuilder {
    /// Builds the grid configuration into a complete Bevy component bundle.
    /// 
    /// This method performs the complex translation from our high-level grid
    /// configuration to Bevy's low-level UI components, handling:
    /// - Grid track conversion to Bevy format
    /// - Gap configuration application
    /// - Color resolution from theme or explicit values
    /// - Border and styling setup
    /// 
    /// # Returns
    /// A tuple of Bevy components ready for entity spawning
    pub fn build(mut self) -> impl Bundle {
        // Configure Node for CSS Grid display
        self.node.display = Display::Grid;

        // Convert grid template columns to Bevy's format
        match &self.grid_config.template_columns {
            GridTrack::None => {
                // No explicit columns - grid will create implicit tracks
            }
            GridTrack::Repeat(count, size) => {
                // Convert repeated track pattern to Bevy format
                self.node.grid_template_columns = vec![track_size_to_bevy(*count, &size)];
            }
            GridTrack::Sizes(sizes) => {
                // Convert individual track sizes to Bevy format
                self.node.grid_template_columns = sizes
                    .iter()
                    .map(|size| track_size_to_bevy(1, size))
                    .collect();
            }
            GridTrack::Template(_) => {
                // Template string parsing not fully implemented yet
                // Fallback to single flexible track
                self.node.grid_template_columns = vec![RepeatedGridTrack::fr(1, 1.0)];
            }
        }

        // Convert grid template rows to Bevy's format
        match &self.grid_config.template_rows {
            GridTrack::None => {
                // No explicit rows - grid will create implicit tracks
            }
            GridTrack::Repeat(count, size) => {
                // Convert repeated track pattern to Bevy format
                self.node.grid_template_rows = vec![track_size_to_bevy(*count, &size)];
            }
            GridTrack::Sizes(sizes) => {
                // Convert individual track sizes to Bevy format
                self.node.grid_template_rows = sizes
                    .iter()
                    .map(|size| track_size_to_bevy(1, size))
                    .collect();
            }
            GridTrack::Template(_) => {
                // Template string parsing not fully implemented yet
                // Fallback to single flexible track
                self.node.grid_template_rows = vec![RepeatedGridTrack::fr(1, 1.0)];
            }
        }

        // Apply gap configuration
        match self.grid_config.gap {
            GridGap::None => {
                // No gaps between grid items
            }
            GridGap::Uniform(gap) => {
                // Same gap for both directions
                self.node.column_gap = Val::Px(gap);
                self.node.row_gap = Val::Px(gap);
            }
            GridGap::Axis { row, column } => {
                // Different gaps for each direction
                self.node.row_gap = Val::Px(row);
                self.node.column_gap = Val::Px(column);
            }
        }

        // Resolve colors: explicit overrides take precedence over theme colors
        let background_color = self
            .explicit_colors
            .background
            .unwrap_or(self.grid_config.color_palette.bg_subtle);

        let border_color = self
            .explicit_colors
            .border
            .unwrap_or(self.grid_config.color_palette.border);

        (
            Name::new(self.name),
            self.grid_config,
            self.node,
            BackgroundColor(background_color),
            BorderColor(border_color),
            BorderRadius::all(Val::Px(UiLayout::default().radius.base)),
            Pickable::IGNORE,
        )
    }
}

/// Converts GridTrackSize to Bevy's RepeatedGridTrack format.
/// 
/// This function handles the translation between our grid track sizing
/// system and Bevy's internal grid track representation, supporting
/// all sizing modes and complex constraints.
/// 
/// # Arguments
/// * `repetition` - Number of times to repeat this track
/// * `size` - The GridTrackSize to convert
/// 
/// # Returns
/// A RepeatedGridTrack compatible with Bevy's grid system
fn track_size_to_bevy(repetition: u16, size: &GridTrackSize) -> RepeatedGridTrack {
    match size {
        GridTrackSize::Auto => RepeatedGridTrack::auto(repetition),
        GridTrackSize::Px(px) => RepeatedGridTrack::px(repetition, *px),
        GridTrackSize::Fr(fr) => RepeatedGridTrack::fr(repetition, *fr),
        GridTrackSize::Percent(percent) => RepeatedGridTrack::percent(repetition, *percent),
        GridTrackSize::MinContent => RepeatedGridTrack::min_content(repetition),
        GridTrackSize::MaxContent => RepeatedGridTrack::max_content(repetition),
        GridTrackSize::FitContent(size) => RepeatedGridTrack::fit_content_px(repetition, *size),
        GridTrackSize::MinMax(min, max) => {
            let min_fn = match **min {
                GridTrackSize::Auto => MinTrackSizingFunction::Auto,
                GridTrackSize::Px(px) => MinTrackSizingFunction::Px(px),
                GridTrackSize::Percent(percent) => MinTrackSizingFunction::Percent(percent),
                GridTrackSize::MinContent => MinTrackSizingFunction::MinContent,
                GridTrackSize::MaxContent => MinTrackSizingFunction::MaxContent,
                _ => MinTrackSizingFunction::Auto, // Fallback for complex cases
            };
            let max_fn = match **max {
                GridTrackSize::Auto => MaxTrackSizingFunction::Auto,
                GridTrackSize::Px(px) => MaxTrackSizingFunction::Px(px),
                GridTrackSize::Fr(fr) => MaxTrackSizingFunction::Fraction(fr),
                GridTrackSize::Percent(percent) => MaxTrackSizingFunction::Percent(percent),
                GridTrackSize::MinContent => MaxTrackSizingFunction::MinContent,
                GridTrackSize::MaxContent => MaxTrackSizingFunction::MaxContent,
                GridTrackSize::FitContent(size) => MaxTrackSizingFunction::FitContentPx(size),
                _ => MaxTrackSizingFunction::Auto, // Fallback for complex cases
            };
            RepeatedGridTrack::minmax(repetition, min_fn, max_fn)
        }
    }
}