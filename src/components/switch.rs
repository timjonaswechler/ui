use crate::{
    theme::{
        color::{accent_palette, UiColorPalette},
    },
    utilities::ComponentBuilder,
};
use bevy::prelude::*;
use bevy_picking::prelude::{Click, Out, Over, Pickable, Pointer, Pressed, Released};

#[derive(Event, Debug, Clone)]
pub struct SwitchChangeEvent {
    pub switch_entity: Entity,
    pub checked: bool,
    pub size: SwitchSize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwitchState {
    Normal,
    Hover,
    Active,
    Disabled,
}

#[derive(Component, Debug, Clone)]
pub struct SwitchComponent {
    pub size: SwitchSize,
    pub variant: SwitchVariant,
    pub color: UiColorPalette,
    pub checked: bool,
    pub disabled: bool,
    pub high_contrast: bool,
    pub current_state: SwitchState,
    pub is_pressed: bool, // Track if currently pressed
}

impl Default for SwitchComponent {
    fn default() -> Self {
        Self {
            size: SwitchSize::Size2,
            variant: SwitchVariant::Surface,
            color: accent_palette(),
            checked: false,
            disabled: false,
            high_contrast: false,
            current_state: SwitchState::Normal,
            is_pressed: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SwitchSize {
    /// Small size - 16px height
    Size1,
    /// Default size - 20px height
    #[default]
    Size2,
    /// Large size - 24px height
    Size3,
}

impl SwitchSize {
    pub fn height(self) -> f32 {
        match self {
            SwitchSize::Size1 => 16.0,
            SwitchSize::Size2 => 20.0,
            SwitchSize::Size3 => 24.0,
        }
    }

    pub fn width(self) -> f32 {
        self.height() * 1.8 // Width is roughly 1.8x height for switch appearance
    }

    pub fn thumb_size(self) -> f32 {
        match self {
            SwitchSize::Size1 => 12.0,
            SwitchSize::Size2 => 16.0,
            SwitchSize::Size3 => 20.0,
        }
    }

    pub fn thumb_padding(self) -> f32 {
        (self.height() - self.thumb_size()) / 2.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SwitchVariant {
    /// Default surface variant with subtle styling
    #[default]
    Surface,
    /// Classic variant with more pronounced styling
    Classic,
    /// Soft variant with muted colors
    Soft,
}

#[derive(Component, Debug)]
pub struct SwitchThumb {
    pub target_x: f32,
    pub animation_speed: f32,
}

impl Default for SwitchThumb {
    fn default() -> Self {
        Self {
            target_x: 0.0,
            animation_speed: 15.0, // Faster animation
        }
    }
}

pub struct SwitchBuilder {
    name: String,
    switch: SwitchComponent,
}

impl SwitchBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: format!("{}_Switch", name.into()),
            switch: SwitchComponent::default(),
        }
    }

    pub fn size(mut self, size: SwitchSize) -> Self {
        self.switch.size = size;
        self
    }

    pub fn size_1(self) -> Self {
        self.size(SwitchSize::Size1)
    }

    pub fn size_2(self) -> Self {
        self.size(SwitchSize::Size2)
    }

    pub fn size_3(self) -> Self {
        self.size(SwitchSize::Size3)
    }

    pub fn variant(mut self, variant: SwitchVariant) -> Self {
        self.switch.variant = variant;
        self
    }

    pub fn surface(self) -> Self {
        self.variant(SwitchVariant::Surface)
    }

    pub fn classic(self) -> Self {
        self.variant(SwitchVariant::Classic)
    }

    pub fn soft(self) -> Self {
        self.variant(SwitchVariant::Soft)
    }

    pub fn color(mut self, color: UiColorPalette) -> Self {
        self.switch.color = color;
        self
    }

    pub fn checked(mut self) -> Self {
        self.switch.checked = true;
        self
    }

    pub fn disabled(mut self) -> Self {
        self.switch.disabled = true;
        self.switch.current_state = SwitchState::Disabled;
        self
    }

    pub fn high_contrast(mut self) -> Self {
        self.switch.high_contrast = true;
        self
    }
}

impl SwitchComponent {
    pub fn new(name: impl Into<String>) -> SwitchBuilder {
        SwitchBuilder::new(name)
    }

    pub fn get_styling(&self, state: SwitchState) -> SwitchStyling {
        let current_state = if self.disabled {
            SwitchState::Disabled
        } else {
            state
        };

        SwitchStyling {
            background_color: self.calculate_background_color(current_state),
            border_color: self.calculate_border_color(current_state),
            thumb_color: self.calculate_thumb_color(current_state),
        }
    }

    fn calculate_background_color(&self, state: SwitchState) -> BackgroundColor {
        let base_color = match (self.checked, self.variant, state) {
            // Checked states
            (true, SwitchVariant::Surface, SwitchState::Normal) => self.color.solid,
            (true, SwitchVariant::Surface, SwitchState::Hover) => self.color.solid_hover,
            (true, SwitchVariant::Surface, SwitchState::Active) => self.color.solid_hover,
            (true, SwitchVariant::Classic, SwitchState::Normal) => self.color.solid,
            (true, SwitchVariant::Classic, SwitchState::Hover) => self.color.solid_hover,
            (true, SwitchVariant::Classic, SwitchState::Active) => self.color.solid_hover,
            (true, SwitchVariant::Soft, SwitchState::Normal) => self.color.bg_active,
            (true, SwitchVariant::Soft, SwitchState::Hover) => self.color.bg_hover,
            (true, SwitchVariant::Soft, SwitchState::Active) => self.color.bg_hover,
            
            // Unchecked states
            (false, _, SwitchState::Normal) => match self.variant {
                SwitchVariant::Surface => self.color.bg_subtle,
                SwitchVariant::Classic => self.color.bg,
                SwitchVariant::Soft => self.color.bg_hover,
            },
            (false, _, SwitchState::Hover) => match self.variant {
                SwitchVariant::Surface => self.color.bg_hover,
                SwitchVariant::Classic => self.color.bg_hover,
                SwitchVariant::Soft => self.color.bg_active,
            },
            (false, _, SwitchState::Active) => self.color.bg_active,
            
            // Disabled state
            (_, _, SwitchState::Disabled) => {
                if self.checked {
                    self.color.solid
                } else {
                    self.color.bg_subtle
                }
            }
        };

        let mut bg_color = BackgroundColor(base_color);

        if state == SwitchState::Disabled {
            let srgba = bg_color.0.to_srgba();
            bg_color.0 = Color::srgba(srgba.red, srgba.green, srgba.blue, 0.6);
        }

        bg_color
    }

    fn calculate_border_color(&self, state: SwitchState) -> BorderColor {
        match (self.variant, state) {
            (SwitchVariant::Surface, _) => BorderColor(Color::NONE),
            (SwitchVariant::Soft, _) => BorderColor(Color::NONE),
            (SwitchVariant::Classic, SwitchState::Normal) => {
                if self.checked {
                    BorderColor(self.color.solid)
                } else {
                    BorderColor(self.color.border)
                }
            }
            (SwitchVariant::Classic, SwitchState::Hover) => {
                if self.checked {
                    BorderColor(self.color.solid_hover)
                } else {
                    BorderColor(self.color.border_hover)
                }
            }
            (SwitchVariant::Classic, SwitchState::Active) => BorderColor(self.color.solid),
            (SwitchVariant::Classic, SwitchState::Disabled) => {
                let color = if self.checked {
                    self.color.solid
                } else {
                    self.color.border
                };
                BorderColor(color.with_alpha(0.6))
            }
        }
    }

    fn calculate_thumb_color(&self, _state: SwitchState) -> Color {
        // Thumb is typically white/light for all variants
        match self.variant {
            SwitchVariant::Surface | SwitchVariant::Classic => Color::WHITE,
            SwitchVariant::Soft => {
                if self.checked {
                    self.color.text_contrast
                } else {
                    self.color.text
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct SwitchStyling {
    pub background_color: BackgroundColor,
    pub border_color: BorderColor,
    pub thumb_color: Color,
}

impl SwitchBuilder {
    fn calculate_style(&self) -> Node {
        let size = self.switch.size;
        let padding = size.thumb_padding();

        Node {
            width: Val::Px(size.width()),
            height: Val::Px(size.height()),
            padding: UiRect::all(Val::Px(padding)),
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Center,
            border: match self.switch.variant {
                SwitchVariant::Classic => UiRect::all(Val::Px(1.0)),
                _ => UiRect::all(Val::Px(0.0)),
            },
            ..default()
        }
    }

    fn calculate_background_color(&self) -> BackgroundColor {
        self.switch.get_styling(SwitchState::Normal).background_color
    }

    fn calculate_border_color(&self) -> BorderColor {
        self.switch.get_styling(SwitchState::Normal).border_color
    }

    fn calculate_border_radius(&self) -> BorderRadius {
        let radius = self.switch.size.height() / 2.0;
        BorderRadius::all(Val::Px(radius))
    }
}

impl ComponentBuilder for SwitchBuilder {
    type Output = (
        Name,
        SwitchComponent,
        Node,
        BackgroundColor,
        BorderColor,
        BorderRadius,
        Pickable,
        Button,
        Interaction,
        SwitchChildSpawner,
    );

    fn build(self) -> Self::Output {
        let node = self.calculate_style();
        let background_color = self.calculate_background_color();
        let border_color = self.calculate_border_color();
        let border_radius = self.calculate_border_radius();
        let size = self.switch.size;
        let checked = self.switch.checked;
        let thumb_color = self.switch.get_styling(SwitchState::Normal).thumb_color;

        (
            Name::new(self.name.clone()),
            self.switch,
            node,
            background_color,
            border_color,
            border_radius,
            Pickable::default(),
            Button,
            Interaction::None,
            SwitchChildSpawner {
                size,
                checked,
                thumb_color,
            },
        )
    }
}

#[derive(Component)]
pub struct SwitchChildSpawner {
    size: SwitchSize,
    checked: bool,
    thumb_color: Color,
}

pub fn spawn_switch_children(
    mut commands: Commands,
    query: Query<(Entity, &SwitchChildSpawner), Added<SwitchChildSpawner>>,
) {
    for (entity, spawner) in &query {
        let thumb_size = spawner.size.thumb_size();
        let target_x = if spawner.checked {
            spawner.size.width() - spawner.size.thumb_padding() - thumb_size
        } else {
            0.0
        };

        commands.entity(entity).with_children(|parent| {
            parent.spawn((
                Name::new("SwitchThumb"),
                SwitchThumb {
                    target_x,
                    animation_speed: 15.0,
                },
                Node {
                    width: Val::Px(thumb_size),
                    height: Val::Px(thumb_size),
                    position_type: PositionType::Absolute,
                    left: Val::Px(target_x),
                    top: Val::Px((spawner.size.height() - thumb_size) / 2.0),
                    ..default()
                },
                BackgroundColor(spawner.thumb_color),
                BorderRadius::all(Val::Px(thumb_size / 2.0)),
                Pickable::IGNORE,
            ));
        });

        // Remove the spawner component as it's no longer needed
        commands.entity(entity).remove::<SwitchChildSpawner>();
    }
}

pub fn setup_switch_interactions(
    mut commands: Commands,
    switches: Query<Entity, Added<SwitchComponent>>,
) {
    for entity in &switches {
        commands
            .entity(entity)
            .observe(on_switch_click)
            .observe(on_switch_hover_start)
            .observe(on_switch_hover_end)
            .observe(on_switch_press)
            .observe(on_switch_release);
    }
}

// Separate system to update thumb positions when switch state changes
pub fn update_switch_thumb_positions(
    switches_query: Query<(Entity, &SwitchComponent), Changed<SwitchComponent>>,
    children_query: Query<&Children>,
    mut thumb_query: Query<&mut SwitchThumb>,
) {
    for (entity, switch) in &switches_query {
        if let Ok(children) = children_query.get(entity) {
            for child in children.iter() {
                if let Ok(mut thumb) = thumb_query.get_mut(child) {
                    let size = switch.size;
                    let new_target_x = if switch.checked {
                        size.width() - size.thumb_padding() - size.thumb_size()
                    } else {
                        0.0
                    };
                    
                    // Only update if target changed to prevent unnecessary animations
                    if (thumb.target_x - new_target_x).abs() > 0.1 {
                        thumb.target_x = new_target_x;
                    }
                }
            }
        }
    }
}

pub fn animate_switch_thumbs(
    time: Res<Time>,
    mut thumb_query: Query<(&mut Node, &SwitchThumb)>,
) {
    for (mut node, thumb) in thumb_query.iter_mut() {
        if let Val::Px(current_x) = node.left {
            let diff = thumb.target_x - current_x;
            
            if diff.abs() > 0.5 {
                // Calculate new position with smooth easing
                let movement = diff * thumb.animation_speed * time.delta_secs();
                let new_x = current_x + movement;
                
                // Hard clamp to ensure thumb never goes outside reasonable bounds
                // Since target_x is calculated correctly, we clamp between 0 and target_x
                let max_bound = if thumb.target_x > 0.0 { 
                    thumb.target_x 
                } else { 
                    100.0 // Fallback max bound
                };
                let clamped_x = new_x.clamp(0.0, max_bound);
                node.left = Val::Px(clamped_x);
            } else {
                // Snap to final position when very close
                node.left = Val::Px(thumb.target_x);
            }
        }
    }
}

// Observer functions for switch interactions
fn on_switch_click(
    _trigger: Trigger<Pointer<Click>>,
    // Don't toggle on click anymore, only on release
) {
    // Empty - we handle toggle on release now
}

fn on_switch_hover_start(
    trigger: Trigger<Pointer<Over>>,
    mut switches: Query<&mut SwitchComponent>,
) {
    let entity = trigger.target();
    if let Ok(mut switch) = switches.get_mut(entity) {
        if !switch.disabled {
            switch.current_state = SwitchState::Hover;
        }
    }
}

fn on_switch_hover_end(
    trigger: Trigger<Pointer<Out>>,
    mut switches: Query<&mut SwitchComponent>,
) {
    let entity = trigger.target();
    if let Ok(mut switch) = switches.get_mut(entity) {
        if !switch.disabled {
            switch.current_state = SwitchState::Normal;
            switch.is_pressed = false; // Reset pressed state when leaving
        }
    }
}

fn on_switch_press(
    trigger: Trigger<Pointer<Pressed>>,
    mut switches: Query<&mut SwitchComponent>,
) {
    let entity = trigger.target();
    if let Ok(mut switch) = switches.get_mut(entity) {
        if !switch.disabled {
            switch.current_state = SwitchState::Active;
            switch.is_pressed = true; // Mark as pressed
        }
    }
}

fn on_switch_release(
    trigger: Trigger<Pointer<Released>>,
    mut switches: Query<&mut SwitchComponent>,
    mut events: EventWriter<SwitchChangeEvent>,
    children_query: Query<&Children>,
    mut thumb_query: Query<(&mut Node, &mut SwitchThumb), With<SwitchThumb>>,
) {
    let entity = trigger.target();
    if let Ok(mut switch) = switches.get_mut(entity) {
        if !switch.disabled && switch.is_pressed {
            // Toggle the switch on release (only if it was pressed first)
            switch.checked = !switch.checked;
            switch.is_pressed = false;
            switch.current_state = SwitchState::Hover;

            // Immediately update thumb position (no animation)
            if let Ok(children) = children_query.get(entity) {
                for child in children.iter() {
                    if let Ok((mut node, mut thumb)) = thumb_query.get_mut(child) {
                        let size = switch.size;
                        let new_x = if switch.checked {
                            size.width() - size.thumb_padding() - size.thumb_size()
                        } else {
                            0.0
                        };
                        thumb.target_x = new_x;
                        node.left = Val::Px(new_x); // Set position immediately
                    }
                }
            }

            // Send change event
            events.write(SwitchChangeEvent {
                switch_entity: entity,
                checked: switch.checked,
                size: switch.size,
            });
        }
    }
}

// System to update visual styling when switch state changes
pub fn update_switch_styling(
    switches_query: Query<(Entity, &SwitchComponent), Changed<SwitchComponent>>,
    mut bg_colors: Query<&mut BackgroundColor, (With<SwitchComponent>, Without<SwitchThumb>)>,
    children_query: Query<&Children>,
    mut thumb_bg_query: Query<&mut BackgroundColor, (With<SwitchThumb>, Without<SwitchComponent>)>,
) {
    for (entity, switch) in &switches_query {
        let styling = switch.get_styling(switch.current_state);

        // Update switch background color
        if let Ok(mut bg_color) = bg_colors.get_mut(entity) {
            *bg_color = styling.background_color;
        }

        // Update thumb color
        if let Ok(children) = children_query.get(entity) {
            for child in children.iter() {
                if let Ok(mut thumb_bg) = thumb_bg_query.get_mut(child) {
                    *thumb_bg = BackgroundColor(styling.thumb_color);
                }
            }
        }
    }
}

pub type Switch = SwitchComponent;