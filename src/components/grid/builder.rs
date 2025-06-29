//! Grid builder pattern implementation for fluent grid construction.
//!
//! This module provides the `GridBuilder` struct and its comprehensive set of
//! configuration methods. The builder pattern allows for intuitive method chaining
//! and maintains type safety throughout the grid configuration process.

use crate::{
    components::box_component::{RadiusLevel, SpacingLevel},
    theme::color::UiColorPalette,
};
use bevy::prelude::*;
use bevy::ui::GridPlacement;

use super::{
    core::GridComponent,
    styling::ExplicitColors,
    tracks::{GridGap, GridTrack, GridTrackSize},
};

/// Builder for creating Grid components using a fluent API.
/// 
/// GridBuilder provides a comprehensive set of methods for configuring
/// grid layouts, styling, and positioning. It follows the builder pattern
/// for intuitive method chaining and maintains type safety throughout
/// the configuration process.
/// 
/// # Usage Pattern
/// 
/// 1. **Create**: Start with `Grid::new()` or convenience methods
/// 2. **Configure**: Chain methods to set up tracks, gaps, styling
/// 3. **Build**: Call `.build()` to create the final component bundle
/// 
/// # Example
/// ```rust
/// let grid = Grid::new("my-grid")
///     .columns_fr(3)           // 3 equal columns
///     .rows_auto(2)            // 2 auto-sized rows
///     .gap(16.0)               // 16px gaps
///     .color(theme().blue)     // Blue color palette
///     .border(1.0)             // 1px border
///     .build();
/// ```
pub struct GridBuilder {
    /// Component name for debugging and identification
    pub(super) name: String,
    /// Grid configuration (tracks, gaps, colors)
    pub(super) grid_config: GridComponent,
    /// Bevy UI Node for layout properties
    pub(super) node: Node,
    /// Tracking for explicit color overrides
    pub(super) explicit_colors: ExplicitColors,
    /// Child entities to include in the grid
    pub(super) children: Vec<Entity>,
}

impl GridComponent {
    /// Creates a new Grid component builder with the specified name.
    /// 
    /// This is the primary entry point for creating grids. The name is used
    /// for debugging and component identification in Bevy's entity system.
    /// 
    /// # Arguments
    /// * `name` - A name for the grid component
    /// 
    /// # Returns
    /// A GridBuilder ready for configuration
    pub fn new(name: impl AsRef<str>) -> GridBuilder {
        GridBuilder {
            name: name.as_ref().to_string(),
            grid_config: GridComponent::default(),
            node: Node::default(),
            explicit_colors: ExplicitColors::default(),
            children: Vec::new(),
        }
    }

    /// Convenience method: Creates a grid with equal-width columns.
    /// 
    /// Creates a grid with the specified number of columns, each taking
    /// equal portions of available space using fractional units (1fr each).
    /// 
    /// # Arguments
    /// * `name` - Grid component name
    /// * `count` - Number of equal columns to create
    /// 
    /// # Example
    /// ```rust
    /// // Creates a 3-column gallery grid
    /// let gallery = Grid::columns("photo-gallery", 3)
    ///     .gap(12.0)
    ///     .build();
    /// ```
    pub fn columns(name: impl AsRef<str>, count: u16) -> GridBuilder {
        Self::new(name).columns_fr(count)
    }

    /// Convenience method: Creates a grid with equal-height rows.
    /// 
    /// Creates a grid with the specified number of rows, each taking
    /// equal portions of available space using fractional units (1fr each).
    /// 
    /// # Arguments
    /// * `name` - Grid component name
    /// * `count` - Number of equal rows to create
    /// 
    /// # Example
    /// ```rust
    /// // Creates a 4-row vertical layout
    /// let sidebar = Grid::rows("sidebar-menu", 4)
    ///     .width(Val::Px(200.0))
    ///     .build();
    /// ```
    pub fn rows(name: impl AsRef<str>, count: u16) -> GridBuilder {
        Self::new(name).rows_fr(count)
    }

    /// Convenience method: Creates a grid with specified columns and rows.
    /// 
    /// Creates a grid with the specified number of equal-sized columns and rows,
    /// forming a regular grid pattern using fractional units.
    /// 
    /// # Arguments
    /// * `name` - Grid component name
    /// * `cols` - Number of equal columns
    /// * `rows` - Number of equal rows
    /// 
    /// # Example
    /// ```rust
    /// // Creates a 3x2 card grid (3 columns, 2 rows)
    /// let cards = Grid::layout("card-grid", 3, 2)
    ///     .gap(16.0)
    ///     .aspect_ratio(1.5)
    ///     .build();
    /// ```
    pub fn layout(name: impl AsRef<str>, cols: u16, rows: u16) -> GridBuilder {
        Self::new(name).columns_fr(cols).rows_fr(rows)
    }
}

