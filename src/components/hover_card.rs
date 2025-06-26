use crate::{
    components::text::{Text, TextBuilder},
    theme::{
        color::{accent_palette, TextColor as TextColorEnum, TextContrastLevel, UiColorPalette},
        layout::UiLayout,
        typography::{FontFamily, TextSize, TextWeight},
    },
    utilities::{portal::Portal, ui_root::UIRoot, ComponentBuilder},
};
use bevy::{ecs::spawn::SpawnWith, input::keyboard::KeyCode, prelude::*};
use bevy_picking::prelude::Pickable;
use std::time::Duration;

// Events
#[derive(Event, Debug, Clone)]
pub struct HoverCardOpenEvent {
    pub hover_card: Entity,
}

#[derive(Event, Debug, Clone)]
pub struct HoverCardCloseEvent {
    pub hover_card: Entity,
}

// Enumerations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HoverCardSide {
    Top,
    Right,
    Bottom,
    Left,
}

impl Default for HoverCardSide {
    fn default() -> Self {
        Self::Bottom
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HoverCardAlign {
    Start,
    Center,
    End,
}

impl Default for HoverCardAlign {
    fn default() -> Self {
        Self::Center
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HoverCardSticky {
    Partial,
    Always,
}

impl Default for HoverCardSticky {
    fn default() -> Self {
        Self::Partial
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HoverCardState {
    Closed,
    Opening,
    Open,
    Closing,
}

impl Default for HoverCardState {
    fn default() -> Self {
        Self::Closed
    }
}

// Timer component for managing delays
#[derive(Component, Debug)]
pub struct HoverCardTimer {
    pub timer: Timer,
    pub action: HoverCardTimerAction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HoverCardTimerAction {
    Open,
    Close,
}

// Core Components
#[derive(Component, Debug, Clone)]
pub struct HoverCard {
    pub open_delay: Duration,
    pub close_delay: Duration,
    pub controlled: bool,
    pub open: bool,
    pub state: HoverCardState,
}

impl Default for HoverCard {
    fn default() -> Self {
        Self {
            open_delay: Duration::from_millis(700),
            close_delay: Duration::from_millis(300),
            controlled: false,
            open: false,
            state: HoverCardState::Closed,
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct HoverCardTrigger {
    pub hover_card: Entity,
    pub disabled: bool,
    pub is_hovered: bool,
}

impl HoverCardTrigger {
    pub fn new(hover_card: Entity) -> Self {
        Self {
            hover_card,
            disabled: false,
            is_hovered: false,
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct HoverCardContent {
    pub hover_card: Entity,
    pub side: HoverCardSide,
    pub side_offset: f32,
    pub align: HoverCardAlign,
    pub align_offset: f32,
    pub avoid_collisions: bool,
    pub collision_boundary: Option<Entity>,
    pub collision_padding: UiRect,
    pub arrow: bool,
    pub arrow_width: f32,
    pub arrow_height: f32,
    pub sticky: HoverCardSticky,
    pub hide_when_detached: bool,
}

impl HoverCardContent {
    pub fn new(hover_card: Entity) -> Self {
        Self {
            hover_card,
            side: HoverCardSide::default(),
            side_offset: 4.0,
            align: HoverCardAlign::default(),
            align_offset: 0.0,
            avoid_collisions: true,
            collision_boundary: None,
            collision_padding: UiRect::all(Val::Px(10.0)),
            arrow: false,
            arrow_width: 10.0,
            arrow_height: 5.0,
            sticky: HoverCardSticky::default(),
            hide_when_detached: true,
        }
    }
}

#[derive(Component, Debug)]
pub struct HoverCardArrow {
    pub content: Entity,
    pub width: f32,
    pub height: f32,
}

impl HoverCardArrow {
    pub fn new(content: Entity, width: f32, height: f32) -> Self {
        Self {
            content,
            width,
            height,
        }
    }
}

// Marker component to identify hover card content that should be hidden initially
#[derive(Component, Debug)]
pub struct HoverCardContentMarker;

// Marker component for hover card managed text
#[derive(Component, Debug)]
pub struct HoverCardManagedText;

// Component for keyboard navigation support
#[derive(Component, Debug)]
pub struct HoverCardKeyboardNavigable {
    pub can_focus: bool,
    pub close_on_escape: bool,
}

impl Default for HoverCardKeyboardNavigable {
    fn default() -> Self {
        Self {
            can_focus: true,
            close_on_escape: true,
        }
    }
}

// Enhanced theme configuration for hover cards
#[derive(Debug, Clone)]
pub struct HoverCardTheme {
    pub background: Color,
    pub border: Color,
    pub text: Color,
    pub shadow_color: Color,
    pub border_width: f32,
    pub shadow_offset: Vec2,
    pub shadow_blur: f32,
}

impl HoverCardTheme {
    pub fn from_palette(palette: &UiColorPalette) -> Self {
        Self {
            background: palette.bg,
            border: palette.border,
            text: palette.text,
            shadow_color: Color::srgba(0.0, 0.0, 0.0, 0.1),
            border_width: UiLayout::default().border.xs,
            shadow_offset: Vec2::new(0.0, 2.0),
            shadow_blur: 8.0,
        }
    }

    pub fn elevated(palette: &UiColorPalette) -> Self {
        Self {
            background: palette.bg_subtle,
            border: palette.border_hover,
            text: palette.text,
            shadow_color: Color::srgba(0.0, 0.0, 0.0, 0.15),
            border_width: UiLayout::default().border.xs,
            shadow_offset: Vec2::new(0.0, 4.0),
            shadow_blur: 12.0,
        }
    }
}

// Component to track hover card relationships
#[derive(Component, Debug)]
pub struct HoverCardRelationship {
    pub root: Entity,
    pub trigger: Option<Entity>,
    pub content: Option<Entity>,
    pub arrow: Option<Entity>,
}

impl HoverCardRelationship {
    pub fn new(root: Entity) -> Self {
        Self {
            root,
            trigger: None,
            content: None,
            arrow: None,
        }
    }
}

// Builder Implementations
pub struct HoverCardBuilder {
    name: String,
    hover_card: HoverCard,
}

impl HoverCardBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: format!("{}_HoverCard", name.into()),
            hover_card: HoverCard::default(),
        }
    }

    pub fn open_delay(mut self, delay: Duration) -> Self {
        self.hover_card.open_delay = delay;
        self
    }

    pub fn close_delay(mut self, delay: Duration) -> Self {
        self.hover_card.close_delay = delay;
        self
    }

    pub fn controlled(mut self) -> Self {
        self.hover_card.controlled = true;
        self
    }

    pub fn default_open(mut self, open: bool) -> Self {
        self.hover_card.open = open;
        self.hover_card.state = if open {
            HoverCardState::Open
        } else {
            HoverCardState::Closed
        };
        self
    }

    pub fn build(self) -> impl Bundle {
        (
            Name::new(self.name),
            self.hover_card,
            HoverCardRelationship::new(Entity::PLACEHOLDER), // Will be set by spawning system
            Visibility::Inherited,
        )
    }
}

pub struct HoverCardTriggerBuilder {
    name: String,
    trigger: HoverCardTrigger,
    text: Option<String>,
    text_builder: Option<TextBuilder>,
}

impl HoverCardTriggerBuilder {
    pub fn new(name: impl Into<String>, hover_card: Entity) -> Self {
        Self {
            name: format!("{}_HoverCardTrigger", name.into()),
            trigger: HoverCardTrigger::new(hover_card),
            text: None,
            text_builder: None,
        }
    }

    pub fn disabled(mut self) -> Self {
        self.trigger.disabled = true;
        self
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    // Text styling methods similar to Button
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

    pub fn text_color(mut self, color: TextColorEnum) -> Self {
        let text_content = self.text.clone().unwrap_or_default();
        let builder = self
            .text_builder
            .take()
            .unwrap_or_else(|| Text::new(text_content));
        self.text_builder = Some(builder.color(color));
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

    pub fn build(self) -> impl Bundle {
        let display_text = self.text.clone().unwrap_or_default();
        let layout = UiLayout::default();

        (
            Name::new(self.name),
            self.trigger,
            Node {
                padding: UiRect::all(Val::Px(layout.padding.xs)),
                ..default()
            },
            Pickable::default(),
            Interaction::default(),
            Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
                if let Some(text_builder) = self.text_builder {
                    parent.spawn((text_builder.build(), HoverCardManagedText));
                } else if !display_text.is_empty() {
                    parent.spawn((Text::body(display_text).build(), HoverCardManagedText));
                }
            })),
        )
    }
}

pub struct HoverCardContentBuilder {
    name: String,
    content: HoverCardContent,
    theme: UiColorPalette,
    custom_theme: Option<HoverCardTheme>,
    radius: f32,
    padding: f32,
    children: Vec<Entity>,
    keyboard_navigable: bool,
}

impl HoverCardContentBuilder {
    pub fn new(name: impl Into<String>, hover_card: Entity) -> Self {
        Self {
            name: format!("{}_HoverCardContent", name.into()),
            content: HoverCardContent::new(hover_card),
            theme: accent_palette(),
            custom_theme: None,
            radius: UiLayout::default().radius.base,
            padding: UiLayout::default().padding.base,
            children: Vec::new(),
            keyboard_navigable: true,
        }
    }

    pub fn side(mut self, side: HoverCardSide) -> Self {
        self.content.side = side;
        self
    }

    pub fn side_offset(mut self, offset: f32) -> Self {
        self.content.side_offset = offset;
        self
    }

    pub fn align(mut self, align: HoverCardAlign) -> Self {
        self.content.align = align;
        self
    }

    pub fn align_offset(mut self, offset: f32) -> Self {
        self.content.align_offset = offset;
        self
    }

    pub fn avoid_collisions(mut self, avoid: bool) -> Self {
        self.content.avoid_collisions = avoid;
        self
    }

    pub fn collision_boundary(mut self, entity: Entity) -> Self {
        self.content.collision_boundary = Some(entity);
        self
    }

    pub fn collision_padding(mut self, padding: UiRect) -> Self {
        self.content.collision_padding = padding;
        self
    }

    pub fn arrow(mut self) -> Self {
        self.content.arrow = true;
        self
    }

    pub fn arrow_size(mut self, width: f32, height: f32) -> Self {
        self.content.arrow_width = width;
        self.content.arrow_height = height;
        self
    }

    pub fn sticky(mut self, sticky: HoverCardSticky) -> Self {
        self.content.sticky = sticky;
        self
    }

    pub fn hide_when_detached(mut self, hide: bool) -> Self {
        self.content.hide_when_detached = hide;
        self
    }

    pub fn theme(mut self, theme: UiColorPalette) -> Self {
        self.theme = theme;
        self
    }

    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }

    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    pub fn child(mut self, entity: Entity) -> Self {
        self.children.push(entity);
        self
    }

    pub fn custom_theme(mut self, theme: HoverCardTheme) -> Self {
        self.custom_theme = Some(theme);
        self
    }

    pub fn elevated_theme(mut self) -> Self {
        self.custom_theme = Some(HoverCardTheme::elevated(&self.theme));
        self
    }

    pub fn keyboard_navigable(mut self, navigable: bool) -> Self {
        self.keyboard_navigable = navigable;
        self
    }

    pub fn build(self) -> impl Bundle {
        let layout = UiLayout::default();
        let effective_theme = self
            .custom_theme
            .unwrap_or_else(|| HoverCardTheme::from_palette(&self.theme));

        let mut bundle = (
            Name::new(self.name),
            self.content,
            HoverCardContentMarker,
            Node {
                position_type: PositionType::Absolute,
                padding: UiRect::all(Val::Px(self.padding)),
                border: UiRect::all(Val::Px(effective_theme.border_width)),
                max_width: Val::Px(300.0), // Default max width
                ..default()
            },
            BorderRadius::all(Val::Px(self.radius)),
            BackgroundColor(effective_theme.background),
            BorderColor(effective_theme.border),
            Visibility::Hidden, // Start hidden
            ZIndex(1000),       // Ensure it renders above other content
        );

        // Add keyboard navigation components if enabled
        if self.keyboard_navigable {
            // Use a simpler approach for now
            bundle
        } else {
            bundle
        }
    }
}

impl Default for HoverCardBuilder {
    fn default() -> Self {
        Self::new("HoverCard")
    }
}

// System Implementations

/// System to handle hover card trigger interactions and timing
pub fn hover_card_interaction_system(
    mut commands: Commands,
    time: Res<Time>,
    mut trigger_query: Query<(Entity, &mut HoverCardTrigger, &Interaction), Changed<Interaction>>,
    mut hover_card_query: Query<&mut HoverCard>,
    mut timer_query: Query<(Entity, &mut HoverCardTimer)>,
) {
    // Handle trigger interaction changes
    for (_trigger_entity, mut trigger, interaction) in trigger_query.iter_mut() {
        if trigger.disabled {
            continue;
        }

        let hover_card_entity = trigger.hover_card;
        let Ok(hover_card) = hover_card_query.get_mut(hover_card_entity) else {
            continue;
        };

        match interaction {
            Interaction::Hovered => {
                if !trigger.is_hovered {
                    trigger.is_hovered = true;

                    // Cancel any existing close timer
                    for (timer_entity, timer) in timer_query.iter() {
                        if matches!(timer.action, HoverCardTimerAction::Close) {
                            commands.entity(timer_entity).despawn();
                        }
                    }

                    // Start open timer if not controlled and not already open
                    if !hover_card.controlled && hover_card.state == HoverCardState::Closed {
                        let mut timer = Timer::new(hover_card.open_delay, TimerMode::Once);
                        timer.tick(Duration::ZERO);

                        commands.spawn(HoverCardTimer {
                            timer,
                            action: HoverCardTimerAction::Open,
                        });
                    }
                }
            }
            Interaction::None => {
                if trigger.is_hovered {
                    trigger.is_hovered = false;

                    // Cancel any existing open timer
                    for (timer_entity, timer) in timer_query.iter() {
                        if matches!(timer.action, HoverCardTimerAction::Open) {
                            commands.entity(timer_entity).despawn();
                        }
                    }

                    // Start close timer if not controlled and currently open
                    if !hover_card.controlled && hover_card.state == HoverCardState::Open {
                        let mut timer = Timer::new(hover_card.close_delay, TimerMode::Once);
                        timer.tick(Duration::ZERO);

                        commands.spawn(HoverCardTimer {
                            timer,
                            action: HoverCardTimerAction::Close,
                        });
                    }
                }
            }
            Interaction::Pressed => {
                // Optional: Handle click interactions if needed
            }
        }
    }

    // Update timers
    for (timer_entity, mut hover_timer) in timer_query.iter_mut() {
        hover_timer.timer.tick(time.delta());

        if hover_timer.timer.finished() {
            match hover_timer.action {
                HoverCardTimerAction::Open => {
                    // Find the associated hover card and open it
                    // This is a simplified approach - in a full implementation,
                    // we'd need to track which timer belongs to which hover card
                    for mut hover_card in hover_card_query.iter_mut() {
                        if hover_card.state == HoverCardState::Closed {
                            hover_card.state = HoverCardState::Opening;
                            break;
                        }
                    }
                }
                HoverCardTimerAction::Close => {
                    // Find the associated hover card and close it
                    for mut hover_card in hover_card_query.iter_mut() {
                        if hover_card.state == HoverCardState::Open {
                            hover_card.state = HoverCardState::Closing;
                            break;
                        }
                    }
                }
            }

            // Remove the finished timer
            commands.entity(timer_entity).despawn();
        }
    }
}

/// System to manage hover card state transitions and visibility
pub fn hover_card_state_system(
    _commands: Commands,
    mut hover_card_query: Query<(Entity, &mut HoverCard), Changed<HoverCard>>,
    content_query: Query<Entity, With<HoverCardContent>>,
    mut visibility_query: Query<&mut Visibility>,
    mut open_events: EventWriter<HoverCardOpenEvent>,
    mut close_events: EventWriter<HoverCardCloseEvent>,
) {
    for (hover_card_entity, mut hover_card) in hover_card_query.iter_mut() {
        match hover_card.state {
            HoverCardState::Opening => {
                hover_card.open = true;
                hover_card.state = HoverCardState::Open;

                // Show content
                for content_entity in content_query.iter() {
                    if let Ok(mut visibility) = visibility_query.get_mut(content_entity) {
                        *visibility = Visibility::Inherited;
                    }
                }

                // Emit open event
                open_events.write(HoverCardOpenEvent {
                    hover_card: hover_card_entity,
                });
            }
            HoverCardState::Closing => {
                hover_card.open = false;
                hover_card.state = HoverCardState::Closed;

                // Hide content
                for content_entity in content_query.iter() {
                    if let Ok(mut visibility) = visibility_query.get_mut(content_entity) {
                        *visibility = Visibility::Hidden;
                    }
                }

                // Emit close event
                close_events.write(HoverCardCloseEvent {
                    hover_card: hover_card_entity,
                });
            }
            _ => {}
        }
    }
}

/// System to position hover card content relative to triggers
pub fn hover_card_positioning_system(
    mut content_query: Query<(Entity, &HoverCardContent, &mut Node), With<HoverCardContentMarker>>,
    trigger_query: Query<(Entity, &HoverCardTrigger, &GlobalTransform), (With<HoverCardTrigger>, Without<HoverCardContentMarker>)>,
    node_query: Query<&Node, Without<HoverCardContentMarker>>,
    hover_card_query: Query<&HoverCard>,
    hover_card_changed: Query<Entity, (With<HoverCard>, Changed<HoverCard>)>,
    window_query: Query<&Window>,
) {
    let Ok(window) = window_query.single() else {
        return;
    };

    // Check if any hover card state changed
    let has_state_changes = hover_card_changed.iter().any(|_| true);
    
    if !has_state_changes {
        return; // No positioning updates needed
    }

    for (_content_entity, content, mut content_node) in content_query.iter_mut() {
        // Get hover card state
        let Ok(hover_card) = hover_card_query.get(content.hover_card) else {
            continue;
        };

        // Update position for open or opening hover cards
        if hover_card.state == HoverCardState::Open || hover_card.state == HoverCardState::Opening {
            // Find the matching trigger for this content
            let trigger_data = trigger_query
                .iter()
                .find(|(_, trigger, _)| trigger.hover_card == content.hover_card);

            if let Some((trigger_entity, _, trigger_transform)) = trigger_data {
                // Get trigger position and size
                let trigger_pos = trigger_transform.translation();
                let trigger_size = if let Ok(trigger_node) = node_query.get(trigger_entity) {
                    extract_node_size(trigger_node)
                } else {
                    Vec2::new(100.0, 40.0) // Fallback size
                };

                // Calculate content position based on trigger position and side preference
                let (offset_x, offset_y) = calculate_content_position_improved(
                    trigger_pos,
                    trigger_size,
                    content.side,
                    content.side_offset,
                    content.align,
                    content.align_offset,
                );

                // Apply collision detection if enabled
                let (final_x, final_y) = if content.avoid_collisions {
                    apply_collision_detection(
                        offset_x,
                        offset_y,
                        window.width(),
                        window.height(),
                        content.collision_padding,
                    )
                } else {
                    (offset_x, offset_y)
                };

                // Update position - ensure it's positioned relative to the window
                content_node.left = Val::Px(final_x);
                content_node.top = Val::Px(final_y);
                
                // Also ensure the content uses absolute positioning
                content_node.position_type = PositionType::Absolute;
                
                info!(
                    "Positioning hover card: trigger_pos={:?}, trigger_size={:?}, side={:?}, final_pos=({}, {})",
                    trigger_pos, trigger_size, content.side, final_x, final_y
                );
            }
        }
    }
}

/// Extract size information from a Node
fn extract_node_size(node: &Node) -> Vec2 {
    let width = match node.width {
        Val::Px(w) => w,
        Val::Percent(p) => p * 8.0, // Rough estimate
        _ => 100.0, // Default fallback
    };
    
    let height = match node.height {
        Val::Px(h) => h,
        Val::Percent(p) => p * 6.0, // Rough estimate  
        _ => 40.0, // Default fallback
    };
    
    Vec2::new(width, height)
}

/// Calculate the initial position for hover card content (legacy)
fn calculate_content_position(
    trigger_pos: Vec3,
    side: HoverCardSide,
    side_offset: f32,
    _align: HoverCardAlign,
    align_offset: f32,
) -> (f32, f32) {
    let base_x = trigger_pos.x;
    let base_y = trigger_pos.y;

    match side {
        HoverCardSide::Top => (base_x + align_offset, base_y - side_offset),
        HoverCardSide::Bottom => (base_x + align_offset, base_y + side_offset),
        HoverCardSide::Left => (base_x - side_offset, base_y + align_offset),
        HoverCardSide::Right => (base_x + side_offset, base_y + align_offset),
    }
}

/// Calculate the improved position for hover card content with proper trigger size consideration
fn calculate_content_position_improved(
    trigger_pos: Vec3,
    trigger_size: Vec2,
    side: HoverCardSide,
    side_offset: f32,
    align: HoverCardAlign,
    align_offset: f32,
) -> (f32, f32) {
    // Convert Bevy world coordinates to UI coordinates
    // Bevy UI has origin at top-left, Y increases downward
    let base_x = trigger_pos.x;
    let base_y = trigger_pos.y;
    
    let half_width = trigger_size.x / 2.0;
    let half_height = trigger_size.y / 2.0;

    let (mut content_x, mut content_y) = match side {
        HoverCardSide::Top => {
            // Position above the trigger
            (base_x, base_y - half_height - side_offset)
        }
        HoverCardSide::Bottom => {
            // Position below the trigger  
            (base_x, base_y + half_height + side_offset)
        }
        HoverCardSide::Left => {
            // Position to the left of the trigger
            (base_x - half_width - side_offset, base_y)
        }
        HoverCardSide::Right => {
            // Position to the right of the trigger
            (base_x + half_width + side_offset, base_y)
        }
    };

    // Apply alignment
    match side {
        HoverCardSide::Top | HoverCardSide::Bottom => {
            // For top/bottom positioning, align horizontally
            match align {
                HoverCardAlign::Start => content_x = base_x - half_width + align_offset,
                HoverCardAlign::Center => content_x = base_x + align_offset,
                HoverCardAlign::End => content_x = base_x + half_width + align_offset,
            }
        }
        HoverCardSide::Left | HoverCardSide::Right => {
            // For left/right positioning, align vertically
            match align {
                HoverCardAlign::Start => content_y = base_y - half_height + align_offset,
                HoverCardAlign::Center => content_y = base_y + align_offset,
                HoverCardAlign::End => content_y = base_y + half_height + align_offset,
            }
        }
    }

    (content_x, content_y)
}

/// Apply collision detection to prevent content from going off-screen
fn apply_collision_detection(
    x: f32,
    y: f32,
    window_width: f32,
    window_height: f32,
    padding: UiRect,
) -> (f32, f32) {
    let padding_left = match padding.left {
        Val::Px(p) => p,
        _ => 10.0,
    };
    let padding_right = match padding.right {
        Val::Px(p) => p,
        _ => 10.0,
    };
    let padding_top = match padding.top {
        Val::Px(p) => p,
        _ => 10.0,
    };
    let padding_bottom = match padding.bottom {
        Val::Px(p) => p,
        _ => 10.0,
    };

    let final_x = x.clamp(padding_left, window_width - padding_right);
    let final_y = y.clamp(padding_top, window_height - padding_bottom);

    (final_x, final_y)
}

/// System to integrate hover card content with the portal system
pub fn hover_card_portal_system(
    mut commands: Commands,
    content_query: Query<Entity, (With<HoverCardContent>, Added<HoverCardContentMarker>)>,
    ui_root_query: Query<Entity, With<UIRoot>>,
) {
    for content_entity in content_query.iter() {
        // Find or create a UI root for portals
        if let Ok(_ui_root_entity) = ui_root_query.single() {
            // Move the hover card content to the portal layer
            commands.entity(content_entity).insert(Portal {
                container: Some("HoverCard_UIRoot".to_string()),
            });
        } else {
            // Create a UI root if none exists
            let _ui_root = commands
                .spawn((
                    Name::new("HoverCard_UIRoot"),
                    UIRoot {
                        name: "HoverCard_UIRoot".to_string(),
                    },
                    Node {
                        position_type: PositionType::Absolute,
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    ZIndex(999), // Below hover cards but above normal UI
                ))
                .id();

            commands.entity(content_entity).insert(Portal {
                container: Some("HoverCard_UIRoot".to_string()),
            });
        }
    }
}

/// System to handle keyboard navigation for hover cards
pub fn hover_card_keyboard_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut hover_card_query: Query<&mut HoverCard>,
    content_query: Query<(&HoverCardContent, &HoverCardKeyboardNavigable)>,
    trigger_query: Query<(&HoverCardTrigger, &Interaction), Changed<Interaction>>,
) {
    // Handle Escape key to close hover cards
    if keyboard_input.just_pressed(KeyCode::Escape) {
        for mut hover_card in hover_card_query.iter_mut() {
            if hover_card.state == HoverCardState::Open {
                hover_card.state = HoverCardState::Closing;
            }
        }
    }

    // Handle Tab navigation to open hover cards on focus
    for (trigger, interaction) in trigger_query.iter() {
        if *interaction == Interaction::None {
            continue;
        }

        let hover_card_entity = trigger.hover_card;
        if let Ok(mut hover_card) = hover_card_query.get_mut(hover_card_entity) {
            // Check if this hover card has keyboard navigation enabled
            let has_keyboard_nav = content_query
                .iter()
                .any(|(content, nav)| content.hover_card == hover_card_entity && nav.can_focus);

            if has_keyboard_nav && *interaction == Interaction::Hovered {
                if hover_card.state == HoverCardState::Closed {
                    hover_card.state = HoverCardState::Opening;
                }
            }
        }
    }
}

/// System to add smooth animations for hover card transitions
pub fn hover_card_animation_system(
    time: Res<Time>,
    mut content_query: Query<
        (&HoverCardContent, &mut Transform, &mut BackgroundColor),
        With<HoverCardContentMarker>,
    >,
    hover_card_query: Query<&HoverCard>,
) {
    for (content, mut transform, mut bg_color) in content_query.iter_mut() {
        if let Ok(hover_card) = hover_card_query.get(content.hover_card) {
            match hover_card.state {
                HoverCardState::Opening => {
                    // Animate scale and opacity on open
                    let scale = 0.95 + 0.05 * time.elapsed_secs().sin().abs();
                    transform.scale = Vec3::splat(scale);

                    // Fade in effect
                    if let Color::Srgba(srgba) = bg_color.0 {
                        let alpha = (time.elapsed_secs() * 4.0).min(1.0);
                        *bg_color = BackgroundColor(Color::srgba(
                            srgba.red,
                            srgba.green,
                            srgba.blue,
                            alpha,
                        ));
                    }
                }
                HoverCardState::Open => {
                    // Stable state
                    transform.scale = Vec3::ONE;
                    if let Color::Srgba(srgba) = bg_color.0 {
                        *bg_color =
                            BackgroundColor(Color::srgba(srgba.red, srgba.green, srgba.blue, 1.0));
                    }
                }
                HoverCardState::Closing => {
                    // Animate scale down and fade out
                    let scale = 0.95;
                    transform.scale = Vec3::splat(scale);

                    if let Color::Srgba(srgba) = bg_color.0 {
                        let alpha = (1.0 - time.elapsed_secs() * 6.0).max(0.0);
                        *bg_color = BackgroundColor(Color::srgba(
                            srgba.red,
                            srgba.green,
                            srgba.blue,
                            alpha,
                        ));
                    }
                }
                _ => {}
            }
        }
    }
}
