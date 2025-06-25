//! Badge Component for Forge UI
//!
//! The Badge component is a small UI element used to display status, notifications,
//! or metadata in a compact and visually distinct way. Badges are typically used to
//! show counts, status indicators, labels, or other brief information.
//!
//! ## Features
//!
//! - **4 Visual Variants**: Solid, Soft, Surface, and Outline styles
//! - **3 Size Options**: Small (Size1), Medium (Size2), and Large (Size3)
//! - **Theme Integration**: Full support for Radix UI color palette system
//! - **High Contrast Mode**: Enhanced accessibility with increased contrast
//! - **Flexible Border Radius**: From sharp corners to pill-shaped badges
//! - **Text Integration**: Built-in text support with proper typography scaling
//! - **Custom Content**: Support for custom child elements and text builders
//!
//! ## Examples
//!
//! ### Basic Usage
//! ```rust
//! use forge_ui::Badge;
//!
//! // Simple status badge
//! let status_badge = Badge::builder("status")
//!     .text("Active")
//!     .solid()
//!     .accent()
//!     .build();
//!
//! // Notification count badge
//! let count_badge = Badge::builder("notifications")
//!     .text("3")
//!     .soft()
//!     .size_1()
//!     .pill()
//!     .build();
//! ```
//!
//! ### Advanced Configuration
//! ```rust
//! use forge_ui::{Badge, BadgeVariant, BadgeSize, theme};
//!
//! // High contrast outline badge with custom styling
//! let warning_badge = Badge::builder("warning")
//!     .text("Important")
//!     .variant(BadgeVariant::Outline)
//!     .size(BadgeSize::Size3)
//!     .color(theme().orange)
//!     .high_contrast(true)
//!     .rounded_medium()
//!     .build();
//! ```
//!
//! ## Variant Descriptions
//!
//! - **Solid**: High-contrast badge with solid background color
//! - **Soft**: Medium-contrast badge with subtle background
//! - **Surface**: Low-contrast badge that blends with the surface
//! - **Outline**: Border-only badge with transparent background
//!
//! ## Size Guidelines
//!
//! - **Size1 (Small)**: Compact badges for dense UIs (6px/2px padding)
//! - **Size2 (Medium)**: Default size for general use (8px/4px padding)
//! - **Size3 (Large)**: Prominent badges for emphasis (12px/6px padding)
//!
//! ## Accessibility
//!
//! - Use `high_contrast(true)` for better visibility
//! - Choose appropriate colors for colorblind users
//! - Ensure sufficient color contrast ratios
//! - Consider text alternatives for color-coded information

use crate::{
    components::text::{Text, TextBuilder},
    theme::{
        color::{accent_palette, theme, TextColor, UiColorPalette},
        typography::{TextSize, TextWeight},
    },
    utilities::ComponentBuilder,
};
use bevy::{ecs::spawn::SpawnWith, prelude::*};

/// Core Badge component that defines the appearance and behavior of a badge UI element.
/// 
/// Badges are small, compact UI elements used to display status information, counts,
/// labels, or other metadata. They support multiple visual variants, sizes, and
/// can be extensively customized through the theme system.
/// 
/// # Fields
/// 
/// * `variant` - Visual style variant (Solid, Soft, Surface, Outline)
/// * `size` - Size variant affecting padding and text size
/// * `color` - Color palette from the theme system
/// * `high_contrast` - Enhanced contrast for better accessibility
/// * `radius` - Border radius configuration
#[derive(Component, Debug, Clone)]
pub struct Badge {
    pub variant: BadgeVariant,
    pub size: BadgeSize,
    pub color: UiColorPalette,
    pub high_contrast: bool,
    pub radius: BadgeRadius,
}

impl Default for Badge {
    /// Creates a default Badge with sensible defaults:
    /// - Surface variant for subtle appearance
    /// - Size2 (medium) for general use
    /// - Gray color for neutral appearance
    /// - Normal contrast for standard visibility
    /// - Full radius for pill shape
    fn default() -> Self {
        Self {
            variant: BadgeVariant::Surface,
            size: BadgeSize::Size2,
            color: theme().gray,
            high_contrast: false,
            radius: BadgeRadius::Full,
        }
    }
}

