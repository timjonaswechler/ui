# IconButton Component Implementation Plan

## Overview

This plan outlines the implementation of an IconButton component for the Radix UI-inspired component library for Bevy 0.16. The IconButton is a specialized button designed specifically for usage with a single icon, providing optimal styling for square buttons with consistent sizing and accessibility features.

## Research Summary

### Radix UI IconButton Specification

- **Purpose**: Button designed specifically for usage with a single icon
- **Key Features**: Size variants (1-4), visual variants (classic, solid, soft, surface, outline, ghost), color themes, high contrast option, loading state
- **Default Size**: Size "2" (medium) if not specified
- **Base Element**: Standard HTML button element with icon optimization
- **Accessibility**: Built-in ARIA support, keyboard navigation, focus management

### Existing Button Component Analysis

- **Architecture**: Component struct with Builder pattern, supports variants (Solid, Soft, Outline, Ghost)
- **Size System**: Small, Default, Large with automatic padding and text size scaling
- **State Management**: Normal, Hover, Active, Disabled, Loading states
- **Event System**: Uses Bevy Observer pattern with `ButtonClickEvent`
- **Theme Integration**: Full color palette support with high contrast options
- **Text Support**: Advanced TextBuilder integration with automatic contrast

### Icon Asset Management System

- **Comprehensive System**: Macro-generated icon categories with type safety
- **Size Support**: 16px (Small), 24px (Medium), 32px (Large), 64px (ExtraLarge)
- **Categories**: Interface (306 icons), Controllers, Loading system
- **Builder Pattern**: Namespace access (`Interface::ArrowRight::new()`) and component-based approach
- **Asset Loading**: Automatic texture atlas loading with multi-resolution support
- **Performance**: Zero-cost abstractions, efficient atlasing, shared texture resources

## Component Architecture

### Core Components

#### 1. IconButton (Main Component)

```rust
#[derive(Component, Debug, Clone)]
pub struct IconButton {
    pub variant: ButtonVariant,
    pub size: IconButtonSize,
    pub color: UiColorPalette,
    pub high_contrast: bool,
    pub radius: ButtonRadius,
    pub loading: bool,
    pub disabled: bool,
    pub current_state: ButtonState,
    pub icon_config: IconConfig,
}
```

#### 2. IconConfig

```rust
#[derive(Debug, Clone)]
pub struct IconConfig {
    pub icon_type: IconType,
    pub icon_size: Option<IconSize>,
    pub icon_tint: Option<Color>,
}

#[derive(Debug, Clone)]
pub enum IconType {
    Interface(InterfaceIconId),
    Controller(ControllersIconId),
    Custom { texture: Handle<Image>, index: usize },
}
```

### Enumerations

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum IconButtonSize {
    Size1, // 24px (Small)
    #[default]
    Size2, // 32px (Medium/Default)  
    Size3, // 40px (Large)
    Size4, // 48px (Extra Large)
}

// Reuse existing ButtonVariant, ButtonState, ButtonRadius from Button component
```

### Events

```rust
#[derive(Event, Debug, Clone)]
pub struct IconButtonClickEvent {
    pub button_entity: Entity,
    pub button_variant: ButtonVariant,
    pub icon_type: IconType,
}
```

## Builder Pattern Implementation

### IconButtonBuilder

```rust
pub struct IconButtonBuilder {
    name: String,
    button: IconButton,
    children: Vec<Entity>,
}

impl IconButtonBuilder {
    pub fn new(name: impl Into<String>) -> Self
    
    // Icon Configuration
    pub fn icon(mut self, icon_type: IconType) -> Self
    pub fn interface_icon(mut self, icon: InterfaceIconId) -> Self
    pub fn controller_icon(mut self, icon: ControllersIconId) -> Self
    pub fn custom_icon(mut self, texture: Handle<Image>, index: usize) -> Self
    
    // Icon Styling
    pub fn icon_size(mut self, size: IconSize) -> Self
    pub fn icon_tint(mut self, color: Color) -> Self
    pub fn auto_icon_size(mut self) -> Self // Derive from button size
    pub fn auto_icon_tint(mut self) -> Self // Derive from button theme
    
    // Button Configuration (inherited from Button)
    pub fn variant(mut self, variant: ButtonVariant) -> Self
    pub fn size(mut self, size: IconButtonSize) -> Self
    pub fn color(mut self, color: UiColorPalette) -> Self
    pub fn high_contrast(mut self) -> Self
    pub fn radius(mut self, radius: ButtonRadius) -> Self
    pub fn loading(mut self) -> Self
    pub fn disabled(mut self) -> Self
    
