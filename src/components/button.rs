use crate::theme::{
    color::{UiColorPalette, UiColorPalettes, ACCENT_COLOR_PALETTE},
    layout::UiLayout,
};
use bevy::{ecs::spawn::SpawnWith, prelude::*};
use bevy_picking::prelude::{Click, Out, Over, Pickable, Pointer};

#[derive(Event, Debug, Clone)]
pub struct ButtonClickEvent {
    pub button_entity: Entity,
    pub button_variant: ButtonVariant,
}

#[derive(Component, Debug, Clone)]
pub struct Button {
    pub variant: ButtonVariant,
    pub size: ButtonSize,
    pub color: UiColorPalette,
    pub high_contrast: bool,
    pub radius: Option<Val>,
    pub loading: bool,
    pub disabled: bool,
}

impl Default for Button {
    fn default() -> Self {
        Self {
            variant: ButtonVariant::Solid,
            size: ButtonSize::Default,
            color: ACCENT_COLOR_PALETTE.clone(),
            high_contrast: false,
            radius: None,
            loading: false,
            disabled: false,
        }
    }
}

/// Defines the visual style variant of a button
///
/// Buttons can have different visual styles which affect their background,
/// border, and text colors. Each variant provides a different level of
/// visual prominence and purpose.
///
/// # Variants
/// * `Solid` - Full background color with high contrast text (default)
/// * `Soft` - Light background color with darker text
/// * `Outline` - Transparent background with border and dark text
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Component)]
pub enum ButtonVariant {
    /// Full background color with high contrast text (default)
    #[default]
    Solid,
    /// Light background color with darker text
    Soft,
    /// Transparent background with border and dark text
    Outline,
    Ghost,
}

/// Defines the size variant of a button
///
/// Buttons can have different size presets that affect their padding,
/// text size, and overall dimensions.
///
/// # Variants
/// * `Default` - Standard size for most scenarios (default)
/// * `Small` - Compact size for space-constrained areas
/// * `Large` - Expanded size for emphasis or improved touch targets
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Component)]
pub enum ButtonSize {
    /// Standard size for most scenarios (default)
    #[default]
    Default,
    /// Compact size for space-constrained areas
    Small,
    /// Expanded size for emphasis or improved touch targets
    Large,
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
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: format!("{}_Button", name.into()),
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

    pub fn color(mut self, color: UiColorPalette) -> Self {
        self.button.color = color;
        self
    }

    pub fn high_contrast(mut self) -> Self {
        self.button.high_contrast = true;
        self
    }

    pub fn radius(mut self, radius: Val) -> Self {
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
        let node = self.calculate_style();
        let background_color = self.calculate_background_color();
        let border_color = self.calculate_border_color();
        let border_radius = BorderRadius::all(self.button.radius.unwrap_or(Val::Px(10.0)));
        let text_color = self.calculate_text_color();
        let display_text = self.text.unwrap_or_default();
        let is_loading = self.button.loading;

        (
            Name::new(format!("{}_Button", self.name)),
            self.button,
            node,
            border_color,
            border_radius,
            background_color,
            Pickable::default(),
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
                    parent.spawn((
                        Name::new("Button Inner"),
                        Text::new(display_text),
                        text_color,
                    ));
                }
            })),
        )
    }
}

impl ButtonBuilder {
    fn calculate_style(&self) -> Node {
        let padding = UiRect::all(Val::Px(match self.button.size {
            ButtonSize::Default => UiLayout::default().padding.base,
            ButtonSize::Small => UiLayout::default().padding.sm,
            ButtonSize::Large => UiLayout::default().padding.lg,
        }));

        Node {
            padding,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(1.0)),
            ..default()
        }
    }

    fn calculate_background_color(&self) -> BackgroundColor {
        let mut bg_color = match self.button.variant {
            (ButtonVariant::Solid) => BackgroundColor(self.button.color.step09),
            (ButtonVariant::Ghost) => BackgroundColor(self.button.color.step01),
            (ButtonVariant::Soft) | (ButtonVariant::Outline) => {
                BackgroundColor(self.button.color.step03)
            }
        };

        // Apply disabled state
        if self.button.disabled {
            let srgba = bg_color.0.to_srgba();
            bg_color.0 = Color::srgba(srgba.red, srgba.green, srgba.blue, 0.6);
        }

        bg_color
    }

    fn calculate_border_color(&self) -> BorderColor {
        match (self.button.variant) {
            (ButtonVariant::Solid) | (ButtonVariant::Soft) | (ButtonVariant::Ghost) => {
                BorderColor(Color::NONE)
            }
            (ButtonVariant::Outline) => BorderColor(self.button.color.step11),
        }
    }

    fn calculate_text_color(&self) -> TextColor {
        let mut text_color = self.button.color.step01;

        if self.button.disabled {
            text_color = text_color.mix(&UiColorPalettes::default().black.step08, 0.5);
        }

        TextColor(text_color)
    }
}

