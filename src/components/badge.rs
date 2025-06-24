use crate::{
    components::text::{Text, TextBuilder},
    theme::{
        color::{accent_palette, theme, TextColor, UiColorPalette},
        typography::{TextSize, TextWeight},
    },
    utilities::ComponentBuilder,
};
use bevy::{ecs::spawn::SpawnWith, prelude::*};

#[derive(Component, Debug, Clone)]
pub struct Badge {
    pub variant: BadgeVariant,
    pub size: BadgeSize,
    pub color: UiColorPalette,
    pub high_contrast: bool,
    pub radius: BadgeRadius,
}

impl Default for Badge {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum BadgeVariant {
    Solid,
    Soft,
    #[default]
    Surface,
    Outline,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum BadgeSize {
    Size1,
    #[default]
    Size2,
    Size3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum BadgeRadius {
    None,
    Small,
    Medium,
    Large,
    #[default]
    Full,
}

pub struct BadgeBuilder {
    name: String,
    badge: Badge,
    text: Option<String>,
    text_builder: Option<TextBuilder>,
    children: Vec<Entity>,
}

impl BadgeBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            badge: Badge::default(),
            text: None,
            text_builder: None,
            children: Vec::new(),
        }
    }

    pub fn variant(mut self, variant: BadgeVariant) -> Self {
        self.badge.variant = variant;
        self
    }

    pub fn size(mut self, size: BadgeSize) -> Self {
        self.badge.size = size;
        self
    }

    pub fn color(mut self, color: UiColorPalette) -> Self {
        self.badge.color = color;
        self
    }

    pub fn high_contrast(mut self, high_contrast: bool) -> Self {
        self.badge.high_contrast = high_contrast;
        self
    }

    pub fn radius(mut self, radius: BadgeRadius) -> Self {
        self.badge.radius = radius;
        self
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn text_builder(mut self, text_builder: TextBuilder) -> Self {
        self.text_builder = Some(text_builder);
        self
    }

    pub fn child(mut self, entity: Entity) -> Self {
        self.children.push(entity);
        self
    }

    pub fn children(mut self, entities: Vec<Entity>) -> Self {
        self.children.extend(entities);
        self
    }

    // Convenience methods for variants
    pub fn solid(self) -> Self {
        self.variant(BadgeVariant::Solid)
    }

    pub fn soft(self) -> Self {
        self.variant(BadgeVariant::Soft)
    }

    pub fn surface(self) -> Self {
        self.variant(BadgeVariant::Surface)
    }

    pub fn outline(self) -> Self {
        self.variant(BadgeVariant::Outline)
    }

    // Convenience methods for sizes
    pub fn size_1(self) -> Self {
        self.size(BadgeSize::Size1)
    }

    pub fn size_2(self) -> Self {
        self.size(BadgeSize::Size2)
    }

    pub fn size_3(self) -> Self {
        self.size(BadgeSize::Size3)
    }

    // Convenience methods for radius
    pub fn rounded_none(self) -> Self {
        self.radius(BadgeRadius::None)
    }

    pub fn rounded_small(self) -> Self {
        self.radius(BadgeRadius::Small)
    }

    pub fn rounded_medium(self) -> Self {
        self.radius(BadgeRadius::Medium)
    }

    pub fn rounded_large(self) -> Self {
        self.radius(BadgeRadius::Large)
    }

    pub fn pill(self) -> Self {
        self.radius(BadgeRadius::Full)
    }

    // Convenience methods for colors
    pub fn accent(self) -> Self {
        self.color(accent_palette())
    }

    pub fn gray(self) -> Self {
        self.color(theme().gray)
    }
}

impl BadgeBuilder {
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
    pub fn builder(name: impl Into<String>) -> BadgeBuilder {
        BadgeBuilder::new(name)
    }
}
