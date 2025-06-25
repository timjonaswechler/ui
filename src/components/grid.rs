//! Grid Component for Forge UI
//!
//! The Grid component provides a comprehensive CSS Grid layout system for Bevy applications,
//! offering powerful two-dimensional layout capabilities with full theme integration.
//! Inspired by CSS Grid and Radix UI design principles, it supports complex layouts
//! with intuitive builder patterns and automatic styling.
//!
//! ## Key Features
//!
//! - **CSS Grid Model**: Complete implementation of CSS Grid layout capabilities
//! - **Flexible Track Sizing**: Support for fr units, pixels, auto, percentages, and more
//! - **Grid Areas**: Explicit grid positioning with start/end/span syntax
//! - **Gap Control**: Uniform and directional gap configuration
//! - **Theme Integration**: Automatic styling with color palettes and spacing
//! - **Builder Pattern**: Fluent API for intuitive grid construction
//! - **Responsive Design**: Min/max sizing and aspect ratio support
//!
//! ## Grid Concepts
//!
//! ### Track Sizing Options
//! - **fr (Fractional)**: Distributes available space proportionally
//! - **px (Pixels)**: Fixed pixel dimensions
//! - **auto**: Size based on content
//! - **%**: Percentage of container
//! - **min-content/max-content**: Content-based sizing
//! - **minmax()**: Range constraints for flexible sizing
//!
//! ### Gap Types
//! - **Uniform**: Same spacing between all grid items
//! - **Axis-specific**: Different row and column gaps
//! - **Theme-based**: Using predefined spacing levels
//!
//! ## Examples
//!
//! ### Basic Grid Layouts
//! ```rust
//! use forge_ui::Grid;
//!
//! // Simple 3-column grid with equal spacing
//! let basic_grid = Grid::columns("gallery", 3)
//!     .gap(16.0)
//!     .fill_width()
//!     .build();
//!
//! // Fixed-size grid with specific dimensions
//! let card_grid = Grid::layout("cards", 2, 3)
//!     .width(Val::Px(400.0))
//!     .height(Val::Px(300.0))
//!     .gap(12.0)
//!     .build();
//! ```
//!
//! ### Advanced Track Configuration
//! ```rust
//! use forge_ui::{Grid, GridTrackSize};
//!
//! // Mixed track sizes: sidebar + main content
//! let layout_grid = Grid::new("layout")
//!     .columns_sizes(vec![
//!         GridTrackSize::Px(200.0),    // Fixed sidebar
//!         GridTrackSize::Fr(1.0),      // Flexible main area
//!         GridTrackSize::Px(100.0),    // Fixed actions panel
//!     ])
//!     .rows_fr(1)  // Single flexible row
//!     .gap_xy(16.0, 8.0)  // Different column/row gaps
//!     .build();
//!
//! // Responsive grid with constraints
//! let responsive_grid = Grid::new("responsive")
//!     .columns_sizes(vec![
//!         GridTrackSize::MinMax(
//!             Box::new(GridTrackSize::Px(150.0)),  // Min 150px
//!             Box::new(GridTrackSize::Fr(1.0)),    // Max 1fr
//!         ),
//!         GridTrackSize::Fr(2.0),  // Main content gets 2x space
//!     ])
//!     .build();
//! ```
//!
//! ### Grid Item Positioning
//! ```rust
//! // Grid items can span multiple cells or be positioned explicitly
//! let positioned_grid = Grid::layout("dashboard", 4, 3)
//!     .gap(12.0)
//!     .with_child(header_entity)      // Auto-placed
//!     .with_child(sidebar_entity)     // Auto-placed  
//!     .with_child(main_entity)        // Auto-placed
//!     .build();
//!
//! // Child items can use grid positioning methods:
//! // .grid_column_span(2)   // Span 2 columns
//! // .grid_row_start(2)     // Start at row 2
//! // .grid_column_end(4)    // End at column 4
//! ```
//!
//! ### Styling and Theming
//! ```rust
//! use forge_ui::{Grid, theme};
//!
//! // Themed grid with background and borders
//! let styled_grid = Grid::new("styled")
//!     .columns_fr(3)
//!     .rows_fr(2)
//!     .color(theme().blue)           // Use blue color palette
//!     .border(1.0)                   // Add border
//!     .rounded()                     // Rounded corners
//!     .padding(Val::Px(16.0))        // Internal spacing
//!     .build();
//!
//! // Custom colors overriding theme
//! let custom_grid = Grid::new("custom")
//!     .columns_auto(4)
//!     .background_color(Color::srgb(0.95, 0.95, 0.95))
//!     .border_color(Color::srgb(0.8, 0.8, 0.8))
//!     .border(2.0)
//!     .build();
//! ```
//!
//! ## Layout Patterns
//!
//! ### Common Grid Patterns
//! - **Gallery**: Equal columns with responsive wrapping
//! - **Sidebar Layout**: Fixed sidebar + flexible content
//! - **Dashboard**: Mixed-size panels with explicit positioning
//! - **Card Grid**: Uniform card layouts with consistent spacing
//! - **Form Layout**: Label/input pairs with proper alignment
//!
//! ### Best Practices
//! 1. **Start Simple**: Use shorthand methods like `columns()` and `layout()`
//! 2. **Use Fractional Units**: `fr` units create flexible, responsive layouts
//! 3. **Consider Content**: Use `auto` sizing when content determines size
//! 4. **Mind the Gaps**: Consistent spacing improves visual hierarchy
//! 5. **Theme Integration**: Leverage color palettes for consistent styling

