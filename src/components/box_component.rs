use crate::{
    theme::{
        color::{accent_palette, UiColorPalette},
        layout::{UiLayout, UiRadius, UiSpacing},
    },
    utilities::ComponentBuilder,
};
use bevy::prelude::*;
use bevy_picking::prelude::Pickable;

/// Box component - fundamental layout building block inspired by Radix UI
///
/// The Box component serves as the foundation for all layout containers,
/// providing spacing, sizing, and visual styling capabilities while
/// maintaining consistent theme integration.
#[derive(Component, Debug, Clone)]
pub struct BoxComponent {
    pub variant: BoxVariant,
    pub color_palette: UiColorPalette,
    pub styling_config: BoxStyling,
}

impl Default for BoxComponent {
    fn default() -> Self {
        Self {
            variant: BoxVariant::Surface,
            color_palette: accent_palette(),
            styling_config: BoxStyling::default(),
        }
    }
}

/// Container size presets following Radix UI specifications
///
/// Provides standardized max-width constraints for content containers:
/// - Size1: 448px (mobile-first, compact content)
/// - Size2: 688px (tablet, medium content)
/// - Size3: 880px (desktop, standard content)
/// - Size4: 1136px (wide desktop, large content)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContainerSize {
    /// 448px max-width for mobile-first, compact content
    Size1,
    /// 688px max-width for tablet, medium content
    Size2,
    /// 880px max-width for desktop, standard content
    Size3,
    /// 1136px max-width for wide desktop, large content
    Size4,
}

impl ContainerSize {
    /// Convert container size to pixel value
    pub fn to_pixels(self) -> f32 {
        match self {
            ContainerSize::Size1 => CONTAINER_SIZE_1,
            ContainerSize::Size2 => CONTAINER_SIZE_2,
            ContainerSize::Size3 => CONTAINER_SIZE_3,
            ContainerSize::Size4 => CONTAINER_SIZE_4,
        }
    }
}

/// Container size constants following Radix UI specifications
pub const CONTAINER_SIZE_1: f32 = 448.0;
pub const CONTAINER_SIZE_2: f32 = 688.0;
pub const CONTAINER_SIZE_3: f32 = 880.0;
pub const CONTAINER_SIZE_4: f32 = 1136.0;

/// Visual appearance variants for Box component
///
/// Each variant provides different levels of visual prominence:
/// - Surface: Subtle background for content areas
/// - Panel: More prominent background for grouped content  
/// - Card: Elevated appearance with shadow for standalone content
/// - Classic: Enhanced border and background for prominent cards
/// - Ghost: Transparent with hover background for minimal cards
/// - Outline: Border-only appearance for lightweight containers
/// - Container: Max-width constrained layout containers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum BoxVariant {
    /// Subtle background for content areas (default)
    #[default]
    Surface,
    /// More prominent background for grouped content
    Panel,
    /// Elevated appearance with shadow for standalone content
    Card,
    /// Enhanced border and background for prominent cards
    Classic,
    /// Transparent with hover background for minimal cards
    Ghost,
    /// Border-only appearance for lightweight containers
    Outline,
    /// Max-width constrained layout container
    Container(ContainerSize),
}

/// Styling configuration for Box component
#[derive(Debug, Clone, Default)]
pub struct BoxStyling {
    pub background_alpha: f32,
    pub border_width: Option<f32>,
    pub has_shadow: bool,
    pub explicit_background: Option<Color>,
    pub explicit_border: Option<Color>,
}

/// Spacing level enum for theme-integrated spacing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpacingLevel {
    None,
    Xs,
    Sm,
    Base,
    Lg,
    Xl,
    X2l,
    X3l,
    X4l,
    X5l,
}

impl SpacingLevel {
    pub fn to_val(self, spacing: &UiSpacing) -> Val {
        Val::Px(match self {
            SpacingLevel::None => 0.0,
            SpacingLevel::Xs => spacing.xs,
            SpacingLevel::Sm => spacing.sm,
            SpacingLevel::Base => spacing.base,
            SpacingLevel::Lg => spacing.lg,
            SpacingLevel::Xl => spacing.xl,
            SpacingLevel::X2l => spacing.x2l,
            SpacingLevel::X3l => spacing.x3l,
            SpacingLevel::X4l => spacing.x4l,
            SpacingLevel::X5l => spacing.x5l,
        })
    }
}

