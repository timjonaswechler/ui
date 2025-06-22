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

/// Grid component - CSS Grid layout container inspired by Radix UI
///
/// The Grid component provides grid layout capabilities using CSS Grid
/// model, with theme integration and convenient builder methods for common
/// grid patterns.
#[derive(Component, Debug, Clone)]
pub struct GridComponent {
    pub template_columns: GridTrack,
    pub template_rows: GridTrack,
    pub auto_columns: Option<GridTrack>,
    pub auto_rows: Option<GridTrack>,
    pub gap: GridGap,
    pub color_palette: UiColorPalette,
    pub styling_config: GridStyling,
}

impl Default for GridComponent {
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

/// Grid track configuration
#[derive(Debug, Clone, PartialEq)]
pub enum GridTrack {
    /// No explicit tracks
    None,
    /// Repeat a track size n times
    Repeat(u16, GridTrackSize),
    /// Explicit track sizes
    Sizes(Vec<GridTrackSize>),
    /// Grid template string (e.g., "1fr 2fr 1fr")
    Template(String),
}

impl GridTrack {
    pub fn none() -> Self {
        Self::None
    }

    pub fn repeat(count: u16, size: GridTrackSize) -> Self {
        Self::Repeat(count, size)
    }

    pub fn sizes(sizes: Vec<GridTrackSize>) -> Self {
        Self::Sizes(sizes)
    }

    pub fn template(template: impl Into<String>) -> Self {
        Self::Template(template.into())
    }

    /// Create columns with equal fractional units
    pub fn fr(count: u16) -> Self {
        Self::Repeat(count, GridTrackSize::Fr(1.0))
    }

    /// Create columns with equal pixel sizes
    pub fn px(count: u16, size: f32) -> Self {
        Self::Repeat(count, GridTrackSize::Px(size))
    }

    /// Create columns with auto sizing
    pub fn auto(count: u16) -> Self {
        Self::Repeat(count, GridTrackSize::Auto)
    }
}

/// Individual grid track size
#[derive(Debug, Clone, PartialEq)]
pub enum GridTrackSize {
    /// Auto-sized track
    Auto,
    /// Fixed pixel size
    Px(f32),
    /// Fractional unit
    Fr(f32),
    /// Percentage
    Percent(f32),
    /// Minimum content size
    MinContent,
    /// Maximum content size
    MaxContent,
    /// Fit content with max size
    FitContent(f32),
    /// Min-max range
    MinMax(Box<GridTrackSize>, Box<GridTrackSize>),
}

/// Gap configuration for grid layouts
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GridGap {
    None,
    Uniform(f32),
    Axis { row: f32, column: f32 },
}

impl Default for GridGap {
    fn default() -> Self {
        Self::None
    }
}

/// Styling configuration for Grid component
#[derive(Debug, Clone, Default)]
pub struct GridStyling {
    pub background_alpha: f32,
    pub border_width: Option<f32>,
    pub explicit_background: Option<Color>,
    pub explicit_border: Option<Color>,
}

/// Tracking for explicit color overrides
#[derive(Debug, Clone, Default)]
struct ExplicitColors {
    background: Option<Color>,
    border: Option<Color>,
}

/// Builder for creating Grid components with fluent API
pub struct GridBuilder {
    name: String,
    grid_config: GridComponent,
    node: Node,
    explicit_colors: ExplicitColors,
    children: Vec<Entity>,
}

impl GridComponent {
    /// Create a new Grid component builder
    pub fn new(name: impl AsRef<str>) -> GridBuilder {
        GridBuilder {
            name: name.as_ref().to_string(),
            grid_config: GridComponent::default(),
            node: Node::default(),
            explicit_colors: ExplicitColors::default(),
            children: Vec::new(),
        }
    }

    /// Create a simple grid with equal columns (shorthand)
    pub fn columns(name: impl AsRef<str>, count: u16) -> GridBuilder {
        Self::new(name).columns_fr(count)
    }

    /// Create a simple grid with equal rows (shorthand)
    pub fn rows(name: impl AsRef<str>, count: u16) -> GridBuilder {
        Self::new(name).rows_fr(count)
    }

    /// Create a grid with specified columns and rows (shorthand)
    pub fn layout(name: impl AsRef<str>, cols: u16, rows: u16) -> GridBuilder {
        Self::new(name).columns_fr(cols).rows_fr(rows)
    }
}

impl GridBuilder {
    // === GRID TEMPLATE COLUMNS METHODS ===

    /// Set grid template columns to repeat n fractional units
    pub fn columns_fr(mut self, count: u16) -> Self {
        self.grid_config.template_columns = GridTrack::fr(count);
        self
    }

    /// Set grid template columns to repeat n pixel-sized columns
    pub fn columns_px(mut self, count: u16, size: f32) -> Self {
        self.grid_config.template_columns = GridTrack::px(count, size);
        self
    }

    /// Set grid template columns to repeat n auto-sized columns
    pub fn columns_auto(mut self, count: u16) -> Self {
        self.grid_config.template_columns = GridTrack::auto(count);
        self
    }

