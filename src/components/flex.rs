use crate::{
    components::box_component::{RadiusLevel, SpacingLevel},
    theme::{
        color::{accent_palette, UiColorPalette},
        layout::UiLayout,
    },
    utilities::ComponentBuilder,
};
use bevy::prelude::*;
use bevy_picking::prelude::Pickable;

/// Flex component - flexbox layout container inspired by Radix UI
///
/// The Flex component provides flexible layout capabilities using CSS Flexbox
/// model, with theme integration and convenient builder methods for common
/// flex patterns.
#[derive(Component, Debug, Clone)]
pub struct FlexComponent {
    pub direction: FlexDirection,
    pub wrap: FlexWrap,
    pub justify_content: JustifyContent,
    pub align_items: AlignItems,
    pub align_content: AlignContent,
    pub gap: FlexGap,
    pub color_palette: UiColorPalette,
    pub styling_config: FlexStyling,
}

impl Default for FlexComponent {
    fn default() -> Self {
        Self {
            direction: FlexDirection::Row,
            wrap: FlexWrap::NoWrap,
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Stretch,
            align_content: AlignContent::Start,
            gap: FlexGap::None,
            color_palette: accent_palette(),
            styling_config: FlexStyling::default(),
        }
    }
}

/// Gap configuration for flex layouts
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FlexGap {
    None,
    Uniform(f32),
    Axis { row: f32, column: f32 },
}

impl Default for FlexGap {
    fn default() -> Self {
        Self::None
    }
}

/// Styling configuration for Flex component
#[derive(Debug, Clone, Default)]
pub struct FlexStyling {
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

/// Builder for creating Flex components with fluent API
pub struct FlexBuilder {
    name: String,
    flex_config: FlexComponent,
    node: Node,
    explicit_colors: ExplicitColors,
    children: Vec<Entity>,
}

impl FlexComponent {
    /// Create a new Flex component builder
    pub fn new(name: impl AsRef<str>) -> FlexBuilder {
        FlexBuilder {
            name: name.as_ref().to_string(),
            flex_config: FlexComponent::default(),
            node: Node::default(),
            explicit_colors: ExplicitColors::default(),
            children: Vec::new(),
        }
    }

    /// Create a row flex layout (shorthand)
    pub fn row(name: impl AsRef<str>) -> FlexBuilder {
        Self::new(name).row()
    }

    /// Create a column flex layout (shorthand)
    pub fn column(name: impl AsRef<str>) -> FlexBuilder {
        Self::new(name).column()
    }

    /// Create a centered flex layout (shorthand)
    pub fn center(name: impl AsRef<str>) -> FlexBuilder {
        Self::new(name).justify_center().align_center()
    }
}

impl FlexBuilder {
    // === FLEX DIRECTION METHODS ===

    /// Set flex direction to row (default)
    pub fn row(mut self) -> Self {
        self.flex_config.direction = FlexDirection::Row;
        self
    }

    /// Set flex direction to row reverse
    pub fn row_reverse(mut self) -> Self {
        self.flex_config.direction = FlexDirection::RowReverse;
        self
    }

    /// Set flex direction to column
    pub fn column(mut self) -> Self {
        self.flex_config.direction = FlexDirection::Column;
        self
    }

    /// Set flex direction to column reverse
    pub fn column_reverse(mut self) -> Self {
        self.flex_config.direction = FlexDirection::ColumnReverse;
        self
    }

    /// Set flex direction explicitly
    pub fn direction(mut self, direction: FlexDirection) -> Self {
        self.flex_config.direction = direction;
        self
    }

    // === FLEX WRAP METHODS ===

    /// Set flex wrap to nowrap (default)
    pub fn nowrap(mut self) -> Self {
        self.flex_config.wrap = FlexWrap::NoWrap;
        self
    }

    /// Set flex wrap to wrap
    pub fn wrap(mut self) -> Self {
        self.flex_config.wrap = FlexWrap::Wrap;
        self
    }

    /// Set flex wrap to wrap reverse
    pub fn wrap_reverse(mut self) -> Self {
        self.flex_config.wrap = FlexWrap::WrapReverse;
        self
    }

    /// Set flex wrap explicitly
    pub fn flex_wrap(mut self, wrap: FlexWrap) -> Self {
        self.flex_config.wrap = wrap;
        self
    }

    // === JUSTIFY CONTENT METHODS ===

