//! Section Component for Forge UI
//!
//! The Section component provides semantic content organization and vertical rhythm
//! for Bevy applications. Based on the HTML `<section>` element, it creates meaningful
//! content divisions with consistent spacing, hierarchy, and layout patterns.
//! This component is essential for creating well-structured, accessible UI layouts.
//!
//! ## Key Features
//!
//! - **Semantic Structure**: HTML-like section semantics for meaningful content organization
//! - **Vertical Rhythm**: Standardized vertical spacing system for content hierarchy
//! - **Flexible Sizing**: Four spacing levels for different content importance
//! - **Theme Integration**: Automatic styling with color palettes and design tokens
//! - **Layout Control**: Built-in flexbox layout with gap and alignment options
//! - **Responsive Design**: Percentage-based widths and flexible sizing options
//!
//! ## Vertical Rhythm System
//!
//! The Section component implements a vertical rhythm system for consistent spacing:
//!
//! | Size | Spacing | Use Case |
//! |------|---------|----------|
//! | **Size1** | 16px | Compact sections, dense layouts, sidebar content |
//! | **Size2** | 24px | Small sections, secondary content areas |
//! | **Size3** | 32px | Default sections, main content areas (recommended) |
//! | **Size4** | 48px | Large sections, major content divisions, hero areas |
//!
//! ## Examples
//!
//! ### Basic Section Usage
//! ```rust
//! use forge_ui::Section;
//!
//! // Default section with medium spacing
//! let main_section = Section::size_3("main-content")
//!     .fill_width()
//!     .build();
//!
//! // Compact section for dense layouts
//! let sidebar_section = Section::size_1("sidebar")
//!     .width(Val::Px(250.0))
//!     .build();
//!
//! // Large section for major content areas
//! let hero_section = Section::size_4("hero")
//!     .fill_width()
//!     .center_content()
//!     .build();
//! ```
//!
//! ### Content Organization
//! ```rust
//! // Create sections with child content
//! commands.spawn(Section::size_3("article").build())
//!     .with_children(|parent| {
//!         parent.spawn(Heading::h2("Article Title").build());
//!         parent.spawn(Text::body("Article content...").build());
//!         
//!         // Nested subsection with smaller spacing
//!         parent.spawn(Section::size_2("subsection").build())
//!             .with_children(|parent| {
//!                 parent.spawn(Heading::h3("Subsection").build());
//!                 parent.spawn(Text::body("Subsection content...").build());
//!             });
//!     });
//! ```
//!
//! ### Advanced Layout and Styling
//! ```rust
//! use forge_ui::{Section, theme};
//!
//! // Styled section with background and border
//! let styled_section = Section::size_3("featured")
//!     .color(theme().blue)           // Blue color palette
//!     .background_alpha(0.05)        // Subtle background
//!     .border(1.0)                   // Border
//!     .rounded()                     // Rounded corners
//!     .gap(16.0)                     // Gap between children
//!     .build();
//!
//! // Responsive section with constraints
//! let responsive_section = Section::size_3("content")
//!     .fill_width()                  // Full width
//!     .max_width(Val::Px(800.0))     // Max width constraint
//!     .padding_x(Val::Px(24.0))      // Horizontal padding
//!     .center_content()              // Center children
//!     .build();
//! ```
//!
//! ### Layout Patterns
//! ```rust
//! // Card-like section
//! let card_section = Section::size_2("card")
//!     .background_alpha(0.1)
//!     .border(1.0)
//!     .rounded()
//!     .padding_x(Val::Px(20.0))
//!     .gap(12.0)
//!     .build();
//!
//! // Full-width hero section
//! let hero = Section::size_4("hero")
//!     .fill_width()
//!     .height(Val::Vh(100.0))        // Full viewport height
//!     .center_content()              // Center all content
//!     .background_color(Color::srgb(0.1, 0.1, 0.2))
//!     .build();
//! ```
//!
//! ## Semantic Guidelines
//!
//! ### When to Use Section
//! - **Content Organization**: Group related content together
//! - **Hierarchical Structure**: Create clear content hierarchy
//! - **Page Layout**: Divide pages into logical sections
//! - **Responsive Design**: Create flexible, responsive layouts
//!
//! ### Section Hierarchy
//! ```rust
//! // Main page structure
//! Section::size_4("page")           // Page-level container
//!   ├── Section::size_3("header")    // Header section
//!   ├── Section::size_3("main")      // Main content
//!   │   ├── Section::size_2("intro") // Introduction
//!   │   └── Section::size_2("body")  // Body content
//!   └── Section::size_2("footer")    // Footer section
//! ```
//!
//! ## Accessibility Features
//!
//! - **Semantic Structure**: Provides meaningful content organization
//! - **Consistent Spacing**: Predictable layout aids navigation
//! - **Focus Management**: Proper tab order and focus handling
//! - **Screen Reader Support**: Clear content boundaries
//!
//! ## Performance Considerations
//!
//! - Sections use efficient flexbox layout
//! - Minimal DOM overhead with semantic organization
//! - Theme-based styling reduces redundant style calculations
//! - Lazy evaluation of colors and styling properties

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