    /// Set grid template columns with explicit track sizes
    pub fn columns_sizes(mut self, sizes: Vec<GridTrackSize>) -> Self {
        self.grid_config.template_columns = GridTrack::sizes(sizes);
        self
    }

    /// Set grid template columns with template string
    pub fn columns_template(mut self, template: impl Into<String>) -> Self {
        self.grid_config.template_columns = GridTrack::template(template);
        self
    }

    /// Set grid template columns explicitly
    pub fn columns(mut self, columns: GridTrack) -> Self {
        self.grid_config.template_columns = columns;
        self
    }

    // === GRID TEMPLATE ROWS METHODS ===

    /// Set grid template rows to repeat n fractional units
    pub fn rows_fr(mut self, count: u16) -> Self {
        self.grid_config.template_rows = GridTrack::fr(count);
        self
    }

    /// Set grid template rows to repeat n pixel-sized rows
    pub fn rows_px(mut self, count: u16, size: f32) -> Self {
        self.grid_config.template_rows = GridTrack::px(count, size);
        self
    }

    /// Set grid template rows to repeat n auto-sized rows
    pub fn rows_auto(mut self, count: u16) -> Self {
        self.grid_config.template_rows = GridTrack::auto(count);
        self
    }

    /// Set grid template rows with explicit track sizes
    pub fn rows_sizes(mut self, sizes: Vec<GridTrackSize>) -> Self {
        self.grid_config.template_rows = GridTrack::sizes(sizes);
        self
    }

    /// Set grid template rows with template string
    pub fn rows_template(mut self, template: impl Into<String>) -> Self {
        self.grid_config.template_rows = GridTrack::template(template);
        self
    }

    /// Set grid template rows explicitly
    pub fn rows(mut self, rows: GridTrack) -> Self {
        self.grid_config.template_rows = rows;
        self
    }

    // === GRID AUTO METHODS ===

    /// Set grid auto columns
    pub fn auto_columns(mut self, track: GridTrack) -> Self {
        self.grid_config.auto_columns = Some(track);
        self
    }

    /// Set grid auto rows
    pub fn auto_rows(mut self, track: GridTrack) -> Self {
        self.grid_config.auto_rows = Some(track);
        self
    }

    // === GAP METHODS ===

    /// Set uniform gap for both row and column
    pub fn gap(mut self, gap: f32) -> Self {
        self.grid_config.gap = GridGap::Uniform(gap);
        self
    }

    /// Set gap using theme spacing level
    pub fn gap_level(mut self, _level: SpacingLevel) -> Self {
        // We'll resolve this during build when we have access to the theme
        self.grid_config.gap = GridGap::Uniform(0.0); // Placeholder
        self
    }

    /// Set different gaps for row and column
    pub fn gap_xy(mut self, column_gap: f32, row_gap: f32) -> Self {
        self.grid_config.gap = GridGap::Axis {
            row: row_gap,
            column: column_gap,
        };
        self
    }

    /// Set row gap (gap between rows)
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

    /// Set column gap (gap between columns)
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
    type Output = (
        Name,
        GridComponent,
        Node,
        BackgroundColor,
        BorderColor,
        BorderRadius,
        Pickable,
    );

    fn build(mut self) -> Self::Output {
        // Update Node with grid properties
        self.node.display = Display::Grid;

        // Convert grid template columns to Bevy format
        match &self.grid_config.template_columns {
            GridTrack::None => {}
            GridTrack::Repeat(count, size) => {
                self.node.grid_template_columns = vec![track_size_to_bevy(*count, size)];
            }
            GridTrack::Sizes(sizes) => {
                self.node.grid_template_columns = sizes
                    .iter()
                    .map(|size| track_size_to_bevy(1, size))
                    .collect();
            }
            GridTrack::Template(_) => {
                // For template strings, we'd need to parse them - for now use 1fr
                self.node.grid_template_columns = vec![RepeatedGridTrack::fr(1, 1.0)];
            }
        }

        // Convert grid template rows to Bevy format
        match &self.grid_config.template_rows {
            GridTrack::None => {}
            GridTrack::Repeat(count, size) => {
                self.node.grid_template_rows = vec![track_size_to_bevy(*count, size)];
            }
            GridTrack::Sizes(sizes) => {
                self.node.grid_template_rows = sizes
                    .iter()
                    .map(|size| track_size_to_bevy(1, size))
                    .collect();
            }
            GridTrack::Template(_) => {
                // For template strings, we'd need to parse them - for now use 1fr
                self.node.grid_template_rows = vec![RepeatedGridTrack::fr(1, 1.0)];
            }
        }

        // Set gap if specified
        match self.grid_config.gap {
            GridGap::None => {}
            GridGap::Uniform(gap) => {
                self.node.column_gap = Val::Px(gap);
                self.node.row_gap = Val::Px(gap);
            }
            GridGap::Axis { row, column } => {
                self.node.row_gap = Val::Px(row);
                self.node.column_gap = Val::Px(column);
            }
        }

        // Determine colors based on explicit overrides or theme
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

/// Convert our GridTrackSize to Bevy's RepeatedGridTrack
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

// Type alias for convenience
pub type Grid = GridComponent;
