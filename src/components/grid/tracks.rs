//! Grid track definition system for column and row layouts.
//!
//! This module provides the track definition types and utilities for configuring
//! grid layouts. It includes track sizing options, gap configuration, and
//! convenience methods for common track patterns.

use bevy::prelude::*;

/// Grid track configuration for defining column and row layouts.
/// 
/// GridTrack represents different ways to define grid tracks (rows or columns),
/// providing flexibility from simple repeated patterns to complex custom layouts.
/// This maps closely to CSS Grid track definition syntax.
#[derive(Debug, Clone, PartialEq)]
pub enum GridTrack {
    /// No explicit tracks - grid will create implicit tracks as needed.
    /// Use when you want the grid to automatically size based on content.
    None,
    
    /// Repeat a single track size multiple times.
    /// 
    /// **Example**: `Repeat(3, GridTrackSize::Fr(1.0))` creates "1fr 1fr 1fr"
    /// **Usage**: Uniform grids like photo galleries or card layouts
    Repeat(u16, GridTrackSize),
    
    /// Explicit list of different track sizes.
    /// 
    /// **Example**: `[Px(200), Fr(1.0), Px(100)]` creates "200px 1fr 100px"
    /// **Usage**: Complex layouts like sidebars + main content + panels
    Sizes(Vec<GridTrackSize>),
    
    /// CSS Grid template string for advanced patterns.
    /// 
    /// **Example**: "minmax(200px, 1fr) repeat(3, 100px) auto"
    /// **Usage**: Complex responsive patterns (currently limited support)
    Template(String),
}

impl GridTrack {
    /// Creates a track with no explicit definition.
    /// Grid will create implicit tracks as needed based on content.
    pub fn none() -> Self {
        Self::None
    }

    /// Creates tracks by repeating a single size pattern.
    /// 
    /// # Arguments
    /// * `count` - Number of times to repeat the track
    /// * `size` - The track size to repeat
    pub fn repeat(count: u16, size: GridTrackSize) -> Self {
        Self::Repeat(count, size)
    }

    /// Creates tracks with explicit individual sizes.
    /// 
    /// # Arguments
    /// * `sizes` - Vector of track sizes, one for each track
    pub fn sizes(sizes: Vec<GridTrackSize>) -> Self {
        Self::Sizes(sizes)
    }

    /// Creates tracks from a CSS Grid template string.
    /// 
    /// # Arguments
    /// * `template` - CSS Grid template string (e.g., "1fr 200px auto")
    pub fn template(template: impl Into<String>) -> Self {
        Self::Template(template.into())
    }

    /// Convenience method: Creates equal fractional unit tracks.
    /// 
    /// Creates `count` tracks, each taking equal portions of available space.
    /// Equivalent to CSS: `repeat(count, 1fr)`
    /// 
    /// # Arguments
    /// * `count` - Number of equal-width tracks to create
    pub fn fr(count: u16) -> Self {
        Self::Repeat(count, GridTrackSize::Fr(1.0))
    }

    /// Convenience method: Creates equal pixel-sized tracks.
    /// 
    /// Creates `count` tracks, each with the specified pixel width.
    /// Equivalent to CSS: `repeat(count, {size}px)`
    /// 
    /// # Arguments
    /// * `count` - Number of tracks to create
    /// * `size` - Width/height of each track in pixels
    pub fn px(count: u16, size: f32) -> Self {
        Self::Repeat(count, GridTrackSize::Px(size))
    }

    /// Convenience method: Creates auto-sized tracks.
    /// 
    /// Creates `count` tracks that size themselves based on content.
    /// Equivalent to CSS: `repeat(count, auto)`
    /// 
    /// # Arguments
    /// * `count` - Number of auto-sized tracks to create
    pub fn auto(count: u16) -> Self {
        Self::Repeat(count, GridTrackSize::Auto)
    }
}

/// Individual grid track sizing options.
/// 
/// GridTrackSize defines how a single grid track (row or column) should be sized.
/// These correspond directly to CSS Grid track sizing functions and provide
/// flexible control over layout behavior.
#[derive(Debug, Clone, PartialEq)]
pub enum GridTrackSize {
    /// Auto-sized track that fits content.
    /// 
    /// **Behavior**: Track sizes to fit its content, similar to `min-content`
    /// **Usage**: When track size should depend on content dimensions
    Auto,
    
    /// Fixed pixel size.
    /// 
    /// **Behavior**: Track has exact pixel dimensions regardless of content
    /// **Usage**: Fixed-width sidebars, toolbars, or rigid layout components
    Px(f32),
    
    /// Fractional unit of available space.
    /// 
    /// **Behavior**: Track takes proportional share of remaining space
    /// **Usage**: Flexible content areas that should grow/shrink with container
    /// **Example**: `Fr(2.0)` takes twice as much space as `Fr(1.0)`
    Fr(f32),
    
    /// Percentage of container size.
    /// 
    /// **Behavior**: Track takes specified percentage of container dimension
    /// **Usage**: Layouts based on container proportions
    Percent(f32),
    
    /// Minimum content size.
    /// 
    /// **Behavior**: Track shrinks to smallest size that fits content without overflow
    /// **Usage**: Compact layouts where space is at a premium
    MinContent,
    
    /// Maximum content size.
    /// 
    /// **Behavior**: Track expands to largest intrinsic size of content
    /// **Usage**: When content should determine track size without constraints
    MaxContent,
    
    /// Fit content with maximum constraint.
    /// 
    /// **Behavior**: Like `max-content` but capped at specified size
    /// **Usage**: Responsive content that shouldn't exceed certain dimensions
    FitContent(f32),
    
    /// Minimum and maximum size constraints.
    /// 
    /// **Behavior**: Track size is constrained between min and max values
    /// **Usage**: Responsive layouts with size bounds
    /// **Example**: `MinMax(Px(100), Fr(1))` - at least 100px, grows to available space
    MinMax(Box<GridTrackSize>, Box<GridTrackSize>),
}

/// Gap configuration for spacing between grid items.
/// 
/// GridGap controls the spacing between grid tracks (rows and columns),
/// providing options for uniform spacing or different row/column gaps.
/// This corresponds to CSS Grid's `gap`, `row-gap`, and `column-gap` properties.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GridGap {
    /// No spacing between grid items.
    /// Grid items will be adjacent with no gaps.
    None,
    
    /// Uniform spacing between all grid items.
    /// 
    /// **Behavior**: Same gap size for both rows and columns
    /// **Usage**: Consistent spacing in all directions
    Uniform(f32),
    
    /// Different spacing for rows and columns.
    /// 
    /// **Behavior**: Separate control over row and column gaps
    /// **Usage**: Layouts where horizontal and vertical spacing needs differ
    Axis { row: f32, column: f32 },
}

impl Default for GridGap {
    /// Default to no spacing between grid items.
    fn default() -> Self {
        Self::None
    }
}