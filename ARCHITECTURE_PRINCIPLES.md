# Bevy UI Architecture Principles - Deep Analysis

*Basiert auf der Analyse von Bevy 0.17 Core Widgets, bevy_core_widgets und bevy_editor_prototypes*

## Überblick

Dieses Dokument dokumentiert die fundamentalen Architektur-Prinzipien, die sich in der modernen Bevy UI-Entwicklung etabliert haben. Die Analyse von drei Hauptprojekten zeigt eine konsistente Architektur-Philosophie, die als Referenz für robuste UI-Systeme dient.

## 1. ECS-First Architecture Principle

### Das Zentrale Paradigma

Alle modernen Bevy UI-Systeme implementieren eine **ECS-First** Architektur, die Bevy's Entity-Component-System als primäres Organisationsprinzip nutzt:

```rust
// Widget State als Component
#[derive(Component)]
pub struct ButtonState {
    pub pressed: bool,
    pub hovered: bool,
    pub disabled: bool,
}

// Interaktion als Event-driven System
fn button_interaction_system(
    mut query: Query<&mut ButtonState, With<CoreButton>>,
    mut events: EventReader<Pointer<Click>>,
) {
    // Reaktive State-Updates basierend auf Events
    for event in events.read() {
        if let Ok(mut state) = query.get_mut(event.target) {
            state.pressed = true;
        }
    }
}
```

### Architektur-Vorteile

- **Daten-orientiert**: Components sind reine Datenstrukturen ohne Verhalten
- **Reaktiv**: Systems reagieren automatisch auf Component-Änderungen
- **Composable**: Komponenten können beliebig kombiniert werden
- **Performance**: ECS-optimierte Query-Performance durch Cache-Locality

### Design-Pattern Implementation

```rust
// Component Composition Pattern
#[derive(Bundle)]
pub struct InteractiveWidgetBundle {
    pub widget: Widget,
    pub interaction_state: InteractionState,
    pub accessibility: AccessibilityNode,
    pub focus_policy: FocusPolicy,
    pub style: Style,
}

// System Composition Pattern
fn widget_update_systems() -> SystemSet {
    SystemSet::new()
        .with_system(update_interaction_state)
        .with_system(update_accessibility)
        .with_system(update_visual_feedback)
        .with_system(handle_keyboard_navigation)
}
```

## 2. Observer-Pattern Evolution (Bevy 0.16+)

### Moderne Event-Architektur

Die Einführung des Observer-Patterns in Bevy 0.16 revolutioniert die Event-Behandlung:

```rust
// Alte Event-Architektur (< 0.16)
fn old_button_system(
    mut events: EventReader<ButtonClicked>,
    mut query: Query<&mut SomeComponent>,
) {
    for event in events.read() {
        // Global event handling - ineffizient
        for mut component in query.iter_mut() {
            // Alle Entities müssen geprüft werden
        }
    }
}

// Neue Observer-Architektur (0.16+)
commands.spawn((
    ButtonBundle::default(),
    Observer::new(|trigger: Trigger<Pointer<Click>>, mut commands: Commands| {
        // Entity-spezifische Event-Behandlung
        let entity = trigger.entity();
        commands.entity(entity).insert(ButtonPressed);
    }),
));
```

### Observer-Pattern Implementierung

```rust
// Multi-Observer Pattern
commands.spawn((
    ButtonBundle::default(),
    Observer::new(handle_button_click),
    Observer::new(handle_button_hover),
    Observer::new(handle_button_focus),
));

// Observer mit State Access
fn handle_button_click(
    trigger: Trigger<Pointer<Click>>,
    mut commands: Commands,
    mut query: Query<&mut ButtonState>,
) {
    let entity = trigger.entity();
    if let Ok(mut state) = query.get_mut(entity) {
        state.clicked = true;
        commands.trigger_targets(ButtonClicked, entity);
    }
}
```

### Vorteile der Observer-Architektur