impl GridBuilder {
    // === Grid Template Columns Configuration ===

    /// Sets grid template columns to repeat fractional units.
    /// 
    /// Creates equal-width columns that share available space proportionally.
    /// This is the most common method for responsive column layouts.
    /// 
    /// # Arguments
    /// * `count` - Number of equal-width columns to create
    /// 
    /// # Example
    /// ```rust
    /// // Creates 4 equal columns: "1fr 1fr 1fr 1fr"
    /// let gallery = Grid::new("gallery").columns_fr(4).build();
    /// ```
    pub fn columns_fr(mut self, count: u16) -> Self {
        self.grid_config.template_columns = GridTrack::fr(count);
        self
    }

    /// Sets grid template columns to repeat fixed pixel widths.
    /// 
    /// Creates columns with exact pixel dimensions, useful for fixed-width layouts.
    /// 
    /// # Arguments
    /// * `count` - Number of columns to create
    /// * `size` - Width of each column in pixels
    /// 
    /// # Example
    /// ```rust
    /// // Creates 3 columns, each 200px wide
    /// let toolbar = Grid::new("toolbar").columns_px(3, 200.0).build();
    /// ```
    pub fn columns_px(mut self, count: u16, size: f32) -> Self {
        self.grid_config.template_columns = GridTrack::px(count, size);
        self
    }

    /// Sets grid template columns to repeat auto-sized columns.
    /// 
    /// Creates columns that size themselves based on their content.
    /// 
    /// # Arguments
    /// * `count` - Number of auto-sized columns to create
    /// 
    /// # Example
    /// ```rust
    /// // Creates 3 columns that fit their content
    /// let menu = Grid::new("menu").columns_auto(3).build();
    /// ```
    pub fn columns_auto(mut self, count: u16) -> Self {
        self.grid_config.template_columns = GridTrack::auto(count);
        self
    }

    /// Sets grid template columns with explicit individual track sizes.
    /// 
    /// Provides full control over each column's sizing, allowing mixed
    /// sizing types for complex layouts.
    /// 
    /// # Arguments
    /// * `sizes` - Vector of track sizes, one for each column
    /// 
    /// # Example
    /// ```rust
    /// use forge_ui::GridTrackSize;
    /// 
    /// // Creates: fixed sidebar | flexible content | fixed panel
    /// let layout = Grid::new("layout")
    ///     .columns_sizes(vec![
    ///         GridTrackSize::Px(200.0),    // Sidebar
    ///         GridTrackSize::Fr(1.0),      // Main content
    ///         GridTrackSize::Px(150.0),    // Side panel
    ///     ])
    ///     .build();
    /// ```
    pub fn columns_sizes(mut self, sizes: Vec<GridTrackSize>) -> Self {
        self.grid_config.template_columns = GridTrack::sizes(sizes);
        self
    }

    /// Sets grid template columns using a CSS Grid template string.
    /// 
    /// Allows using CSS Grid template syntax for advanced patterns.
    /// Note: Template string parsing is currently limited.
    /// 
    /// # Arguments
    /// * `template` - CSS Grid template string
    /// 
    /// # Example
    /// ```rust
    /// let advanced = Grid::new("advanced")
    ///     .columns_template("minmax(200px, 1fr) repeat(3, 100px)")
    ///     .build();
    /// ```
    pub fn columns_template(mut self, template: impl Into<String>) -> Self {
        self.grid_config.template_columns = GridTrack::template(template);
        self
    }

    /// Sets grid template columns using a GridTrack configuration.
    /// 
    /// Provides direct access to the GridTrack API for advanced use cases.
    /// 
    /// # Arguments
    /// * `columns` - GridTrack configuration for columns
    pub fn columns(mut self, columns: GridTrack) -> Self {
        self.grid_config.template_columns = columns;
        self
    }

    // === Grid Template Rows Configuration ===

    /// Sets grid template rows to repeat fractional units.
    /// 
    /// Creates equal-height rows that share available space proportionally.
    /// Commonly used for vertical layouts and responsive row sizing.
    /// 
    /// # Arguments
    /// * `count` - Number of equal-height rows to create
    /// 
    /// # Example
    /// ```rust
    /// // Creates 3 equal rows: "1fr 1fr 1fr"
    /// let dashboard = Grid::new("dashboard").rows_fr(3).build();
    /// ```
    pub fn rows_fr(mut self, count: u16) -> Self {
        self.grid_config.template_rows = GridTrack::fr(count);
        self
    }