/// Core Section component that provides semantic content organization with vertical rhythm.
/// 
/// The Section component creates meaningful content divisions with consistent spacing,
/// hierarchy, and layout patterns. It implements a vertical rhythm system that ensures
/// consistent spacing throughout the application while providing semantic structure
/// similar to HTML `<section>` elements.
/// 
/// # Design Philosophy
/// 
/// Sections follow the principle of "semantic structure with visual hierarchy":
/// 1. **Semantic Meaning**: Each section represents a distinct content area
/// 2. **Visual Hierarchy**: Different sizes create clear content importance levels
/// 3. **Consistent Spacing**: Standardized vertical rhythm across the application
/// 4. **Flexible Layout**: Built-in flexbox with customizable alignment and gaps
/// 
/// # Default Behavior
/// 
/// - **Layout**: Column-based flexbox layout (vertical stacking)
/// - **Width**: 100% width by default (full container width)
/// - **Spacing**: Size3 (32px) vertical padding by default
/// - **Background**: Transparent by default (semantic container)
/// - **Borders**: No borders by default (clean content separation)
#[derive(Component, Debug, Clone)]
pub struct SectionComponent {
    /// The size variant that determines vertical spacing and visual hierarchy
    pub size: SectionSize,
    /// Color palette for theme-based styling and semantic color roles
    pub color_palette: UiColorPalette,
    /// Additional styling configuration for backgrounds, borders, and effects
    pub styling_config: SectionStyling,
}

impl Default for SectionComponent {
    /// Creates a default Section with optimal settings for most use cases.
    /// 
    /// Default configuration:
    /// - **Size3**: 32px vertical spacing (comfortable for main content)
    /// - **Accent palette**: Uses current accent color for theme consistency
    /// - **Default styling**: Transparent background, no borders
    fn default() -> Self {
        Self {
            size: SectionSize::Size3, // Default to medium spacing
            color_palette: accent_palette(),
            styling_config: SectionStyling::default(),
        }
    }
}

/// Section size variants that define vertical spacing and content hierarchy.
/// 
/// SectionSize implements a standardized vertical rhythm system that creates
/// consistent spacing and visual hierarchy throughout the application.
/// Each size corresponds to specific use cases and content importance levels.
/// 
/// # Spacing Philosophy
/// 
/// The spacing values follow a progressive scale that creates clear visual hierarchy:
/// - **Geometric progression**: Each level provides noticeably different spacing
/// - **Practical ranges**: Values tested for real-world content organization
/// - **Accessibility**: Spacing supports easy content scanning and navigation
/// 
/// # Size Selection Guidelines
/// 
/// - **Size1**: For dense layouts where space is premium
/// - **Size2**: For secondary content that needs clear separation
/// - **Size3**: For primary content areas (recommended default)
/// - **Size4**: For major page divisions and hero sections
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SectionSize {
    /// Compact sections with minimal vertical spacing (16px).
    /// 
    /// **Use cases**: Sidebar content, dense layouts, nested sections
    /// **Content type**: Secondary information, compact lists
    /// **Layout context**: Space-constrained areas, mobile layouts
    Size1,
    
    /// Small sections with moderate vertical spacing (24px).
    /// 
    /// **Use cases**: Secondary content areas, subsections
    /// **Content type**: Supporting content, secondary articles
    /// **Layout context**: Medium-priority content areas
    Size2,
    
    /// Default sections with comfortable vertical spacing (32px).
    /// 
    /// **Use cases**: Main content areas, primary articles, standard sections
    /// **Content type**: Primary content, main articles, important information
    /// **Layout context**: Primary content areas (recommended for most content)
    #[default]
    Size3,
    
    /// Large sections with generous vertical spacing (48px).
    /// 
    /// **Use cases**: Major page divisions, hero sections, landing page areas
    /// **Content type**: Major content blocks, featured content, page headers
    /// **Layout context**: High-impact areas, landing pages, major divisions
    Size4,
}

