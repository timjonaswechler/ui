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
//! // Mixed track sizing for complex layouts
//! let layout = Grid::new("main-layout")
//!     .columns_sizes(vec![
//!         GridTrackSize::Px(250.0),    // Fixed sidebar
//!         GridTrackSize::Fr(1.0),      // Flexible main area
//!         GridTrackSize::Px(200.0),    // Fixed panel
//!     ])
//!     .rows_sizes(vec![
//!         GridTrackSize::Px(60.0),     // Header
//!         GridTrackSize::Fr(1.0),      // Content
//!         GridTrackSize::Px(40.0),     // Footer
//!     ])
//!     .gap_xy(16.0, 8.0)  // Different horizontal and vertical gaps
//!     .build();
//! ```
//!
//! ### Responsive Grids
//! ```rust
//! use forge_ui::{Grid, GridTrackSize};
//!
//! // Grid that adapts to content and container size
//! let responsive = Grid::new("responsive-gallery")
//!     .columns_sizes(vec![
//!         GridTrackSize::MinMax(
//!             Box::new(GridTrackSize::Px(200.0)),  // Min 200px
//!             Box::new(GridTrackSize::Fr(1.0)),    // Grow to available space
//!         ),
//!         GridTrackSize::MinMax(
//!             Box::new(GridTrackSize::Px(200.0)),
//!             Box::new(GridTrackSize::Fr(1.0)),
//!         ),
//!         GridTrackSize::MinMax(
//!             Box::new(GridTrackSize::Px(200.0)),
//!             Box::new(GridTrackSize::Fr(1.0)),
//!         ),
//!     ])
//!     .gap(16.0)
//!     .fill_width()
//!     .build();
//! ```

// Module declarations
pub mod builder;
pub mod conversion;
pub mod core;
pub mod styling;
pub mod tracks;

// Re-export all public types for backward compatibility
pub use self::core::GridComponent;
pub use self::builder::GridBuilder;
pub use self::styling::GridStyling;
pub use self::tracks::{GridGap, GridTrack, GridTrackSize};

// Import the conversion functionality (for the build method)
pub use self::conversion::*;

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