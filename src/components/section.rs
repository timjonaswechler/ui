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

/// Section component - semantic content container with vertical rhythm
///
/// The Section component provides consistent vertical spacing between larger
/// parts of page content, creating hierarchy and separation. Based on HTML
/// section element for semantic meaning.
///
/// # Usage
/// ```rust
/// // Spawn section and add children using Bevy's with_children pattern
/// let section_entity = commands.spawn(Section::size_3("main").build()).id();
/// commands.entity(section_entity).with_children(|parent| {
///     parent.spawn(Heading::h2("Title").build());
///     parent.spawn(Text::body("Content").build());  
/// });
/// ```
#[derive(Component, Debug, Clone)]
pub struct SectionComponent {
    pub size: SectionSize,
    pub color_palette: UiColorPalette,
    pub styling_config: SectionStyling,
}

impl Default for SectionComponent {
    fn default() -> Self {
        Self {
            size: SectionSize::Size3, // Default to medium spacing
            color_palette: accent_palette(),
            styling_config: SectionStyling::default(),
        }
    }
}

/// Section size variants for different vertical spacing levels
///
/// Provides standardized vertical spacing to create content hierarchy:
/// - Size1: Compact sections with minimal spacing (16px)
/// - Size2: Small sections with moderate spacing (24px)  
/// - Size3: Default sections with comfortable spacing (32px)
/// - Size4: Large sections with generous spacing (48px)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SectionSize {
    /// Compact sections with minimal vertical spacing (16px)
    Size1,
    /// Small sections with moderate vertical spacing (24px)
    Size2,
    /// Default sections with comfortable vertical spacing (32px)
    #[default]
    Size3,
    /// Large sections with generous vertical spacing (48px)
    Size4,
}

impl SectionSize {
    /// Convert section size to vertical padding pixels
    pub fn to_vertical_padding(self) -> f32 {
        match self {
            SectionSize::Size1 => 16.0,
            SectionSize::Size2 => 24.0,
            SectionSize::Size3 => 32.0,
            SectionSize::Size4 => 48.0,
        }
    }
}

/// Styling configuration for Section component
#[derive(Debug, Clone, Default)]
pub struct SectionStyling {
    pub background_alpha: f32,
    pub border_width: Option<f32>,
    pub explicit_background: Option<Color>,
    pub explicit_border: Option<Color>,
}

/// Builder for creating Section components with fluent API
pub struct SectionBuilder {
    name: String,
    section_config: SectionComponent,
    node: Node,
    explicit_colors: ExplicitColors,
}

#[derive(Default)]
struct ExplicitColors {
    background: Option<Color>,
    border: Option<Color>,
}

impl SectionComponent {
    /// Create a new Section component builder
    pub fn new(name: impl Into<String>) -> SectionBuilder {
        SectionBuilder::new(name)
    }

    /// Create a Section with Size1 (compact, 16px vertical spacing)
    pub fn size_1(name: impl Into<String>) -> SectionBuilder {
        Self::new(name).size(SectionSize::Size1)
    }

    /// Create a Section with Size2 (small, 24px vertical spacing)
    pub fn size_2(name: impl Into<String>) -> SectionBuilder {
        Self::new(name).size(SectionSize::Size2)
    }

    /// Create a Section with Size3 (default, 32px vertical spacing)
    pub fn size_3(name: impl Into<String>) -> SectionBuilder {
        Self::new(name).size(SectionSize::Size3)
    }

    /// Create a Section with Size4 (large, 48px vertical spacing)
    pub fn size_4(name: impl Into<String>) -> SectionBuilder {
        Self::new(name).size(SectionSize::Size4)
    }
}