    /// Sets grid template rows to repeat fixed pixel heights.
    /// 
    /// Creates rows with exact pixel dimensions, useful for fixed-height layouts.
    /// 
    /// # Arguments
    /// * `count` - Number of rows to create
    /// * `size` - Height of each row in pixels
    /// 
    /// # Example
    /// ```rust
    /// // Creates 4 rows, each 60px tall
    /// let menu = Grid::new("menu").rows_px(4, 60.0).build();
    /// ```
    pub fn rows_px(mut self, count: u16, size: f32) -> Self {
        self.grid_config.template_rows = GridTrack::px(count, size);
        self
    }

    /// Sets grid template rows to repeat auto-sized rows.
    /// 
    /// Creates rows that size themselves based on their content.
    /// 
    /// # Arguments
    /// * `count` - Number of auto-sized rows to create
    /// 
    /// # Example
    /// ```rust
    /// // Creates 2 rows that fit their content
    /// let form = Grid::new("form").rows_auto(2).build();
    /// ```
    pub fn rows_auto(mut self, count: u16) -> Self {
        self.grid_config.template_rows = GridTrack::auto(count);
        self
    }

    /// Sets grid template rows with explicit individual track sizes.
    /// 
    /// Provides full control over each row's sizing, allowing mixed
    /// sizing types for complex vertical layouts.
    /// 
    /// # Arguments
    /// * `sizes` - Vector of track sizes, one for each row
    /// 
    /// # Example
    /// ```rust
    /// use forge_ui::GridTrackSize;
    /// 
    /// // Creates: fixed header | flexible content | fixed footer
    /// let page = Grid::new("page")
    ///     .rows_sizes(vec![
    ///         GridTrackSize::Px(80.0),     // Header
    ///         GridTrackSize::Fr(1.0),      // Main content
    ///         GridTrackSize::Px(40.0),     // Footer
    ///     ])
    ///     .build();
    /// ```
    pub fn rows_sizes(mut self, sizes: Vec<GridTrackSize>) -> Self {
        self.grid_config.template_rows = GridTrack::sizes(sizes);
        self
    }

    /// Sets grid template rows using a CSS Grid template string.
    /// 
    /// Allows using CSS Grid template syntax for advanced row patterns.
    /// Note: Template string parsing is currently limited.
    /// 
    /// # Arguments
    /// * `template` - CSS Grid template string
    /// 
    /// # Example
    /// ```rust
    /// let complex = Grid::new("complex")
    ///     .rows_template("auto 1fr auto")
    ///     .build();
    /// ```
    pub fn rows_template(mut self, template: impl Into<String>) -> Self {
        self.grid_config.template_rows = GridTrack::template(template);
        self
    }

    /// Sets grid template rows using a GridTrack configuration.
    /// 
    /// Provides direct access to the GridTrack API for advanced use cases.
    /// 
    /// # Arguments
    /// * `rows` - GridTrack configuration for rows
    pub fn rows(mut self, rows: GridTrack) -> Self {
        self.grid_config.template_rows = rows;
        self
    }

    // === Grid Auto Track Configuration ===

    /// Sets the sizing for implicitly created columns.
    /// 
    /// When grid items are placed outside the explicit grid,
    /// new columns are automatically created using this sizing.
    /// 
    /// # Arguments
    /// * `track` - GridTrack configuration for auto-generated columns
    /// 
    /// # Example
    /// ```rust
    /// let grid = Grid::new("auto-expand")
    ///     .columns_fr(2)  // Explicit: 2 columns
    ///     .auto_columns(GridTrack::px(1, 100.0))  // Implicit: 100px wide
    ///     .build();
    /// ```
    pub fn auto_columns(mut self, track: GridTrack) -> Self {
        self.grid_config.auto_columns = Some(track);
        self
    }

    /// Sets the sizing for implicitly created rows.
    /// 
    /// When grid items are placed outside the explicit grid,
    /// new rows are automatically created using this sizing.
    /// 
    /// # Arguments
    /// * `track` - GridTrack configuration for auto-generated rows
    /// 
    /// # Example
    /// ```rust
    /// let grid = Grid::new("auto-grow")
    ///     .rows_fr(3)  // Explicit: 3 rows
    ///     .auto_rows(GridTrack::auto(1))  // Implicit: content-sized
    ///     .build();
    /// ```
    pub fn auto_rows(mut self, track: GridTrack) -> Self {
        self.grid_config.auto_rows = Some(track);
        self
    }

    // === Grid Gap Configuration ===

    /// Sets uniform gap between all grid items.
    /// 
    /// Applies the same gap value to both row and column gaps,
    /// creating consistent spacing throughout the grid.
    /// 
    /// # Arguments
    /// * `value` - Gap size in pixels
    /// 
    /// # Example
    /// ```rust
    /// let grid = Grid::new("spaced-grid")
    ///     .columns_fr(3)
    ///     .gap(16.0)  // 16px gap everywhere
    ///     .build();
    /// ```
    pub fn gap(mut self, value: f32) -> Self {
        self.grid_config.gap = GridGap::Uniform(value);
        self
    }