/// Radius level enum for theme-integrated border radius
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RadiusLevel {
    None,
    Xs,
    Sm,
    Base,
    Lg,
    Xl,
    X2l,
    X3l,
    X4l,
    Full,
}

impl RadiusLevel {
    pub fn to_val(self, radius: &UiRadius) -> Val {
        Val::Px(match self {
            RadiusLevel::None => radius.none,
            RadiusLevel::Xs => radius.xs,
            RadiusLevel::Sm => radius.sm,
            RadiusLevel::Base => radius.base,
            RadiusLevel::Lg => radius.lg,
            RadiusLevel::Xl => radius.xl,
            RadiusLevel::X2l => radius.x2l,
            RadiusLevel::X3l => radius.x3l,
            RadiusLevel::X4l => radius.x4l,
            RadiusLevel::Full => radius.full,
        })
    }
}

/// Builder for creating Box components with fluent API
pub struct BoxBuilder {
    name: String,
    box_config: BoxComponent,
    node: Node,
    explicit_colors: ExplicitColors,
    children: Vec<Entity>,
}

#[derive(Default)]
struct ExplicitColors {
    background: Option<Color>,
    border: Option<Color>,
}

impl BoxComponent {
    /// Create a new Box component builder
    pub fn new(name: impl Into<String>) -> BoxBuilder {
        BoxBuilder::new(name)
    }

    /// Create a container with Size1 (448px max-width)
    pub fn container_1(name: impl Into<String>) -> BoxBuilder {
        Self::new(name).container(ContainerSize::Size1)
    }

    /// Create a container with Size2 (688px max-width)
    pub fn container_2(name: impl Into<String>) -> BoxBuilder {
        Self::new(name).container(ContainerSize::Size2)
    }

    /// Create a container with Size3 (880px max-width)
    pub fn container_3(name: impl Into<String>) -> BoxBuilder {
        Self::new(name).container(ContainerSize::Size3)
    }

    /// Create a container with Size4 (1136px max-width)
    pub fn container_4(name: impl Into<String>) -> BoxBuilder {
        Self::new(name).container(ContainerSize::Size4)
    }
}

