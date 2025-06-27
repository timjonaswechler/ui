use crate::theme::{
    color::{TextContrastLevel, UiColorPalette},
    layout::UiLayout,
};
use bevy::prelude::*;

use super::core::{Button, ButtonRadius, ButtonSize, ButtonState, ButtonVariant};

#[derive(Debug, Clone)]
pub struct ButtonStyling {
    pub background_color: BackgroundColor,
    pub border_color: BorderColor,
    pub text_color: TextColor,
}

impl Button {
    pub fn get_styling(&self, state: ButtonState) -> ButtonStyling {
        let current_state = if self.disabled {
            ButtonState::Disabled
        } else if self.loading {
            ButtonState::Loading
        } else {
            state
        };

        ButtonStyling {
            background_color: self.calculate_background_color(current_state),
            border_color: self.calculate_border_color(current_state),
            text_color: self.calculate_text_color(current_state),
        }
    }

    fn calculate_background_color(&self, state: ButtonState) -> BackgroundColor {
        let base_color = match (self.variant, state) {
            (ButtonVariant::Solid, ButtonState::Normal) => self.color.solid,
            (ButtonVariant::Solid, ButtonState::Hover) => self.color.solid_hover,
            (ButtonVariant::Solid, ButtonState::Active) => self.color.bg_active,
            (ButtonVariant::Ghost, ButtonState::Normal) => self.color.bg_subtle,
            (ButtonVariant::Ghost, ButtonState::Hover) => self.color.bg_hover,
            (ButtonVariant::Ghost, ButtonState::Active) => self.color.bg_active,
            (ButtonVariant::Soft, ButtonState::Normal)
            | (ButtonVariant::Outline, ButtonState::Normal) => self.color.bg,
            (ButtonVariant::Soft, ButtonState::Hover)
            | (ButtonVariant::Outline, ButtonState::Hover) => self.color.bg_hover,
            (ButtonVariant::Soft, ButtonState::Active)
            | (ButtonVariant::Outline, ButtonState::Active) => self.color.bg_active,
            (_, ButtonState::Disabled) => match self.variant {
                ButtonVariant::Solid => self.color.solid,
                ButtonVariant::Ghost => self.color.base,
                ButtonVariant::Soft | ButtonVariant::Outline => self.color.bg_hover,
            },
            (_, ButtonState::Loading) => match self.variant {
                ButtonVariant::Solid => self.color.solid,
                ButtonVariant::Ghost => self.color.base,
                ButtonVariant::Soft | ButtonVariant::Outline => self.color.bg_hover,
            },
        };

        let mut bg_color = BackgroundColor(base_color);

        if state == ButtonState::Disabled {
            let srgba = bg_color.0.to_srgba();
            bg_color.0 = Color::srgba(srgba.red, srgba.green, srgba.blue, 0.6);
        }

        bg_color
    }

    fn calculate_border_color(&self, state: ButtonState) -> BorderColor {
        match self.variant {
            ButtonVariant::Solid | ButtonVariant::Soft | ButtonVariant::Ghost => {
                BorderColor(Color::NONE)
            }
            ButtonVariant::Outline => match state {
                ButtonState::Normal | ButtonState::Active | ButtonState::Loading => {
                    BorderColor(self.color.border)
                }
                ButtonState::Hover => BorderColor(self.color.border_hover),
                ButtonState::Disabled => BorderColor(self.color.border),
            },
        }
    }

