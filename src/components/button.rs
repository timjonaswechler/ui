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
        let style = self.calculate_style();
        let background_color = self.calculate_background_color();
        let border_color = self.calculate_border_color();
        let border_radius = self.calculate_border_radius();
        let text_color = self.calculate_text_color();
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

        let mut color = match self.button.variant {
            ButtonVariant::Classic => color_scale.step_9,
            ButtonVariant::Solid => color_scale.step_9,
            ButtonVariant::Soft => color_scale.step_3,
            ButtonVariant::Surface => color_scale.step_2,
            ButtonVariant::Outline => Color::NONE,
            ButtonVariant::Ghost => Color::NONE,
        };

        // Apply disabled state
        if self.button.disabled {
            let srgba = color.to_srgba();
            color = Color::srgba(srgba.red, srgba.green, srgba.blue, 0.5);
        }

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

    fn calculate_text_color(&self) -> TextColor {
        let color = match self.button.variant {
            ButtonVariant::Classic | ButtonVariant::Solid => Color::WHITE,
            ButtonVariant::Soft | ButtonVariant::Surface => Color::BLACK,
            ButtonVariant::Outline | ButtonVariant::Ghost => Color::BLACK,
        };

        if self.button.disabled {
            TextColor(Color::srgba(
                color.to_srgba().red,
                color.to_srgba().green,
                color.to_srgba().blue,
                0.5,
            ))
        } else {
            TextColor(color)
        }
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

            // Hover-Effekt: Farbe etwas aufhellen
            let current = bg_color.0;
            let srgba = current.to_srgba();
            let r = srgba.red;
            let g = srgba.green;
            let b = srgba.blue;
            let a = srgba.alpha;
            bg_color.0 = Color::srgba(
                (r + 0.1).min(1.0),
                (g + 0.1).min(1.0),
                (b + 0.1).min(1.0),
                a,
            );
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
            // Hover-Effekt zurücksetzen: Original-Farbe wiederherstellen
            let theme_colors = ThemeColors {
                accent: indigo_scale(),
                gray: gray_scale(),
                background: Color::WHITE,
                panel_solid: Color::WHITE,
                panel_translucent: Color::srgba(1.0, 1.0, 1.0, 0.9),
                surface: Color::srgb(0.98, 0.98, 0.98),
                overlay: Color::srgba(0.0, 0.0, 0.0, 0.5),
            };

            let color_scale = &theme_colors.accent;
            let mut original_color = match button.variant {
                ButtonVariant::Classic => color_scale.step_9,
                ButtonVariant::Solid => color_scale.step_9,
                ButtonVariant::Soft => color_scale.step_3,
                ButtonVariant::Surface => color_scale.step_2,
                ButtonVariant::Outline => Color::NONE,
                ButtonVariant::Ghost => Color::NONE,
            };

            // Apply disabled state
            if button.disabled {
                let srgba = original_color.to_srgba();
                original_color = Color::srgba(srgba.red, srgba.green, srgba.blue, 0.5);
            }

            bg_color.0 = original_color;
        }
    }
}