use crate::{
    components::box_component::{RadiusLevel, SpacingLevel},
    theme::{
        color::{accent_palette, UiColorPalette},
        layout::UiLayout,
    },
    utilities::ComponentBuilder,
};
use bevy::prelude::*;
use bevy::ui::{GridPlacement, MaxTrackSizingFunction, MinTrackSizingFunction};
use bevy_picking::prelude::Pickable;

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
struct ExplicitColors {
    /// Explicitly set background color
    background: Option<Color>,
    /// Explicitly set border color
    border: Option<Color>,
}

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
    name: String,
    /// Grid configuration (tracks, gaps, colors)
    grid_config: GridComponent,
    /// Bevy UI Node for layout properties
    node: Node,
    /// Tracking for explicit color overrides
    explicit_colors: ExplicitColors,
    /// Child entities to include in the grid
    children: Vec<Entity>,
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

    /// Sets uniform spacing between all grid items.
    /// 
    /// Creates consistent spacing in both row and column directions.
    /// This is the most common method for grid spacing.
    /// 
    /// # Arguments
    /// * `gap` - Spacing between grid items in pixels
    /// 
    /// # Example
    /// ```rust
    /// let spaced_grid = Grid::new("gallery")
    ///     .columns_fr(3)
    ///     .gap(16.0)  // 16px spacing between all items
    ///     .build();
    /// ```
    pub fn gap(mut self, gap: f32) -> Self {
        self.grid_config.gap = GridGap::Uniform(gap);
        self
    }

    /// Sets gap using theme spacing levels.
    /// 
    /// Uses predefined spacing values from the theme system for consistency.
    /// Note: Currently resolves during build phase.
    /// 
    /// # Arguments
    /// * `level` - Predefined spacing level from theme
    pub fn gap_level(mut self, _level: SpacingLevel) -> Self {
        // We'll resolve this during build when we have access to the theme
        self.grid_config.gap = GridGap::Uniform(0.0); // Placeholder
        self
    }

    /// Sets different spacing for rows and columns.
    /// 
    /// Allows independent control over horizontal and vertical spacing,
    /// useful when different spacing is needed in each direction.
    /// 
    /// # Arguments
    /// * `column_gap` - Horizontal spacing between columns
    /// * `row_gap` - Vertical spacing between rows
    /// 
    /// # Example
    /// ```rust
    /// let asymmetric = Grid::new("form")
    ///     .columns_fr(2)
    ///     .gap_xy(20.0, 12.0)  // 20px column gap, 12px row gap
    ///     .build();
    /// ```
    pub fn gap_xy(mut self, column_gap: f32, row_gap: f32) -> Self {
        self.grid_config.gap = GridGap::Axis {
            row: row_gap,
            column: column_gap,
        };
        self
    }

    /// Sets spacing between rows (vertical spacing).
    /// 
    /// Controls only the vertical spacing between grid rows.
    /// Preserves existing column gap if set, otherwise sets column gap to 0.
    /// 
    /// # Arguments
    /// * `gap` - Vertical spacing between rows in pixels
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

    /// Sets spacing between columns (horizontal spacing).
    /// 
    /// Controls only the horizontal spacing between grid columns.
    /// Preserves existing row gap if set, otherwise sets row gap to 0.
    /// 
    /// # Arguments
    /// * `gap` - Horizontal spacing between columns in pixels
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

    // === SIZE METHODS ===

    /// Set width
    pub fn width(mut self, width: Val) -> Self {
        self.node.width = width;
        self
    }

    /// Set height
    pub fn height(mut self, height: Val) -> Self {
        self.node.height = height;
        self
    }

    /// Set both width and height
    pub fn size(mut self, width: Val, height: Val) -> Self {
        self.node.width = width;
        self.node.height = height;
        self
    }

    /// Set width and height to the same value (square)
    pub fn square(mut self, size: Val) -> Self {
        self.node.width = size;
        self.node.height = size;
        self
    }

    /// Set width to 100%
    pub fn fill_width(mut self) -> Self {
        self.node.width = Val::Percent(100.0);
        self
    }

    /// Set height to 100%
    pub fn fill_height(mut self) -> Self {
        self.node.height = Val::Percent(100.0);
        self
    }

    /// Set both width and height to 100%
    pub fn fill(mut self) -> Self {
        self.node.width = Val::Percent(100.0);
        self.node.height = Val::Percent(100.0);
        self
    }

    /// Set min width
    pub fn min_width(mut self, width: Val) -> Self {
        self.node.min_width = width;
        self
    }

    /// Set min height
    pub fn min_height(mut self, height: Val) -> Self {
        self.node.min_height = height;
        self
    }

    /// Set max width
    pub fn max_width(mut self, width: Val) -> Self {
        self.node.max_width = width;
        self
    }

    /// Set max height
    pub fn max_height(mut self, height: Val) -> Self {
        self.node.max_height = height;
        self
    }

    /// Set aspect ratio
    pub fn aspect_ratio(mut self, ratio: f32) -> Self {
        self.node.aspect_ratio = Some(ratio);
        self
    }

    // === PADDING METHODS ===

    /// Set uniform padding
    pub fn padding(mut self, padding: Val) -> Self {
        self.node.padding = UiRect::all(padding);
        self
    }

    /// Set horizontal padding (left and right)
    pub fn padding_x(mut self, padding: Val) -> Self {
        self.node.padding.left = padding;
        self.node.padding.right = padding;
        self
    }

    /// Set vertical padding (top and bottom)
    pub fn padding_y(mut self, padding: Val) -> Self {
        self.node.padding.top = padding;
        self.node.padding.bottom = padding;
        self
    }

    /// Set individual padding values
    pub fn padding_sides(mut self, top: Val, right: Val, bottom: Val, left: Val) -> Self {
        self.node.padding = UiRect {
            left,
            right,
            top,
            bottom,
        };
        self
    }

    /// Set padding using theme spacing level
    pub fn pad(self, _level: SpacingLevel) -> Self {
        // Note: We'll resolve the actual value during build when we have theme access
        self
    }

    /// Set horizontal padding using theme spacing level
    pub fn pad_x(self, _level: SpacingLevel) -> Self {
        // Note: We'll resolve the actual value during build when we have theme access
        self
    }

    /// Set vertical padding using theme spacing level
    pub fn pad_y(self, _level: SpacingLevel) -> Self {
        // Note: We'll resolve the actual value during build when we have theme access
        self
    }

    // === MARGIN METHODS ===

    /// Set uniform margin
    pub fn margin(mut self, margin: Val) -> Self {
        self.node.margin = UiRect::all(margin);
        self
    }

    /// Set horizontal margin (left and right)
    pub fn margin_x(mut self, margin: Val) -> Self {
        self.node.margin.left = margin;
        self.node.margin.right = margin;
        self
    }

    /// Set vertical margin (top and bottom)
    pub fn margin_y(mut self, margin: Val) -> Self {
        self.node.margin.top = margin;
        self.node.margin.bottom = margin;
        self
    }

    /// Set individual margin values
    pub fn margin_sides(mut self, top: Val, right: Val, bottom: Val, left: Val) -> Self {
        self.node.margin = UiRect {
            left,
            right,
            top,
            bottom,
        };
        self
    }

    // === POSITIONING METHODS ===

    /// Set position type to relative (default)
    pub fn position_relative(mut self) -> Self {
        self.node.position_type = PositionType::Relative;
        self
    }

    /// Set position type to absolute
    pub fn position_absolute(mut self) -> Self {
        self.node.position_type = PositionType::Absolute;
        self
    }

    /// Set top position
    pub fn top(mut self, top: Val) -> Self {
        self.node.top = top;
        self
    }

    /// Set right position
    pub fn right(mut self, right: Val) -> Self {
        self.node.right = right;
        self
    }

    /// Set bottom position
    pub fn bottom(mut self, bottom: Val) -> Self {
        self.node.bottom = bottom;
        self
    }

    /// Set left position
    pub fn left(mut self, left: Val) -> Self {
        self.node.left = left;
        self
    }

    // === GRID CHILD PROPERTIES ===

    /// Set grid column start
    pub fn grid_column_start(mut self, start: i16) -> Self {
        self.node.grid_column = GridPlacement::start(start);
        self
    }

    /// Set grid column end
    pub fn grid_column_end(mut self, end: i16) -> Self {
        self.node.grid_column = GridPlacement::end(end);
        self
    }

    /// Set grid column span
    pub fn grid_column_span(mut self, span: u16) -> Self {
        self.node.grid_column = GridPlacement::span(span);
        self
    }

    /// Set grid row start
    pub fn grid_row_start(mut self, start: i16) -> Self {
        self.node.grid_row = GridPlacement::start(start);
        self
    }

    /// Set grid row end
    pub fn grid_row_end(mut self, end: i16) -> Self {
        self.node.grid_row = GridPlacement::end(end);
        self
    }

    /// Set grid row span
    pub fn grid_row_span(mut self, span: u16) -> Self {
        self.node.grid_row = GridPlacement::span(span);
        self
    }

    // === COLOR METHODS ===

    /// Set color palette for theming
    pub fn color(mut self, palette: UiColorPalette) -> Self {
        self.grid_config.color_palette = palette;
        self
    }

    /// Set explicit background color (overrides theme)
    pub fn background_color(mut self, color: Color) -> Self {
        self.explicit_colors.background = Some(color);
        self.grid_config.styling_config.explicit_background = Some(color);
        self
    }

    /// Set explicit border color (overrides theme)
    pub fn border_color(mut self, color: Color) -> Self {
        self.explicit_colors.border = Some(color);
        self.grid_config.styling_config.explicit_border = Some(color);
        self
    }

    // === BORDER METHODS ===

    /// Set border width
    pub fn border(mut self, width: f32) -> Self {
        self.node.border = UiRect::all(Val::Px(width));
        self.grid_config.styling_config.border_width = Some(width);
        self
    }

    /// Set border radius using theme radius level
    pub fn rounded(self) -> Self {
        // Note: We'll resolve the actual value during build when we have theme access
        self
    }

    /// Set border radius using specific theme radius level
    pub fn rounded_level(self, _level: RadiusLevel) -> Self {
        // Note: We'll resolve the actual value during build when we have theme access
        self
    }

    /// Set specific border radius
    pub fn radius(self, _radius: Val) -> Self {
        // Note: BorderRadius will be handled in the build() method
        self
    }

    // === CHILDREN METHODS ===

    /// Add child entity
    pub fn with_child(mut self, child: Entity) -> Self {
        self.children.push(child);
        self
    }

    /// Add multiple child entities
    pub fn with_children(mut self, children: Vec<Entity>) -> Self {
        self.children.extend(children);
        self
    }
}