impl SectionBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: format!("{}_Section", name.into()),
            section_config: SectionComponent::default(),
            node: Node {
                // Default to column layout for sections
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.0),
                // Apply default Size3 vertical padding
                padding: UiRect {
                    top: Val::Px(SectionSize::Size3.to_vertical_padding()),
                    bottom: Val::Px(SectionSize::Size3.to_vertical_padding()),
                    left: Val::Px(0.0),
                    right: Val::Px(0.0),
                },
                ..default()
            },
            explicit_colors: ExplicitColors::default(),
        }
    }

    // =========================================================================
    // SIZE CONTROL METHODS 
    // =========================================================================

    /// Set the section size (affects vertical spacing)
    pub fn size(mut self, size: SectionSize) -> Self {
        self.section_config.size = size;
        // Apply vertical padding based on size
        let padding = Val::Px(size.to_vertical_padding());
        self.node.padding.top = padding;
        self.node.padding.bottom = padding;
        self
    }

    /// Use Size1 (compact, 16px vertical spacing)
    pub fn size_1(self) -> Self {
        self.size(SectionSize::Size1)
    }

    /// Use Size2 (small, 24px vertical spacing)
    pub fn size_2(self) -> Self {
        self.size(SectionSize::Size2)
    }

    /// Use Size3 (default, 32px vertical spacing)
    pub fn size_3(self) -> Self {
        self.size(SectionSize::Size3)
    }

    /// Use Size4 (large, 48px vertical spacing)
    pub fn size_4(self) -> Self {
        self.size(SectionSize::Size4)
    }

    // =========================================================================
    // LAYOUT METHODS
    // =========================================================================

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
    pub fn size_dimensions(mut self, width: Val, height: Val) -> Self {
        self.node.width = width;
        self.node.height = height;
        self
    }

    /// Set width to 100%
    pub fn fill_width(mut self) -> Self {
        self.node.width = Val::Percent(100.0);
        self
    }

    /// Set maximum width
    pub fn max_width(mut self, max_width: Val) -> Self {
        self.node.max_width = max_width;
        self
    }

    // =========================================================================
    // SPACING METHODS (ADDITIONAL TO SIZE-BASED PADDING)
    // =========================================================================

    /// Add horizontal padding (in addition to vertical spacing)
    pub fn padding_x(mut self, padding: Val) -> Self {
        self.node.padding.left = padding;
        self.node.padding.right = padding;
        self
    }

    /// Set horizontal padding using theme spacing level
    pub fn pad_x(mut self, level: SpacingLevel) -> Self {
        let padding = level.to_val(&UiLayout::default().padding);
        self.node.padding.left = padding;
        self.node.padding.right = padding;
        self
    }

    /// Set margin on all sides
    pub fn margin(mut self, margin: Val) -> Self {
        self.node.margin = UiRect::all(margin);
        self
    }

    /// Set vertical margin (top and bottom)
    pub fn margin_y(mut self, margin: Val) -> Self {
        self.node.margin.top = margin;
        self.node.margin.bottom = margin;
        self
    }

    /// Set horizontal margin (left and right)
    pub fn margin_x(mut self, margin: Val) -> Self {
        self.node.margin.left = margin;
        self.node.margin.right = margin;
        self
    }

    /// Set bottom margin (useful for section separation)
    pub fn margin_bottom(mut self, margin: Val) -> Self {
        self.node.margin.bottom = margin;
        self
    }

    // =========================================================================
    // FLEX LAYOUT METHODS
    // =========================================================================

    /// Set gap between child elements
    pub fn gap(mut self, gap: f32) -> Self {
        self.node.row_gap = Val::Px(gap);
        self.node.column_gap = Val::Px(gap);
        self
    }

    /// Set gap using theme spacing level
    pub fn gap_level(mut self, level: SpacingLevel) -> Self {
        let gap_val = level.to_val(&UiLayout::default().padding);
        self.node.row_gap = gap_val;
        self.node.column_gap = gap_val;
        self
    }

    /// Set justify content for child alignment
    pub fn justify_content(mut self, justify: JustifyContent) -> Self {
        self.node.justify_content = justify;
        self
    }

    /// Set align items for child alignment
    pub fn align_items(mut self, align: AlignItems) -> Self {
        self.node.align_items = align;
        self
    }

    /// Center all child content
    pub fn center_content(mut self) -> Self {
        self.node.justify_content = JustifyContent::Center;
        self.node.align_items = AlignItems::Center;
        self
    }

    // =========================================================================
    // STYLING METHODS
    // =========================================================================

    /// Set color palette for theming
    pub fn color(mut self, palette: UiColorPalette) -> Self {
        self.section_config.color_palette = palette;
        self
    }

    /// Set explicit background color (overrides theme)
    pub fn background_color(mut self, color: Color) -> Self {
        self.explicit_colors.background = Some(color);
        self.section_config.styling_config.explicit_background = Some(color);
        self
    }

    /// Set explicit border color (overrides theme)
    pub fn border_color(mut self, color: Color) -> Self {
        self.explicit_colors.border = Some(color);
        self.section_config.styling_config.explicit_border = Some(color);
        self
    }

    /// Add border with specified width
    pub fn border(mut self, width: f32) -> Self {
        self.node.border = UiRect::all(Val::Px(width));
        self.section_config.styling_config.border_width = Some(width);
        self
    }

    /// Set border radius using theme radius level
    pub fn rounded(self) -> Self {
        // Note: BorderRadius will be calculated in build() method
        self
    }

    /// Set border radius using specific theme radius level
    pub fn rounded_level(self, _level: RadiusLevel) -> Self {
        // Note: BorderRadius will be calculated in build() method
        self
    }

    /// Set background alpha for transparency
    pub fn background_alpha(mut self, alpha: f32) -> Self {
        self.section_config.styling_config.background_alpha = alpha.clamp(0.0, 1.0);
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
    // BUILD HELPER METHODS
    // =========================================================================

    /// Calculate background color based on configuration
    fn calculate_background_color(&self) -> BackgroundColor {
        if let Some(explicit_color) = self.section_config.styling_config.explicit_background {
            return BackgroundColor(explicit_color);
        }

        // Sections are typically transparent by default (semantic containers)
        let alpha = self.section_config.styling_config.background_alpha;
        if alpha > 0.0 {
            let base_color = self.section_config.color_palette.bg_subtle;
            BackgroundColor(base_color.with_alpha(alpha))
        } else {
            BackgroundColor(Color::NONE)
        }
    }

    /// Calculate border color based on configuration
    fn calculate_border_color(&self) -> BorderColor {
        if let Some(explicit_color) = self.section_config.styling_config.explicit_border {
            return BorderColor(explicit_color);
        }

        if self.section_config.styling_config.border_width.is_some() {
            BorderColor(self.section_config.color_palette.border)
        } else {
            BorderColor(Color::NONE)
        }
    }

    /// Calculate border radius
    fn calculate_border_radius(&self) -> BorderRadius {
        // Sections typically have minimal/no border radius for clean content separation
        let radius_val = Val::Px(UiLayout::default().radius.sm);
        BorderRadius {
            top_left: radius_val,
            top_right: radius_val,
            bottom_left: radius_val,
            bottom_right: radius_val,
        }
    }
}

impl ComponentBuilder for SectionBuilder {
    type Output = (
        Name,
        SectionComponent,
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
            self.section_config,
            self.node,
            background_color,
            border_color,
            border_radius,
            Pickable::default(),
        )
    }
}

// Convenience type alias
pub type Section = SectionComponent;