    /// Set justify content to start (default)
    pub fn justify_start(mut self) -> Self {
        self.flex_config.justify_content = JustifyContent::Start;
        self
    }

    /// Set justify content to end
    pub fn justify_end(mut self) -> Self {
        self.flex_config.justify_content = JustifyContent::End;
        self
    }

    /// Set justify content to center
    pub fn justify_center(mut self) -> Self {
        self.flex_config.justify_content = JustifyContent::Center;
        self
    }

    /// Set justify content to space between
    pub fn justify_between(mut self) -> Self {
        self.flex_config.justify_content = JustifyContent::SpaceBetween;
        self
    }

    /// Set justify content to space around
    pub fn justify_around(mut self) -> Self {
        self.flex_config.justify_content = JustifyContent::SpaceAround;
        self
    }

    /// Set justify content to space evenly
    pub fn justify_evenly(mut self) -> Self {
        self.flex_config.justify_content = JustifyContent::SpaceEvenly;
        self
    }

    /// Set justify content explicitly
    pub fn justify_content(mut self, justify: JustifyContent) -> Self {
        self.flex_config.justify_content = justify;
        self
    }

    // === ALIGN ITEMS METHODS ===

    /// Set align items to start
    pub fn align_start(mut self) -> Self {
        self.flex_config.align_items = AlignItems::Start;
        self
    }

    /// Set align items to end
    pub fn align_end(mut self) -> Self {
        self.flex_config.align_items = AlignItems::End;
        self
    }

    /// Set align items to center
    pub fn align_center(mut self) -> Self {
        self.flex_config.align_items = AlignItems::Center;
        self
    }

    /// Set align items to stretch (default)
    pub fn align_stretch(mut self) -> Self {
        self.flex_config.align_items = AlignItems::Stretch;
        self
    }

    /// Set align items to baseline
    pub fn align_baseline(mut self) -> Self {
        self.flex_config.align_items = AlignItems::Baseline;
        self
    }

    /// Set align items explicitly
    pub fn align_items(mut self, align: AlignItems) -> Self {
        self.flex_config.align_items = align;
        self
    }

    // === ALIGN CONTENT METHODS ===

    /// Set align content to start (default)
    pub fn align_content_start(mut self) -> Self {
        self.flex_config.align_content = AlignContent::Start;
        self
    }

    /// Set align content to end
    pub fn align_content_end(mut self) -> Self {
        self.flex_config.align_content = AlignContent::End;
        self
    }

    /// Set align content to center
    pub fn align_content_center(mut self) -> Self {
        self.flex_config.align_content = AlignContent::Center;
        self
    }

    /// Set align content to stretch
    pub fn align_content_stretch(mut self) -> Self {
        self.flex_config.align_content = AlignContent::Stretch;
        self
    }

    /// Set align content to space between
    pub fn align_content_between(mut self) -> Self {
        self.flex_config.align_content = AlignContent::SpaceBetween;
        self
    }

    /// Set align content to space around
    pub fn align_content_around(mut self) -> Self {
        self.flex_config.align_content = AlignContent::SpaceAround;
        self
    }

    /// Set align content to space evenly
    pub fn align_content_evenly(mut self) -> Self {
        self.flex_config.align_content = AlignContent::SpaceEvenly;
        self
    }

    /// Set align content explicitly
    pub fn align_content(mut self, align: AlignContent) -> Self {
        self.flex_config.align_content = align;
        self
    }

    // === GAP METHODS ===

    /// Set uniform gap for both row and column
    pub fn gap(mut self, gap: f32) -> Self {
        self.flex_config.gap = FlexGap::Uniform(gap);
        self
    }

    /// Set gap using theme spacing level
    pub fn gap_level(mut self, _level: SpacingLevel) -> Self {
        // We'll resolve this during build when we have access to the theme
        self.flex_config.gap = FlexGap::Uniform(0.0); // Placeholder
        self
    }

    /// Set different gaps for row and column
    pub fn gap_xy(mut self, column_gap: f32, row_gap: f32) -> Self {
        self.flex_config.gap = FlexGap::Axis {
            row: row_gap,
            column: column_gap,
        };
        self
    }

    /// Set row gap (gap between rows in wrapped flex)
    pub fn row_gap(mut self, gap: f32) -> Self {
        match self.flex_config.gap {
            FlexGap::Axis { column, .. } => {
                self.flex_config.gap = FlexGap::Axis { row: gap, column };
            }
            _ => {
                self.flex_config.gap = FlexGap::Axis {
                    row: gap,
                    column: 0.0,
                };
            }
        }
        self
    }