#[derive(Component)]
pub struct SpinnerAnimation {
    pub rotation_speed: f32,
}

impl Default for SpinnerAnimation {
    fn default() -> Self {
        Self {
            rotation_speed: 360.0, // Grad pro Sekunde
        }
    }
}

// System für Button-Interaktionen
pub fn setup_button_interactions(mut commands: Commands, buttons: Query<Entity, Added<Button>>) {
    for entity in &buttons {
        commands
            .entity(entity)
            .observe(on_button_click)
            .observe(on_button_hover)
            .observe(on_button_hover_out);
    }
}

// System für Loading-Animation (rotiert PNG-Spinner)
pub fn animate_loading_spinners(
    time: Res<Time>,
    mut spinners: Query<(&mut Transform, &SpinnerAnimation)>,
) {
    for (mut transform, spinner) in spinners.iter_mut() {
        let rotation_delta = spinner.rotation_speed * time.delta_secs();
        transform.rotation *= Quat::from_rotation_z(rotation_delta.to_radians());
    }
}

// System um Spinner-Texture zu laden
pub fn setup_spinner_textures(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    spinners: Query<Entity, (With<SpinnerAnimation>, Without<ImageNode>)>,
) {
    let spinner_texture: Handle<Image> = asset_server.load("texture/spinner_loading_icon.png");

    for entity in spinners.iter() {
        commands.entity(entity).insert((
            ImageNode::new(spinner_texture.clone()),
            Node {
                width: Val::Px(16.0),
                height: Val::Px(16.0),
                ..default()
            },
        ));
    }
}

fn on_button_click(
    trigger: Trigger<Pointer<Click>>,
    buttons: Query<&Button>,
    mut events: EventWriter<ButtonClickEvent>,
) {
    let entity = trigger.target();
    if let Ok(button) = buttons.get(entity) {
        // Don't trigger if disabled or loading
        if button.disabled || button.loading {
            return;
        }

        info!("Button clicked! Variant: {:?}", button.variant);

        // Send custom event
        events.write(ButtonClickEvent {
            button_entity: entity,
            button_variant: button.variant,
        });
    }
}

fn on_button_hover(
    trigger: Trigger<Pointer<Over>>,
    buttons: Query<&Button>,
    mut bg_colors: Query<&mut BackgroundColor>,
) {
    let entity = trigger.target();
    if let Ok(button) = buttons.get(entity) {
        if let Ok(mut bg_color) = bg_colors.get_mut(entity) {
            // Don't apply hover effect if disabled or loading
            if button.disabled || button.loading {
                return;
            }

            // Apply hover effect based on button variant
            *bg_color = match button.variant {
                ButtonVariant::Solid => BackgroundColor(button.color.step08),
                ButtonVariant::Ghost => BackgroundColor(button.color.step02),
                ButtonVariant::Soft | ButtonVariant::Outline => {
                    BackgroundColor(button.color.step04)
                }
            };
        }
    }
}

fn on_button_hover_out(
    trigger: Trigger<Pointer<Out>>,
    buttons: Query<&Button>,
    mut bg_colors: Query<&mut BackgroundColor>,
) {
    let entity = trigger.target();
    if let Ok(button) = buttons.get(entity) {
        if let Ok(mut bg_color) = bg_colors.get_mut(entity) {
            let mut bg = match button.variant {
                (ButtonVariant::Solid) => BackgroundColor(button.color.step09),
                (ButtonVariant::Ghost) => BackgroundColor(button.color.step01),
                (ButtonVariant::Soft) | (ButtonVariant::Outline) => {
                    BackgroundColor(button.color.step03)
                }
            };

            // Apply disabled state
            if button.disabled {
                let srgba = bg.0.to_srgba();
                bg.0 = Color::srgba(srgba.red, srgba.green, srgba.blue, 0.6);
            }

            *bg_color = bg;
        }
    }
}
