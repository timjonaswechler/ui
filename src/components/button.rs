use crate::theme::*;
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
        let default_tokens = ThemeTokens::default();
        self.build_with_theme(&default_tokens)
    }
    
    pub fn build_with_theme(self, tokens: &ThemeTokens) -> impl Bundle {
        let style = self.calculate_style(tokens);
        let background_color = self.calculate_background_color(tokens);
        let border_color = self.calculate_border_color(tokens);
        let border_radius = self.calculate_border_radius(tokens);
        let text_color = self.calculate_text_color(tokens);
        let display_text = self.text.unwrap_or_default();
        let is_loading = self.button.loading;
        
        (
            Name::new(format!("{}_Button", self.name)),
            self.button,
            style,
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
    fn calculate_style(&self, tokens: &ThemeTokens) -> Node {
        let (width, height, padding_size) = match self.button.size {
            ButtonSize::Size1 => (Val::Auto, Val::Px(24.0), SpacingSize::Sm),
            ButtonSize::Size2 => (Val::Auto, Val::Px(32.0), SpacingSize::Md),
            ButtonSize::Size3 => (Val::Auto, Val::Px(40.0), SpacingSize::Lg),
        };

        let padding = self.themed_padding(&tokens.spacing, padding_size);

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

    fn calculate_background_color(&self, tokens: &ThemeTokens) -> BackgroundColor {
        let variant = match self.button.variant {
            ButtonVariant::Classic => BackgroundVariant::Primary,
            ButtonVariant::Solid => BackgroundVariant::Primary,
            ButtonVariant::Soft => BackgroundVariant::Secondary,
            ButtonVariant::Surface => BackgroundVariant::Muted,
            ButtonVariant::Outline => BackgroundVariant::Transparent,
            ButtonVariant::Ghost => BackgroundVariant::Transparent,
        };

        let mut bg_color = self.themed_background_color(&tokens.semantic, variant);

        // Apply disabled state
        if self.button.disabled {
            let srgba = bg_color.0.to_srgba();
            bg_color.0 = Color::srgba(srgba.red, srgba.green, srgba.blue, 0.5);
        }

        bg_color
    }

    fn calculate_border_color(&self, tokens: &ThemeTokens) -> BorderColor {
        let variant = match self.button.variant {
            ButtonVariant::Outline => BorderVariant::Default,
            _ => BorderVariant::Default, // Will be transparent since no border for other variants
        };

        let mut border_color = self.themed_border_color(&tokens.semantic, variant);

        // Make border transparent for non-outline variants
        if !matches!(self.button.variant, ButtonVariant::Outline) {
            border_color.0 = Color::NONE;
        }

        border_color
    }
    fn calculate_border_radius(&self, tokens: &ThemeTokens) -> BorderRadius {
        let radius_size = match self.button.radius {
            Some(ButtonRadius::None) => RadiusSize::None,
            Some(ButtonRadius::Small) => RadiusSize::Sm,
            Some(ButtonRadius::Medium) => RadiusSize::Md,
            Some(ButtonRadius::Large) => RadiusSize::Lg,
            Some(ButtonRadius::Full) => RadiusSize::Full,
            None => RadiusSize::Md,
        };

        self.themed_border_radius(&tokens.radius, radius_size)
    }

    fn calculate_text_color(&self, tokens: &ThemeTokens) -> TextColor {
        let variant = match self.button.variant {
            ButtonVariant::Classic | ButtonVariant::Solid => TextVariant::Primary,
            ButtonVariant::Soft | ButtonVariant::Surface => TextVariant::Default,
            ButtonVariant::Outline | ButtonVariant::Ghost => TextVariant::Default,
        };

        let mut text_color = self.themed_text_color(&tokens.semantic, variant);

        if self.button.disabled {
            let srgba = text_color.0.to_srgba();
            text_color.0 = Color::srgba(srgba.red, srgba.green, srgba.blue, 0.5);
        }

        text_color
    }

    // Backward compatibility method for the old build()
    fn calculate_style_default(&self) -> Node {
        let default_tokens = ThemeTokens::default();
        self.calculate_style(&default_tokens)
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
    tokens: Res<ThemeTokens>,
) {
    let entity = trigger.target();
    if let Ok(button) = buttons.get(entity) {
        if let Ok(mut bg_color) = bg_colors.get_mut(entity) {
            // Don't apply hover effect if disabled or loading
            if button.disabled || button.loading {
                return;
            }

            // Apply hover effect based on button variant
            let hover_color = match button.variant {
                ButtonVariant::Classic | ButtonVariant::Solid => {
                    // Slightly darker primary color
                    let current = bg_color.0;
                    let srgba = current.to_srgba();
                    Color::srgba(
                        (srgba.red * 0.9).max(0.0),
                        (srgba.green * 0.9).max(0.0),
                        (srgba.blue * 0.9).max(0.0),
                        srgba.alpha,
                    )
                },
                ButtonVariant::Soft | ButtonVariant::Surface => {
                    // Slightly more opaque secondary color
                    let current = bg_color.0;
                    let srgba = current.to_srgba();
                    Color::srgba(
                        srgba.red,
                        srgba.green,
                        srgba.blue,
                        (srgba.alpha + 0.1).min(1.0),
                    )
                },
                ButtonVariant::Outline | ButtonVariant::Ghost => {
                    // Add subtle background for outline/ghost variants
                    tokens.semantic.muted
                },
            };

            bg_color.0 = hover_color;
        }
    }
}

fn on_button_hover_out(
    trigger: Trigger<Pointer<Out>>,
    buttons: Query<&Button>,
    mut bg_colors: Query<&mut BackgroundColor>,
    tokens: Res<ThemeTokens>,
) {
    let entity = trigger.target();
    if let Ok(button) = buttons.get(entity) {
        if let Ok(mut bg_color) = bg_colors.get_mut(entity) {
            // Restore original background color using theme tokens
            let variant = match button.variant {
                ButtonVariant::Classic => BackgroundVariant::Primary,
                ButtonVariant::Solid => BackgroundVariant::Primary,
                ButtonVariant::Soft => BackgroundVariant::Secondary,
                ButtonVariant::Surface => BackgroundVariant::Muted,
                ButtonVariant::Outline => BackgroundVariant::Transparent,
                ButtonVariant::Ghost => BackgroundVariant::Transparent,
            };

            let original_color = match variant {
                BackgroundVariant::Primary => tokens.semantic.primary,
                BackgroundVariant::Secondary => tokens.semantic.secondary,
                BackgroundVariant::Muted => tokens.semantic.muted,
                BackgroundVariant::Transparent => Color::NONE,
                _ => tokens.semantic.background,
            };

            // Apply disabled state
            let final_color = if button.disabled {
                let srgba = original_color.to_srgba();
                Color::srgba(srgba.red, srgba.green, srgba.blue, 0.5)
            } else {
                original_color
            };

            bg_color.0 = final_color;
        }
    }
}
