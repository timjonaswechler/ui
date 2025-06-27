use crate::theme::color::{accent_palette, UiColorPalette};
use bevy::prelude::*;
use bevy_picking::prelude::{Click, Out, Over, Pickable, Pointer, Pressed, Released};

#[derive(Event, Debug, Clone)]
pub struct ToggleChangeEvent {
    pub toggle_entity: Entity,
    pub pressed: bool,
    pub size: ToggleSize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToggleState {
    Normal,
    Hover,
    Active,
    Disabled,
}

#[derive(Component, Debug, Clone)]
pub struct ToggleComponent {
    pub size: ToggleSize,
    pub variant: ToggleVariant,
    pub color: UiColorPalette,
    pub pressed: bool,
    pub disabled: bool,
    pub high_contrast: bool,
    pub current_state: ToggleState,
    pub is_clicking: bool,
}

impl Default for ToggleComponent {
    fn default() -> Self {
        Self {
            size: ToggleSize::Size2,
            variant: ToggleVariant::Soft,
            color: accent_palette(),
            pressed: false,
            disabled: false,
            high_contrast: false,
            current_state: ToggleState::Normal,
            is_clicking: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ToggleSize {
    /// Small size - 24px height
    Size1,
    /// Default size - 32px height
    #[default]
    Size2,
    /// Large size - 40px height
    Size3,
    /// Extra large size - 48px height
    Size4,
}

impl ToggleSize {
    pub fn height(self) -> f32 {
        match self {
            ToggleSize::Size1 => 24.0,
            ToggleSize::Size2 => 32.0,
            ToggleSize::Size3 => 40.0,
            ToggleSize::Size4 => 48.0,
        }
    }

    pub fn padding(self) -> f32 {
        match self {
            ToggleSize::Size1 => 4.0,
            ToggleSize::Size2 => 8.0,
            ToggleSize::Size3 => 12.0,
            ToggleSize::Size4 => 16.0,
        }
    }

    pub fn font_size(self) -> f32 {
        match self {
            ToggleSize::Size1 => 12.0,
            ToggleSize::Size2 => 14.0,
            ToggleSize::Size3 => 16.0,
            ToggleSize::Size4 => 18.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ToggleVariant {
    /// Solid variant with pronounced styling
    Solid,
    /// Default soft variant with subtle styling
    #[default]
    Soft,
    /// Surface variant with minimal styling
    Surface,
    /// Outline variant with border
    Outline,
}

pub struct ToggleBuilder {
    name: String,
    toggle: ToggleComponent,
    text: Option<String>,
}

impl ToggleBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: format!("{}_Toggle", name.into()),
            toggle: ToggleComponent::default(),
            text: None,
        }
    }

    pub fn size(mut self, size: ToggleSize) -> Self {
        self.toggle.size = size;
        self
    }

    pub fn size_1(self) -> Self {
        self.size(ToggleSize::Size1)
    }

    pub fn size_2(self) -> Self {
        self.size(ToggleSize::Size2)
    }

    pub fn size_3(self) -> Self {
        self.size(ToggleSize::Size3)
    }

    pub fn size_4(self) -> Self {
        self.size(ToggleSize::Size4)
    }

    pub fn variant(mut self, variant: ToggleVariant) -> Self {
        self.toggle.variant = variant;
        self
    }

    pub fn solid(self) -> Self {
        self.variant(ToggleVariant::Solid)
    }

    pub fn soft(self) -> Self {
        self.variant(ToggleVariant::Soft)
    }

    pub fn surface(self) -> Self {
        self.variant(ToggleVariant::Surface)
    }

    pub fn outline(self) -> Self {
        self.variant(ToggleVariant::Outline)
    }

    pub fn color(mut self, color: UiColorPalette) -> Self {
        self.toggle.color = color;
        self
    }

    pub fn pressed(mut self) -> Self {
        self.toggle.pressed = true;
        self
    }

    pub fn disabled(mut self) -> Self {
        self.toggle.disabled = true;
        self.toggle.current_state = ToggleState::Disabled;
        self
    }

    pub fn high_contrast(mut self) -> Self {
        self.toggle.high_contrast = true;
        self
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }
}

impl ToggleComponent {
    pub fn new(name: impl Into<String>) -> ToggleBuilder {
        ToggleBuilder::new(name)
    }

    pub fn get_styling(&self, state: ToggleState) -> ToggleStyling {
        let current_state = if self.disabled {
            ToggleState::Disabled
        } else {
            state
        };

        ToggleStyling {
            background_color: self.calculate_background_color(current_state),
            border_color: self.calculate_border_color(current_state),
            text_color: self.calculate_text_color(current_state),
        }
    }

    fn calculate_background_color(&self, state: ToggleState) -> BackgroundColor {
        let base_color = match (self.pressed, self.variant, state) {
            // Pressed states
            (true, ToggleVariant::Solid, ToggleState::Normal) => self.color.solid,
            (true, ToggleVariant::Solid, ToggleState::Hover) => self.color.solid_hover,
            (true, ToggleVariant::Solid, ToggleState::Active) => self.color.solid_hover,
            (true, ToggleVariant::Soft, ToggleState::Normal) => self.color.bg_active,
            (true, ToggleVariant::Soft, ToggleState::Hover) => self.color.bg_hover,
            (true, ToggleVariant::Soft, ToggleState::Active) => self.color.bg_hover,
            (true, ToggleVariant::Surface, ToggleState::Normal) => self.color.bg_subtle,
            (true, ToggleVariant::Surface, ToggleState::Hover) => self.color.bg_hover,
            (true, ToggleVariant::Surface, ToggleState::Active) => self.color.bg_active,
            (true, ToggleVariant::Outline, ToggleState::Normal) => self.color.bg_subtle,
            (true, ToggleVariant::Outline, ToggleState::Hover) => self.color.bg_hover,
            (true, ToggleVariant::Outline, ToggleState::Active) => self.color.bg_active,

            // Unpressed states
            (false, ToggleVariant::Solid, ToggleState::Normal) => Color::NONE,
            (false, ToggleVariant::Solid, ToggleState::Hover) => self.color.bg_hover,
            (false, ToggleVariant::Solid, ToggleState::Active) => self.color.bg_active,
            (false, ToggleVariant::Soft, ToggleState::Normal) => Color::NONE,
            (false, ToggleVariant::Soft, ToggleState::Hover) => self.color.bg_hover,
            (false, ToggleVariant::Soft, ToggleState::Active) => self.color.bg_active,
            (false, ToggleVariant::Surface, ToggleState::Normal) => Color::NONE,
            (false, ToggleVariant::Surface, ToggleState::Hover) => self.color.bg_hover,
            (false, ToggleVariant::Surface, ToggleState::Active) => self.color.bg_active,
            (false, ToggleVariant::Outline, ToggleState::Normal) => Color::NONE,
            (false, ToggleVariant::Outline, ToggleState::Hover) => self.color.bg_hover,
            (false, ToggleVariant::Outline, ToggleState::Active) => self.color.bg_active,

            // Disabled state
            (_, _, ToggleState::Disabled) => {
                if self.pressed {
                    self.color.bg_subtle
                } else {
                    Color::NONE
                }
            }
        };

        let mut bg_color = BackgroundColor(base_color);

        if state == ToggleState::Disabled {
            let srgba = bg_color.0.to_srgba();
            bg_color.0 = Color::srgba(srgba.red, srgba.green, srgba.blue, 0.5);
        }

        bg_color
    }

    fn calculate_border_color(&self, state: ToggleState) -> BorderColor {
        match (self.variant, state) {
            (ToggleVariant::Outline, ToggleState::Normal) => {
                if self.pressed {
                    BorderColor(self.color.solid)
                } else {
                    BorderColor(self.color.border)
                }
            }
            (ToggleVariant::Outline, ToggleState::Hover) => {
                if self.pressed {
                    BorderColor(self.color.solid_hover)
                } else {
                    BorderColor(self.color.border_hover)
                }
            }
            (ToggleVariant::Outline, ToggleState::Active) => BorderColor(self.color.solid),
            (ToggleVariant::Outline, ToggleState::Disabled) => {
                let color = if self.pressed {
                    self.color.solid
                } else {
                    self.color.border
                };
                BorderColor(color.with_alpha(0.5))
            }
            _ => BorderColor(Color::NONE),
        }
    }

    fn calculate_text_color(&self, state: ToggleState) -> Color {
        match (self.pressed, self.variant, state) {
            // Pressed states
            (true, ToggleVariant::Solid, _) => self.color.text_contrast,
            (true, ToggleVariant::Soft, _) => self.color.text,
            (true, ToggleVariant::Surface, _) => self.color.text,
            (true, ToggleVariant::Outline, _) => self.color.text,

            // Unpressed states
            (false, _, _) => self.color.text,
            // Disabled state overrides
        }
        .with_alpha(if state == ToggleState::Disabled {
            0.5
        } else {
            1.0
        })
    }
}

#[derive(Debug, Clone)]
pub struct ToggleStyling {
    pub background_color: BackgroundColor,
    pub border_color: BorderColor,
    pub text_color: Color,
}

impl ToggleBuilder {
    fn calculate_style(&self) -> Node {
        let size = self.toggle.size;
        let padding = size.padding();

        Node {
            height: Val::Px(size.height()),
            padding: UiRect::all(Val::Px(padding)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: match self.toggle.variant {
                ToggleVariant::Outline => UiRect::all(Val::Px(1.0)),
                _ => UiRect::all(Val::Px(0.0)),
            },
            ..default()
        }
    }

    fn calculate_background_color(&self) -> BackgroundColor {
        self.toggle
            .get_styling(ToggleState::Normal)
            .background_color
    }

    fn calculate_border_color(&self) -> BorderColor {
        self.toggle.get_styling(ToggleState::Normal).border_color
    }

    fn calculate_border_radius(&self) -> BorderRadius {
        BorderRadius::all(Val::Px(4.0))
    }
}

impl ToggleBuilder {
    pub fn build(self) -> impl Bundle {
        let node = self.calculate_style();
        let background_color = self.calculate_background_color();
        let border_color = self.calculate_border_color();
        let border_radius = self.calculate_border_radius();

        let bundle = (
            self.toggle.clone(),
            node,
            background_color,
            border_color,
            border_radius,
            Pickable::default(),
            Button,
            Interaction::None,
        );

        // Always add text spawner
        if let Some(text) = self.text {
            (
                bundle,
                ToggleTextSpawner {
                    text,
                    font_size: self.toggle.size.font_size(),
                    text_color: self.toggle.get_styling(ToggleState::Normal).text_color,
                },
            )
        } else {
            (
                bundle,
                ToggleTextSpawner {
                    text: String::new(),
                    font_size: 0.0,
                    text_color: Color::NONE,
                },
            )
        }
    }
}

#[derive(Component)]
pub struct EmptyToggleSpawner;

#[derive(Component)]
pub struct ToggleTextSpawner {
    text: String,
    font_size: f32,
    text_color: Color,
}

pub fn spawn_toggle_children(
    mut commands: Commands,
    query: Query<(Entity, &ToggleTextSpawner), Added<ToggleTextSpawner>>,
) {
    for (entity, spawner) in &query {
        // Only spawn text if it's not empty
        if !spawner.text.is_empty() && spawner.font_size > 0.0 {
            commands.entity(entity).with_children(|parent| {
                parent.spawn((
                    Name::new("ToggleText"),
                    Text::new(spawner.text.clone()),
                    TextColor(spawner.text_color),
                    TextFont {
                        font_size: spawner.font_size,
                        ..default()
                    },
                    Pickable::IGNORE,
                ));
            });
        }

        // Remove the spawner component
        commands.entity(entity).remove::<ToggleTextSpawner>();
    }
}

pub fn setup_toggle_interactions(
    mut commands: Commands,
    toggles: Query<Entity, Added<ToggleComponent>>,
) {
    for entity in &toggles {
        commands
            .entity(entity)
            .observe(on_toggle_click)
            .observe(on_toggle_hover_start)
            .observe(on_toggle_hover_end)
            .observe(on_toggle_press)
            .observe(on_toggle_release);
    }
}

// Observer functions for toggle interactions
fn on_toggle_click(_trigger: Trigger<Pointer<Click>>) {
    // Empty - we handle toggle on click
}

fn on_toggle_hover_start(
    trigger: Trigger<Pointer<Over>>,
    mut toggles: Query<&mut ToggleComponent>,
) {
    let entity = trigger.target();
    if let Ok(mut toggle) = toggles.get_mut(entity) {
        if !toggle.disabled {
            toggle.current_state = ToggleState::Hover;
        }
    }
}

fn on_toggle_hover_end(trigger: Trigger<Pointer<Out>>, mut toggles: Query<&mut ToggleComponent>) {
    let entity = trigger.target();
    if let Ok(mut toggle) = toggles.get_mut(entity) {
        if !toggle.disabled {
            toggle.current_state = ToggleState::Normal;
            toggle.is_clicking = false;
        }
    }
}

fn on_toggle_press(trigger: Trigger<Pointer<Pressed>>, mut toggles: Query<&mut ToggleComponent>) {
    let entity = trigger.target();
    if let Ok(mut toggle) = toggles.get_mut(entity) {
        if !toggle.disabled {
            toggle.current_state = ToggleState::Active;
            toggle.is_clicking = true;
        }
    }
}

fn on_toggle_release(
    trigger: Trigger<Pointer<Released>>,
    mut toggles: Query<&mut ToggleComponent>,
    mut events: EventWriter<ToggleChangeEvent>,
) {
    let entity = trigger.target();
    if let Ok(mut toggle) = toggles.get_mut(entity) {
        if !toggle.disabled && toggle.is_clicking {
            // Toggle the pressed state
            toggle.pressed = !toggle.pressed;
            toggle.is_clicking = false;
            toggle.current_state = ToggleState::Hover;

            // Send change event
            events.write(ToggleChangeEvent {
                toggle_entity: entity,
                pressed: toggle.pressed,
                size: toggle.size,
            });
        }
    }
}

// System to update visual styling when toggle state changes
pub fn update_toggle_styling(
    toggles_query: Query<(Entity, &ToggleComponent), Changed<ToggleComponent>>,
    mut bg_colors: Query<&mut BackgroundColor, (With<ToggleComponent>, Without<Text>)>,
    mut border_colors: Query<&mut BorderColor, With<ToggleComponent>>,
    children_query: Query<&Children>,
    mut text_colors: Query<&mut TextColor, With<Text>>,
) {
    for (entity, toggle) in &toggles_query {
        let styling = toggle.get_styling(toggle.current_state);

        // Update toggle background color
        if let Ok(mut bg_color) = bg_colors.get_mut(entity) {
            *bg_color = styling.background_color;
        }

        // Update border color
        if let Ok(mut border_color) = border_colors.get_mut(entity) {
            *border_color = styling.border_color;
        }

        // Update text color
        if let Ok(children) = children_query.get(entity) {
            for child in children.iter() {
                if let Ok(mut text_color) = text_colors.get_mut(child) {
                    *text_color = TextColor(styling.text_color);
                }
            }
        }
    }
}

pub type Toggle = ToggleComponent;
