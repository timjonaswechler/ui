use crate::theme::*;
use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct Button {
    pub variant: ButtonVariant,
    pub size: ButtonSize,
    pub color: Option<AccentColor>,
    pub high_contrast: bool,
    pub radius: Option<ButtonRadius>,
    pub loading: bool,
    pub disabled: bool,
}

impl Default for Button {
    fn default() -> Self {
        Self {
            variant: ButtonVariant::Solid,
            size: ButtonSize::Size2,
            color: None,
            high_contrast: false,
            radius: None,
            loading: false,
            disabled: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    Classic,
    Solid,
    Soft,
    Surface,
    Outline,
    Ghost,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonSize {
    Size1,
    Size2,
    Size3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonRadius {
    None,
    Small,
    Medium,
    Large,
    Full,
}

pub struct ButtonBuilder {
    name: String,
    button: Button,
    text: Option<String>,
    children: Vec<Entity>,
}

impl ButtonBuilder {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            name: format!("{}_Button", text.into()),
            button: Button::default(),
            text: None,
            children: Vec::new(),
        }
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.button.variant = variant;
        self
    }

    pub fn size(mut self, size: ButtonSize) -> Self {
        self.button.size = size;
        self
    }

    pub fn color(mut self, color: AccentColor) -> Self {
        self.button.color = Some(color);
        self
    }

    pub fn high_contrast(mut self) -> Self {
        self.button.high_contrast = true;
        self
    }

    pub fn radius(mut self, radius: ButtonRadius) -> Self {
        self.button.radius = Some(radius);
        self
    }

    pub fn loading(mut self) -> Self {
        self.button.loading = true;
        self
    }

    pub fn disabled(mut self) -> Self {
        self.button.disabled = true;
        self
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn child(mut self, entity: Entity) -> Self {
        self.children.push(entity);
        self
    }
}

impl ButtonBuilder {
    pub fn build(self) -> impl Bundle {
        let style = self.calculate_style();
        let background_color = self.calculate_background_color();
        let border_color = self.calculate_border_color();
        let border_radius = self.calculate_border_radius();

        (
            Name::new(self.name),
            style,
            background_color,
            border_color,
            border_radius,
            self.button,
        )
    }
}

impl ButtonBuilder {
    fn calculate_style(&self) -> Node {
        let (width, height, padding) = match self.button.size {
            ButtonSize::Size1 => (Val::Auto, Val::Px(24.0), UiRect::all(Val::Px(8.0))),
            ButtonSize::Size2 => (Val::Auto, Val::Px(32.0), UiRect::all(Val::Px(12.0))),
            ButtonSize::Size3 => (Val::Auto, Val::Px(40.0), UiRect::all(Val::Px(16.0))),
        };

        Node {
            width,
            height,
            padding,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(1.0)),
            ..default()
        }
    }

    fn calculate_background_color(&self) -> BackgroundColor {
        let theme_colors = ThemeColors {
            accent: indigo_scale(),
            gray: gray_scale(),
            background: Color::WHITE,
            panel_solid: Color::WHITE,
            panel_translucent: Color::srgba(1.0, 1.0, 1.0, 0.9),
            surface: Color::srgb(0.98, 0.98, 0.98),
            overlay: Color::srgba(0.0, 0.0, 0.0, 0.5),
        };

        let color_scale = match self.button.color {
            Some(_) => &theme_colors.accent,
            None => &theme_colors.accent,
        };

        let color = match self.button.variant {
            ButtonVariant::Classic => color_scale.step_9,
            ButtonVariant::Solid => color_scale.step_9,
            ButtonVariant::Soft => color_scale.step_3,
            ButtonVariant::Surface => color_scale.step_2,
            ButtonVariant::Outline => Color::NONE,
            ButtonVariant::Ghost => Color::NONE,
        };

        BackgroundColor(color)
    }

    fn calculate_border_color(&self) -> BorderColor {
        let theme_colors = ThemeColors {
            accent: indigo_scale(),
            gray: gray_scale(),
            background: Color::WHITE,
            panel_solid: Color::WHITE,
            panel_translucent: Color::srgba(1.0, 1.0, 1.0, 0.9),
            surface: Color::srgb(0.98, 0.98, 0.98),
            overlay: Color::srgba(0.0, 0.0, 0.0, 0.5),
        };

        let color_scale = match self.button.color {
            Some(_) => &theme_colors.accent,
            None => &theme_colors.accent,
        };

        let color = match self.button.variant {
            ButtonVariant::Outline => color_scale.step_7,
            _ => Color::NONE,
        };

        BorderColor(color)
    }
    fn calculate_border_radius(&self) -> BorderRadius {
        match self.button.radius {
            Some(ButtonRadius::None) => BorderRadius::all(Val::Px(0.0)),
            Some(ButtonRadius::Small) => BorderRadius::all(Val::Px(2.0)),
            Some(ButtonRadius::Medium) => BorderRadius::all(Val::Px(4.0)),
            Some(ButtonRadius::Large) => BorderRadius::all(Val::Px(8.0)),
            Some(ButtonRadius::Full) => BorderRadius::all(Val::Px(9999.0)),
            None => BorderRadius::all(Val::Px(4.0)),
        }
    }
}
