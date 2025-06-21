use crate::{
    assets::audio::{sound_effect, SfxAssets},
    components::text::{Text, TextColor as TextColorEnum, TextSize, TextWeight},
    theme::{
        color::{accent_palette, TextContrastLevel, UiColorPalette},
        layout::UiLayout,
    },
    utilities::ComponentBuilder,
};
use bevy::{ecs::spawn::SpawnWith, prelude::*};
use bevy_picking::prelude::{Click, Out, Over, Pickable, Pointer, Pressed, Released};

#[derive(Event, Debug, Clone)]
pub struct ButtonClickEvent {
    pub button_entity: Entity,
    pub button_variant: ButtonVariant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonState {
    Normal,
    Hover,
    Active,
    Disabled,
    Loading,
}

#[derive(Component, Debug, Clone)]
pub struct Button {
    pub variant: ButtonVariant,
    pub size: ButtonSize,
    pub color: UiColorPalette,
    pub high_contrast: bool,
    pub radius: ButtonRadius,
    pub loading: bool,
    pub disabled: bool,
    pub current_state: ButtonState,
}

impl Default for Button {
    fn default() -> Self {
        Self {
            variant: ButtonVariant::Solid,
            size: ButtonSize::Default,
            color: accent_palette(),
            high_contrast: false,
            radius: ButtonRadius::Base,
            loading: false,
            disabled: false,
            current_state: ButtonState::Normal,
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
    ExtraSmall,
    Small,
    Base,
    Large,
    ExtraLarge,
    Extra2Large,
    Extra3Large,
    Extra4Large,
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

    pub fn child(mut self, entity: Entity) -> Self {
        self.children.push(entity);
        self
    }
}

impl ButtonBuilder {
    /// Get appropriate text size for button size
    fn get_button_text_size(&self) -> TextSize {
        match self.button.size {
            ButtonSize::Small => TextSize::Sm,
            ButtonSize::Default => TextSize::Base,
            ButtonSize::Large => TextSize::Lg,
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

    pub fn build(self) -> impl Bundle {
        let node = self.calculate_style();
        let background_color = self.calculate_background_color();
        let border_color = self.calculate_border_color();
        let border_radius = match self.button.radius {
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
        let display_text = self.text.clone().unwrap_or_default();
        let is_loading = self.button.loading;
        let text_size = self.get_button_text_size();
        let text_weight = self.get_button_text_weight();
        let text_color_enum = self.get_text_color_enum();

        (
            Name::new(format!("{}_Button", self.name)),
            self.button,
            node,
            border_color,
            BorderRadius {
                top_left: border_radius,
                top_right: border_radius,
                bottom_left: border_radius,
                bottom_right: border_radius,
            },
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
                    parent.spawn(
                        Text::label(display_text.clone())
                            .color(text_color_enum)
                            .size(text_size)
                            .weight(text_weight)
                            .center()
                            .build(),
                    );
                }
            })),
        )
    }
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
            (ButtonVariant::Ghost, ButtonState::Normal) => self.color.base_a,
            (ButtonVariant::Ghost, ButtonState::Hover) => self.color.bg_hover_a,
            (ButtonVariant::Ghost, ButtonState::Active) => self.color.bg_active_a,
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
        let background_color = match (self.variant, state) {
            (ButtonVariant::Solid, _)
            | (ButtonVariant::Soft, _)
            | (ButtonVariant::Outline, _)
            | (ButtonVariant::Ghost, _) => self.color.text_contrast,
        };

        let contrast_level = if self.high_contrast {
            TextContrastLevel::Accessible
        } else {
            TextContrastLevel::High
        };

        let mut text_color = self
            .color
            .get_text_color_for_contrast_level(&background_color, contrast_level);

        if state == ButtonState::Disabled {
            let background_luminance = UiColorPalette::calculate_luminance(&background_color);
            if background_luminance > 0.5 {
                text_color = self.color.solid_hover;
            } else {
                text_color = self.color.bg;
            }

            let srgba = text_color.to_srgba();
            text_color = Color::srgba(srgba.red, srgba.green, srgba.blue, 0.7);
        }

        TextColor(text_color)
    }
}

#[derive(Debug, Clone)]
pub struct ButtonStyling {
    pub background_color: BackgroundColor,
    pub border_color: BorderColor,
    pub text_color: TextColor,
}

impl ButtonBuilder {
    fn calculate_style(&self) -> Node {
        let padding = UiRect::axes(
            Val::Px(match self.button.size {
                ButtonSize::Default => UiLayout::default().padding.base + 4.0,
                ButtonSize::Small => UiLayout::default().padding.sm + 2.0,
                ButtonSize::Large => UiLayout::default().padding.lg + 8.0,
            }),
            Val::Px(match self.button.size {
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

    fn calculate_background_color(&self) -> BackgroundColor {
        self.button
            .get_styling(ButtonState::Normal)
            .background_color
    }

    fn calculate_border_color(&self) -> BorderColor {
        self.button.get_styling(ButtonState::Normal).border_color
    }

    fn calculate_text_color(&self) -> TextColor {
        self.button.get_styling(ButtonState::Normal).text_color
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
            .observe(on_button_hover_out)
            .observe(on_button_pressed)
            .observe(on_button_released);
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
    mut commands: Commands,
    sfx_assets: Res<SfxAssets>,
) {
    let entity = trigger.target();
    if let Ok(button) = buttons.get(entity) {
        // Don't trigger if disabled or loading
        if button.disabled || button.loading {
            return;
        }

        info!("Button clicked! Variant: {:?}", button.variant);

        // Play tap sound effect
        commands.spawn(sound_effect(sfx_assets.tap.clone()));

        // Send custom event
        events.write(ButtonClickEvent {
            button_entity: entity,
            button_variant: button.variant,
        });
    }
}

fn apply_button_styling(
    entity: Entity,
    button: &Button,
    state: ButtonState,
    bg_colors: &mut Query<&mut BackgroundColor>,
    text_colors: &mut Query<&mut TextColor>,
) {
    let styling = button.get_styling(state);

    if let Ok(mut bg_color) = bg_colors.get_mut(entity) {
        *bg_color = styling.background_color;
    }

    if let Ok(mut text_color) = text_colors.get_mut(entity) {
        *text_color = styling.text_color;
    }
}

fn on_button_hover(
    trigger: Trigger<Pointer<Over>>,
    mut buttons: Query<&mut Button>,
    mut bg_colors: Query<&mut BackgroundColor>,
    mut text_colors: Query<&mut TextColor>,
) {
    let entity = trigger.target();
    if let Ok(mut button) = buttons.get_mut(entity) {
        if button.disabled || button.loading {
            return;
        }

        button.current_state = ButtonState::Hover;
        apply_button_styling(
            entity,
            &button,
            ButtonState::Hover,
            &mut bg_colors,
            &mut text_colors,
        );
    }
}

fn on_button_hover_out(
    trigger: Trigger<Pointer<Out>>,
    mut buttons: Query<&mut Button>,
    mut bg_colors: Query<&mut BackgroundColor>,
    mut text_colors: Query<&mut TextColor>,
) {
    let entity = trigger.target();
    if let Ok(mut button) = buttons.get_mut(entity) {
        button.current_state = ButtonState::Normal;
        apply_button_styling(
            entity,
            &button,
            ButtonState::Normal,
            &mut bg_colors,
            &mut text_colors,
        );
    }
}

fn on_button_pressed(
    trigger: Trigger<Pointer<Pressed>>,
    mut buttons: Query<&mut Button>,
    mut bg_colors: Query<&mut BackgroundColor>,
    mut text_colors: Query<&mut TextColor>,
) {
    let entity = trigger.target();
    if let Ok(mut button) = buttons.get_mut(entity) {
        if button.disabled || button.loading {
            return;
        }

        button.current_state = ButtonState::Active;
        apply_button_styling(
            entity,
            &button,
            ButtonState::Active,
            &mut bg_colors,
            &mut text_colors,
        );
    }
}

fn on_button_released(
    trigger: Trigger<Pointer<Released>>,
    mut buttons: Query<&mut Button>,
    mut bg_colors: Query<&mut BackgroundColor>,
    mut text_colors: Query<&mut TextColor>,
) {
    let entity = trigger.target();
    if let Ok(mut button) = buttons.get_mut(entity) {
        if button.disabled || button.loading {
            return;
        }

        button.current_state = ButtonState::Hover;
        apply_button_styling(
            entity,
            &button,
            ButtonState::Hover,
            &mut bg_colors,
            &mut text_colors,
        );
    }
}
