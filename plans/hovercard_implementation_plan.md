# HoverCard Component Implementation Plan

## Overview

This plan outlines the implementation of a HoverCard component for the Radix UI-inspired component library for Bevy 0.16. The HoverCard provides a floating preview of content available behind a link or interactive element, designed primarily for sighted users.

## Research Summary

### Radix UI HoverCard Specification
- **Purpose**: Preview content available behind a link for sighted users
- **Key Features**: Controllable/uncontrolled state, customizable positioning, optional arrow, custom delays, screen reader ignorance
- **API Structure**: Root → Trigger → Portal → Content → Arrow hierarchy
- **Timing**: Default open delay 700ms, close delay 300ms
- **Positioning**: Side-based positioning with collision detection
- **Accessibility**: Limited keyboard interaction, primarily for sighted users

### Existing Architecture Analysis
- **Component Pattern**: Builder pattern with fluent API (`ComponentBuilder::new().build()`)
- **Bundle Creation**: Components return `impl Bundle` for ECS integration
- **Event System**: Uses Bevy Observer pattern, not legacy Events
- **Theme Integration**: Full theme system with color palettes and responsive scaling
- **Portal System**: Existing portal implementation for rendering outside normal hierarchy
- **Interaction**: Uses `bevy_picking` for hover, click, focus management

### Bevy UI Positioning System
- **Z-Index**: Local (`ZIndex`) and Global (`GlobalZIndex`) layering control
- **UI Stack**: UI elements always render above 2D sprites
- **Interaction**: `ui_focus_system` manages interaction capture with depth ordering
- **Layout**: Uses taffy crate for layout calculations with responsive scaling

## Component Architecture

### Core Components

#### 1. HoverCard (Root Component)
```rust
#[derive(Component, Debug, Clone)]
pub struct HoverCard {
    pub open_delay: Duration,
    pub close_delay: Duration,
    pub controlled: bool,
    pub open: bool,
}
```

#### 2. HoverCardTrigger
```rust
#[derive(Component, Debug, Clone)]
pub struct HoverCardTrigger {
    pub hover_card: Entity,
    pub disabled: bool,
}
```

#### 3. HoverCardContent
```rust
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
```

#### 4. HoverCardArrow
```rust
#[derive(Component, Debug)]
pub struct HoverCardArrow {
    pub content: Entity,
    pub width: f32,
    pub height: f32,
}
```