- **Lokalisiert**: Events sind an spezifische Entities gebunden
- **Typsicher**: Compile-time Event-Type-Checking
- **Performance**: Keine globale Event-Queue-Iteration
- **Komposition**: Multiple Observer pro Entity möglich
- **Maintainability**: Event-Logic nahe am Entity-Definition

## 3. Headless Widget Principle

### Konzeptuelle Trennung

Das Headless Widget Principle trennt strikt Logik von Präsentation:

```rust
// Logik-Layer (Headless)
#[derive(Component)]
pub struct CoreSlider {
    pub min: f32,
    pub max: f32,
    pub value: f32,
    pub step: f32,
    pub on_change: Option<SystemId>,
    // Keine visuellen Eigenschaften!
}

// Presentation-Layer (Separat)
#[derive(Component)]
pub struct SliderVisuals {
    pub track_color: Color,
    pub thumb_color: Color,
    pub track_height: f32,
    pub thumb_radius: f32,
    // Nur visuelle Properties
}

// Behavior-Layer
#[derive(Component)]
pub struct SliderBehavior {
    pub drag_sensitivity: f32,
    pub keyboard_increment: f32,
    pub smooth_animation: bool,
}
```

### Headless Implementation Patterns

```rust
// Generic Headless Widget Trait
pub trait HeadlessWidget {
    type State: Component;
    type Config: Component;
    type Events: Event;
    
    fn create_logic_bundle() -> impl Bundle;
    fn update_system() -> impl System;
}

// Concrete Implementation
impl HeadlessWidget for CoreSlider {
    type State = SliderState;
    type Config = SliderConfig;
    type Events = SliderValueChanged;
    
    fn create_logic_bundle() -> impl Bundle {
        (
            Self::default(),
            SliderState::default(),
            SliderConfig::default(),
            Observer::new(handle_slider_interaction),
        )
    }
}
```

### Headless Architecture Benefits

- **Flexibilität**: Beliebige visuelle Implementierungen
- **Testbarkeit**: Logik ohne Rendering testbar
- **Wiederverwendbarkeit**: Logik plattformübergreifend nutzbar
- **Maintainability**: Klare Separation of Concerns
- **Themability**: Visuelle Styles komplett austauschbar

## 4. Plugin-basierte Modularität

### Granulare Plugin-Architektur

```rust
// Micro-Plugin Ansatz - Ein Plugin pro Widget
pub struct CoreButtonPlugin;
pub struct CoreSliderPlugin;
pub struct CoreCheckboxPlugin;

// Aggregation-Plugin für convenience
pub struct CoreWidgetsPlugin;

impl Plugin for CoreWidgetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            CoreButtonPlugin,
            CoreSliderPlugin,
            CoreCheckboxPlugin,
            // Selektive Feature-Aktivierung möglich
        ));
    }
}
```

### Plugin-Design-Patterns

```rust
impl Plugin for CoreButtonPlugin {
    fn build(&self, app: &mut App) {
        app
            // Component Registration
            .register_type::<CoreButton>()
            .register_type::<ButtonState>()
            // Event Registration
            .add_event::<ButtonClicked>()
            .add_event::<ButtonPressed>()
            // System Registration
            .add_systems(Update, (
                button_interaction_system,
                button_state_system,
                button_accessibility_system,
            ).chain()) // Execution order matters
            // Observer Registration
            .observe(on_button_pressed::<CoreButton>)
            .observe(on_button_released::<CoreButton>)
            // Resource Initialization
            .init_resource::<ButtonDefaults>()
            // Startup Systems
            .add_systems(Startup, setup_button_resources);
    }
}
```

### Advanced Plugin Patterns

```rust
// Conditional Plugin Loading
pub struct ConditionalWidgetsPlugin {
    pub enable_advanced_widgets: bool,
    pub enable_experimental_features: bool,
}

impl Plugin for ConditionalWidgetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CoreWidgetsPlugin);
        
        if self.enable_advanced_widgets {
            app.add_plugins(AdvancedWidgetsPlugin);
        }
        
        if self.enable_experimental_features {
            app.add_plugins(ExperimentalWidgetsPlugin);
        }
    }
}

// Plugin Dependencies
pub struct DependentWidgetPlugin;

impl Plugin for DependentWidgetPlugin {
    fn build(&self, app: &mut App) {
        // Ensure dependencies are loaded
        if !app.is_plugin_added::<CoreWidgetsPlugin>() {
            app.add_plugins(CoreWidgetsPlugin);
        }
        
        // Add dependent functionality
        app.add_systems(Update, dependent_widget_system);
    }
}
```