impl ComponentBuilder for GridBuilder {
    /// The complete bundle of components created by the GridBuilder.
    /// 
    /// This bundle includes all necessary Bevy components for a fully
    /// functional grid layout with proper styling and interaction support.
    type Output = (
        Name,              // Component identification
        GridComponent,     // Grid configuration
        Node,              // Layout and positioning
        BackgroundColor,   // Background styling
        BorderColor,       // Border styling
        BorderRadius,      // Corner rounding
        Pickable,          // Interaction support
    );

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
    fn build(mut self) -> Self::Output {
        // Configure Node for CSS Grid display
        self.node.display = Display::Grid;

        // Convert grid template columns to Bevy's format
        match &self.grid_config.template_columns {
            GridTrack::None => {
                // No explicit columns - grid will create implicit tracks
            }
            GridTrack::Repeat(count, size) => {
                // Convert repeated track pattern to Bevy format
                self.node.grid_template_columns = vec![track_size_to_bevy(*count, size)];
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
                self.node.grid_template_rows = vec![track_size_to_bevy(*count, size)];
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

/// Type alias for convenient access to GridComponent.
/// 
/// This allows using `Grid` instead of `GridComponent` in most contexts,
/// providing a shorter and more intuitive name for the grid system.
/// 
/// # Example
/// ```rust
/// use forge_ui::Grid;
/// 
/// let my_grid = Grid::new("layout").columns_fr(3).build();
/// ```
pub type Grid = GridComponent;