    // Accessibility
    pub fn aria_label(mut self, label: impl Into<String>) -> Self
    pub fn tooltip(mut self, text: impl Into<String>) -> Self
    
    pub fn build(self) -> impl Bundle
}
```

### Convenience Builders

```rust
// Direct icon-based builders
impl IconButtonBuilder {
    pub fn with_interface_icon(name: impl Into<String>, icon: InterfaceIconId) -> Self {
        Self::new(name).interface_icon(icon)
    }
    
    pub fn with_controller_icon(name: impl Into<String>, icon: ControllersIconId) -> Self {
        Self::new(name).controller_icon(icon)
    }
}

// Pre-configured common buttons
impl IconButtonBuilder {
    pub fn close_button() -> Self {
        Self::new("close")
            .interface_icon(InterfaceIconId::Cross2)
            .variant(ButtonVariant::Ghost)
            .size(IconButtonSize::Size2)
    }
    
    pub fn menu_button() -> Self {
        Self::new("menu")
            .interface_icon(InterfaceIconId::HamburgerMenu)
            .variant(ButtonVariant::Ghost)
            .size(IconButtonSize::Size2)
    }
    
    pub fn back_button() -> Self {
        Self::new("back")
            .interface_icon(InterfaceIconId::ArrowLeft)
            .variant(ButtonVariant::Ghost)
            .size(IconButtonSize::Size2)
    }
    
    pub fn search_button() -> Self {
        Self::new("search")
            .interface_icon(InterfaceIconId::MagnifyingGlass)
            .variant(ButtonVariant::Outline)
            .size(IconButtonSize::Size2)
    }
}
```

## Icon Integration System

### Icon Resolution System

```rust
impl IconButtonBuilder {
    fn resolve_icon_size(&self) -> IconSize {
        if let Some(size) = self.button.icon_config.icon_size {
            size
        } else {
            // Auto-derive from button size
            match self.button.size {
                IconButtonSize::Size1 => IconSize::Small,    // 16px
                IconButtonSize::Size2 => IconSize::Medium,   // 24px
                IconButtonSize::Size3 => IconSize::Large,    // 32px  
                IconButtonSize::Size4 => IconSize::ExtraLarge, // 64px
            }
        }
    }
    
    fn resolve_icon_tint(&self) -> Color {
        if let Some(tint) = self.button.icon_config.icon_tint {
            tint
        } else {
            // Auto-derive from button theme and state
            let styling = self.button.get_styling(ButtonState::Normal);
            styling.text_color.0 // Use same color as text would be
        }
    }
}
```

### Icon Spawning Integration

```rust
impl IconButtonBuilder {
    pub fn build(self) -> impl Bundle {
        let node = self.calculate_style();
        let background_color = self.calculate_background_color();
        let border_color = self.calculate_border_color();
        let border_radius = self.calculate_border_radius();
        
        let icon_size = self.resolve_icon_size();
        let icon_tint = self.resolve_icon_tint();
        let is_loading = self.button.loading;
        
        (
            Name::new(format!("{}_IconButton", self.name)),
            self.button,
            node,
            border_color,
            border_radius,
            background_color,
            Pickable::default(),
            Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
                if is_loading {
                    // Use existing spinner from Button component
                    parent.spawn((
                        Name::new("IconButton Spinner"),
                        Node {
                            width: Val::Px(icon_size as u8 as f32),
                            height: Val::Px(icon_size as u8 as f32),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        SpinnerAnimation::default(),
                    ));
                } else {
                    // Spawn the actual icon
                    parent.spawn((
                        Name::new("IconButton Icon"),
                        IconButtonIcon {
                            button_entity: parent.parent_entity(),
                            icon_config: self.button.icon_config.clone(),
                        },
                        Node {
                            width: Val::Px(icon_size as u8 as f32),
                            height: Val::Px(icon_size as u8 as f32),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                    ));
                }
            })),
        )
    }
}
```

## System Implementation

### Core IconButton Systems

#### 1. IconButton Setup System

```rust
pub fn setup_iconbutton_interactions(
    mut commands: Commands,
    buttons: Query<Entity, Added<IconButton>>,
    button_query: Query<&IconButton>,
    mut bg_colors: Query<&mut BackgroundColor>,
    children_query: Query<&Children>,
    icon_query: Query<&IconButtonIcon>,
) {
    for entity in &buttons {
        // Reuse existing button interaction setup
        commands
            .entity(entity)
            .observe(on_iconbutton_click)
            .observe(on_iconbutton_hover)
            .observe(on_iconbutton_hover_out)
            .observe(on_iconbutton_pressed)
            .observe(on_iconbutton_released);

        // Apply initial styling
        if let Ok(button) = button_query.get(entity) {
            apply_iconbutton_styling(
                entity,
                button,
                ButtonState::Normal,
                &mut bg_colors,
                &children_query,
                &icon_query,
            );
        }
    }
}
```

#### 2. Icon Resolution System

```rust
#[derive(Component, Debug, Clone)]
pub struct IconButtonIcon {
    pub button_entity: Entity,
    pub icon_config: IconConfig,
}