## 5. State Management Principles

### Resource-basierte Global State

```rust
#[derive(Resource, Reflect, Default)]
pub struct UIGlobalState {
    pub focused_entity: Option<Entity>,
    pub modal_stack: Vec<Entity>,
    pub hover_entity: Option<Entity>,
    pub drag_state: Option<DragState>,
    pub keyboard_navigation_enabled: bool,
}

// Automatic State Synchronization
fn sync_ui_state_system(
    mut global_state: ResMut<UIGlobalState>,
    focused_query: Query<Entity, With<Focused>>,
    hovered_query: Query<Entity, With<Hovered>>,
    modal_query: Query<Entity, With<Modal>>,
) {
    global_state.focused_entity = focused_query.get_single().ok();
    global_state.hover_entity = hovered_query.get_single().ok();
    global_state.modal_stack = modal_query.iter().collect();
}
```

### Component-basierte Local State

```rust
#[derive(Component, Reflect)]
pub struct InteractionState {
    pub hovered: bool,
    pub pressed: bool,
    pub focused: bool,
    pub disabled: bool,
    pub transition_timer: Timer,
}

// State-Composition Pattern
#[derive(Bundle)]
pub struct InteractiveWidgetBundle {
    pub interaction_state: InteractionState,
    pub accessibility: AccessibilityNode,
    pub focus_policy: FocusPolicy,
    pub cursor_icon: CursorIcon,
}
```

### State Management Patterns

```rust
// State Machine Pattern
#[derive(Component)]
pub enum WidgetState {
    Idle,
    Hovered { since: Duration },
    Pressed { position: Vec2 },
    Focused { via_keyboard: bool },
    Disabled,
}

impl WidgetState {
    pub fn transition_to(&mut self, new_state: Self) -> bool {
        match (self, &new_state) {
            (Self::Disabled, _) => false, // Disabled widgets don't transition
            (Self::Pressed { .. }, Self::Idle) => {
                *self = new_state;
                true
            }
            _ => {
                *self = new_state;
                true
            }
        }
    }
}
```

## 6. Accessibility-First Architecture

### AccessKit Integration Pattern

```rust
impl From<&CoreButton> for AccessKitNode {
    fn from(button: &CoreButton) -> Self {
        AccessKitNode {
            role: AccessKitRole::Button,
            name: button.label.clone(),
            description: button.description.clone(),
            // Automatic ARIA attribute mapping
            default_action_verb: ActionVerb::Click,
            states: if button.disabled {
                AccessKitStateSet::DISABLED
            } else {
                AccessKitStateSet::empty()
            },
            // Keyboard navigation support
            supports_text_input: false,
            clickable: !button.disabled,
        }
    }
}

// Automatic Accessibility Sync
fn update_accessibility_system(
    query: Query<(Entity, &CoreButton, &InteractionState), Changed<InteractionState>>,
    mut accessibility_query: Query<&mut AccessibilityNode>,
) {
    for (entity, button, state) in query.iter() {
        if let Ok(mut node) = accessibility_query.get_mut(entity) {
            node.states = AccessKitStateSet::from(state);
            if state.focused {
                node.states |= AccessKitStateSet::FOCUSED;
            }
        }
    }
}
```

### Screen Reader Support Patterns