impl BoxBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: format!("{}_Box", name.into()),
            box_config: BoxComponent::default(),
            node: Node::default(),
            explicit_colors: ExplicitColors::default(),
            children: Vec::new(),
        }
    }

    // =========================================================================
    // VARIANT CONTROL METHODS
    // =========================================================================

    /// Set the visual variant of the box
    pub fn variant(mut self, variant: BoxVariant) -> Self {
        self.box_config.variant = variant;
        self
    }

    /// Use surface variant (subtle background)
    pub fn surface(self) -> Self {
        self.variant(BoxVariant::Surface)
    }

    /// Use panel variant (prominent background)
    pub fn panel(self) -> Self {
        self.variant(BoxVariant::Panel)
    }

    /// Use card variant (elevated with shadow)
    pub fn card(self) -> Self {
        self.variant(BoxVariant::Card)
    }

    /// Use classic variant (enhanced border and background)
    pub fn classic(self) -> Self {
        self.variant(BoxVariant::Classic)
    }

    /// Use ghost variant (transparent with hover)
    pub fn ghost(self) -> Self {
        self.variant(BoxVariant::Ghost)
    }

    /// Use outline variant (border only)
    pub fn outline(self) -> Self {
        self.variant(BoxVariant::Outline)
    }

    /// Use container variant with specified size
    pub fn container(mut self, size: ContainerSize) -> Self {
        self.box_config.variant = BoxVariant::Container(size);
        self.node.max_width = Val::Px(size.to_pixels());
        self.node.width = Val::Percent(100.0);
        self
    }

    /// Use container variant with Size1 (448px max-width)
    pub fn container_1(self) -> Self {
        self.container(ContainerSize::Size1)
    }

    /// Use container variant with Size2 (688px max-width)
    pub fn container_2(self) -> Self {
        self.container(ContainerSize::Size2)
    }

    /// Use container variant with Size3 (880px max-width)
    pub fn container_3(self) -> Self {
        self.container(ContainerSize::Size3)
    }

    /// Use container variant with Size4 (1136px max-width)
    pub fn container_4(self) -> Self {
        self.container(ContainerSize::Size4)
    }

    // =========================================================================
    // SIZE CONTROL METHODS
    // =========================================================================

    /// Set width of the box
    pub fn width(mut self, width: Val) -> Self {
        self.node.width = width;
        self
    }

    /// Set height of the box
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

    /// Set minimum width and height
    pub fn min_size(mut self, min_width: Val, min_height: Val) -> Self {
        self.node.min_width = min_width;
        self.node.min_height = min_height;
        self
    }

    /// Set maximum width and height
    pub fn max_size(mut self, max_width: Val, max_height: Val) -> Self {
        self.node.max_width = max_width;
        self.node.max_height = max_height;
        self
    }
    /// Set maximum width
    pub fn max_width(mut self, max_width: Val) -> Self {
        self.node.max_width = max_width;
        self
    }
    /// Make box square with given size
    pub fn square(self, size: Val) -> Self {
        self.size(size, size)
    }

    /// Fill available width
    pub fn fill_width(mut self) -> Self {
        self.node.width = Val::Percent(100.0);
        self
    }

    /// Fill available height
    pub fn fill_height(mut self) -> Self {
        self.node.height = Val::Percent(100.0);
        self
    }

    /// Fill both width and height
    pub fn fill(mut self) -> Self {
        self.node.width = Val::Percent(100.0);
        self.node.height = Val::Percent(100.0);
        self
    }

    /// Set aspect ratio
    pub fn aspect_ratio(mut self, ratio: f32) -> Self {
        self.node.aspect_ratio = Some(ratio);
        self
    }

    // =========================================================================
    // PADDING CONTROL METHODS
    // =========================================================================

    /// Set padding on all sides
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

    /// Set individual padding sides
    pub fn padding_sides(mut self, top: Val, right: Val, bottom: Val, left: Val) -> Self {
        self.node.padding = UiRect {
            left,
            right,
            top,
            bottom,
        };
        self
    }

    /// Set top padding
    pub fn padding_top(mut self, padding: Val) -> Self {
        self.node.padding.top = padding;
        self
    }

    /// Set right padding
    pub fn padding_right(mut self, padding: Val) -> Self {
        self.node.padding.right = padding;
        self
    }

    /// Set bottom padding
    pub fn padding_bottom(mut self, padding: Val) -> Self {
        self.node.padding.bottom = padding;
        self
    }

    /// Set left padding
    pub fn padding_left(mut self, padding: Val) -> Self {
        self.node.padding.left = padding;
        self
    }

    // Theme-integrated padding methods
    /// Set padding using theme spacing level
    pub fn pad(mut self, level: SpacingLevel) -> Self {
        let padding = level.to_val(&UiLayout::default().padding);
        self.node.padding = UiRect::all(padding);
        self
    }

    /// Set horizontal padding using theme spacing level
    pub fn pad_x(mut self, level: SpacingLevel) -> Self {
        let padding = level.to_val(&UiLayout::default().padding);
        self.node.padding.left = padding;
        self.node.padding.right = padding;
        self
    }

    /// Set vertical padding using theme spacing level
    pub fn pad_y(mut self, level: SpacingLevel) -> Self {
        let padding = level.to_val(&UiLayout::default().padding);
        self.node.padding.top = padding;
        self.node.padding.bottom = padding;
        self
    }

    // =========================================================================
    // MARGIN CONTROL METHODS
    // =========================================================================

    /// Set margin on all sides
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

    /// Set individual margin sides
    pub fn margin_sides(mut self, top: Val, right: Val, bottom: Val, left: Val) -> Self {
        self.node.margin = UiRect {
            left,
            right,
            top,
            bottom,
        };
        self
    }

    /// Set top margin
    pub fn margin_top(mut self, margin: Val) -> Self {
        self.node.margin.top = margin;
        self
    }

    /// Set right margin
    pub fn margin_right(mut self, margin: Val) -> Self {
        self.node.margin.right = margin;
        self
    }

    /// Set bottom margin
    pub fn margin_bottom(mut self, margin: Val) -> Self {
        self.node.margin.bottom = margin;
        self
    }

    /// Set left margin
    pub fn margin_left(mut self, margin: Val) -> Self {
        self.node.margin.left = margin;
        self
    }

    // =========================================================================
    // POSITIONING CONTROL METHODS
    // =========================================================================

    /// Set position type to relative
    pub fn position_relative(mut self) -> Self {
        self.node.position_type = PositionType::Relative;
        self
    }

    /// Set position type to absolute
    pub fn position_absolute(mut self) -> Self {
        self.node.position_type = PositionType::Absolute;
        self
    }

    /// Set position offsets for absolute positioning
    pub fn position_offset(mut self, top: Val, right: Val, bottom: Val, left: Val) -> Self {
        self.node.top = top;
        self.node.right = right;
        self.node.bottom = bottom;
        self.node.left = left;
        self
    }

    /// Set top position offset
    pub fn top(mut self, top: Val) -> Self {
        self.node.top = top;
        self
    }

    /// Set right position offset
    pub fn right(mut self, right: Val) -> Self {
        self.node.right = right;
        self
    }

    /// Set bottom position offset
    pub fn bottom(mut self, bottom: Val) -> Self {
        self.node.bottom = bottom;
        self
    }

    /// Set left position offset
    pub fn left(mut self, left: Val) -> Self {
        self.node.left = left;
        self
    }

    // =========================================================================
    // FLEX CHILD CONTROL METHODS
    // =========================================================================

    /// Set flex grow factor
    pub fn flex_grow(mut self, grow: f32) -> Self {
        self.node.flex_grow = grow;
        self
    }

    /// Set flex shrink factor
    pub fn flex_shrink(mut self, shrink: f32) -> Self {
        self.node.flex_shrink = shrink;
        self
    }

    /// Set flex basis
    pub fn flex_basis(mut self, basis: Val) -> Self {
        self.node.flex_basis = basis;
        self
    }

    /// Set flex: none (no grow/shrink)
    pub fn flex_none(mut self) -> Self {
        self.node.flex_grow = 0.0;
        self.node.flex_shrink = 0.0;
        self
    }

    /// Set flex: auto (grow and shrink as needed)
    pub fn flex_auto(mut self) -> Self {
        self.node.flex_grow = 1.0;
        self.node.flex_shrink = 1.0;
        self.node.flex_basis = Val::Auto;
        self
    }

    /// Set flex: 1 (fill available space)
    pub fn flex_1(mut self) -> Self {
        self.node.flex_grow = 1.0;
        self.node.flex_shrink = 1.0;
        self.node.flex_basis = Val::Percent(0.0);
        self
    }

    /// Set align self
    pub fn align_self(mut self, align: AlignSelf) -> Self {
        self.node.align_self = align;
        self
    }

    /// Set justify self  
    pub fn justify_self(mut self, justify: JustifySelf) -> Self {
        self.node.justify_self = justify;
        self
    }

    // =========================================================================
    // GRID CHILD CONTROL METHODS
    // =========================================================================

    /// Set grid column placement
    pub fn grid_column(mut self, start: i16, end: i16) -> Self {
        self.node.grid_column = GridPlacement::start_end(start, end);
        self
    }

    /// Set grid row placement
    pub fn grid_row(mut self, start: i16, end: i16) -> Self {
        self.node.grid_row = GridPlacement::start_end(start, end);
        self
    }

    /// Set grid column span
    pub fn grid_column_span(mut self, span: u16) -> Self {
        self.node.grid_column = GridPlacement::span(span);
        self
    }

    /// Set grid row span
    pub fn grid_row_span(mut self, span: u16) -> Self {
        self.node.grid_row = GridPlacement::span(span);
        self
    }

    // =========================================================================
    // OVERFLOW CONTROL METHODS
    // =========================================================================

    /// Set overflow behavior
    pub fn overflow(mut self, overflow: Overflow) -> Self {
        self.node.overflow = overflow;
        self
    }

    /// Set overflow to hidden
    pub fn overflow_hidden(mut self) -> Self {
        self.node.overflow = Overflow::clip();
        self
    }

    /// Set overflow to scroll
    pub fn overflow_scroll(mut self) -> Self {
        self.node.overflow = Overflow::scroll();
        self
    }

    /// Set overflow to auto (scroll when needed)
    pub fn overflow_auto(mut self) -> Self {
        self.node.overflow = Overflow::scroll();
        self
    }

    // =========================================================================
    // COLOR CONTROL METHODS
    // =========================================================================

    /// Set color palette for theme integration
    pub fn color(mut self, palette: UiColorPalette) -> Self {
        self.box_config.color_palette = palette;
        self
    }

    /// Set explicit background color (overrides theme)
    pub fn background_color(mut self, color: Color) -> Self {
        self.explicit_colors.background = Some(color);
        self.box_config.styling_config.explicit_background = Some(color);
        self
    }

    /// Set explicit border color (overrides theme)
    pub fn border_color(mut self, color: Color) -> Self {
        self.explicit_colors.border = Some(color);
        self.box_config.styling_config.explicit_border = Some(color);
        self
    }

    // =========================================================================
    // BORDER RADIUS CONTROL METHODS
    // =========================================================================

    /// Set border radius using theme radius level
    pub fn radius(self, level: RadiusLevel) -> Self {
        let _radius = level.to_val(&UiLayout::default().radius);
        // Note: BorderRadius will be calculated in build() method
        self
    }

    /// Set rounded corners (base level)
    pub fn rounded(self) -> Self {
        self.radius(RadiusLevel::Base)
    }

    /// Set small rounded corners
    pub fn rounded_sm(self) -> Self {
        self.radius(RadiusLevel::Sm)
    }

    /// Set large rounded corners
    pub fn rounded_lg(self) -> Self {
        self.radius(RadiusLevel::Lg)
    }

    /// Set fully rounded corners
    pub fn rounded_full(self) -> Self {
        self.radius(RadiusLevel::Full)
    }

    // =========================================================================
    // BORDER CONTROL METHODS
    // =========================================================================

    /// Set border width
    pub fn border_width(mut self, width: f32) -> Self {
        self.node.border = UiRect::all(Val::Px(width));
        self.box_config.styling_config.border_width = Some(width);
        self
    }

    /// Add border with default width
    pub fn border(self) -> Self {
        self.border_width(1.0)
    }

    // =========================================================================
    // VISUAL ENHANCEMENT METHODS
    // =========================================================================

    /// Add shadow effect (automatically applied to Card variant)
    pub fn shadow(mut self) -> Self {
        self.box_config.styling_config.has_shadow = true;
        self
    }

    /// Set background alpha for transparency
    pub fn background_alpha(mut self, alpha: f32) -> Self {
        self.box_config.styling_config.background_alpha = alpha.clamp(0.0, 1.0);
        self
    }

    // =========================================================================
    // CHILD MANAGEMENT METHODS
    // =========================================================================

    /// Add child entity
    pub fn child(mut self, entity: Entity) -> Self {
        self.children.push(entity);
        self
    }

    /// Add multiple child entities
    pub fn children(mut self, entities: impl IntoIterator<Item = Entity>) -> Self {
        self.children.extend(entities);
        self
    }

    // =========================================================================
    // THEME HELPER METHODS
    // =========================================================================

    /// Use accent color palette
    pub fn accent(self) -> Self {
        self.color(accent_palette())
    }

    // =========================================================================
    // PICKING METHODS
    // =========================================================================

    /// Make the box pickable (enables interaction)
    pub fn pickable(mut self) -> Self {
        self.box_config.styling_config.has_shadow = true; // Ensure shadow is enabled for pickable boxes
        Pickable::default(); // Default pickable component
        self
    }

    // =========================================================================
    // BUILD METHOD
    // =========================================================================

    /// Calculate styling based on variant and configuration
    fn calculate_styling(&self) -> BoxStyling {
        let mut styling = self.box_config.styling_config.clone();

        // Set defaults based on variant
        match self.box_config.variant {
            BoxVariant::Surface => {
                styling.background_alpha = styling.background_alpha.max(0.3);
            }
            BoxVariant::Panel => {
                styling.background_alpha = styling.background_alpha.max(0.5);
            }
            BoxVariant::Card => {
                styling.background_alpha = styling.background_alpha.max(0.8);
                styling.has_shadow = true;
            }
            BoxVariant::Classic => {
                styling.background_alpha = styling.background_alpha.max(0.9);
                styling.border_width = Some(styling.border_width.unwrap_or(2.0));
                styling.has_shadow = true;
            }
            BoxVariant::Ghost => {
                styling.background_alpha = 0.0;
            }
            BoxVariant::Outline => {
                styling.background_alpha = 0.0;
                styling.border_width = Some(styling.border_width.unwrap_or(1.0));
            }
            BoxVariant::Container(_) => {
                // Container uses surface-level styling as default
                styling.background_alpha = styling.background_alpha.max(0.1);
            }
        }

        styling
    }

    /// Calculate background color based on variant and configuration
    fn calculate_background_color(&self) -> BackgroundColor {
        if let Some(explicit_color) = self.box_config.styling_config.explicit_background {
            return BackgroundColor(explicit_color);
        }

        let styling = self.calculate_styling();
        let base_color = match self.box_config.variant {
            BoxVariant::Surface => self.box_config.color_palette.bg_subtle,
            BoxVariant::Panel => self.box_config.color_palette.bg,
            BoxVariant::Card => self.box_config.color_palette.bg,
            BoxVariant::Classic => self.box_config.color_palette.bg,
            BoxVariant::Ghost => self.box_config.color_palette.bg_subtle,
            BoxVariant::Outline => Color::NONE,
            BoxVariant::Container(_) => self.box_config.color_palette.bg_subtle,
        };

        let mut color = base_color.with_alpha(styling.background_alpha);
        if styling.background_alpha == 0.0 {
            color = Color::NONE;
        }

        BackgroundColor(color)
    }

    /// Calculate border color based on variant and configuration
    fn calculate_border_color(&self) -> BorderColor {
        if let Some(explicit_color) = self.box_config.styling_config.explicit_border {
            return BorderColor(explicit_color);
        }

        match self.box_config.variant {
            BoxVariant::Classic => BorderColor(self.box_config.color_palette.border_hover),
            BoxVariant::Outline => BorderColor(self.box_config.color_palette.border),
            _ if self.box_config.styling_config.border_width.is_some() => {
                BorderColor(self.box_config.color_palette.border)
            }
            _ => BorderColor(Color::NONE),
        }
    }

    /// Calculate border radius
    fn calculate_border_radius(&self) -> BorderRadius {
        // For now, use a simple uniform radius
        // This could be enhanced to support per-corner radius in the future
        let radius_val = Val::Px(UiLayout::default().radius.base);
        BorderRadius {
            top_left: radius_val,
            top_right: radius_val,
            bottom_left: radius_val,
            bottom_right: radius_val,
        }
    }
}

impl ComponentBuilder for BoxBuilder {
    type Output = (
        Name,
        BoxComponent,
        Node,
        BackgroundColor,
        BorderColor,
        BorderRadius,
        Pickable,
    );

    fn build(self) -> Self::Output {
        let background_color = self.calculate_background_color();
        let border_color = self.calculate_border_color();
        let border_radius = self.calculate_border_radius();

        (
            Name::new(self.name),
            self.box_config,
            self.node,
            background_color,
            border_color,
            border_radius,
            Pickable::default(),
        )
    }
}

// Convenience type alias
pub type Box = BoxComponent;