pub fn resolve_iconbutton_icons(
    mut commands: Commands,
    icon_query: Query<(Entity, &IconButtonIcon), Added<IconButtonIcon>>,
    button_query: Query<&IconButton>,
    interface_atlases: Option<Res<InterfaceAtlases>>,
    controllers_atlases: Option<Res<ControllersAtlases>>,
) {
    for (icon_entity, icon_button_icon) in icon_query.iter() {
        if let Ok(button) = button_query.get(icon_button_icon.button_entity) {
            let icon_size = resolve_icon_size_for_button(button, &icon_button_icon.icon_config);
            let icon_tint = resolve_icon_tint_for_button(button, &icon_button_icon.icon_config);
            
            match &icon_button_icon.icon_config.icon_type {
                IconType::Interface(icon_id) => {
                    if let Some(atlases) = &interface_atlases {
                        let icon_component = InterfaceIcon::new(*icon_id)
                            .size(icon_size)
                            .tint(icon_tint);
                        
                        commands.entity(icon_entity)
                            .insert(icon_component.bundle(atlases));
                    }
                }
                IconType::Controller(icon_id) => {
                    if let Some(atlases) = &controllers_atlases {
                        let icon_component = ControllersIcon::new(*icon_id)
                            .size(icon_size)
                            .tint(icon_tint);
                        
                        commands.entity(icon_entity)
                            .insert(icon_component.bundle(atlases));
                    }
                }
                IconType::Custom { texture, index } => {
                    commands.entity(icon_entity).insert((
                        ImageNode::new(texture.clone()),
                        TextureAtlas {
                            layout: Handle::default(), // Would need layout handle
                            index: *index,
                        },
                        Node {
                            width: icon_size.as_val(),
                            height: icon_size.as_val(),
                            ..default()
                        },
                    ));
                }
            }
        }
    }
}
```

#### 3. IconButton State Management System

```rust
pub fn iconbutton_state_system(
    mut icon_query: Query<(Entity, &IconButtonIcon, &mut BackgroundColor), With<IconButtonIcon>>,
    button_query: Query<&IconButton, Changed<IconButton>>,
    interface_icons: Query<&InterfaceIcon>,
    controllers_icons: Query<&ControllersIcon>,
) {
    for (icon_entity, icon_button_icon, mut bg_color) in icon_query.iter_mut() {
        if let Ok(button) = button_query.get(icon_button_icon.button_entity) {
            // Update icon tint based on button state
            let new_tint = resolve_icon_tint_for_button(button, &icon_button_icon.icon_config);
            
            // Update the icon's tint by modifying its ImageNode color
            // This would require access to the icon's ImageNode component
            *bg_color = BackgroundColor(new_tint);
        }
    }
}
```

## Theme Integration

### Size Calculation

```rust
impl IconButtonBuilder {
    fn calculate_style(&self) -> Node {
        let layout = UiLayout::default();
        
        // Calculate padding to make button square
        let (button_size, padding_adjustment) = match self.button.size {
            IconButtonSize::Size1 => (28.0, 6.0),  // 16px icon + 12px padding
            IconButtonSize::Size2 => (40.0, 8.0),  // 24px icon + 16px padding
            IconButtonSize::Size3 => (48.0, 8.0),  // 32px icon + 16px padding
            IconButtonSize::Size4 => (64.0, 8.0),  // 64px icon + 16px padding (special case)
        };
        
        let padding = UiRect::all(Val::Px(padding_adjustment));
        
        Node {
            width: Val::Px(button_size),
            height: Val::Px(button_size),
            padding,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(1.0)),
            ..default()
        }
    }
}
```

### Color Integration

```rust
impl IconButton {
    pub fn get_styling(&self, state: ButtonState) -> IconButtonStyling {
        // Reuse Button styling logic but ensure icon tint coordination
        let button_styling = Button::get_styling(self, state);
        
        IconButtonStyling {
            background_color: button_styling.background_color,
            border_color: button_styling.border_color,
            icon_tint: button_styling.text_color.0, // Use text color for icon
        }
    }
}