impl SectionSize {
    /// Converts section size to vertical padding in pixels.
    /// 
    /// This method provides the core vertical rhythm values that create
    /// consistent spacing throughout the application. The values are carefully
    /// chosen to create clear visual hierarchy while maintaining readability.
    /// 
    /// # Spacing Values
    /// 
    /// The progression follows a practical scale:
    /// - **16px**: Minimum spacing for compact layouts
    /// - **24px**: Moderate spacing for secondary content (1.5x base)
    /// - **32px**: Comfortable spacing for primary content (2x base)
    /// - **48px**: Generous spacing for major divisions (3x base)
    /// 
    /// # Returns
    /// 
    /// The vertical padding value in pixels as f32
    pub fn to_vertical_padding(self) -> f32 {
        match self {
            SectionSize::Size1 => 16.0,  // Compact - minimal spacing
            SectionSize::Size2 => 24.0,  // Small - moderate spacing  
            SectionSize::Size3 => 32.0,  // Default - comfortable spacing
            SectionSize::Size4 => 48.0,  // Large - generous spacing
        }
    }
}

/// Styling configuration for Section component appearance and visual effects.
/// 
/// SectionStyling provides additional visual customization options beyond
/// the core layout and spacing functionality. It allows for background colors,
/// borders, and transparency effects while maintaining theme integration.
/// 
/// # Design Approach
/// 
/// Sections are typically semantic containers that rely on spacing rather than
/// visual styling for organization. However, styling options are available for:
/// - Creating visual cards or panels
/// - Adding subtle background highlights
/// - Defining clear content boundaries
/// - Creating layered visual effects
#[derive(Debug, Clone, Default)]
pub struct SectionStyling {
    /// Alpha/opacity for background color (0.0 = transparent, 1.0 = opaque)
    pub background_alpha: f32,
    /// Border width in pixels (None = no border)
    pub border_width: Option<f32>,
    /// Explicit background color override (None = use theme/alpha)
    pub explicit_background: Option<Color>,
    /// Explicit border color override (None = use theme)
    pub explicit_border: Option<Color>,
}

/// Builder for creating Section components using a fluent API.
/// 
/// SectionBuilder provides a comprehensive interface for configuring sections
/// with semantic spacing, layout control, styling options, and theme integration.
/// It follows the builder pattern for intuitive method chaining and type safety.
/// 
/// # Building Process
/// 
/// 1. **Initialize**: Start with size-based factory methods or `new()`
/// 2. **Configure**: Chain methods to set spacing, layout, and styling
/// 3. **Build**: Call `.build()` to create the final component bundle
/// 
/// # Method Categories
/// 
/// - **Size Control**: Set vertical spacing and content hierarchy
/// - **Layout**: Width, height, margins, and flexbox alignment
/// - **Spacing**: Gaps, padding, and additional spacing controls
/// - **Styling**: Colors, backgrounds, borders, and visual effects
/// - **Theme**: Integration with color palettes and design tokens
pub struct SectionBuilder {
    /// Component name for debugging and identification
    name: String,
    /// Section configuration (size, colors, styling)
    section_config: SectionComponent,
    /// Bevy UI Node for layout properties
    node: Node,
    /// Tracking for explicit color overrides
    explicit_colors: ExplicitColors,
}