```rust
#[derive(Component)]
pub struct ScreenReaderText {
    pub text: String,
    pub announce_changes: bool,
    pub live_region_type: LiveRegionType,
}

pub enum LiveRegionType {
    Off,
    Polite,
    Assertive,
}

// Live Region Updates
fn announce_changes_system(
    query: Query<&ScreenReaderText, Changed<ScreenReaderText>>,
    mut accessibility_events: EventWriter<AccessibilityAnnouncement>,
) {
    for text in query.iter() {
        if text.announce_changes {
            accessibility_events.send(AccessibilityAnnouncement {
                text: text.text.clone(),
                priority: match text.live_region_type {
                    LiveRegionType::Assertive => AnnouncementPriority::High,
                    LiveRegionType::Polite => AnnouncementPriority::Normal,
                    LiveRegionType::Off => continue,
                },
            });
        }
    }
}
```

### Keyboard Navigation Architecture

```rust
#[derive(Resource)]
pub struct KeyboardNavigationState {
    pub current_focus: Option<Entity>,
    pub focus_history: Vec<Entity>,
    pub navigation_mode: NavigationMode,
}

pub enum NavigationMode {
    Mouse,
    Keyboard,
    Mixed,
}

fn keyboard_navigation_system(
    mut nav_state: ResMut<KeyboardNavigationState>,
    keyboard_input: Res<Input<KeyCode>>,
    focusable_query: Query<Entity, With<Focusable>>,
    mut commands: Commands,
) {
    if keyboard_input.just_pressed(KeyCode::Tab) {
        let next_entity = if keyboard_input.pressed(KeyCode::ShiftLeft) {
            find_previous_focusable(&focusable_query, nav_state.current_focus)
        } else {
            find_next_focusable(&focusable_query, nav_state.current_focus)
        };
        
        if let Some(entity) = next_entity {
            commands.entity(entity).insert(Focused);
            nav_state.current_focus = Some(entity);
            nav_state.navigation_mode = NavigationMode::Keyboard;
        }
    }
}
```

## 7. Theme System Architecture

### OKLCH-basierte Farbtheorie

```rust
#[derive(Resource)]
pub struct Theme {
    // Perceptually uniform color space
    pub primary: OklchColor,
    pub secondary: OklchColor,
    pub accent: OklchColor,
    
    // Semantic color mapping
    pub success: OklchColor,
    pub warning: OklchColor,
    pub error: OklchColor,
    pub info: OklchColor,
    
    // Surface colors
    pub background: OklchColor,
    pub surface: OklchColor,
    pub surface_variant: OklchColor,
    
    // Text colors
    pub on_primary: OklchColor,
    pub on_surface: OklchColor,
    pub on_surface_variant: OklchColor,
}

// Accessibility-compliant contrast calculation
impl Theme {
    pub fn ensure_contrast_ratio(&mut self, min_ratio: f32) {
        // Automatic lightness adjustment for WCAG compliance
        if self.calculate_contrast_ratio(self.primary, self.on_primary) < min_ratio {
            self.on_primary = self.adjust_lightness_for_contrast(
                self.on_primary, 
                self.primary, 
                min_ratio
            );
        }
    }
    
    fn calculate_contrast_ratio(&self, color1: OklchColor, color2: OklchColor) -> f32 {
        // WCAG contrast ratio calculation in OKLCH space
        let l1 = color1.l.max(color2.l);
        let l2 = color1.l.min(color2.l);
        (l1 + 0.05) / (l2 + 0.05)
    }
}
```

### Hierarchical Theme System

```rust
pub enum ThemeLevel {
    Global,    // Application-wide defaults
    Context,   // Modal, Panel, Drawer, etc.
    Component, // Widget-specific overrides
    State,     // Hover, pressed, focused states
}

#[derive(Component)]
pub struct ThemedComponent {
    pub theme_overrides: HashMap<String, ThemeValue>,
    pub inherit_from: Option<Entity>, // Theme inheritance chain
    pub cascade_to_children: bool,
}

// Theme Inheritance System
fn apply_theme_inheritance_system(
    mut themed_query: Query<(Entity, &mut ThemedComponent, &Parent)>,
    parent_query: Query<&ThemedComponent, Without<Parent>>,
) {
    for (entity, mut themed, parent) in themed_query.iter_mut() {
        if let Some(inherit_from) = themed.inherit_from {
            if let Ok(parent_theme) = parent_query.get(inherit_from) {
                // Merge parent theme with local overrides
                themed.merge_with_parent(parent_theme);
            }
        }
    }
}
```