/// Visual variant options for Badge appearance.
/// 
/// Each variant provides different levels of visual prominence and contrast,
/// allowing badges to blend with or stand out from their surroundings as needed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum BadgeVariant {
    /// High-contrast solid background with contrasting text.
    /// Use for important status indicators or primary actions.
    Solid,
    
    /// Medium-contrast with subtle background color.
    /// Good balance between visibility and subtlety.
    Soft,
    
    /// Low-contrast that blends with the surface.
    /// Default variant for non-critical information.
    #[default]
    Surface,
    
    /// Border-only with transparent background.
    /// Minimal visual impact while maintaining clear boundaries.
    Outline,
}

/// Size variants for Badge components.
/// 
/// Controls the overall dimensions, padding, and text size of badges.
/// Choose based on the UI density and importance of the badge content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum BadgeSize {
    /// Small/Compact size (6px/2px padding, XS text).
    /// Ideal for dense interfaces or secondary information.
    Size1,
    
    /// Medium/Default size (8px/4px padding, SM text).
    /// Good balance for most use cases.
    #[default]
    Size2,
    
    /// Large/Prominent size (12px/6px padding, Base text).
    /// Use for important information that needs emphasis.
    Size3,
}

/// Border radius options for Badge appearance.
/// 
/// Controls the roundness of badge corners, from sharp rectangular
/// badges to completely rounded pill-shaped badges.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum BadgeRadius {
    /// Sharp corners (0px radius).
    /// Creates rectangular badges with crisp edges.
    None,
    
    /// Slightly rounded corners (2px radius).
    /// Subtle rounding for softer appearance.
    Small,
    
    /// Moderately rounded corners (4px radius).
    /// Good balance between sharp and rounded.
    Medium,
    
    /// Well-rounded corners (8px radius).
    /// Noticeably rounded but not pill-shaped.
    Large,
    
    /// Fully rounded (pill shape, 9999px radius).
    /// Creates oval/pill-shaped badges.
    #[default]
    Full,
}

/// Builder for constructing Badge components using a fluent API.
/// 
/// The BadgeBuilder provides a convenient way to create and customize Badge components
/// with method chaining. It handles the complex logic of mapping badge properties
/// to Bevy UI components and styling.
/// 
/// # Example
/// ```rust
/// let badge = Badge::builder("status")
///     .text("Online")
///     .solid()
///     .accent()
///     .size_2()
///     .pill()
///     .build();
/// ```
pub struct BadgeBuilder {
    /// Component name for debugging and identification
    name: String,
    /// Badge configuration
    badge: Badge,
    /// Simple text content (alternative to text_builder)
    text: Option<String>,
    /// Advanced text configuration (overrides simple text)
    text_builder: Option<TextBuilder>,
    /// Additional child entities to include in the badge
    children: Vec<Entity>,
}