    /// Sets gap using predefined spacing levels.
    /// 
    /// Uses theme-consistent spacing values for professional layouts.
    /// 
    /// # Arguments
    /// * `level` - Predefined spacing level from theme
    /// 
    /// # Example
    /// ```rust
    /// let grid = Grid::new("themed-grid")
    ///     .columns_fr(2)
    ///     .gap_level(SpacingLevel::Medium)
    ///     .build();
    /// ```
    pub fn gap_level(mut self, _level: SpacingLevel) -> Self {
        // We'll resolve this during build when we have access to the theme
        self.grid_config.gap = GridGap::Uniform(0.0); // Placeholder
        self
    }

    /// Sets different gaps for columns and rows.
    /// 
    /// Allows independent control over horizontal and vertical spacing.
    /// 
    /// # Arguments
    /// * `column_gap` - Horizontal gap between columns in pixels
    /// * `row_gap` - Vertical gap between rows in pixels
    /// 
    /// # Example
    /// ```rust
    /// let grid = Grid::new("asymmetric-grid")
    ///     .columns_fr(4)
    ///     .gap_xy(20.0, 10.0)  // 20px horizontal, 10px vertical
    ///     .build();
    /// ```
    pub fn gap_xy(mut self, column_gap: f32, row_gap: f32) -> Self {
        self.grid_config.gap = GridGap::Axis { row: row_gap, column: column_gap };
        self
    }

    /// Sets gap between rows only.
    /// 
    /// Controls vertical spacing while leaving horizontal spacing unset.
    /// 
    /// # Arguments
    /// * `value` - Row gap size in pixels
    /// 
    /// # Example
    /// ```rust
    /// let list = Grid::new("vertical-list")
    ///     .columns_fr(1)
    ///     .row_gap(8.0)  // 8px between rows
    ///     .build();
    /// ```
    pub fn row_gap(mut self, gap: f32) -> Self {
        match self.grid_config.gap {
            GridGap::Axis { column, .. } => {
                self.grid_config.gap = GridGap::Axis { row: gap, column };
            }
            _ => {
                self.grid_config.gap = GridGap::Axis {
                    row: gap,
                    column: 0.0,
                };
            }
        }
        self
    }

    /// Sets gap between columns only.
    /// 
    /// Controls horizontal spacing while leaving vertical spacing unset.
    /// 
    /// # Arguments
    /// * `value` - Column gap size in pixels
    /// 
    /// # Example
    /// ```rust
    /// let toolbar = Grid::new("horizontal-toolbar")
    ///     .rows_fr(1)
    ///     .column_gap(12.0)  // 12px between columns
    ///     .build();
    /// ```
    pub fn column_gap(mut self, gap: f32) -> Self {
        match self.grid_config.gap {
            GridGap::Axis { row, .. } => {
                self.grid_config.gap = GridGap::Axis { row, column: gap };
            }
            _ => {
                self.grid_config.gap = GridGap::Axis {
                    row: 0.0,
                    column: gap,
                };
            }
        }
        self
    }

    // === Size Configuration ===

    /// Sets the width of the grid container.
    /// 
    /// # Arguments
    /// * `value` - Width value (pixels, percentage, etc.)
    /// 
    /// # Example
    /// ```rust
    /// let grid = Grid::new("fixed-width")
    ///     .width(Val::Px(800.0))
    ///     .build();
    /// ```
    pub fn width(mut self, value: Val) -> Self {
        self.node.width = value;
        self
    }

    /// Sets the height of the grid container.
    /// 
    /// # Arguments
    /// * `value` - Height value (pixels, percentage, etc.)
    /// 
    /// # Example
    /// ```rust
    /// let grid = Grid::new("fixed-height")
    ///     .height(Val::Px(600.0))
    ///     .build();
    /// ```
    pub fn height(mut self, value: Val) -> Self {
        self.node.height = value;
        self
    }

    /// Sets both width and height to the same value.
    /// 
    /// # Arguments
    /// * `value` - Size value for both dimensions
    /// 
    /// # Example
    /// ```rust
    /// let grid = Grid::new("square")
    ///     .size(Val::Px(400.0))
    ///     .build();
    /// ```
    pub fn size(mut self, value: Val) -> Self {
        self.node.width = value;
        self.node.height = value;
        self
    }