    fn calculate_text_color(&self, state: ButtonState) -> TextColor {
        // Get the actual background color for this state
        let _background_color = match (self.variant, state) {
            (ButtonVariant::Solid, ButtonState::Normal) => self.color.solid,
            (ButtonVariant::Solid, ButtonState::Hover) => self.color.solid_hover,
            (ButtonVariant::Solid, ButtonState::Active) => self.color.bg_active,
            (ButtonVariant::Ghost, ButtonState::Normal) => self.color.bg_subtle,
            (ButtonVariant::Ghost, ButtonState::Hover) => self.color.bg_hover,
            (ButtonVariant::Ghost, ButtonState::Active) => self.color.bg_active,
            (ButtonVariant::Soft, ButtonState::Normal)
            | (ButtonVariant::Outline, ButtonState::Normal) => self.color.bg,
            (ButtonVariant::Soft, ButtonState::Hover)
            | (ButtonVariant::Outline, ButtonState::Hover) => self.color.bg_hover,
            (ButtonVariant::Soft, ButtonState::Active)
            | (ButtonVariant::Outline, ButtonState::Active) => self.color.bg_active,
            (_, ButtonState::Disabled) => match self.variant {
                ButtonVariant::Solid => self.color.solid,
                ButtonVariant::Ghost => self.color.base,
                ButtonVariant::Soft | ButtonVariant::Outline => self.color.bg_hover,
            },
            (_, ButtonState::Loading) => match self.variant {
                ButtonVariant::Solid => self.color.solid,
                ButtonVariant::Ghost => self.color.base,
                ButtonVariant::Soft | ButtonVariant::Outline => self.color.bg_hover,
            },
        };

        let _contrast_level = if self.high_contrast {
            TextContrastLevel::Accessible
        } else {
            TextContrastLevel::High
        };

        let mut text_color = match self.variant {
            ButtonVariant::Solid => {
                // Use text_contrast for solid buttons - this should be consistent
                self.color.text_contrast
            }
            ButtonVariant::Soft => {
                // Use normal text color for soft buttons
                self.color.text
            }
            ButtonVariant::Outline => {
                // Use normal text color for outline buttons
                self.color.text
            }
            ButtonVariant::Ghost => {
                // Use normal text color for ghost buttons
                self.color.text
            }
        };

        if state == ButtonState::Disabled {
            // Keep the same text color but reduce opacity for disabled state
            let srgba = text_color.to_srgba();
            text_color = Color::srgba(srgba.red, srgba.green, srgba.blue, 0.8);
        }

        TextColor(text_color)
    }
}

pub fn calculate_button_style(button: &Button) -> Node {
    let padding = UiRect::axes(
        Val::Px(match button.size {
            ButtonSize::Default => UiLayout::default().padding.base + 2.0,
            ButtonSize::Small => UiLayout::default().padding.sm + 2.0,
            ButtonSize::Large => UiLayout::default().padding.lg + 8.0,
        }),
        Val::Px(match button.size {
            ButtonSize::Default => UiLayout::default().padding.base,
            ButtonSize::Small => UiLayout::default().padding.sm,
            ButtonSize::Large => UiLayout::default().padding.lg,
        }),
    );

    Node {
        padding,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        border: UiRect::all(Val::Px(1.0)),
        ..default()
    }
}

pub fn calculate_border_radius(radius: ButtonRadius) -> BorderRadius {
    let border_radius = match radius {
        ButtonRadius::None => Val::Px(0.0),
        ButtonRadius::ExtraSmall => Val::Px(UiLayout::default().radius.xs),
        ButtonRadius::Small => Val::Px(UiLayout::default().radius.sm),
        ButtonRadius::Base => Val::Px(UiLayout::default().radius.base),
        ButtonRadius::Large => Val::Px(UiLayout::default().radius.lg),
        ButtonRadius::ExtraLarge => Val::Px(UiLayout::default().radius.xl),
        ButtonRadius::Extra2Large => Val::Px(UiLayout::default().radius.x2l),
        ButtonRadius::Extra3Large => Val::Px(UiLayout::default().radius.x3l),
        ButtonRadius::Extra4Large => Val::Px(UiLayout::default().radius.x4l),
        ButtonRadius::Full => Val::Px(UiLayout::default().radius.full),
    };
    
    BorderRadius {
        top_left: border_radius,
        top_right: border_radius,
        bottom_left: border_radius,
        bottom_right: border_radius,
    }
}