### Enumerations

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HoverCardSide {
    Top,
    Right, 
    Bottom,
    Left,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HoverCardAlign {
    Start,
    Center,
    End,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HoverCardSticky {
    Partial,
    Always,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HoverCardState {
    Closed,
    Opening,
    Open,
    Closing,
}
```

### Events

```rust
#[derive(Event, Debug, Clone)]
pub struct HoverCardOpenEvent {
    pub hover_card: Entity,
}

#[derive(Event, Debug, Clone)]
pub struct HoverCardCloseEvent {
    pub hover_card: Entity,
}
```

## Builder Pattern Implementation

### HoverCardBuilder
```rust
pub struct HoverCardBuilder {
    hover_card: HoverCard,
    children: Vec<Entity>,
}

impl HoverCardBuilder {
    pub fn new() -> Self
    pub fn open_delay(mut self, delay: Duration) -> Self
    pub fn close_delay(mut self, delay: Duration) -> Self
    pub fn controlled(mut self) -> Self
    pub fn default_open(mut self, open: bool) -> Self
    pub fn build(self) -> impl Bundle
}
```

### HoverCardTriggerBuilder
```rust
pub struct HoverCardTriggerBuilder {
    trigger: HoverCardTrigger,
    content: String,
    text_builder: Option<TextBuilder>,
}

impl HoverCardTriggerBuilder {
    pub fn new(hover_card: Entity) -> Self
    pub fn text(mut self, text: impl Into<String>) -> Self
    pub fn disabled(mut self) -> Self
    // Text styling methods similar to Button
    pub fn build(self) -> impl Bundle
}
```

### HoverCardContentBuilder
```rust
pub struct HoverCardContentBuilder {
    content: HoverCardContent,
    theme: UiColorPalette,
    radius: ButtonRadius,
    padding: f32,
    children: Vec<Entity>,
}

impl HoverCardContentBuilder {
    pub fn new(hover_card: Entity) -> Self
    pub fn side(mut self, side: HoverCardSide) -> Self
    pub fn side_offset(mut self, offset: f32) -> Self  
    pub fn align(mut self, align: HoverCardAlign) -> Self
    pub fn align_offset(mut self, offset: f32) -> Self
    pub fn avoid_collisions(mut self, avoid: bool) -> Self
    pub fn collision_boundary(mut self, entity: Entity) -> Self
    pub fn collision_padding(mut self, padding: UiRect) -> Self
    pub fn arrow(mut self) -> Self
    pub fn arrow_size(mut self, width: f32, height: f32) -> Self
    pub fn sticky(mut self, sticky: HoverCardSticky) -> Self
    pub fn hide_when_detached(mut self, hide: bool) -> Self
    pub fn theme(mut self, theme: UiColorPalette) -> Self
    pub fn radius(mut self, radius: ButtonRadius) -> Self
    pub fn padding(mut self, padding: f32) -> Self
    pub fn child(mut self, entity: Entity) -> Self
    pub fn build(self) -> impl Bundle
}
```

## System Implementation

### Core Systems

#### 1. HoverCard Interaction System
```rust
pub fn hover_card_interaction_system(
    mut commands: Commands,
    trigger_query: Query<(Entity, &HoverCardTrigger, &Interaction), Changed<Interaction>>,
    hover_card_query: Query<(Entity, &mut HoverCard)>,
    time: Res<Time>,
) {
    // Handle trigger hover events
    // Manage timing delays for open/close
    // Update HoverCard state
}
```

#### 2. HoverCard State Management System  
```rust
pub fn hover_card_state_system(
    mut commands: Commands,
    mut hover_card_query: Query<(Entity, &mut HoverCard), Changed<HoverCard>>,
    content_query: Query<Entity, With<HoverCardContent>>,
    mut visibility_query: Query<&mut Visibility>,
    mut events: EventWriter<HoverCardOpenEvent>,
) {
    // Manage open/close state transitions
    // Show/hide content based on state
    // Emit events for state changes
}
```

#### 3. HoverCard Positioning System
```rust
pub fn hover_card_positioning_system(
    mut content_query: Query<(Entity, &HoverCardContent, &mut Transform), Changed<HoverCardContent>>,
    trigger_query: Query<&GlobalTransform, With<HoverCardTrigger>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    ui_scale: Res<UiScale>,
) {
    // Calculate optimal position based on trigger
    // Handle collision detection and boundary constraints
    // Apply side/align positioning with offsets  
    // Position arrow if present
}
```

#### 4. HoverCard Portal Integration System
```rust
pub fn hover_card_portal_system(
    mut commands: Commands,
    content_query: Query<(Entity, &HoverCardContent), Added<HoverCardContent>>,
    portal_query: Query<Entity, With<Portal>>,
    ui_root_query: Query<Entity, With<UIRoot>>,
) {
    // Integrate with existing portal system
    // Move content to overlay layer for proper z-index
    // Ensure content renders above other UI elements
}
```

### Animation and Timing Systems

#### 5. HoverCard Animation System
```rust
pub fn hover_card_animation_system(
    time: Res<Time>,
    mut hover_card_query: Query<(Entity, &mut HoverCard, &HoverCardState)>,
    mut transform_query: Query<&mut Transform>,
    mut background_query: Query<&mut BackgroundColor>,
) {
    // Handle opening/closing animations
    // Scale and fade transitions
    // Coordinate with portal system
}
```

## Theme Integration

### Color Scheme
- **Background**: `theme.bg_elevated` for floating appearance
- **Border**: `theme.border_subtle` for definition
- **Text**: `theme.text` with proper contrast
- **Shadow**: Elevated shadow for depth perception

### Layout System Integration
```rust
impl HoverCardContentBuilder {
    fn calculate_style(&self) -> Node {
        let layout = UiLayout::default();
        Node {
            padding: UiRect::all(Val::Px(self.padding)),
            border: UiRect::all(Val::Px(layout.border.xs)),
            position_type: PositionType::Absolute, // For floating behavior
            ..default()
        }
    }
}
```

## Accessibility Considerations

### Keyboard Navigation
- **Tab Focus**: Can trigger hover card open/close
- **Escape Key**: Closes open hover cards
- **Arrow Keys**: No navigation within hover card (following Radix spec)

### Screen Reader Support
- **Limited Support**: Following Radix pattern of being primarily for sighted users
- **Aria Attributes**: Minimal ARIA implementation 
- **Alternative Content**: Encourage developers to provide accessible alternatives

## Portal and Z-Index Strategy

### Portal Integration
1. **Overlay Layer**: Create dedicated `UIRoot` for hover card overlays
2. **Portal Content**: Use existing portal system to move content out of normal flow
3. **Z-Index Management**: Use `GlobalZIndex` to ensure proper layering above other UI

### Positioning Strategy
1. **Absolute Positioning**: Use `PositionType::Absolute` for floating behavior
2. **Collision Detection**: Calculate viewport boundaries and adjust position
3. **Arrow Positioning**: Dynamic arrow placement based on content position relative to trigger

## Performance Considerations

### Optimization Strategies
1. **Lazy Content**: Only create content entities when hover card opens
2. **Pooling**: Reuse content entities to avoid allocation overhead
3. **Culling**: Hide content when outside viewport boundaries
4. **Batch Updates**: Group positioning updates to minimize system overhead

### Memory Management  
1. **Component Cleanup**: Remove content entities when hover card is destroyed
2. **Event Cleanup**: Proper event listener cleanup on entity despawn
3. **Resource Tracking**: Track portal content relationships for cleanup

## Integration Points

### Existing Systems
1. **Portal System**: Leverage for content rendering outside normal hierarchy
2. **Theme System**: Full integration with color palettes and responsive scaling
3. **Text Components**: Support for rich text content with TextBuilder
4. **Interaction System**: Integration with bevy_picking for hover detection

### Component Interoperability
1. **Button Integration**: Hover cards can be triggered by any interactive component
2. **Text Links**: Special integration for text-based triggers
3. **Custom Triggers**: Support for any entity with Interaction component

## Implementation Phases

### Phase 1: Core Component Structure
- [ ] Define core components and enums
- [ ] Implement basic builder patterns
- [ ] Create fundamental bundle structures

### Phase 2: Interaction and State Management
- [ ] Implement hover detection and timing systems
- [ ] Create state transition logic
- [ ] Add event emission for state changes

### Phase 3: Positioning and Layout
- [ ] Implement positioning calculation system
- [ ] Add collision detection and boundary handling
- [ ] Create arrow positioning logic

### Phase 4: Portal and Rendering Integration
- [ ] Integrate with existing portal system
- [ ] Implement z-index and layering strategy
- [ ] Add animation and transition effects

### Phase 5: Theme and Accessibility
- [ ] Full theme system integration
- [ ] Implement accessibility features
- [ ] Add keyboard navigation support

### Phase 6: Testing and Optimization
- [ ] Create comprehensive example implementations
- [ ] Performance testing and optimization
- [ ] Documentation and API refinement

## Example Usage

```rust
fn setup_hover_card_example(mut commands: Commands) {
    // Create hover card root
    let hover_card = commands.spawn(
        HoverCard::new()
            .open_delay(Duration::from_millis(700))
            .close_delay(Duration::from_millis(300))
            .build()
    ).id();

    // Create trigger
    commands.spawn(
        HoverCardTrigger::new(hover_card)
            .text("Hover me for details")
            .build()
    );

    // Create content
    commands.spawn(
        HoverCardContent::new(hover_card)
            .side(HoverCardSide::Top)
            .arrow()
            .theme(accent_palette())
            .build()
    ).with_children(|parent| {
        parent.spawn(
            Text::body("This is additional information shown on hover.")
                .build()
        );
    });
}
```

## Success Criteria

1. **API Consistency**: Follows established builder pattern and component architecture
2. **Radix Compatibility**: Matches Radix UI HoverCard behavior and API surface
3. **Performance**: Smooth hover interactions without frame drops
4. **Theme Integration**: Full support for design system colors, spacing, and typography
5. **Accessibility**: Basic keyboard navigation and screen reader considerations
6. **Documentation**: Comprehensive examples and API documentation
7. **Testing**: Robust example implementations demonstrating various use cases

This implementation plan provides a comprehensive roadmap for creating a production-ready HoverCard component that integrates seamlessly with the existing Bevy UI component library while maintaining consistency with Radix UI patterns and accessibility standards.