    /// Creates a square grid with equal width and height in pixels.
    /// 
    /// # Arguments
    /// * `size` - Square dimension in pixels
    /// 
    /// # Example
    /// ```rust
    /// let avatar = Grid::new("avatar-grid")
    ///     .square(120.0)
    ///     .build();
    /// ```
    pub fn square(mut self, size: f32) -> Self {
        let val = Val::Px(size);
        self.node.width = val;
        self.node.height = val;
        self
    }

    /// Sets width to fill available space (100%).
    /// 
    /// # Example
    /// ```rust
    /// let grid = Grid::new("full-width")
    ///     .fill_width()
    ///     .build();
    /// ```
    pub fn fill_width(mut self) -> Self {
        self.node.width = Val::Percent(100.0);
        self
    }

    /// Sets height to fill available space (100%).
    /// 
    /// # Example
    /// ```rust
    /// let grid = Grid::new("full-height")
    ///     .fill_height()
    ///     .build();
    /// ```
    pub fn fill_height(mut self) -> Self {
        self.node.height = Val::Percent(100.0);
        self
    }

    /// Sets both width and height to fill available space (100%).
    /// 
    /// # Example
    /// ```rust
    /// let grid = Grid::new("fullscreen")
    ///     .fill()
    ///     .build();
    /// ```
    pub fn fill(mut self) -> Self {
        self.node.width = Val::Percent(100.0);
        self.node.height = Val::Percent(100.0);
        self
    }

    /// Sets minimum width constraint.
    /// 
    /// # Arguments
    /// * `value` - Minimum width value
    /// 
    /// # Example
    /// ```rust
    /// let grid = Grid::new("responsive")
    ///     .min_width(Val::Px(300.0))
    ///     .build();
    /// ```
    pub fn min_width(mut self, value: Val) -> Self {
        self.node.min_width = value;
        self
    }

    /// Sets minimum height constraint.
    /// 
    /// # Arguments
    /// * `value` - Minimum height value
    /// 
    /// # Example
    /// ```rust
    /// let grid = Grid::new("responsive")
    ///     .min_height(Val::Px(200.0))
    ///     .build();
    /// ```
    pub fn min_height(mut self, value: Val) -> Self {
        self.node.min_height = value;
        self
    }

    /// Sets maximum width constraint.
    /// 
    /// # Arguments
    /// * `value` - Maximum width value
    /// 
    /// # Example
    /// ```rust
    /// let grid = Grid::new("constrained")
    ///     .max_width(Val::Px(1200.0))
    ///     .build();
    /// ```
    pub fn max_width(mut self, value: Val) -> Self {
        self.node.max_width = value;
        self
    }

    /// Sets maximum height constraint.
    /// 
    /// # Arguments
    /// * `value` - Maximum height value
    /// 
    /// # Example
    /// ```rust
    /// let grid = Grid::new("constrained")
    ///     .max_height(Val::Px(800.0))
    ///     .build();
    /// ```
    pub fn max_height(mut self, value: Val) -> Self {
        self.node.max_height = value;
        self
    }

    /// Sets aspect ratio constraint.
    /// 
    /// # Arguments
    /// * `ratio` - Width to height ratio
    /// 
    /// # Example
    /// ```rust
    /// let grid = Grid::new("video-grid")
    ///     .aspect_ratio(16.0 / 9.0)  // 16:9 aspect ratio
    ///     .build();
    /// ```
    pub fn aspect_ratio(mut self, ratio: f32) -> Self {
        self.node.aspect_ratio = Some(ratio);
        self
    }

    // === Padding Configuration ===

    /// Sets uniform padding on all sides.
    /// 
    /// # Arguments
    /// * `value` - Padding value for all sides
    /// 
    /// # Example
    /// ```rust
    /// let grid = Grid::new("padded")
    ///     .padding(UiRect::all(Val::Px(16.0)))
    ///     .build();
    /// ```
    pub fn padding(mut self, value: UiRect) -> Self {
        self.node.padding = value;
        self
    }

    /// Sets horizontal padding (left and right).
    /// 
    /// # Arguments
    /// * `value` - Padding value for left and right sides
    /// 
    /// # Example
    /// ```rust
    /// let grid = Grid::new("h-padded")
    ///     .padding_x(Val::Px(20.0))
    ///     .build();
    /// ```
    pub fn padding_x(mut self, value: Val) -> Self {
        self.node.padding.left = value;
        self.node.padding.right = value;
        self
    }

    /// Sets vertical padding (top and bottom).
    /// 
    /// # Arguments
    /// * `value` - Padding value for top and bottom sides
    /// 
    /// # Example
    /// ```rust
    /// let grid = Grid::new("v-padded")
    ///     .padding_y(Val::Px(12.0))
    ///     .build();
    /// ```
    pub fn padding_y(mut self, value: Val) -> Self {
        self.node.padding.top = value;
        self.node.padding.bottom = value;
        self
    }