    /// Set column gap (gap between columns)
    pub fn column_gap(mut self, gap: f32) -> Self {
        match self.flex_config.gap {
            FlexGap::Axis { row, .. } => {
                self.flex_config.gap = FlexGap::Axis { row, column: gap };
            }
            _ => {
                self.flex_config.gap = FlexGap::Axis {
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

    // === FLEX CHILD PROPERTIES ===

    /// Set flex grow
    pub fn flex_grow(mut self, grow: f32) -> Self {
        self.node.flex_grow = grow;
        self
    }

    /// Set flex shrink
    pub fn flex_shrink(mut self, shrink: f32) -> Self {
        self.node.flex_shrink = shrink;
        self
    }

    /// Set flex basis
    pub fn flex_basis(mut self, basis: Val) -> Self {
        self.node.flex_basis = basis;
        self
    }

    /// Set flex: 1 (grow: 1, shrink: 1, basis: 0)
    pub fn flex_1(mut self) -> Self {
        self.node.flex_grow = 1.0;
        self.node.flex_shrink = 1.0;
        self.node.flex_basis = Val::Px(0.0);
        self
    }

    /// Set flex: auto (grow: 1, shrink: 1, basis: auto)
    pub fn flex_auto(mut self) -> Self {
        self.node.flex_grow = 1.0;
        self.node.flex_shrink = 1.0;
        self.node.flex_basis = Val::Auto;
        self
    }

    /// Set flex: none (grow: 0, shrink: 0, basis: auto)
    pub fn flex_none(mut self) -> Self {
        self.node.flex_grow = 0.0;
        self.node.flex_shrink = 0.0;
        self.node.flex_basis = Val::Auto;
        self
    }

    // === COLOR METHODS ===

    /// Set color palette for theming
    pub fn color(mut self, palette: UiColorPalette) -> Self {
        self.flex_config.color_palette = palette;
        self
    }

    /// Set explicit background color (overrides theme)
    pub fn background_color(mut self, color: Color) -> Self {
        self.explicit_colors.background = Some(color);
        self.flex_config.styling_config.explicit_background = Some(color);
        self
    }

    /// Set explicit border color (overrides theme)
    pub fn border_color(mut self, color: Color) -> Self {
        self.explicit_colors.border = Some(color);
        self.flex_config.styling_config.explicit_border = Some(color);
        self
    }

    // === BORDER METHODS ===

    /// Set border width
    pub fn border(mut self, width: f32) -> Self {
        self.node.border = UiRect::all(Val::Px(width));
        self.flex_config.styling_config.border_width = Some(width);
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

impl ComponentBuilder for FlexBuilder {
    type Output = (
        Name,
        FlexComponent,
        Node,
        BackgroundColor,
        BorderColor,
        BorderRadius,
        Pickable,
    );

    fn build(mut self) -> Self::Output {
        // Update Node with flex properties
        self.node.display = Display::Flex;
        self.node.flex_direction = self.flex_config.direction;
        self.node.flex_wrap = self.flex_config.wrap;
        self.node.justify_content = self.flex_config.justify_content;
        self.node.align_items = self.flex_config.align_items;
        self.node.align_content = self.flex_config.align_content;

        // Set gap if specified
        match self.flex_config.gap {
            FlexGap::None => {}
            FlexGap::Uniform(gap) => {
                self.node.column_gap = Val::Px(gap);
                self.node.row_gap = Val::Px(gap);
            }
            FlexGap::Axis { row, column } => {
                self.node.row_gap = Val::Px(row);
                self.node.column_gap = Val::Px(column);
            }
        }

        // Determine colors based on explicit overrides or theme
        let background_color = self
            .explicit_colors
            .background
            .unwrap_or(self.flex_config.color_palette.bg_subtle);

        let border_color = self
            .explicit_colors
            .border
            .unwrap_or(self.flex_config.color_palette.border);

        (
            Name::new(self.name),
            self.flex_config,
            self.node,
            BackgroundColor(background_color),
            BorderColor(border_color),
            BorderRadius::all(Val::Px(UiLayout::default().radius.base)),
            Pickable::IGNORE,
        )
    }
}

// Type alias for convenience
pub type Flex = FlexComponent;
