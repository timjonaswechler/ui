# Box Component Research & Implementation Guide

## Overview

This document provides comprehensive research findings for implementing a Box component in the Bevy UI library, inspired by Radix UI's design principles and adapted for Bevy's ECS architecture.

## Table of Contents

1. [Radix UI Box Component Analysis](#radix-ui-box-component-analysis)
2. [Bevy UI Capabilities Assessment](#bevy-ui-capabilities-assessment) 
3. [Implementation Strategy](#implementation-strategy)
4. [API Design Recommendations](#api-design-recommendations)
5. [Performance Considerations](#performance-considerations)

---

## Radix UI Box Component Analysis

### Purpose and Design Philosophy

The Box component serves as the **fundamental layout building block** in Radix Themes, embodying core UI development principles:

- **Separation of Concerns**: Clearly separates layout responsibilities from content and interactivity
- **Composition Over Inheritance**: Designed to be composed with other components
- **Responsive-First**: Built with responsive design as a primary concern
- **Type Safety**: Fully typed API with predictable behavior

### Core Responsibilities

1. **Spacing Management**: Provides spacing control through padding/margin properties
2. **Size Constraints**: Imposes sizing constraints through width/height properties  
3. **Layout Control**: Controls behavior within flex/grid containers
4. **Responsive Visibility**: Conditional rendering based on viewport size

### Complete API Surface

#### Box-Specific Props
```typescript
interface BoxProps {
  as?: 'div' | 'span'           // Element type (default: 'div')
  asChild?: boolean             // Composition with child element
  display?: Responsive<Display> // Display behavior (unique to Box)
}
```

#### Shared Layout Props
The Box shares extensive layout props with other layout components:

**Padding Properties:**
- `p` - All sides padding
- `px` - Horizontal padding (left/right)
- `py` - Vertical padding (top/bottom)  
- `pt`, `pr`, `pb`, `pl` - Individual side padding

**Dimension Properties:**
- `width`, `minWidth`, `maxWidth` - Width constraints
- `height`, `minHeight`, `maxHeight` - Height constraints

**Positioning Properties:**
- `position` - CSS position type
- `inset` - All-sides offset
- `top`, `right`, `bottom`, `left` - Individual offsets

**Overflow Properties:**
- `overflow`, `overflowX`, `overflowY` - Overflow behavior

**Flex Child Properties:**
- `flexBasis`, `flexShrink`, `flexGrow` - Flex child behaviors

**Grid Child Properties:**
- `gridArea`, `gridColumn`, `gridRow` - Grid placement
- `gridColumnStart`, `gridColumnEnd` - Column boundaries
- `gridRowStart`, `gridRowEnd` - Row boundaries

**Margin Properties:**
- `m`, `mx`, `my`, `mt`, `mr`, `mb`, `ml` - Margin controls

### Usage Patterns

#### Basic Layout Container
```jsx
<Box width="64px" height="64px" p="4">
  <DecorativeBox />
</Box>
```

#### Responsive Design
```jsx
<Box 
  width={{ initial: '100%', md: 'auto' }}
  p={{ initial: '2', md: '4', lg: '6' }}
  display={{ initial: 'block', md: 'flex' }}
/>
```

#### Layout Integration
```jsx
<Box 
  position="relative"
  inset="4"
  flexGrow="1"
  gridArea="header"
/>
```

### Ergonomic Design Principles

1. **Responsive-First Design**: All layout props support responsive object values
2. **Type Safety**: Fully-typed API with consistent prop interfaces
3. **Flexible Value System**: Supports both theme scale and CSS values
4. **Consistent API Pattern**: Shared props across all layout components
5. **Composition-Friendly**: Seamless integration with other components

---

## Bevy UI Capabilities Assessment

### Node Struct - Core Layout Engine

Bevy's `Node` struct provides comprehensive layout capabilities equivalent to CSS flexbox and grid:

```rust
pub struct Node {
    // Size properties
    pub width: Val,
    pub height: Val,
    pub min_width: Val,
    pub min_height: Val,
    pub max_width: Val,
    pub max_height: Val,
    pub aspect_ratio: Option<f32>,
    
    // Box model
    pub margin: UiRect,
    pub padding: UiRect,
    pub border: UiRect,
    
    // Flexbox properties
    pub flex_direction: FlexDirection,
    pub flex_wrap: FlexWrap,
    pub flex_grow: f32,
    pub flex_shrink: f32,
    pub flex_basis: Val,
    pub row_gap: Val,
    pub column_gap: Val,
    
    // Alignment
    pub align_items: AlignItems,
    pub justify_items: JustifyItems,
    pub align_self: AlignSelf,
    pub justify_self: JustifySelf,
    pub align_content: AlignContent,
    pub justify_content: JustifyContent,
    
    // Grid properties
    pub grid_auto_flow: GridAutoFlow,
    pub grid_template_rows: Vec<RepeatedGridTrack>,
    pub grid_template_columns: Vec<RepeatedGridTrack>,
    pub grid_auto_rows: Vec<GridTrack>,
    pub grid_auto_columns: Vec<GridTrack>,
    pub grid_row: GridPlacement,
    pub grid_column: GridPlacement,
    
    // Display and overflow
    pub display: Display,
    pub overflow: Overflow,
    pub position_type: PositionType,
    pub left: Val,
    pub right: Val,
    pub top: Val,
    pub bottom: Val,
}
```

### Value System

Bevy supports flexible value types:
- `Val::Px(f32)` - Pixel values
- `Val::Percent(f32)` - Percentage values  
- `Val::Vw(f32)` - Viewport width percentage
- `Val::Vh(f32)` - Viewport height percentage
- `Val::VMin(f32)` - Minimum viewport dimension percentage
- `Val::VMax(f32)` - Maximum viewport dimension percentage
- `Val::Auto` - Automatic sizing

### Styling Components

**Core Styling System:**
- `BackgroundColor(Color)` - Background color control
- `BorderColor(Color)` - Border color control
- `BorderRadius` - Per-corner border radius control
- `BoxShadow` - Multiple shadow effects
- `Outline` - Outline styling

**Theme Integration:**
- `UiColorPalette` - Comprehensive color system
- `UiLayout` - Spacing and sizing constants
- `UiSpacing` - Consistent spacing scale

### Component Composition Patterns

Based on existing components (Button, Text), the preferred pattern:

```rust
pub struct ComponentBuilder {
    name: String,
    component_config: ComponentData,
    node: Node,
    styling: StylingOptions,
}

impl ComponentBuilder {
    pub fn build(self) -> impl Bundle {
        (
            Name::new(self.name),
            self.component_config,
            self.node,
            // ... styling components
        )
    }
}
```

---

## Implementation Strategy

### Component Architecture

**Core Component Structure:**
```rust
#[derive(Component, Debug, Clone)]
pub struct Box {
    pub variant: BoxVariant,
    pub color_palette: UiColorPalette,
    pub styling_config: BoxStyling,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoxVariant {
    Surface,    // Subtle background (default)
    Panel,      // More prominent background
    Card,       // Elevated appearance with shadow
    Outline,    // Border-only appearance
}

#[derive(Debug, Clone)]
pub struct BoxStyling {
    pub padding: Option<UiSpacing>,
    pub margin: Option<UiSpacing>,
    pub border_width: Option<f32>,
    pub border_radius: Option<BoxRadius>,
    pub background_alpha: f32,
}
```

**Builder Pattern Implementation:**
```rust
pub struct BoxBuilder {
    name: String,
    box_config: Box,
    node: Node,
    explicit_colors: ExplicitColors,
    children: Vec<Entity>,
}

#[derive(Default)]
struct ExplicitColors {
    background: Option<Color>,
    border: Option<Color>,
}
```

### Integration with Existing Systems

**Theme System Integration:**
- Use existing `UiColorPalette` for color management
- Integrate with `UiLayout` for consistent spacing
- Follow `accent_palette()` pattern for default colors

**Builder Pattern Consistency:**
- Implement `ComponentBuilder` trait
- Follow naming conventions from `ButtonBuilder`
- Maintain fluent API design

**Bundle Composition:**
- Return `impl Bundle` from `build()` method
- Include `Name` component for debugging
- Add `Pickable::default()` for interaction capability

---

## API Design Recommendations

### Primary Builder Methods

#### Size Control
```rust
impl BoxBuilder {
    // Dimension methods
    pub fn width(mut self, width: Val) -> Self
    pub fn height(mut self, height: Val) -> Self
    pub fn size(mut self, width: Val, height: Val) -> Self
    pub fn min_size(mut self, min_width: Val, min_height: Val) -> Self
    pub fn max_size(mut self, max_width: Val, max_height: Val) -> Self
    
    // Convenience methods
    pub fn square(self, size: Val) -> Self
    pub fn fill_width(self) -> Self
    pub fn fill_height(self) -> Self
    pub fn fill(self) -> Self
}
```

#### Spacing Control
```rust
impl BoxBuilder {
    // Padding methods
    pub fn padding(mut self, padding: Val) -> Self
    pub fn padding_x(mut self, padding: Val) -> Self
    pub fn padding_y(mut self, padding: Val) -> Self
    pub fn padding_top(mut self, padding: Val) -> Self
    pub fn padding_right(mut self, padding: Val) -> Self
    pub fn padding_bottom(mut self, padding: Val) -> Self
    pub fn padding_left(mut self, padding: Val) -> Self
    
    // Convenience methods using theme
    pub fn pad(mut self, level: SpacingLevel) -> Self
    pub fn pad_x(mut self, level: SpacingLevel) -> Self
    pub fn pad_y(mut self, level: SpacingLevel) -> Self
    
    // Margin methods (similar pattern)
    pub fn margin(mut self, margin: Val) -> Self
    // ... similar margin methods
}

#[derive(Debug, Clone, Copy)]
pub enum SpacingLevel {
    None,
    Xs,
    Sm,
    Base,
    Lg,
    Xl,
    X2l,
    X3l,
    X4l,
}
```

#### Visual Styling
```rust
impl BoxBuilder {
    // Variant control
    pub fn variant(mut self, variant: BoxVariant) -> Self
    pub fn surface(self) -> Self
    pub fn panel(self) -> Self  
    pub fn card(self) -> Self
    pub fn outline(self) -> Self
    
    // Color control
    pub fn color(mut self, palette: UiColorPalette) -> Self
    pub fn background_color(mut self, color: Color) -> Self
    pub fn border_color(mut self, color: Color) -> Self
    
    // Border radius control
    pub fn radius(mut self, radius: BoxRadius) -> Self
    pub fn rounded(self) -> Self
    pub fn rounded_sm(self) -> Self
    pub fn rounded_lg(self) -> Self
    pub fn rounded_full(self) -> Self
}
```

#### Positioning Control
```rust
impl BoxBuilder {
    // Position methods
    pub fn position_relative(mut self) -> Self
    pub fn position_absolute(mut self) -> Self
    pub fn position_offset(mut self, top: Val, right: Val, bottom: Val, left: Val) -> Self
    
    // Flex child methods
    pub fn flex_grow(mut self, grow: f32) -> Self
    pub fn flex_shrink(mut self, shrink: f32) -> Self
    pub fn flex_basis(mut self, basis: Val) -> Self
    pub fn flex_none(self) -> Self
    pub fn flex_auto(self) -> Self
    
    // Grid child methods
    pub fn grid_area(mut self, area: &str) -> Self
    pub fn grid_column(mut self, start: i16, end: i16) -> Self
    pub fn grid_row(mut self, start: i16, end: i16) -> Self
}
```

#### Interaction Control
```rust
impl BoxBuilder {
    // Overflow control
    pub fn overflow_hidden(mut self) -> Self
    pub fn overflow_scroll(mut self) -> Self
    pub fn overflow_auto(mut self) -> Self
    
    // Display control
    pub fn display_flex(mut self) -> Self
    pub fn display_grid(mut self) -> Self
    pub fn display_none(mut self) -> Self
    
    // Interaction
    pub fn hoverable(mut self) -> Self
    pub fn clickable(mut self) -> Self
}
```

### Ergonomic Design Features

#### Method Chaining Patterns
```rust
// Common patterns should be optimized for brevity
let box_component = Box::new("content")
    .card()
    .pad(SpacingLevel::Base)
    .rounded()
    .fill_width()
    .build();

// Advanced configuration remains explicit
let complex_box = Box::new("complex")
    .variant(BoxVariant::Panel)
    .padding_x(Val::Px(24.0))
    .padding_y(Val::Px(16.0))
    .border_color(Color::srgb(0.2, 0.2, 0.3))
    .radius(BoxRadius::Custom(8.0))
    .position_absolute()
    .position_offset(Val::Px(10.0), Val::Auto, Val::Auto, Val::Px(10.0))
    .build();
```

#### Theme Integration Helpers
```rust
impl BoxBuilder {
    // Theme-aware methods
    pub fn surface_1(self) -> Self  // Uses theme's surface-1 color
    pub fn surface_2(self) -> Self  // Uses theme's surface-2 color
    pub fn accent(self) -> Self     // Uses theme's accent color
    pub fn gray(self) -> Self       // Uses theme's gray color
    
    // Semantic variants
    pub fn success(self) -> Self    // Green color palette
    pub fn warning(self) -> Self    // Orange color palette
    pub fn error(self) -> Self      // Red color palette
    pub fn info(self) -> Self       // Blue color palette
}
```

---

## Performance Considerations

### Component Optimization

**Sparse Component Sets:**
- Only add styling components when explicitly set
- Use `Option<T>` for optional styling properties
- Leverage Bevy's change detection for updates

**Memory Efficiency:**
- Store common configurations in theme system
- Use `Default` implementations for common cases
- Avoid redundant data in component structs

**Bundle Composition:**
```rust
impl BoxBuilder {
    pub fn build(self) -> impl Bundle {
        let mut bundle = (
            Name::new(self.name),
            self.box_config,
            self.node,
            Pickable::default(),
        );
        
        // Only add styling components if explicitly set
        if let Some(bg_color) = self.explicit_colors.background {
            bundle = (bundle, BackgroundColor(bg_color));
        }
        
        if let Some(border_color) = self.explicit_colors.border {
            bundle = (bundle, BorderColor(border_color));
        }
        
        bundle
    }
}
```

### Layout Performance

**Efficient Layout Patterns:**
- Prefer relative positioning over absolute when possible
- Use percentage-based sizing for responsive layouts
- Minimize deep nesting of layout components

**Change Detection Optimization:**
- Use Bevy's change detection for styling updates
- Batch layout changes when possible
- Avoid unnecessary component updates

### System Integration

**Query Optimization:**
- Use specific component queries rather than broad queries
- Leverage Bevy's query filters for efficient updates
- Consider using `Changed<T>` filters for styling updates

**Event System Integration:**
- Follow existing event patterns from Button component
- Use Observer pattern for interaction events
- Minimize event propagation overhead

---

## Conclusion

The Box component implementation should leverage Bevy's comprehensive UI capabilities while maintaining Radix UI's ergonomic design principles. The proposed API design balances ease of use with powerful customization options, ensuring both beginner-friendly simple usage and advanced customization capabilities.

Key implementation priorities:
1. **Ergonomic API**: Fluent builder pattern with sensible defaults
2. **Theme Integration**: Seamless integration with existing color and spacing systems
3. **Performance**: Efficient component composition and change detection
4. **Consistency**: Following established patterns from Button and Text components
5. **Flexibility**: Supporting both simple and advanced use cases

This foundation will enable the Box component to serve as the cornerstone for all future layout components in the Bevy UI library.