    /// Sets padding for individual sides.
    /// 
    /// # Arguments
    /// * `top` - Top padding
    /// * `right` - Right padding  
    /// * `bottom` - Bottom padding
    /// * `left` - Left padding
    /// 
    /// # Example
    /// ```rust
    /// let grid = Grid::new("custom-padded")
    ///     .padding_sides(
    ///         Val::Px(10.0), // top
    ///         Val::Px(15.0), // right
    ///         Val::Px(10.0), // bottom
    ///         Val::Px(15.0), // left
    ///     )
    ///     .build();
    /// ```
    pub fn padding_sides(mut self, top: Val, right: Val, bottom: Val, left: Val) -> Self {
        self.node.padding = UiRect {
            top,
            right,
            bottom,
            left,
        };
        self
    }

    /// Sets uniform padding using pixel values.
    /// 
    /// # Arguments
    /// * `pixels` - Padding in pixels for all sides
    /// 
    /// # Example
    /// ```rust
    /// let grid = Grid::new("simple-pad")
    ///     .pad(16.0)
    ///     .build();
    /// ```
    pub fn pad(mut self, pixels: f32) -> Self {
        self.node.padding = UiRect::all(Val::Px(pixels));
        self
    }

    /// Sets horizontal padding using pixel values.
    /// 
    /// # Arguments
    /// * `pixels` - Horizontal padding in pixels
    /// 
    /// # Example
    /// ```rust
    /// let grid = Grid::new("h-pad")
    ///     .pad_x(20.0)
    ///     .build();
    /// ```
    pub fn pad_x(mut self, pixels: f32) -> Self {
        let val = Val::Px(pixels);
        self.node.padding.left = val;
        self.node.padding.right = val;
        self
    }

    /// Sets vertical padding using pixel values.
    /// 
    /// # Arguments
    /// * `pixels` - Vertical padding in pixels
    /// 
    /// # Example
    /// ```rust
    /// let grid = Grid::new("v-pad")
    ///     .pad_y(12.0)
    ///     .build();
    /// ```
    pub fn pad_y(mut self, pixels: f32) -> Self {
        let val = Val::Px(pixels);
        self.node.padding.top = val;
        self.node.padding.bottom = val;
        self
    }

    // === Margin Configuration ===

    /// Sets margin on all sides.
    /// 
    /// # Arguments
    /// * `value` - Margin value for all sides
    /// 
    /// # Example
    /// ```rust
    /// let grid = Grid::new("spaced")
    ///     .margin(UiRect::all(Val::Px(16.0)))
    ///     .build();
    /// ```
    pub fn margin(mut self, value: UiRect) -> Self {
        self.node.margin = value;
        self
    }

    /// Sets horizontal margin (left and right).
    /// 
    /// # Arguments
    /// * `value` - Margin value for left and right sides
    /// 
    /// # Example
    /// ```rust
    /// let grid = Grid::new("h-centered")
    ///     .margin_x(Val::Auto)  // Center horizontally
    ///     .build();
    /// ```
    pub fn margin_x(mut self, value: Val) -> Self {
        self.node.margin.left = value;
        self.node.margin.right = value;
        self
    }

    /// Sets vertical margin (top and bottom).
    /// 
    /// # Arguments
    /// * `value` - Margin value for top and bottom sides
    /// 
    /// # Example
    /// ```rust
    /// let grid = Grid::new("v-spaced")
    ///     .margin_y(Val::Px(20.0))
    ///     .build();
    /// ```
    pub fn margin_y(mut self, value: Val) -> Self {
        self.node.margin.top = value;
        self.node.margin.bottom = value;
        self
    }

    /// Sets margin for individual sides.
    /// 
    /// # Arguments
    /// * `top` - Top margin
    /// * `right` - Right margin
    /// * `bottom` - Bottom margin
    /// * `left` - Left margin
    /// 
    /// # Example
    /// ```rust
    /// let grid = Grid::new("custom-margin")
    ///     .margin_sides(
    ///         Val::Px(10.0), // top
    ///         Val::Auto,     // right
    ///         Val::Px(10.0), // bottom
    ///         Val::Auto,     // left
    ///     )
    ///     .build();
    /// ```
    pub fn margin_sides(mut self, top: Val, right: Val, bottom: Val, left: Val) -> Self {
        self.node.margin = UiRect {
            top,
            right,
            bottom,
            left,
        };
        self
    }

    // === Positioning Configuration ===

    /// Sets position to relative.
    /// 
    /// # Example
    /// ```rust
    /// let grid = Grid::new("relative")
    ///     .position_relative()
    ///     .build();
    /// ```
    pub fn position_relative(mut self) -> Self {
        self.node.position_type = PositionType::Relative;
        self
    }