### Dynamic Theme Switching

```rust
#[derive(Resource)]
pub struct ThemeManager {
    pub current_theme: Handle<Theme>,
    pub available_themes: HashMap<String, Handle<Theme>>,
    pub theme_transition_duration: Duration,
}

fn switch_theme_system(
    mut theme_manager: ResMut<ThemeManager>,
    mut theme_events: EventReader<ThemeChangeRequest>,
    mut styled_query: Query<&mut BackgroundColor, With<ThemedComponent>>,
) {
    for event in theme_events.read() {
        if let Some(new_theme_handle) = theme_manager.available_themes.get(&event.theme_name) {
            theme_manager.current_theme = new_theme_handle.clone();
            
            // Animate transition to new theme
            for mut bg_color in styled_query.iter_mut() {
                // Smooth color transition using tweening
            }
        }
    }
}
```

## 8. Performance Optimization Patterns

### Selective Query Updates

```rust
// Change Detection für minimale Updates
fn efficient_hover_system(
    mut interaction_query: Query<
        (Entity, &mut InteractionState, &GlobalTransform), 
        Changed<GlobalTransform>
    >,
    cursor_position: Res<CursorPosition>,
    window_query: Query<&Window>,
) {
    // Query wird nur bei Transform-Änderungen ausgeführt
    for (entity, mut interaction, transform) in interaction_query.iter_mut() {
        let is_hovered = calculate_hover_state(transform, cursor_position);
        if interaction.hovered != is_hovered {
            interaction.hovered = is_hovered;
            // Weitere Updates nur bei tatsächlicher Änderung
        }
    }
}

// Conditional System Execution
fn update_ui_only_when_needed(
    ui_query: Query<
        &UIComponent, 
        Or<(
            Changed<UIComponent>, 
            Changed<InteractionState>,
            Changed<Style>
        )>
    >,
) {
    if ui_query.is_empty() {
        return; // Skip system entirely wenn keine Updates nötig
    }
    
    // Minimale Update-Zyklen
    for component in ui_query.iter() {
        // Process only changed components
    }
}
```

### Batch Operations

```rust
// Batch Entity Operations
fn batch_ui_updates(
    mut commands: Commands,
    updates: Query<(Entity, &PendingUpdate), With<NeedsUIUpdate>>,
) {
    let mut batch_operations = Vec::new();
    
    for (entity, update) in updates.iter() {
        batch_operations.push((entity, update.clone()));
    }
    
    // Single command buffer flush
    for (entity, update) in batch_operations {
        commands.entity(entity)
            .remove::<NeedsUIUpdate>()
            .insert(update.apply());
    }
}

// Memory Pool Pattern für UI-Elemente
#[derive(Resource)]
pub struct UIElementPool {
    available_buttons: Vec<Entity>,
    available_labels: Vec<Entity>,
    in_use: HashSet<Entity>,
}

impl UIElementPool {
    pub fn get_button(&mut self, commands: &mut Commands) -> Entity {
        if let Some(entity) = self.available_buttons.pop() {
            self.in_use.insert(entity);
            entity
        } else {
            let entity = commands.spawn(ButtonBundle::default()).id();
            self.in_use.insert(entity);
            entity
        }
    }
    
    pub fn return_button(&mut self, entity: Entity, commands: &mut Commands) {
        if self.in_use.remove(&entity) {
            commands.entity(entity).despawn_descendants();
            self.available_buttons.push(entity);
        }
    }
}
```

### Query Optimization