impl BadgeBuilder {
    /// Creates a new BadgeBuilder with the specified name.
    /// 
    /// # Arguments
    /// * `name` - A name for the badge component (used for debugging)
    /// 
    /// # Returns
    /// A new BadgeBuilder with default settings
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            badge: Badge::default(),
            text: None,
            text_builder: None,
            children: Vec::new(),
        }
    }

    /// Sets the visual variant of the badge.
    /// 
    /// # Arguments
    /// * `variant` - The BadgeVariant to use (Solid, Soft, Surface, Outline)
    pub fn variant(mut self, variant: BadgeVariant) -> Self {
        self.badge.variant = variant;
        self
    }

    /// Sets the size of the badge.
    /// 
    /// # Arguments
    /// * `size` - The BadgeSize to use (Size1, Size2, Size3)
    pub fn size(mut self, size: BadgeSize) -> Self {
        self.badge.size = size;
        self
    }

    /// Sets the color palette for the badge.
    /// 
    /// # Arguments
    /// * `color` - A UiColorPalette from the theme system
    pub fn color(mut self, color: UiColorPalette) -> Self {
        self.badge.color = color;
        self
    }

    /// Enables or disables high contrast mode for better accessibility.
    /// 
    /// # Arguments
    /// * `high_contrast` - Whether to use high contrast colors
    pub fn high_contrast(mut self, high_contrast: bool) -> Self {
        self.badge.high_contrast = high_contrast;
        self
    }

    /// Sets the border radius of the badge.
    /// 
    /// # Arguments
    /// * `radius` - The BadgeRadius to use
    pub fn radius(mut self, radius: BadgeRadius) -> Self {
        self.badge.radius = radius;
        self
    }

    /// Sets simple text content for the badge.
    /// 
    /// This is a convenience method for basic text. For more complex text
    /// styling, use `text_builder()` instead.
    /// 
    /// # Arguments
    /// * `text` - The text content to display
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    /// Sets advanced text configuration using a TextBuilder.
    /// 
    /// This overrides any simple text set with `text()` and allows
    /// for fine-grained control over text appearance.
    /// 
    /// # Arguments
    /// * `text_builder` - A configured TextBuilder instance
    pub fn text_builder(mut self, text_builder: TextBuilder) -> Self {
        self.text_builder = Some(text_builder);
        self
    }

    /// Adds a child entity to the badge.
    /// 
    /// # Arguments
    /// * `entity` - The entity to add as a child
    pub fn child(mut self, entity: Entity) -> Self {
        self.children.push(entity);
        self
    }

    /// Adds multiple child entities to the badge.
    /// 
    /// # Arguments
    /// * `entities` - Vector of entities to add as children
    pub fn children(mut self, entities: Vec<Entity>) -> Self {
        self.children.extend(entities);
        self
    }

    // === Convenience Methods for Variants ===
    
    /// Convenience method to set the Solid variant.
    /// Creates a high-contrast badge with solid background.
    pub fn solid(self) -> Self {
        self.variant(BadgeVariant::Solid)
    }

    /// Convenience method to set the Soft variant.
    /// Creates a medium-contrast badge with subtle background.
    pub fn soft(self) -> Self {
        self.variant(BadgeVariant::Soft)
    }

    /// Convenience method to set the Surface variant (default).
    /// Creates a low-contrast badge that blends with the surface.
    pub fn surface(self) -> Self {
        self.variant(BadgeVariant::Surface)
    }

    /// Convenience method to set the Outline variant.
    /// Creates a border-only badge with transparent background.
    pub fn outline(self) -> Self {
        self.variant(BadgeVariant::Outline)
    }

    // === Convenience Methods for Sizes ===
    
    /// Convenience method to set Size1 (small).
    /// Creates a compact badge with minimal padding.
    pub fn size_1(self) -> Self {
        self.size(BadgeSize::Size1)
    }

    /// Convenience method to set Size2 (medium, default).
    /// Creates a standard-sized badge for general use.
    pub fn size_2(self) -> Self {
        self.size(BadgeSize::Size2)
    }

    /// Convenience method to set Size3 (large).
    /// Creates a prominent badge with generous padding.
    pub fn size_3(self) -> Self {
        self.size(BadgeSize::Size3)
    }

    // === Convenience Methods for Border Radius ===
    
    /// Convenience method for no border radius (sharp corners).
    pub fn rounded_none(self) -> Self {
        self.radius(BadgeRadius::None)
    }

    /// Convenience method for small border radius (2px).
    pub fn rounded_small(self) -> Self {
        self.radius(BadgeRadius::Small)
    }

    /// Convenience method for medium border radius (4px).
    pub fn rounded_medium(self) -> Self {
        self.radius(BadgeRadius::Medium)
    }

    /// Convenience method for large border radius (8px).
    pub fn rounded_large(self) -> Self {
        self.radius(BadgeRadius::Large)
    }

    /// Convenience method for full border radius (pill shape).
    /// Creates an oval/pill-shaped badge.
    pub fn pill(self) -> Self {
        self.radius(BadgeRadius::Full)
    }

    // === Convenience Methods for Colors ===
    
    /// Convenience method to use the accent color palette.
    /// Sets the badge to use the theme's primary accent color.
    pub fn accent(self) -> Self {
        self.color(accent_palette())
    }

    /// Convenience method to use the gray color palette.
    /// Creates a neutral badge suitable for general information.
    pub fn gray(self) -> Self {
        self.color(theme().gray)
    }
}