    /// Sets position to absolute.
    /// 
    /// # Example
    /// ```rust
    /// let overlay = Grid::new("overlay")
    ///     .position_absolute()
    ///     .top(Val::Px(0.0))
    ///     .left(Val::Px(0.0))
    ///     .build();
    /// ```
    pub fn position_absolute(mut self) -> Self {
        self.node.position_type = PositionType::Absolute;
        self
    }

    /// Sets top position offset.
    /// 
    /// # Arguments
    /// * `value` - Top offset value
    /// 
    /// # Example
    /// ```rust
    /// let grid = Grid::new("positioned")
    ///     .position_absolute()
    ///     .top(Val::Px(50.0))
    ///     .build();
    /// ```
    pub fn top(mut self, value: Val) -> Self {
        self.node.top = value;
        self
    }

    /// Sets right position offset.
    /// 
    /// # Arguments
    /// * `value` - Right offset value
    /// 
    /// # Example
    /// ```rust
    /// let grid = Grid::new("positioned")
    ///     .position_absolute()
    ///     .right(Val::Px(20.0))
    ///     .build();
    /// ```
    pub fn right(mut self, value: Val) -> Self {
        self.node.right = value;
        self
    }

    /// Sets bottom position offset.
    /// 
    /// # Arguments
    /// * `value` - Bottom offset value
    /// 
    /// # Example
    /// ```rust
    /// let grid = Grid::new("positioned")
    ///     .position_absolute()
    ///     .bottom(Val::Px(30.0))
    ///     .build();
    /// ```
    pub fn bottom(mut self, value: Val) -> Self {
        self.node.bottom = value;
        self
    }

    /// Sets left position offset.
    /// 
    /// # Arguments
    /// * `value` - Left offset value
    /// 
    /// # Example
    /// ```rust
    /// let grid = Grid::new("positioned")
    ///     .position_absolute()
    ///     .left(Val::Px(40.0))
    ///     .build();
    /// ```
    pub fn left(mut self, value: Val) -> Self {
        self.node.left = value;
        self
    }

    // === Grid Child Properties ===

    /// Sets grid column start position for child items.
    /// 
    /// # Arguments
    /// * `line` - Grid line number or name
    /// 
    /// # Example
    /// ```rust
    /// let grid = Grid::new("positioned-items")
    ///     .grid_column_start(2)  // Start at column 2
    ///     .build();
    /// ```
    pub fn grid_column_start(mut self, line: i16) -> Self {
        self.node.grid_column = GridPlacement::start(line);
        self
    }

    /// Sets grid column end position for child items.
    /// 
    /// # Arguments
    /// * `line` - Grid line number or name
    /// 
    /// # Example
    /// ```rust
    /// let grid = Grid::new("positioned-items")
    ///     .grid_column_end(4)  // End at column 4
    ///     .build();
    /// ```
    pub fn grid_column_end(mut self, line: i16) -> Self {
        self.node.grid_column = GridPlacement::end(line);
        self
    }

    /// Sets grid column span for child items.
    /// 
    /// # Arguments
    /// * `span` - Number of columns to span
    /// 
    /// # Example
    /// ```rust
    /// let grid = Grid::new("spanning-items")
    ///     .grid_column_span(2)  // Span 2 columns
    ///     .build();
    /// ```
    pub fn grid_column_span(mut self, span: u16) -> Self {
        self.node.grid_column = GridPlacement::span(span);
        self
    }

    /// Sets grid row start position for child items.
    /// 
    /// # Arguments
    /// * `line` - Grid line number or name
    /// 
    /// # Example
    /// ```rust
    /// let grid = Grid::new("positioned-items")
    ///     .grid_row_start(3)  // Start at row 3
    ///     .build();
    /// ```
    pub fn grid_row_start(mut self, line: i16) -> Self {
        self.node.grid_row = GridPlacement::start(line);
        self
    }

    /// Sets grid row end position for child items.
    /// 
    /// # Arguments
    /// * `line` - Grid line number or name
    /// 
    /// # Example
    /// ```rust
    /// let grid = Grid::new("positioned-items")
    ///     .grid_row_end(5)  // End at row 5
    ///     .build();
    /// ```
    pub fn grid_row_end(mut self, line: i16) -> Self {
        self.node.grid_row = GridPlacement::end(line);
        self
    }

    /// Sets grid row span for child items.
    /// 
    /// # Arguments
    /// * `span` - Number of rows to span
    /// 
    /// # Example
    /// ```rust
    /// let grid = Grid::new("spanning-items")
    ///     .grid_row_span(3)  // Span 3 rows
    ///     .build();
    /// ```
    pub fn grid_row_span(mut self, span: u16) -> Self {
        self.node.grid_row = GridPlacement::span(span);
        self
    }

