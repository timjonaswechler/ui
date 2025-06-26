# Toggle Component Implementation Plan

**Component:** Toggle  
**Type:** Interactive Form Control  
**Radix Inspiration:** [@radix-ui/react-toggle](https://github.com/radix-ui/primitives/tree/main/packages/react/toggle)  
**Date:** 2025-06-26  
**Status:** Planning Phase

## Executive Summary

The Toggle component is a two-state button control that allows users to switch between "on" and "off" states. This implementation follows Radix UI principles and integrates seamlessly with the existing Bevy UI architecture, reusing established patterns from the Button component while providing toggle-specific functionality.

## 1. Component Requirements

### 1.1 Core Functionality
- **Binary State Management**: Maintain pressed/unpressed boolean state
- **Visual State Feedback**: Clear visual indication of current state
- **State Change Events**: Emit events when toggle state changes
- **Accessibility**: Full keyboard navigation and screen reader support
- **Disabled State**: Support for non-interactive disabled state

### 1.2 API Requirements
- **Builder Pattern**: Consistent with existing components (Button, Text, etc.)
- **Theme Integration**: Use existing `UiColorPalette` and `UiLayout` systems
- **Variant Support**: Multiple visual styles (Solid, Soft, Outline)
- **Size Support**: Three sizes (Small, Default, Large)
- **Color Customization**: Support for all theme color palettes

### 1.3 Bevy Integration
- **ECS Architecture**: Component-based design with Systems for behavior
- **Event System**: Use Bevy's Observer pattern (not legacy Events)
- **Interaction System**: Integrate with `bevy_picking` for mouse/touch input
- **Accessibility**: Keyboard input support via Bevy input systems

## 2. Technical Architecture

### 2.1 Core Data Structures

```rust
#[derive(Component, Debug, Clone)]
pub struct Toggle {
    pub pressed: bool,              // Current toggle state
    pub size: ToggleSize,          // Visual size variant
    pub variant: ToggleVariant,    // Visual style variant
    pub color: UiColorPalette,     // Color theme
    pub disabled: bool,            // Interaction state
    pub current_state: ToggleState, // Visual interaction state
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToggleState {
    Normal,    // Default state
    Hover,     // Mouse hover state
    Active,    // Mouse pressed state
    Disabled,  // Non-interactive state
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ToggleVariant {
    #[default]
    Solid,    // Full background color
    Soft,     // Light background color
    Outline,  // Border with transparent background
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ToggleSize {
    Small,    // Compact size
    #[default]
    Default,  // Standard size
    Large,    // Expanded size
}

#[derive(Event, Debug, Clone)]
pub struct TogglePressedEvent {
    pub toggle_entity: Entity,
    pub pressed: bool,
}
```

### 2.2 Builder API Design

```rust
pub struct ToggleBuilder {
    name: String,
    toggle: Toggle,
    icon: Option<IconBuilder>,    // Optional icon support
    text: Option<String>,         // Optional text label
    children: Vec<Entity>,        // Custom child elements
}

impl ToggleBuilder {
    pub fn new(name: impl Into<String>) -> Self
    pub fn variant(self, variant: ToggleVariant) -> Self
    pub fn size(self, size: ToggleSize) -> Self
    pub fn color(self, color: UiColorPalette) -> Self
    pub fn pressed(self, pressed: bool) -> Self
    pub fn disabled(self) -> Self
    pub fn icon(self, icon: IconBuilder) -> Self
    pub fn text(self, text: impl Into<String>) -> Self
    pub fn child(self, entity: Entity) -> Self
    pub fn build(self) -> impl Bundle
}

// Usage Example:
let toggle = ToggleBuilder::new("dark_mode")
    .variant(ToggleVariant::Solid)
    .size(ToggleSize::Large)
    .pressed(true)
    .color(accent_palette())
    .text("Dark Mode")
    .build();
```

### 2.3 Bevy ECS Integration

**Bundle Components:**
- `Toggle` - Core component state
- `Node` - Layout and styling
- `BackgroundColor` - Visual state
- `BorderColor` - Border styling
- `BorderRadius` - Corner rounding
- `Pickable` - Enable mouse/touch interaction
- `Focusable` - Enable keyboard navigation
- `Name` - Debug identification

**Systems Architecture:**
```rust
// Setup System - Runs on component addition
pub fn setup_toggle_interactions(
    mut commands: Commands,
    toggles: Query<Entity, Added<Toggle>>,
) {
    // Attach observers for interaction events
    // Apply initial styling
}

// Interaction Systems - Handle user input
pub fn handle_toggle_click(/* ... */) { /* Toggle state on click */ }
pub fn handle_toggle_hover(/* ... */) { /* Visual feedback on hover */ }
pub fn handle_toggle_keyboard(/* ... */) { /* Space/Enter key handling */ }

// Visual Update System - Apply styling changes
pub fn update_toggle_styling(/* ... */) { /* Update colors based on state */ }
```

## 3. Implementation Strategy

### 3.1 Phase 1: Core Toggle Component
**Tasks:**
1. Create `src/components/toggle.rs` with basic structure
2. Implement `Toggle` component and `ToggleBuilder`
3. Add basic styling calculation methods
4. Create simple toggle functionality without advanced features

**Acceptance Criteria:**
- Toggle can be created with ToggleBuilder
- Toggle responds to clicks and changes state
- Basic visual feedback (pressed/unpressed states)
- Integration with existing theme system

### 3.2 Phase 2: Visual Polish & Variants
**Tasks:**
1. Implement all three variants (Solid, Soft, Outline)
2. Add size support (Small, Default, Large)
3. Implement hover and active states
4. Add disabled state support

**Acceptance Criteria:**
- All variants render correctly
- Smooth state transitions
- Proper sizing for all size variants
- Disabled state prevents interaction

### 3.3 Phase 3: Accessibility & Advanced Features
**Tasks:**
1. Add keyboard navigation support
2. Implement focus management
3. Add optional icon and text support
4. Create comprehensive example

**Acceptance Criteria:**
- Full keyboard accessibility (Space/Enter activation)
- Focus indicators work properly
- Icon and text integration
- Working example in `examples/toggle.rs`

### 3.4 Phase 4: Testing & Documentation
**Tasks:**
1. Add component to public API exports
2. Create comprehensive examples
3. Test integration with existing components
4. Performance optimization

**Acceptance Criteria:**
- Component exported in `src/lib.rs`
- Example demonstrates all features
- No performance regressions
- Integration with form-like layouts

## 4. File Structure

### 4.1 New Files
```
src/components/toggle.rs          # Main implementation
examples/toggle.rs                # Usage examples
```

### 4.2 Modified Files
```
src/components/mod.rs             # Add toggle module export
src/lib.rs                        # Add Toggle to public API
src/plugin.rs                     # Add toggle systems to plugin
```

## 5. Design Decisions

### 5.1 State Management
- **Decision**: Use simple boolean `pressed` field rather than enum
- **Rationale**: Matches Radix UI API and simplifies usage
- **Alternative**: Could use `ToggleState::Pressed | Unpressed` enum

### 5.2 Visual Design Philosophy
- **Decision**: Reuse Button component styling patterns
- **Rationale**: Ensures visual consistency across components
- **Implementation**: Adapt Button's color calculation methods

### 5.3 Event Handling
- **Decision**: Use Bevy's Observer pattern for interactions
- **Rationale**: Consistent with Button component and modern Bevy practices
- **Events**: `TogglePressedEvent` for state changes

### 5.4 Accessibility Strategy
- **Decision**: Full keyboard navigation support
- **Implementation**: Space and Enter key activation
- **Focus**: Integrate with existing focus management system

## 6. Integration Points

### 6.1 Reused Components
- **Color System**: `UiColorPalette` from `src/theme/color/`
- **Layout System**: `UiLayout` from `src/theme/layout.rs`
- **Interaction Pattern**: Observer setup from `src/components/button.rs`
- **Typography**: `Text` component for optional labels

### 6.2 Theme Integration
- **Colors**: Use existing palette system with variants
- **Spacing**: Follow established padding and margin patterns
- **Radius**: Use consistent border radius values
- **Typography**: Support theme font families and sizes

## 7. Success Criteria

### 7.1 Functional Requirements
- [ ] Toggle state changes on user interaction
- [ ] Visual feedback for all interaction states
- [ ] Keyboard accessibility (Space/Enter activation)
- [ ] Disabled state prevents interaction
- [ ] Events fired on state changes

### 7.2 Quality Requirements
- [ ] Consistent with existing component patterns
- [ ] No performance regressions
- [ ] Full theme system integration
- [ ] Comprehensive example coverage
- [ ] Clean, maintainable code structure

### 7.3 Integration Requirements
- [ ] Works with existing layout components
- [ ] Integrates with form-like compositions
- [ ] Consistent API with other components
- [ ] Proper error handling

## 8. Risk Assessment

### 8.1 Technical Risks
- **Risk**: State synchronization between visual and logical state
- **Mitigation**: Use single source of truth in Toggle component
- **Risk**: Keyboard focus management complexity
- **Mitigation**: Reuse existing focus patterns from Button

### 8.2 Design Risks
- **Risk**: Visual inconsistency with other components
- **Mitigation**: Strict adherence to existing design patterns
- **Risk**: API complexity creep
- **Mitigation**: Keep API minimal and consistent with Radix UI

## 9. Future Enhancements

### 9.1 Potential Extensions
- **Toggle Group**: Multiple toggles in exclusive/inclusive groups
- **Animation**: Smooth state transition animations
- **Custom Icons**: Full icon system integration
- **Form Integration**: Automatic form value binding

### 9.2 Advanced Features
- **Controlled/Uncontrolled**: Support both usage patterns
- **Validation**: Integration with form validation systems
- **Theming**: Advanced theming beyond basic color variants

## 10. Implementation Checklist

### 10.1 Core Implementation
- [ ] Create `toggle.rs` file with basic structure
- [ ] Implement `Toggle` component struct
- [ ] Create `ToggleBuilder` with builder pattern
- [ ] Add basic styling calculations
- [ ] Implement click handling system

### 10.2 Visual Implementation
- [ ] Add all variant support (Solid, Soft, Outline)
- [ ] Implement size variants (Small, Default, Large)
- [ ] Add hover and active state styling
- [ ] Implement disabled state appearance

### 10.3 Integration
- [ ] Add to component module exports
- [ ] Add to main library exports
- [ ] Integrate with plugin systems
- [ ] Create comprehensive example

### 10.4 Testing & Polish
- [ ] Test all interaction modes
- [ ] Verify accessibility features
- [ ] Performance testing
- [ ] Documentation review

---

**Next Steps:** Begin Phase 1 implementation with core Toggle component structure and basic functionality.