#[derive(Debug, Clone)]
pub struct IconButtonStyling {
    pub background_color: BackgroundColor,
    pub border_color: BorderColor,
    pub icon_tint: Color,
}
```

## Accessibility Implementation

### ARIA Support

```rust
#[derive(Component, Debug, Clone)]
pub struct IconButtonAccessibility {
    pub aria_label: Option<String>,
    pub tooltip_text: Option<String>,
    pub role: String,
}

impl Default for IconButtonAccessibility {
    fn default() -> Self {
        Self {
            aria_label: None,
            tooltip_text: None,
            role: "button".to_string(),
        }
    }
}
```

### Keyboard Navigation

```rust
pub fn iconbutton_keyboard_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut iconbutton_query: Query<(Entity, &mut IconButton, &IconButtonAccessibility)>,
    mut events: EventWriter<IconButtonClickEvent>,
) {
    // Handle Enter and Space key activation
    if keyboard_input.just_pressed(KeyCode::Enter) || keyboard_input.just_pressed(KeyCode::Space) {
        // Trigger click for focused icon button
        for (entity, button, accessibility) in iconbutton_query.iter() {
            if button.current_state == ButtonState::Focused {
                events.write(IconButtonClickEvent {
                    button_entity: entity,
                    button_variant: button.variant,
                    icon_type: button.icon_config.icon_type.clone(),
                });
            }
        }
    }
}
```

## Performance Optimizations

### Icon Caching

```rust
#[derive(Resource, Default)]
pub struct IconButtonCache {
    pub resolved_icons: HashMap<(IconType, IconSize, Color), Entity>,
}