```rust
// Optimized Query Patterns
fn optimized_ui_systems(
    // Split queries für bessere Cache-Performance
    transform_query: Query<&GlobalTransform, With<UIElement>>,
    interaction_query: Query<&mut InteractionState, With<UIElement>>,
    style_query: Query<&Style, (With<UIElement>, Changed<Style>)>,
    
    // Use ParallelCommands für Multi-Threading
    par_commands: ParallelCommands,
) {
    // Process transforms in parallel
    transform_query.par_iter().for_each(|transform| {
        // Parallel processing of UI transforms
    });
    
    // Batch style updates
    let style_updates: Vec<_> = style_query.iter().collect();
    for style in style_updates {
        // Process style changes
    }
}
```

## 9. Event-Driven Architecture Patterns

### Event Hierarchie

```rust
// Base Event Trait
pub trait UIEvent: Event + Clone + Send + Sync + 'static {
    fn propagates(&self) -> bool { true }
    fn target(&self) -> Option<Entity> { None }
    fn consumed(&self) -> bool { false }
    fn consume(&mut self) { /* Mark as consumed */ }
}

// Specific Event Types
#[derive(Event, Clone)]
pub struct WidgetClicked {
    pub entity: Entity,
    pub button: MouseButton,
    pub modifiers: KeyboardModifiers,
    pub position: Vec2,
    consumed: bool,
}

impl UIEvent for WidgetClicked {
    fn target(&self) -> Option<Entity> { Some(self.entity) }
    fn consumed(&self) -> bool { self.consumed }
    fn consume(&mut self) { self.consumed = true; }
}

// Event Bubbling System
fn event_bubbling_system<T: UIEvent>(
    mut events: EventReader<T>,
    mut propagated_events: EventWriter<T>,
    hierarchy_query: Query<&Parent>,
    mut commands: Commands,
) {
    for mut event in events.read() {
        if !event.consumed() && event.propagates() {
            if let Some(target) = event.target() {
                // Bubble event up the hierarchy
                if let Ok(parent) = hierarchy_query.get(target) {
                    let mut bubbled_event = event.clone();
                    // Modify event for parent context
                    propagated_events.send(bubbled_event);
                }
            }
        }
    }
}
```

### Command Pattern Integration

```rust
#[derive(Component)]
pub struct WidgetCommand {
    pub system_id: SystemId,
    pub parameters: Box<dyn Any + Send + Sync>,
}

// Generic Command System
#[derive(Resource)]
pub struct CommandRegistry {
    systems: HashMap<TypeId, SystemId>,
}

impl CommandRegistry {
    pub fn register_command<T: Component + 'static>(&mut self, system: SystemId) {
        self.systems.insert(TypeId::of::<T>(), system);
    }
    
    pub fn execute_command<T: Component + 'static>(
        &self, 
        world: &mut World, 
        params: T
    ) {
        if let Some(&system_id) = self.systems.get(&TypeId::of::<T>()) {
            world.run_system_with_input(system_id, params);
        }
    }
}

// Declarative Command Binding
fn execute_widget_commands(
    trigger: Trigger<Pointer<Click>>,
    command_query: Query<&WidgetCommand>,
    mut command_registry: ResMut<CommandRegistry>,
    mut world: &mut World,
) {
    if let Ok(command) = command_query.get(trigger.entity()) {
        // Execute command with parameters
        world.run_system_with_input(command.system_id, &*command.parameters);
    }
}
```

### Event Sourcing Pattern

```rust
#[derive(Event)]
pub struct UIStateChanged {
    pub entity: Entity,
    pub previous_state: UIState,
    pub new_state: UIState,
    pub timestamp: Duration,
}

#[derive(Resource)]
pub struct EventStore {
    events: VecDeque<UIStateChanged>,
    max_events: usize,
}

impl EventStore {
    pub fn record_state_change(&mut self, event: UIStateChanged) {
        self.events.push_back(event);
        if self.events.len() > self.max_events {
            self.events.pop_front();
        }
    }
    
    pub fn replay_events(&self, from_timestamp: Duration) -> Vec<&UIStateChanged> {
        self.events.iter()
            .filter(|event| event.timestamp >= from_timestamp)
            .collect()
    }
}
```

## 10. Composition Over Inheritance

### Trait-basierte Komposition

