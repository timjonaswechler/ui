use crate::{
    assets::audio::{sound_effect, SfxAssets},
    components::text::{Text, TextBuilder},
    theme::{
        color::{accent_palette, TextColor as TextColorEnum, TextContrastLevel, UiColorPalette},
        layout::UiLayout,
        typography::{FontFamily, TextSize, TextWeight},
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
}

impl ButtonBuilder {
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
        let background_color = match (self.variant, state) {
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

        let contrast_level = if self.high_contrast {
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
                ButtonSize::Default => UiLayout::default().padding.base + 2.0,
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

/// Marker component for text that should automatically update colors based on button state
/// Text with explicit colors (manual_color) will not have this component
#[derive(Component, Debug)]
pub struct ButtonManagedText;

impl Default for SpinnerAnimation {
    fn default() -> Self {
        Self {
            rotation_speed: 360.0, // Grad pro Sekunde
        }
    }
}

// System für Button-Interaktionen
pub fn setup_button_interactions(
    mut commands: Commands,
    buttons: Query<Entity, Added<Button>>,
    button_query: Query<&Button>,
    mut bg_colors: Query<&mut BackgroundColor>,
    mut text_colors: Query<&mut TextColor>,
    children_query: Query<&Children>,
    managed_text_query: Query<&ButtonManagedText>,
) {
    for entity in &buttons {
        commands
            .entity(entity)
            .observe(on_button_click)
            .observe(on_button_hover)
            .observe(on_button_hover_out)
            .observe(on_button_pressed)
            .observe(on_button_released);

        // Immediately apply correct styling to ensure consistency
        if let Ok(button) = button_query.get(entity) {
            apply_button_styling(
                entity,
                button,
                ButtonState::Normal,
                &mut bg_colors,
                &mut text_colors,
                &children_query,
                &managed_text_query,
            );
        }
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
    children_query: &Query<&Children>,
    managed_text_query: &Query<&ButtonManagedText>,
) {
    let styling = button.get_styling(state);

    // Update button background color
    if let Ok(mut bg_color) = bg_colors.get_mut(entity) {
        *bg_color = styling.background_color;
    }

    // Update direct text color (fallback for old button style)
    if let Ok(mut text_color) = text_colors.get_mut(entity) {
        *text_color = styling.text_color;
    }

    // Update text colors in children (for TextBuilder-created text components)
    if let Ok(children) = children_query.get(entity) {
        for child in children.iter() {
            update_text_colors_recursive(
                &child,
                &styling.text_color,
                text_colors,
                children_query,
                managed_text_query,
            );
        }
    }
}

fn update_text_colors_recursive(
    entity: &Entity,
    new_color: &TextColor,
    text_colors: &mut Query<&mut TextColor>,
    children_query: &Query<&Children>,
    managed_text_query: &Query<&ButtonManagedText>,
) {
    // Update text color only if this entity has both TextColor and ButtonManagedText components
    if managed_text_query.get(*entity).is_ok() {
        if let Ok(mut text_color) = text_colors.get_mut(*entity) {
            *text_color = *new_color;
        }
    }

    // Recursively check children
    if let Ok(children) = children_query.get(*entity) {
        for child in children.iter() {
            update_text_colors_recursive(
                &child,
                new_color,
                text_colors,
                children_query,
                managed_text_query,
            );
        }
    }
}

fn on_button_hover(
    trigger: Trigger<Pointer<Over>>,
    mut buttons: Query<&mut Button>,
    mut bg_colors: Query<&mut BackgroundColor>,
    mut text_colors: Query<&mut TextColor>,
    children_query: Query<&Children>,
    managed_text_query: Query<&ButtonManagedText>,
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
            &children_query,
            &managed_text_query,
        );
    }
}

fn on_button_hover_out(
    trigger: Trigger<Pointer<Out>>,
    mut buttons: Query<&mut Button>,
    mut bg_colors: Query<&mut BackgroundColor>,
    mut text_colors: Query<&mut TextColor>,
    children_query: Query<&Children>,
    managed_text_query: Query<&ButtonManagedText>,
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
            &children_query,
            &managed_text_query,
        );
    }
}

fn on_button_pressed(
    trigger: Trigger<Pointer<Pressed>>,
    mut buttons: Query<&mut Button>,
    mut bg_colors: Query<&mut BackgroundColor>,
    mut text_colors: Query<&mut TextColor>,
    children_query: Query<&Children>,
    managed_text_query: Query<&ButtonManagedText>,
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
            &children_query,
            &managed_text_query,
        );
    }
}

fn on_button_released(
    trigger: Trigger<Pointer<Released>>,
    mut buttons: Query<&mut Button>,
    mut bg_colors: Query<&mut BackgroundColor>,
    mut text_colors: Query<&mut TextColor>,
    children_query: Query<&Children>,
    managed_text_query: Query<&ButtonManagedText>,
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
            &children_query,
            &managed_text_query,
        );
    }
}