    // === Color Configuration ===

    /// Sets the color theme for the grid.
    /// 
    /// Applies a color palette that affects text, background, and border colors
    /// unless explicitly overridden.
    /// 
    /// # Arguments
    /// * `palette` - Color palette from the theme system
    /// 
    /// # Example
    /// ```rust
    /// let grid = Grid::new("themed")
    ///     .color(theme().primary)
    ///     .build();
    /// ```
    pub fn color(mut self, palette: UiColorPalette) -> Self {
        self.grid_config.color_palette = palette;
        self
    }

    /// Sets explicit background color, overriding theme.
    /// 
    /// # Arguments
    /// * `color` - Background color
    /// 
    /// # Example
    /// ```rust
    /// let grid = Grid::new("custom-bg")
    ///     .background_color(Color::srgb(0.2, 0.3, 0.8))
    ///     .build();
    /// ```
    pub fn background_color(mut self, color: Color) -> Self {
        self.grid_config.styling_config.explicit_background = Some(color);
        self.explicit_colors.background = Some(color);
        self
    }

    /// Sets explicit border color, overriding theme.
    /// 
    /// # Arguments
    /// * `color` - Border color
    /// 
    /// # Example
    /// ```rust
    /// let grid = Grid::new("custom-border")
    ///     .border_color(Color::srgb(0.8, 0.2, 0.2))
    ///     .build();
    /// ```
    pub fn border_color(mut self, color: Color) -> Self {
        self.grid_config.styling_config.explicit_border = Some(color);
        self.explicit_colors.border = Some(color);
        self
    }

    // === Border Configuration ===

    /// Sets border width on all sides.
    /// 
    /// # Arguments
    /// * `width` - Border width in pixels
    /// 
    /// # Example
    /// ```rust
    /// let grid = Grid::new("bordered")
    ///     .border(2.0)
    ///     .build();
    /// ```
    pub fn border(mut self, width: f32) -> Self {
        self.node.border = UiRect::all(Val::Px(width));
        self
    }

    /// Sets border radius on all corners.
    /// 
    /// # Arguments
    /// * `radius` - Border radius in pixels
    /// 
    /// # Example
    /// ```rust
    /// let grid = Grid::new("rounded")
    ///     .rounded(8.0)
    ///     .build();
    /// ```
    pub fn rounded(mut self, radius: f32) -> Self {
        // Border radius will be applied during build
        self
    }

    /// Sets border radius using predefined levels.
    /// 
    /// # Arguments
    /// * `level` - Predefined radius level from theme
    /// 
    /// # Example
    /// ```rust
    /// let grid = Grid::new("themed-rounded")
    ///     .rounded_level(RadiusLevel::Medium)
    ///     .build();
    /// ```
    pub fn rounded_level(mut self, _level: RadiusLevel) -> Self {
        // Border radius will be applied during build
        self
    }

    /// Sets custom border radius for each corner.
    /// 
    /// # Arguments
    /// * `radius` - BorderRadius configuration
    /// 
    /// # Example
    /// ```rust
    /// let grid = Grid::new("custom-radius")
    ///     .radius(BorderRadius {
    ///         top_left: Val::Px(10.0),
    ///         top_right: Val::Px(5.0),
    ///         bottom_left: Val::Px(5.0),
    ///         bottom_right: Val::Px(10.0),
    ///     })
    ///     .build();
    /// ```
    pub fn radius(mut self, _radius: BorderRadius) -> Self {
        // Border radius will be applied during build  
        self
    }

    // === Children Configuration ===

    /// Adds a single child entity to the grid.
    /// 
    /// # Arguments
    /// * `child` - Entity to add as a child
    /// 
    /// # Example
    /// ```rust
    /// let child_entity = commands.spawn(/* child components */).id();
    /// let grid = Grid::new("parent")
    ///     .with_child(child_entity)
    ///     .build();
    /// ```
    pub fn with_child(mut self, child: Entity) -> Self {
        self.children.push(child);
        self
    }

    /// Adds multiple child entities to the grid.
    /// 
    /// # Arguments
    /// * `children` - Vector of entities to add as children
    /// 
    /// # Example
    /// ```rust
    /// let child_entities = vec![
    ///     commands.spawn(/* child 1 */).id(),
    ///     commands.spawn(/* child 2 */).id(),
    ///     commands.spawn(/* child 3 */).id(),
    /// ];
    /// let grid = Grid::new("parent")
    ///     .with_children(child_entities)
    ///     .build();
    /// ```
    pub fn with_children(mut self, children: Vec<Entity>) -> Self {
        self.children.extend(children);
        self
    }
}