impl BadgeBuilder {
    /// Builds the Badge into a Bevy Bundle ready for spawning.
    /// 
    /// This method performs the complex logic of translating Badge configuration
    /// into appropriate Bevy UI components with proper styling, layout, and content.
    /// 
    /// # Returns
    /// A Bundle containing all necessary components for a fully functional badge
    /// 
    /// # Implementation Details
    /// - Calculates colors based on variant and contrast settings
    /// - Sets appropriate padding and border radius based on size and radius settings
    /// - Creates proper text styling that scales with badge size
    /// - Handles both simple text and advanced TextBuilder content
    /// - Manages border styling for outline variants
    pub fn build(self) -> impl Bundle {
        let badge = self.badge.clone();

        // Calculate border radius based on badge radius
        let border_radius = match badge.radius {
            BadgeRadius::None => BorderRadius::all(Val::Px(0.0)),
            BadgeRadius::Small => BorderRadius::all(Val::Px(2.0)),
            BadgeRadius::Medium => BorderRadius::all(Val::Px(4.0)),
            BadgeRadius::Large => BorderRadius::all(Val::Px(8.0)),
            BadgeRadius::Full => BorderRadius::all(Val::Px(9999.0)), // Large enough for pill shape
        };

        // Calculate padding based on badge size
        let (horizontal_padding, vertical_padding) = match badge.size {
            BadgeSize::Size1 => (6.0, 2.0),  // Compact
            BadgeSize::Size2 => (8.0, 4.0),  // Default
            BadgeSize::Size3 => (12.0, 6.0), // Large
        };

        // Calculate colors based on variant and contrast
        let (background_color, border_color, _text_color) = match badge.variant {
            BadgeVariant::Solid => {
                if badge.high_contrast {
                    (
                        badge.color.solid.with_alpha(1.0),
                        badge.color.solid,
                        badge.color.text_contrast,
                    )
                } else {
                    (
                        badge.color.solid.with_alpha(0.9),
                        badge.color.solid,
                        badge.color.text_contrast,
                    )
                }
            }
            BadgeVariant::Soft => {
                if badge.high_contrast {
                    (
                        badge.color.bg.with_alpha(0.8),
                        Color::NONE,
                        badge.color.text,
                    )
                } else {
                    (
                        badge.color.bg_subtle.with_alpha(0.6),
                        Color::NONE,
                        badge.color.text,
                    )
                }
            }
            BadgeVariant::Surface => {
                if badge.high_contrast {
                    (
                        badge.color.bg_subtle.with_alpha(0.5),
                        badge.color.border,
                        badge.color.text,
                    )
                } else {
                    (
                        badge.color.bg_subtle.with_alpha(0.3),
                        badge.color.border.with_alpha(0.7),
                        badge.color.text,
                    )
                }
            }
            BadgeVariant::Outline => {
                if badge.high_contrast {
                    (Color::NONE, badge.color.border, badge.color.text)
                } else {
                    (
                        Color::NONE,
                        badge.color.border.with_alpha(0.8),
                        badge.color.text.with_alpha(0.9),
                    )
                }
            }
        };

        // Calculate border width for outline variant
        let border = if badge.variant == BadgeVariant::Outline {
            UiRect::all(Val::Px(1.0))
        } else {
            UiRect::default()
        };

        let text_size = match badge.size {
            BadgeSize::Size1 => TextSize::Xs,
            BadgeSize::Size2 => TextSize::Sm,
            BadgeSize::Size3 => TextSize::Base,
        };

        let text_color_enum = TextColor::Custom(_text_color);
        let display_text = self.text.clone().unwrap_or_default();
        let text_builder = self.text_builder.clone();

        (
            badge,
            Name::new(self.name),
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                padding: UiRect {
                    left: Val::Px(horizontal_padding),
                    right: Val::Px(horizontal_padding),
                    top: Val::Px(vertical_padding),
                    bottom: Val::Px(vertical_padding),
                },
                border,
                ..Node::default()
            },
            BackgroundColor(background_color),
            BorderColor(border_color),
            BorderRadius::from(border_radius),
            Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
                if let Some(builder) = text_builder {
                    parent.spawn(builder.center().build());
                } else if !display_text.is_empty() {
                    parent.spawn(
                        Text::label(display_text)
                            .color(text_color_enum)
                            .size(text_size)
                            .weight(TextWeight::Medium)
                            .center()
                            .build(),
                    );
                }
            })),
        )
    }
}

impl Badge {
    /// Creates a new BadgeBuilder for constructing a Badge component.
    /// 
    /// This is the primary entry point for creating badges using the builder pattern.
    /// 
    /// # Arguments
    /// * `name` - A name for the badge component (used for debugging)
    /// 
    /// # Returns
    /// A new BadgeBuilder with default settings
    /// 
    /// # Example
    /// ```rust
    /// let badge = Badge::builder("my-badge")
    ///     .text("Status")
    ///     .solid()
    ///     .build();
    /// ```
    pub fn builder(name: impl Into<String>) -> BadgeBuilder {
        BadgeBuilder::new(name)
    }
}
