# Dialog Component Implementation Plan

## Overview

This plan outlines the implementation of a Dialog component for the Radix UI-inspired component library for Bevy 0.16. The Dialog provides modal and non-modal overlays that follow WAI-ARIA accessibility guidelines, with proper focus management, keyboard navigation, and screen reader support.

## Research Summary

### Radix UI Dialog Specification

- **Purpose**: Window overlaid on primary window or another dialog
- **Modal Behavior**: Can trap focus and make background content inert
- **Component Structure**: Root → Trigger → Portal → Overlay → Content → Close/Title/Description
- **Accessibility**: Full WAI-ARIA compliance with proper role attributes
- **Keyboard Navigation**: Tab/Shift+Tab focus cycling, Escape to close, Enter/Space to activate
- **Focus Management**: Automatic focus trapping and return focus to trigger on close

### WAI-ARIA Dialog Pattern Requirements

- **Role Attributes**: `role="dialog"`, `aria-modal="true"` for modal dialogs
- **Labeling**: `aria-labelledby` for titles, `aria-describedby` for descriptions
- **Focus Management**: Initial focus placement, focus trap within modal, focus return on close
- **Keyboard Support**: Tab cycling, Escape closing, Space/Enter activation
- **Screen Reader**: Proper announcements and content accessibility

### Bevy UI Implementation Constraints

- **Focus System**: No built-in focus management, requires custom implementation
- **Keyboard Input**: Uses `ButtonInput<KeyCode>` resource for keyboard state
- **Modal Overlays**: Requires portal system integration and z-index management
- **Interaction**: Uses `bevy_picking` for UI interaction handling
- **Accessibility**: Limited built-in accessibility features, custom implementation needed

## Component Architecture

### Core Components

#### 1. Dialog (Root Component)

```rust
#[derive(Component, Debug, Clone)]
pub struct Dialog {
    pub modal: bool,
    pub open: bool,
    pub controlled: bool,
    pub default_open: bool,
    pub prevent_scroll: bool,
}
```

#### 2. DialogTrigger

```rust
#[derive(Component, Debug, Clone)]
pub struct DialogTrigger {
    pub dialog: Entity,
    pub disabled: bool,
    pub as_child: bool,
}
```

#### 3. DialogOverlay

```rust
#[derive(Component, Debug, Clone)]
pub struct DialogOverlay {
    pub dialog: Entity,
    pub force_mount: bool,
    pub theme: UiColorPalette,
    pub blur_background: bool,
    pub close_on_click: bool,
}
```

#### 4. DialogContent

```rust
#[derive(Component, Debug, Clone)]
pub struct DialogContent {
    pub dialog: Entity,
    pub force_mount: bool,
    pub trap_focus: bool,
    pub disable_outside_pointer_events: bool,
    pub escape_key_down: bool,
    pub pointer_down_outside: bool,
    pub interact_outside: bool,
    pub focus_outside: bool,
    pub theme: UiColorPalette,
    pub size: DialogSize,
    pub position: DialogPosition,
}
```

#### 5. DialogClose

```rust
#[derive(Component, Debug, Clone)]
pub struct DialogClose {
    pub dialog: Entity,
    pub as_child: bool,
}
```

#### 6. DialogTitle

```rust
#[derive(Component, Debug, Clone)]
pub struct DialogTitle {
    pub dialog: Entity,
    pub level: HeadingLevel,
}
```

#### 7. DialogDescription

```rust
#[derive(Component, Debug, Clone)]
pub struct DialogDescription {
    pub dialog: Entity,
}
```

### Focus Management Components

#### 8. FocusTrap

```rust
#[derive(Component, Debug, Clone)]
pub struct FocusTrap {
    pub active: bool,
    pub initial_focus: Option<Entity>,
    pub final_focus: Option<Entity>,
    pub focus_elements: Vec<Entity>,
    pub current_focus_index: usize,
}
```

#### 9. Focusable

```rust
#[derive(Component, Debug, Clone)]
pub struct Focusable {
    pub focused: bool,
    pub tab_index: i32,
    pub disabled: bool,
}
```