pub fn cache_iconbutton_icons(
    mut cache: ResMut<IconButtonCache>,
    icon_query: Query<(Entity, &IconButtonIcon)>,
) {
    // Cache frequently used icon configurations to avoid repeated resolution
    for (entity, icon_button_icon) in icon_query.iter() {
        let key = (
            icon_button_icon.icon_config.icon_type.clone(),
            icon_button_icon.icon_config.icon_size.unwrap_or(IconSize::Medium),
            icon_button_icon.icon_config.icon_tint.unwrap_or(Color::WHITE),
        );
        cache.resolved_icons.insert(key, entity);
    }
}
```

### Batch Icon Updates

```rust
pub fn batch_iconbutton_updates(
    mut icon_query: Query<&mut IconButtonIcon, Changed<IconButton>>,
    button_query: Query<&IconButton>,
) {
    // Batch update icon properties when button state changes
    let mut updates = Vec::new();
    
    for mut icon_button_icon in icon_query.iter_mut() {
        if let Ok(button) = button_query.get(icon_button_icon.button_entity) {
            let new_tint = resolve_icon_tint_for_button(button, &icon_button_icon.icon_config);
            updates.push((icon_button_icon.as_mut(), new_tint));
        }
    }
    
    // Apply all updates at once
    for (icon_button_icon, new_tint) in updates {
        icon_button_icon.icon_config.icon_tint = Some(new_tint);
    }
}
```

## Integration Points

### Button Component Inheritance

```rust
impl From<IconButton> for Button {
    fn from(icon_button: IconButton) -> Self {
        Button {
            variant: icon_button.variant,
            size: match icon_button.size {
                IconButtonSize::Size1 => ButtonSize::Small,
                IconButtonSize::Size2 => ButtonSize::Default,
                IconButtonSize::Size3 => ButtonSize::Large,
                IconButtonSize::Size4 => ButtonSize::Large, // Map to Large
            },
            color: icon_button.color,
            high_contrast: icon_button.high_contrast,
            radius: icon_button.radius,
            loading: icon_button.loading,
            disabled: icon_button.disabled,
            current_state: icon_button.current_state,
        }
    }
}
```

### Existing System Integration

```rust
// Reuse Button interaction systems with minimal changes
fn on_iconbutton_click(
    trigger: Trigger<Pointer<Click>>,
    buttons: Query<&IconButton>,
    mut events: EventWriter<IconButtonClickEvent>,
    mut commands: Commands,
    sfx_assets: Res<SfxAssets>,
) {
    let entity = trigger.target();
    if let Ok(button) = buttons.get(entity) {
        if button.disabled || button.loading {
            return;
        }

        info!("IconButton clicked! Variant: {:?}, Icon: {:?}", button.variant, button.icon_config.icon_type);

        // Play tap sound effect (reuse from Button)
        commands.spawn(sound_effect(sfx_assets.tap.clone()));

        // Send custom event
        events.write(IconButtonClickEvent {
            button_entity: entity,
            button_variant: button.variant,
            icon_type: button.icon_config.icon_type.clone(),
        });
    }
}
```

## Implementation Phases

### Phase 1: Core Component Structure

- [ ] Define IconButton component and enums
- [ ] Implement basic IconButtonBuilder
- [ ] Create icon configuration system
- [ ] Set up bundle creation logic

### Phase 2: Icon Integration

- [ ] Implement icon type system and resolution
- [ ] Create icon spawning and management
- [ ] Add automatic size and tint derivation
- [ ] Integrate with existing icon asset system

### Phase 3: Button System Integration

- [ ] Extend Button interaction systems for IconButton
- [ ] Implement state management and styling
- [ ] Add event system integration
- [ ] Create visual feedback systems

### Phase 4: Convenience and Accessibility

- [ ] Add pre-configured button builders
- [ ] Implement accessibility features
- [ ] Add keyboard navigation support
- [ ] Create tooltip and ARIA label systems

### Phase 5: Performance and Polish

- [ ] Implement icon caching system
- [ ] Add batch update optimizations
- [ ] Create comprehensive examples
- [ ] Write documentation and usage guides

## Example Usage

### Basic IconButton

```rust
fn setup_iconbutton_example(mut commands: Commands) {
    // Simple close button
    commands.spawn(
        IconButton::new("close")
            .interface_icon(InterfaceIconId::Cross2)
            .variant(ButtonVariant::Ghost)
            .size(IconButtonSize::Size2)
            .build()
    );

    // Search button with custom styling
    commands.spawn(
        IconButton::new("search")
            .interface_icon(InterfaceIconId::MagnifyingGlass)
            .variant(ButtonVariant::Outline)
            .size(IconButtonSize::Size3)
            .color(accent_palette())
            .high_contrast()
            .build()
    );

    // Loading button
    commands.spawn(
        IconButton::new("loading")
            .interface_icon(InterfaceIconId::Reload)
            .variant(ButtonVariant::Solid)
            .loading()
            .build()
    );
}
```

### Pre-configured Buttons

```rust
fn setup_preconfig_buttons(mut commands: Commands) {
    // Use convenience builders
    commands.spawn(IconButton::close_button().build());
    commands.spawn(IconButton::menu_button().build());
    commands.spawn(IconButton::back_button().build());
    commands.spawn(IconButton::search_button().build());
}
```

### Custom Icon Integration

```rust
fn setup_custom_iconbuttons(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let custom_texture = asset_server.load("icons/my_custom_icon.png");
    
    commands.spawn(
        IconButton::new("custom")
            .custom_icon(custom_texture, 0)
            .variant(ButtonVariant::Soft)
            .size(IconButtonSize::Size4)
            .build()
    );
}
```

### Accessibility-Enhanced Buttons

```rust
fn setup_accessible_iconbuttons(mut commands: Commands) {
    commands.spawn(
        IconButton::new("settings")
            .interface_icon(InterfaceIconId::Gear)
            .variant(ButtonVariant::Ghost)
            .aria_label("Open Settings")
            .tooltip("Configure application settings")
            .build()
    );
}
```

## Success Criteria

1. **API Consistency**: Follows established builder pattern and component architecture
2. **Button Integration**: Seamless inheritance from existing Button component systems
3. **Icon System Integration**: Full compatibility with existing icon asset management
4. **Performance**: Efficient icon resolution and rendering without frame drops
5. **Theme Integration**: Complete design system integration with automatic styling
6. **Accessibility**: Proper ARIA support and keyboard navigation
7. **Size Optimization**: Perfect square buttons with consistent icon scaling
8. **Developer Experience**: Intuitive API with convenience builders and clear examples

This implementation plan provides a comprehensive roadmap for creating a production-ready IconButton component that leverages the existing Button architecture while providing specialized functionality for icon-based interactions with seamless integration into the established design system.