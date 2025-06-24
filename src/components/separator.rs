use crate::{
    theme::{
        color::{theme_mode, ThemeMode, UiColorPalette, UiColorPalettes},
        layout::UiLayout,
    },
    utilities::ComponentBuilder,
};
use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct SeparatorComponent {
    pub orientation: SeparatorOrientation,
    pub color: UiColorPalette,
}

impl Default for SeparatorComponent {
    fn default() -> Self {
        Self {
            orientation: SeparatorOrientation::Horizontal,
            color: if theme_mode() == ThemeMode::Dark {
                UiColorPalettes::dark_mode().gray
            } else {
                UiColorPalettes::light_mode().gray
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SeparatorOrientation {
    #[default]
    Horizontal,
    Vertical,
}

pub struct SeparatorBuilder {
    name: String,
    separator: SeparatorComponent,
}

impl SeparatorBuilder {
    pub fn new() -> Self {
        Self {
            name: "Separator".to_string(),
            separator: SeparatorComponent::default(),
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    pub fn orientation(mut self, orientation: SeparatorOrientation) -> Self {
        self.separator.orientation = orientation;
        self
    }

    pub fn horizontal(mut self) -> Self {
        self.separator.orientation = SeparatorOrientation::Horizontal;
        self
    }

    pub fn vertical(mut self) -> Self {
        self.separator.orientation = SeparatorOrientation::Vertical;
        self
    }

    pub fn color(mut self, color: UiColorPalette) -> Self {
        self.separator.color = color;
        self
    }

    pub fn build(self) -> impl Bundle {
        let node = self.calculate_style();
        let background_color = self.calculate_background_color();

        (Name::new(self.name), self.separator, node, background_color)
    }
}

impl SeparatorBuilder {
    fn calculate_style(&self) -> Node {
        let layout = UiLayout::default();

        let (width, height) = match self.separator.orientation {
            SeparatorOrientation::Horizontal => {
                let width = Val::Percent(100.0);

                (width, Val::Px(1.0))
            }
            SeparatorOrientation::Vertical => {
                let height = Val::Percent(100.0);
                (Val::Px(1.0), height)
            }
        };

        Node {
            width,
            height,
            ..default()
        }
    }

    fn calculate_background_color(&self) -> BackgroundColor {
        BackgroundColor(self.separator.color.border)
    }
}

impl Default for SeparatorBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ComponentBuilder for SeparatorBuilder {
    type Output = (Name, SeparatorComponent, Node, BackgroundColor);

    fn build(self) -> Self::Output {
        let node = self.calculate_style();
        let background_color = self.calculate_background_color();

        (Name::new(self.name), self.separator, node, background_color)
    }
}