### Enumerations

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogState {
    Closed,
    Opening,
    Open,
    Closing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogSize {
    Small,
    Medium,
    Large,
    FullScreen,
    Custom { width: f32, height: f32 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogPosition {
    Center,
    Top,
    Bottom,
    Left,
    Right,
    Custom { x: f32, y: f32 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeadingLevel {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}
```

### Events

```rust
#[derive(Event, Debug, Clone)]
pub struct DialogOpenEvent {
    pub dialog: Entity,
}

#[derive(Event, Debug, Clone)]
pub struct DialogCloseEvent {
    pub dialog: Entity,
}

#[derive(Event, Debug, Clone)]
pub struct DialogEscapeEvent {
    pub dialog: Entity,
}

#[derive(Event, Debug, Clone)]
pub struct DialogOutsideClickEvent {
    pub dialog: Entity,
    pub pointer_position: Vec2,
}

#[derive(Event, Debug, Clone)]
pub struct FocusChangeEvent {
    pub from: Option<Entity>,
    pub to: Option<Entity>,
}
```

## Builder Pattern Implementation

### DialogBuilder

```rust
pub struct DialogBuilder {
    dialog: Dialog,
    children: Vec<Entity>,
}

impl DialogBuilder {
    pub fn new() -> Self
    pub fn modal(mut self) -> Self
    pub fn controlled(mut self) -> Self
    pub fn default_open(mut self, open: bool) -> Self
    pub fn prevent_scroll(mut self) -> Self
    pub fn build(self) -> impl Bundle
}
```

### DialogTriggerBuilder

```rust
pub struct DialogTriggerBuilder {
    trigger: DialogTrigger,
    content: String,
    text_builder: Option<TextBuilder>,
    button_style: Option<ButtonVariant>,
}

impl DialogTriggerBuilder {
    pub fn new(dialog: Entity) -> Self
    pub fn text(mut self, text: impl Into<String>) -> Self
    pub fn disabled(mut self) -> Self
    pub fn as_child(mut self) -> Self
    pub fn button_variant(mut self, variant: ButtonVariant) -> Self
    // Text styling methods
    pub fn build(self) -> impl Bundle
}
```

### DialogOverlayBuilder

```rust
pub struct DialogOverlayBuilder {
    overlay: DialogOverlay,
    theme: UiColorPalette,
    opacity: f32,
}

impl DialogOverlayBuilder {
    pub fn new(dialog: Entity) -> Self
    pub fn force_mount(mut self) -> Self
    pub fn theme(mut self, theme: UiColorPalette) -> Self
    pub fn blur_background(mut self) -> Self
    pub fn close_on_click(mut self, close: bool) -> Self
    pub fn opacity(mut self, opacity: f32) -> Self
    pub fn build(self) -> impl Bundle
}
```

### DialogContentBuilder

```rust
pub struct DialogContentBuilder {
    content: DialogContent,
    theme: UiColorPalette,
    padding: f32,
    radius: ButtonRadius,
    children: Vec<Entity>,
}

impl DialogContentBuilder {
    pub fn new(dialog: Entity) -> Self
    pub fn force_mount(mut self) -> Self
    pub fn trap_focus(mut self, trap: bool) -> Self
    pub fn disable_outside_pointer_events(mut self, disable: bool) -> Self
    pub fn escape_key_down(mut self, enabled: bool) -> Self
    pub fn pointer_down_outside(mut self, enabled: bool) -> Self
    pub fn interact_outside(mut self, enabled: bool) -> Self
    pub fn focus_outside(mut self, enabled: bool) -> Self
    pub fn theme(mut self, theme: UiColorPalette) -> Self
    pub fn size(mut self, size: DialogSize) -> Self
    pub fn position(mut self, position: DialogPosition) -> Self
    pub fn padding(mut self, padding: f32) -> Self
    pub fn radius(mut self, radius: ButtonRadius) -> Self
    pub fn child(mut self, entity: Entity) -> Self
    pub fn build(self) -> impl Bundle
}
```

## System Implementation

### Core Dialog Systems

#### 1. Dialog State Management System

```rust
pub fn dialog_state_system(
    mut commands: Commands,
    mut dialog_query: Query<(Entity, &mut Dialog), Changed<Dialog>>,
    overlay_query: Query<Entity, With<DialogOverlay>>,
    content_query: Query<Entity, With<DialogContent>>,
    mut visibility_query: Query<&mut Visibility>,
    mut open_events: EventWriter<DialogOpenEvent>,
    mut close_events: EventWriter<DialogCloseEvent>,
) {
    // Manage dialog open/close state transitions
    // Show/hide overlay and content based on state
    // Emit events for state changes
    // Handle modal vs non-modal behavior
}
```

#### 2. Dialog Trigger System

```rust
pub fn dialog_trigger_system(
    trigger_query: Query<(&DialogTrigger, &Interaction), Changed<Interaction>>,
    mut dialog_query: Query<&mut Dialog>,
    mut commands: Commands,
) {
    // Handle trigger click interactions
    // Toggle dialog open/close state
    // Manage disabled state
}
```

#### 3. Dialog Portal Integration System

```rust
pub fn dialog_portal_system(
    mut commands: Commands,
    dialog_query: Query<(Entity, &Dialog), Changed<Dialog>>,
    overlay_query: Query<(Entity, &DialogOverlay)>,
    content_query: Query<(Entity, &DialogContent)>,
    portal_query: Query<Entity, With<Portal>>,
    ui_root_query: Query<Entity, With<UIRoot>>,
) {
    // Move overlay and content to portal layer
    // Ensure proper z-index layering
    // Handle modal overlay rendering
}
```

### Focus Management Systems

#### 4. Focus Trap System

```rust
pub fn focus_trap_system(
    mut commands: Commands,
    focus_trap_query: Query<(Entity, &mut FocusTrap)>,
    focusable_query: Query<(Entity, &Focusable, &GlobalTransform), With<Visibility>>,
    children_query: Query<&Children>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut focus_events: EventWriter<FocusChangeEvent>,
) {
    // Identify focusable elements within trap
    // Handle Tab/Shift+Tab focus cycling
    // Maintain focus within dialog bounds
    // Prevent focus from leaving modal dialog
}
```

#### 5. Keyboard Navigation System

```rust
pub fn dialog_keyboard_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    dialog_query: Query<(Entity, &Dialog)>,
    mut dialog_state_query: Query<&mut Dialog>,
    focus_trap_query: Query<&FocusTrap>,
    mut escape_events: EventWriter<DialogEscapeEvent>,
) {
    // Handle Escape key to close dialogs
    // Manage keyboard shortcuts
    // Coordinate with focus trap system
}
```

#### 6. Focus Visual System

```rust
pub fn focus_visual_system(
    focusable_query: Query<(Entity, &Focusable), Changed<Focusable>>,
    mut style_query: Query<&mut BorderColor>,
    mut background_query: Query<&mut BackgroundColor>,
    theme: Res<UiTheme>,
) {
    // Update visual focus indicators
    // Apply focus styles (border, background)
    // Handle focus state changes
}
```

### Accessibility Systems

#### 7. Screen Reader System

```rust
pub fn screen_reader_system(
    dialog_query: Query<(Entity, &Dialog), Changed<Dialog>>,
    title_query: Query<&DialogTitle>,
    description_query: Query<&DialogDescription>,
    mut commands: Commands,
) {
    // Manage ARIA attributes
    // Handle screen reader announcements
    // Update accessibility tree
}
```

### Interaction Systems

#### 8. Outside Click System

```rust
pub fn outside_click_system(
    content_query: Query<(Entity, &DialogContent, &GlobalTransform, &Node)>,
    overlay_query: Query<(Entity, &DialogOverlay, &Interaction), Changed<Interaction>>,
    mut dialog_query: Query<&mut Dialog>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    cursor_position: Res<CursorPosition>,
    mut outside_click_events: EventWriter<DialogOutsideClickEvent>,
) {
    // Detect clicks outside dialog content
    // Close dialog on outside click (if enabled)
    // Handle overlay click behavior
}
```

## Theme Integration

### Color Scheme

```rust
impl DialogContentBuilder {
    fn calculate_colors(&self) -> DialogColors {
        DialogColors {
            background: self.theme.bg_elevated,
            border: self.theme.border_subtle,
            shadow: Color::srgba(0.0, 0.0, 0.0, 0.15),
            overlay: Color::srgba(0.0, 0.0, 0.0, 0.5),
        }
    }
}
```

### Layout Integration

```rust
impl DialogContentBuilder {
    fn calculate_style(&self) -> Node {
        let layout = UiLayout::default();
        let (width, height) = match self.content.size {
            DialogSize::Small => (Val::Px(400.0), Val::Auto),
            DialogSize::Medium => (Val::Px(600.0), Val::Auto),
            DialogSize::Large => (Val::Px(800.0), Val::Auto),
            DialogSize::FullScreen => (Val::Percent(100.0), Val::Percent(100.0)),
            DialogSize::Custom { width, height } => (Val::Px(width), Val::Px(height)),
        };

        Node {
            width,
            height,
            padding: UiRect::all(Val::Px(self.padding)),
            border: UiRect::all(Val::Px(layout.border.xs)),
            position_type: PositionType::Absolute,
            ..default()
        }
    }
}
```

## Accessibility Implementation

### ARIA Attributes Simulation

```rust
#[derive(Component, Debug, Clone)]
pub struct AriaAttributes {
    pub role: String,
    pub modal: bool,
    pub labelledby: Option<Entity>,
    pub describedby: Option<Entity>,
    pub live: Option<String>,
}
```

### Focus Management

```rust
#[derive(Resource, Debug)]
pub struct FocusManager {
    pub current_focus: Option<Entity>,
    pub focus_history: Vec<Entity>,
    pub focus_trap_stack: Vec<Entity>,
}
```

## Animation and Transitions

### Animation System

```rust
pub fn dialog_animation_system(
    time: Res<Time>,
    mut dialog_query: Query<(Entity, &Dialog, &mut DialogState)>,
    mut transform_query: Query<&mut Transform>,
    mut background_query: Query<&mut BackgroundColor>,
    overlay_query: Query<Entity, With<DialogOverlay>>,
    content_query: Query<Entity, With<DialogContent>>,
) {
    // Handle opening/closing animations
    // Scale and fade transitions for content
    // Overlay fade in/out
    // Coordinate timing with state changes
}
```

## Performance Considerations

### Optimization Strategies

1. **Lazy Content Creation**: Only create overlay/content entities when dialog opens
2. **Focus Element Caching**: Cache focusable elements to avoid repeated queries
3. **Event Batching**: Group related state changes to minimize system runs
4. **Conditional Systems**: Use run conditions to avoid unnecessary processing

### Memory Management

1. **Component Cleanup**: Remove dialog entities and children on close
2. **Focus History**: Limit focus history size to prevent memory leaks
3. **Event Cleanup**: Proper cleanup of event listeners

## Integration Points

### Existing Systems

1. **Portal System**: Leverage for overlay rendering outside normal hierarchy
2. **Theme System**: Full integration with color palettes and responsive scaling
3. **Button Components**: Integration for trigger and close button styling
4. **Text Components**: Support for titles and descriptions with TextBuilder

### Component Interoperability

1. **Form Integration**: Support for form elements within dialog content
2. **Button Integration**: Seamless trigger and action button integration
3. **Layout Components**: Support for flex, grid, and section layouts within content

## Implementation Phases

### Phase 1: Core Component Structure

- [ ] Define core components and enums
- [ ] Implement basic builder patterns
- [ ] Create fundamental bundle structures

### Phase 2: Dialog State Management

- [ ] Implement dialog open/close logic
- [ ] Create trigger interaction system
- [ ] Add state transition events

### Phase 3: Portal and Overlay Integration

- [ ] Integrate with existing portal system
- [ ] Implement overlay rendering and styling
- [ ] Add z-index management for modal behavior

### Phase 4: Focus Management System

- [ ] Implement focus trap functionality
- [ ] Create keyboard navigation system
- [ ] Add visual focus indicators

### Phase 5: Accessibility Features

- [ ] Implement ARIA attributes simulation
- [ ] Add screen reader support patterns
- [ ] Create accessible keyboard shortcuts

### Phase 6: Advanced Features and Polish

- [ ] Add animation and transition effects
- [ ] Implement outside click detection
- [ ] Create comprehensive examples and documentation

## Example Usage

```rust
fn setup_dialog_example(mut commands: Commands) {
    // Create dialog root
    let dialog = commands.spawn(
        Dialog::new()
            .modal()
            .build()
    ).id();

    // Create trigger
    commands.spawn(
        DialogTrigger::new(dialog)
            .text("Open Dialog")
            .button_variant(ButtonVariant::Solid)
            .build()
    );

    // Create overlay
    commands.spawn(
        DialogOverlay::new(dialog)
            .close_on_click(true)
            .blur_background()
            .build()
    );

    // Create content
    commands.spawn(
        DialogContent::new(dialog)
            .size(DialogSize::Medium)
            .position(DialogPosition::Center)
            .trap_focus(true)
            .theme(accent_palette())
            .build()
    ).with_children(|parent| {
        // Title
        parent.spawn(
            DialogTitle::new(dialog)
                .level(HeadingLevel::H2)
                .build()
        ).with_children(|title| {
            title.spawn(
                Text::heading("Confirm Action")
                    .build()
            );
        });

        // Description
        parent.spawn(
            DialogDescription::new(dialog)
                .build()
        ).with_children(|desc| {
            desc.spawn(
                Text::body("Are you sure you want to continue? This action cannot be undone.")
                    .build()
            );
        });

        // Actions
        parent.spawn(
            Flex::new()
                .direction(FlexDirection::Row)
                .justify_content(JustifyContent::End)
                .gap(UiLayout::default().gap.sm)
                .build()
        ).with_children(|actions| {
            actions.spawn(
                DialogClose::new(dialog)
                    .text("Cancel")
                    .button_variant(ButtonVariant::Outline)
                    .build()
            );
            
            actions.spawn(
                DialogClose::new(dialog)
                    .text("Confirm")
                    .button_variant(ButtonVariant::Solid)
                    .build()
            );
        });
    });
}
```

## Success Criteria

1. **WAI-ARIA Compliance**: Full accessibility pattern implementation
2. **Focus Management**: Proper focus trapping and keyboard navigation
3. **Theme Integration**: Seamless design system integration
4. **Performance**: Smooth animations and responsive interactions
5. **Modal Behavior**: Proper background interaction blocking
6. **Keyboard Support**: Complete keyboard navigation and shortcuts
7. **Screen Reader**: Accessible announcements and navigation
8. **API Consistency**: Follows established builder patterns and component architecture

This implementation plan provides a comprehensive roadmap for creating a production-ready Dialog component that meets modern accessibility standards while integrating seamlessly with the existing Bevy UI component library architecture.