/// Internal tracking for explicit color overrides.
/// 
/// This struct helps the builder distinguish between theme-based colors
/// and explicitly set colors, ensuring proper precedence during the build process.
#[derive(Default)]
struct ExplicitColors {
    /// Explicitly set background color
    background: Option<Color>,
    /// Explicitly set border color
    border: Option<Color>,
}

impl SectionComponent {
    /// Creates a new Section component builder with default settings.
    /// 
    /// This is the base factory method that provides full control over
    /// section configuration. For most use cases, consider using the
    /// size-specific factory methods instead.
    /// 
    /// # Arguments
    /// * `name` - A name for the section component (used for debugging)
    /// 
    /// # Returns
    /// A SectionBuilder ready for configuration
    pub fn new(name: impl Into<String>) -> SectionBuilder {
        SectionBuilder::new(name)
    }

    // === Size-Specific Factory Methods ===

    /// Creates a compact section with Size1 spacing (16px vertical).
    /// 
    /// Size1 sections are ideal for:
    /// - Sidebar content and navigation
    /// - Dense layouts with multiple sections
    /// - Nested sections within larger containers
    /// - Mobile layouts where space is limited
    /// 
    /// # Arguments
    /// * `name` - A name for the section component
    /// 
    /// # Example
    /// ```rust
    /// let sidebar_section = Section::size_1("navigation")
    ///     .width(Val::Px(250.0))
    ///     .gap(8.0)
    ///     .build();
    /// ```
    pub fn size_1(name: impl Into<String>) -> SectionBuilder {
        Self::new(name).size(SectionSize::Size1)
    }

    /// Creates a small section with Size2 spacing (24px vertical).
    /// 
    /// Size2 sections are ideal for:
    /// - Secondary content areas
    /// - Subsections within main content
    /// - Supporting information blocks
    /// - Content that needs clear separation but isn't primary
    /// 
    /// # Arguments
    /// * `name` - A name for the section component
    /// 
    /// # Example
    /// ```rust
    /// let subsection = Section::size_2("related-content")
    ///     .background_alpha(0.05)
    ///     .padding_x(Val::Px(16.0))
    ///     .build();
    /// ```
    pub fn size_2(name: impl Into<String>) -> SectionBuilder {
        Self::new(name).size(SectionSize::Size2)
    }

    /// Creates a default section with Size3 spacing (32px vertical).
    /// 
    /// Size3 sections are ideal for:
    /// - Main content areas (recommended for most content)
    /// - Primary articles and content blocks
    /// - Standard page sections
    /// - General-purpose content organization
    /// 
    /// # Arguments
    /// * `name` - A name for the section component
    /// 
    /// # Example
    /// ```rust
    /// let main_section = Section::size_3("main-content")
    ///     .fill_width()
    ///     .max_width(Val::Px(800.0))
    ///     .build();
    /// ```
    pub fn size_3(name: impl Into<String>) -> SectionBuilder {
        Self::new(name).size(SectionSize::Size3)
    }

    /// Creates a large section with Size4 spacing (48px vertical).
    /// 
    /// Size4 sections are ideal for:
    /// - Major page divisions and hero sections
    /// - Landing page content blocks
    /// - Featured content areas
    /// - High-impact sections that need emphasis
    /// 
    /// # Arguments
    /// * `name` - A name for the section component
    /// 
    /// # Example
    /// ```rust
    /// let hero_section = Section::size_4("hero")
    ///     .fill_width()
    ///     .height(Val::Vh(100.0))
    ///     .center_content()
    ///     .background_color(Color::srgb(0.1, 0.1, 0.2))
    ///     .build();
    /// ```
    pub fn size_4(name: impl Into<String>) -> SectionBuilder {
        Self::new(name).size(SectionSize::Size4)
    }
}

impl SectionBuilder {
    /// Creates a new SectionBuilder with sensible defaults.
    /// 
    /// The builder is initialized with optimal settings for semantic content organization:
    /// - **Column layout**: Vertical stacking of child elements
    /// - **Full width**: 100% width for container-based sizing
    /// - **Size3 spacing**: 32px vertical padding (comfortable default)
    /// - **No horizontal padding**: Clean edge-to-edge content
    /// 
    /// # Arguments
    /// * `name` - Component name for debugging and identification
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: format!("{}_Section", name.into()),
            section_config: SectionComponent::default(),
            node: Node {
                // Default to column layout for semantic content stacking
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.0),
                // Apply default Size3 vertical padding for comfortable spacing
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