```rust
// Behavior Traits statt Class-Hierarchien
pub trait Clickable: Send + Sync + 'static {
    fn on_click(&mut self, world: &mut World, entity: Entity);
    fn can_click(&self, world: &World, entity: Entity) -> bool { true }
}

pub trait Hoverable: Send + Sync + 'static {
    fn on_hover_start(&mut self, world: &mut World, entity: Entity);
    fn on_hover_end(&mut self, world: &mut World, entity: Entity);
    fn hover_cursor(&self) -> CursorIcon { CursorIcon::Pointer }
}

pub trait Focusable: Send + Sync + 'static {
    fn on_focus(&mut self, world: &mut World, entity: Entity);
    fn on_blur(&mut self, world: &mut World, entity: Entity);
    fn focus_order(&self) -> i32 { 0 }
}

// Component-basierte Implementation
#[derive(Component)]
pub struct BehaviorSet {
    pub clickable: Option<Box<dyn Clickable>>,
    pub hoverable: Option<Box<dyn Hoverable>>,
    pub focusable: Option<Box<dyn Focusable>>,
}
```

### Mixin-Pattern via Components

```rust
// Behavior-Components
#[derive(Component)]
pub struct Draggable {
    pub constraints: DragConstraints,
    pub drag_threshold: f32,
    pub current_drag: Option<DragState>,
}

#[derive(Component)]
pub struct Resizable {
    pub min_size: Vec2,
    pub max_size: Vec2,
    pub resize_handles: ResizeHandles,
    pub maintain_aspect_ratio: bool,
}

#[derive(Component)]
pub struct Rotatable {
    pub center_point: Vec2,
    pub snap_angle: Option<f32>,
    pub rotation_sensitivity: f32,
}

// Composition via Bundle
#[derive(Bundle)]
pub struct AdvancedInteractiveWidget {
    // Core widget
    pub widget: Widget,
    pub interaction_state: InteractionState,
    
    // Behavior mixins
    pub draggable: Draggable,
    pub resizable: Resizable,
    pub rotatable: Rotatable,
    
    // Support components
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
}
```

### Dynamic Behavior Composition

```rust
// Runtime Behavior Addition
pub struct BehaviorManager;

impl BehaviorManager {
    pub fn add_behavior<T: Component + 'static>(
        &self,
        commands: &mut Commands,
        entity: Entity,
        behavior: T,
    ) {
        commands.entity(entity).insert(behavior);
    }
    
    pub fn remove_behavior<T: Component + 'static>(
        &self,
        commands: &mut Commands,
        entity: Entity,
    ) {
        commands.entity(entity).remove::<T>();
    }
    
    pub fn has_behavior<T: Component + 'static>(
        &self,
        world: &World,
        entity: Entity,
    ) -> bool {
        world.entity(entity).contains::<T>()
    }
}

// Conditional Behavior Systems
fn conditional_behavior_system(
    draggable_query: Query<Entity, (With<Draggable>, With<InteractionState>)>,
    resizable_query: Query<Entity, (With<Resizable>, With<InteractionState>)>,
    mut commands: Commands,
) {
    // Systems aktivieren nur für Entities mit entsprechenden Behaviors
    for entity in draggable_query.iter() {
        // Apply drag behavior
    }
    
    for entity in resizable_query.iter() {
        // Apply resize behavior
    }
}
```

## Fazit

Diese Architektur-Prinzipien bilden die Grundlage für moderne, skalierbare UI-Systeme in Bevy. Sie kombinieren:

- **ECS-optimierte Performance** durch datenorientierte Patterns
- **Modulare Erweiterbarkeit** durch Plugin-Architektur
- **Accessibility-First Design** für inklusive Anwendungen
- **Type-Safety** durch Rust's Type-System
- **Reaktive Updates** durch Event-driven Architecture
- **Composition-basierte Flexibilität** für komplexe UI-Komponenten

Die konsequente Anwendung dieser Prinzipien führt zu robusten, wartbaren und performanten UI-Systemen, die mit Bevy's Architektur harmonieren und gleichzeitig moderne UX-Standards erfüllen.