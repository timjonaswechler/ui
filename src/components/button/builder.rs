use crate::{
    components::text::{Text, TextBuilder},
    theme::{
        color::{TextColor as TextColorEnum, TextContrastLevel, UiColorPalette},
        typography::{FontFamily, TextSize, TextWeight},
    },
    utilities::ComponentBuilder,
};
use bevy::prelude::*;

use super::core::{Button, ButtonRadius, ButtonSize, ButtonVariant};

pub struct ButtonBuilder {
    name: String,
    button: Button,
    text: Option<String>,
    text_builder: Option<TextBuilder>,
    children: Vec<Entity>,
}

impl ButtonBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: format!("{}_Button", name.into()),
            button: Button::default(),
            text: None,
            text_builder: None,
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

    pub fn color(mut self, color: UiColorPalette) -> Self {
        self.button.color = color;
        self
    }

    pub fn high_contrast(mut self) -> Self {
        self.button.high_contrast = true;
        self
    }

    pub fn auto_contrast(self) -> Self {
        // Aktiviert automatische Kontrastberechnung (ist standardmäßig aktiviert)
        // Diese Methode dient der Klarstellung und kann in Zukunft erweitert werden
        self
    }

    pub fn radius(mut self, radius: ButtonRadius) -> Self {
        self.button.radius = radius;
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

    pub fn text_size(mut self, size: TextSize) -> Self {
        let text_content = self.text.clone().unwrap_or_default();
        let builder = self
            .text_builder
            .take()
            .unwrap_or_else(|| Text::new(text_content));
        self.text_builder = Some(builder.size(size));
        self
    }

    pub fn text_weight(mut self, weight: TextWeight) -> Self {
        let text_content = self.text.clone().unwrap_or_default();
        let builder = self
            .text_builder
            .take()
            .unwrap_or_else(|| Text::new(text_content));
        self.text_builder = Some(builder.weight(weight));
        self
    }

    pub fn text_family(mut self, family: FontFamily) -> Self {
        let text_content = self.text.clone().unwrap_or_default();
        let builder = self
            .text_builder
            .take()
            .unwrap_or_else(|| Text::new(text_content));
        self.text_builder = Some(builder.family(family));
        self
    }

    pub fn text_italic(mut self) -> Self {
        let text_content = self.text.clone().unwrap_or_default();
        let builder = self
            .text_builder
            .take()
            .unwrap_or_else(|| Text::new(text_content));
        self.text_builder = Some(builder.italic());
        self
    }

    pub fn text_align(mut self, align: JustifyText) -> Self {
        let text_content = self.text.clone().unwrap_or_default();
        let builder = self
            .text_builder
            .take()
            .unwrap_or_else(|| Text::new(text_content));
        self.text_builder = Some(builder.align(align));
        self
    }

    pub fn text_center(self) -> Self {
        self.text_align(JustifyText::Center)
    }

    pub fn text_right(self) -> Self {
        self.text_align(JustifyText::Right)
    }

    pub fn text_on_background(mut self, background_color: Color) -> Self {
        let text_content = self.text.clone().unwrap_or_default();
        let builder = self
            .text_builder
            .take()
            .unwrap_or_else(|| Text::new(text_content));
        self.text_builder = Some(builder.on_background(background_color));
        self
    }

    pub fn text_contrast_level(mut self, level: TextContrastLevel) -> Self {
        let text_content = self.text.clone().unwrap_or_default();
        let builder = self
            .text_builder
            .take()
            .unwrap_or_else(|| Text::new(text_content));
        self.text_builder = Some(builder.contrast_level(level));
        self
    }

    pub fn text_high_contrast(mut self) -> Self {
        let text_content = self.text.clone().unwrap_or_default();
        let builder = self
            .text_builder
            .take()
            .unwrap_or_else(|| Text::new(text_content));
        self.text_builder = Some(builder.high_contrast());
        self
    }

    pub fn text_accessible(mut self) -> Self {
        let text_content = self.text.clone().unwrap_or_default();
        let builder = self
            .text_builder
            .take()
            .unwrap_or_else(|| Text::new(text_content));
        self.text_builder = Some(builder.accessible());
        self
    }

    pub fn text_auto_contrast(mut self) -> Self {
        let text_content = self.text.clone().unwrap_or_default();
        let builder = self
            .text_builder
            .take()
            .unwrap_or_else(|| Text::new(text_content));
        self.text_builder = Some(builder.auto_contrast());
        self
    }

    pub fn text_manual_color(mut self) -> Self {
        let text_content = self.text.clone().unwrap_or_default();
        let builder = self
            .text_builder
            .take()
            .unwrap_or_else(|| Text::new(text_content));
        self.text_builder = Some(builder.manual_color());
        self
    }

    pub fn text_color(mut self, color: TextColorEnum) -> Self {
        let text_content = self.text.clone().unwrap_or_default();
        let builder = self
            .text_builder
            .take()
            .unwrap_or_else(|| Text::new(text_content));
        self.text_builder = Some(builder.color(color));
        self
    }

    pub fn child(mut self, entity: Entity) -> Self {
        self.children.push(entity);
        self
    }

    /// Check if explicit text color was set (this affects whether button should manage text color)
    fn has_explicit_text_color(&self) -> bool {
        // If text_builder exists and has explicit color methods called, it should not be managed
        // For now, we'll use a simple heuristic: if any text_color method was called, it's explicit
        // This is a simplified check since we can't access TextBuilder internals
        false // For now, always allow button to manage text color
    }

    /// Get appropriate text size for button size (only if no explicit size set)
    fn get_button_text_size(&self) -> TextSize {
        // If text_builder exists and has explicit size, don't override it
        if self.text_builder.is_some() {
            // Return a default - the actual size will be preserved from TextBuilder
            TextSize::Base
        } else {
            match self.button.size {
                ButtonSize::Small => TextSize::Xs,
                ButtonSize::Default => TextSize::Base,
                ButtonSize::Large => TextSize::X2l,
            }
        }
    }

    /// Get appropriate text weight for button variant
    fn get_button_text_weight(&self) -> TextWeight {
        match self.button.variant {
            ButtonVariant::Solid => TextWeight::Medium,
            ButtonVariant::Soft => TextWeight::Regular,
            ButtonVariant::Outline => TextWeight::Regular,
            ButtonVariant::Ghost => TextWeight::Regular,
        }
    }

    /// Convert button text color to Text component color enum
    fn get_text_color_enum(&self) -> TextColorEnum {
        let calculated_color = self.calculate_text_color();
        TextColorEnum::Custom(calculated_color.0)
    }

    fn calculate_style(&self) -> Node {
        super::styling::calculate_button_style(&self.button)
    }

    fn calculate_background_color(&self) -> BackgroundColor {
        self.button
            .get_styling(super::core::ButtonState::Normal)
            .background_color
    }

    fn calculate_border_color(&self) -> BorderColor {
        self.button.get_styling(super::core::ButtonState::Normal).border_color
    }

    fn calculate_text_color(&self) -> TextColor {
        self.button.get_styling(super::core::ButtonState::Normal).text_color
    }
}