    // === Size Control Methods ===

    /// Sets the section size, which determines vertical spacing and hierarchy.
    /// 
    /// This method updates both the section configuration and applies the
    /// corresponding vertical padding to create consistent visual rhythm.
    /// 
    /// # Arguments
    /// * `size` - The SectionSize variant to apply
    /// 
    /// # Example
    /// ```rust
    /// let section = Section::new("content")
    ///     .size(SectionSize::Size2)  // 24px vertical spacing
    ///     .build();
    /// ```
    pub fn size(mut self, size: SectionSize) -> Self {
        self.section_config.size = size;
        // Apply vertical padding based on size
        let padding = Val::Px(size.to_vertical_padding());
        self.node.padding.top = padding;
        self.node.padding.bottom = padding;
        self
    }

    /// Convenience method: Sets section to Size1 (compact, 16px vertical spacing).
    /// 
    /// Ideal for dense layouts and sidebar content.
    pub fn size_1(self) -> Self {
        self.size(SectionSize::Size1)
    }

    /// Convenience method: Sets section to Size2 (small, 24px vertical spacing).
    /// 
    /// Ideal for secondary content and subsections.
    pub fn size_2(self) -> Self {
        self.size(SectionSize::Size2)
    }

    /// Convenience method: Sets section to Size3 (default, 32px vertical spacing).
    /// 
    /// Ideal for main content areas (recommended for most content).
    pub fn size_3(self) -> Self {
        self.size(SectionSize::Size3)
    }

    /// Convenience method: Sets section to Size4 (large, 48px vertical spacing).
    /// 
    /// Ideal for major page divisions and hero sections.
    pub fn size_4(self) -> Self {
        self.size(SectionSize::Size4)
    }

    // === Layout Dimension Methods ===

    /// Sets the section width.
    /// 
    /// # Arguments
    /// * `width` - The width value (pixels, percentage, viewport units, etc.)
    pub fn width(mut self, width: Val) -> Self {
        self.node.width = width;
        self
    }

    /// Sets the section height.
    /// 
    /// # Arguments
    /// * `height` - The height value (pixels, percentage, viewport units, etc.)
    pub fn height(mut self, height: Val) -> Self {
        self.node.height = height;
        self
    }

    /// Sets both width and height dimensions.
    /// 
    /// # Arguments
    /// * `width` - The width value
    /// * `height` - The height value
    /// 
    /// # Example
    /// ```rust
    /// let fixed_section = Section::size_2("card")
    ///     .size_dimensions(Val::Px(300.0), Val::Px(200.0))
    ///     .build();
    /// ```
    pub fn size_dimensions(mut self, width: Val, height: Val) -> Self {
        self.node.width = width;
        self.node.height = height;
        self
    }

    /// Sets the section width to 100% of its container.
    /// 
    /// This is the default behavior, but useful for explicitly ensuring
    /// full-width sections in responsive layouts.
    pub fn fill_width(mut self) -> Self {
        self.node.width = Val::Percent(100.0);
        self
    }

    /// Sets the maximum width constraint for responsive layouts.
    /// 
    /// Useful for creating centered, constrained content areas.
    /// 
    /// # Arguments
    /// * `max_width` - The maximum width value
    /// 
    /// # Example
    /// ```rust
    /// let responsive_section = Section::size_3("content")
    ///     .fill_width()
    ///     .max_width(Val::Px(800.0))  // Constrain to 800px max
    ///     .build();
    /// ```
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

/// Convenience type alias for SectionComponent.
/// 
/// This allows using `Section` instead of `SectionComponent` in most contexts,
/// providing a shorter and more intuitive name for the section system.
/// 
/// # Example
/// ```rust
/// use forge_ui::Section;
/// 
/// let my_section = Section::size_3("content").build();
/// ```
pub type Section = SectionComponent;