use crate::{
    components::box_component::{BoxBuilder, BoxComponent, BoxVariant},
    theme::color::UiColorPalette,
};
use bevy::prelude::*;

/// Card component - content container inspired by Radix UI Card
///
/// A card is a container that groups related content and actions.
/// It provides different visual styles through variants while maintaining
/// consistent spacing and theming.
#[derive(Component, Debug, Clone)]
pub struct Card {
    pub name: String,
    /// Underlying box component configuration
    pub box_component: BoxComponent,
}

impl Default for Card {
    fn default() -> Self {
        Self {
            name: "Default Card".into(),
            box_component: BoxComponent {
                variant: BoxVariant::Surface,
                ..Default::default()
            },
        }
    }
}

/// Card size options following Radix UI specifications
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CardSize {
    /// Size 1 - Compact card with minimal padding
    Size1,
    /// Size 2 - Default card size with comfortable padding
    Size2,
    /// Size 3 - Large card with generous padding
    Size3,
}

impl Default for CardSize {
    fn default() -> Self {
        Self::Size2
    }
}

/// Card visual variants following Radix UI design
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CardVariant {
    /// Subtle background presentation (default)
    Surface,
    /// Enhanced border and background for prominence
    Classic,
    /// Transparent with hover background for minimal design
    Ghost,
}

impl Default for CardVariant {
    fn default() -> Self {
        Self::Surface
    }
}

impl From<CardVariant> for BoxVariant {
    fn from(variant: CardVariant) -> Self {
        match variant {
            CardVariant::Surface => BoxVariant::Surface,
            CardVariant::Classic => BoxVariant::Classic,
            CardVariant::Ghost => BoxVariant::Ghost,
        }
    }
}

/// Builder for creating Card components with fluent API
pub struct CardBuilder {
    box_builder: BoxBuilder,
    card_size: CardSize,
}

impl CardBuilder {
    /// Create a new card builder
    pub fn new(name: impl Into<String>) -> Self {
        let name = name.into();
        let mut box_builder = BoxBuilder::new(name);
        // Set card-specific defaults
        box_builder = box_builder.surface(); // Default to surface variant

        Self {
            box_builder,
            card_size: CardSize::default(),
        }
    }

    /// Set the card variant
    pub fn variant(mut self, variant: CardVariant) -> Self {
        self.box_builder = match variant {
            CardVariant::Surface => self.box_builder.surface(),
            CardVariant::Classic => self.box_builder.classic(),
            CardVariant::Ghost => self.box_builder.ghost(),
        };
        self
    }

    /// Use surface variant (subtle background) - default
    pub fn surface(self) -> Self {
        self.variant(CardVariant::Surface)
    }

    /// Use classic variant (enhanced border and background)
    pub fn classic(self) -> Self {
        self.variant(CardVariant::Classic)
    }

    /// Use ghost variant (transparent with hover)
    pub fn ghost(self) -> Self {
        self.variant(CardVariant::Ghost)
    }

    /// Set the card size
    pub fn size(mut self, size: CardSize) -> Self {
        self.card_size = size;

        // Apply size-specific styling
        self.box_builder = match size {
            CardSize::Size1 => self.box_builder.padding(Val::Px(8.0)),
            CardSize::Size2 => self.box_builder.padding(Val::Px(16.0)),
            CardSize::Size3 => self.box_builder.padding(Val::Px(24.0)),
        };

        self
    }

    /// Use size 1 (compact)
    pub fn size_1(self) -> Self {
        self.size(CardSize::Size1)
    }

    /// Use size 2 (default)
    pub fn size_2(self) -> Self {
        self.size(CardSize::Size2)
    }

    /// Use size 3 (large)
    pub fn size_3(self) -> Self {
        self.size(CardSize::Size3)
    }

    /// Set the color palette for the card
    pub fn color_palette(mut self, palette: UiColorPalette) -> Self {
        self.box_builder = self.box_builder.color(palette);
        self
    }

    /// Make the card interactive (enables picking and hover effects)
    pub fn interactive(mut self) -> Self {
        self.box_builder = self.box_builder.pickable();
        self
    }

    /// Set custom padding for all sides
    pub fn padding(mut self, padding: f32) -> Self {
        self.box_builder = self.box_builder.padding(Val::Px(padding));
        self
    }

    /// Set custom width
    pub fn width(mut self, width: Val) -> Self {
        self.box_builder = self.box_builder.width(width);
        self
    }

    /// Set custom height
    pub fn height(mut self, height: Val) -> Self {
        self.box_builder = self.box_builder.height(height);
        self
    }

    /// Set maximum width
    pub fn max_width(mut self, max_width: Val) -> Self {
        self.box_builder = self.box_builder.max_width(max_width);
        self
    }

    /// Set explicit background color (overrides theme)
    pub fn background_color(mut self, color: Color) -> Self {
        self.box_builder = self.box_builder.background_color(color);
        self
    }

    /// Set explicit border color (overrides theme)
    pub fn border_color(mut self, color: Color) -> Self {
        self.box_builder = self.box_builder.border_color(color);
        self
    }
}

impl Default for CardBuilder {
    fn default() -> Self {
        Self::new("Default Card")
    }
}

impl CardBuilder {
    pub fn build(self) -> impl Bundle {
        // Since BoxBuilder now returns impl Bundle, we need to build it directly
        // and add our Card component alongside it
        (
            Card {
                name: "Card".to_string(),
                box_component: Default::default(), // We'll use a placeholder since we can't access the components directly
            },
            self.box_builder.build(),
        )
    }
}