impl ComponentBuilder for ButtonBuilder {
    type Output = (
        Name,
        Button,
        Node,
        BorderColor,
        BorderRadius,
        BackgroundColor,
        bevy_picking::prelude::Pickable,
        impl Bundle,
    );

    fn build(self) -> Self::Output {
        use bevy::{ecs::spawn::SpawnWith, prelude::*};
        use crate::components::text::Text;
        use super::{animations::SpinnerAnimation, interactions::ButtonManagedText, styling::calculate_border_radius};

        let node = self.calculate_style();
        let background_color = self.calculate_background_color();
        let border_color = self.calculate_border_color();
        let border_radius = calculate_border_radius(self.button.radius);
        let display_text = self.text.clone().unwrap_or_default();
        let is_loading = self.button.loading;
        let text_size = self.get_button_text_size();
        let text_weight = self.get_button_text_weight();
        let text_color_enum = self.get_text_color_enum();

        // Prepare TextBuilder with automatic contrast optimization if text_builder is used
        let text_builder = if let Some(builder) = self.text_builder.clone() {
            // Apply button background context for intelligent contrast
            let button_bg = self.calculate_background_color().0;
            let contrast_level = if self.button.high_contrast {
                TextContrastLevel::Accessible
            } else {
                TextContrastLevel::High
            };

            Some(
                builder
                    .on_background(button_bg)
                    .contrast_level(contrast_level),
            )
        } else {
            None
        };

        (
            Name::new(format!("{}_Button", self.name)),
            self.button,
            node,
            border_color,
            border_radius,
            background_color,
            bevy_picking::prelude::Pickable::default(),
            Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
                if is_loading {
                    // Spawn rotating spinner image
                    parent.spawn((
                        Name::new("Button Spinner"),
                        Node {
                            width: Val::Px(16.0),
                            height: Val::Px(16.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        SpinnerAnimation::default(),
                    ));
                } else {
                    // Use advanced TextBuilder if available, otherwise fallback to simple text
                    if let Some(builder) = text_builder {
                        parent.spawn((
                            builder.center().build(),
                            ButtonManagedText, // Always add marker for now - will be refined later
                        ));
                    } else {
                        // Fallback text is always managed by button
                        parent.spawn((
                            Text::label(display_text.clone())
                                .color(text_color_enum)
                                .size(text_size)
                                .weight(text_weight)
                                .center()
                                .build(),
                            ButtonManagedText,
                        ));
                    }
                }
            })),
        )
    }
}