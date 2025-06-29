use crate::{
    assets::{Check, Interface},
    theme::{
        color::{accent_palette, UiColorPalette},
        layout::UiLayout,
    },
};
use bevy::prelude::*;
use bevy_picking::prelude::Pickable;

/// Checkbox click event
#[derive(Event, Debug, Clone)]
pub struct CheckboxChangeEvent {
    pub checkbox_entity: Entity,
    pub checked: bool,
    pub size: CheckboxSize,
}

/// Checkbox interaction state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckboxState {
    Normal,
    Hover,
    Active,
    Disabled,
}

/// Checkbox component - binary selection control inspired by Radix UI
///
/// The Checkbox component provides a binary selection interface with visual feedback
/// and keyboard support. Follows WAI-ARIA checkbox pattern for accessibility.
#[derive(Component, Debug, Clone)]
pub struct CheckboxComponent {
    pub size: CheckboxSize,
    pub checked: bool,
    pub disabled: bool,
    pub color_palette: UiColorPalette,
    pub current_state: CheckboxState,
}

impl Default for CheckboxComponent {
    fn default() -> Self {
        Self {
            size: CheckboxSize::Size2,
            checked: false,
            disabled: false,
            color_palette: accent_palette(),
            current_state: CheckboxState::Normal,
        }
    }
}

/// Size variants for Checkbox component
///
/// Provides standardized sizing following Radix UI specifications:
/// - Size1: 16px (compact)
/// - Size2: 20px (default)
/// - Size3: 24px (large)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum CheckboxSize {
    /// 16px - Compact size for dense layouts
    Size1,
    /// 20px - Default size for most use cases
    #[default]
    Size2,
    /// 24px - Large size for better accessibility
    Size3,
}

impl CheckboxSize {
    /// Convert size to pixel dimensions
    pub fn to_pixels(self) -> f32 {
        match self {
            CheckboxSize::Size1 => 16.0,
            CheckboxSize::Size2 => 20.0,
            CheckboxSize::Size3 => 24.0,
        }
    }

    /// Get appropriate checkmark size (80% of checkbox size)
    pub fn checkmark_size(self) -> f32 {
        self.to_pixels() * 0.8
    }

    /// Get border width for the checkbox
    pub fn border_width(self) -> f32 {
        match self {
            CheckboxSize::Size1 => 1.0,
            CheckboxSize::Size2 => 1.5,
            CheckboxSize::Size3 => 2.0,
        }
    }
}

/// Builder for creating Checkbox components with fluent API
pub struct CheckboxBuilder {
    name: String,
    checkbox_config: CheckboxComponent,
    node: Node,
}

impl CheckboxComponent {
    /// Create a new Checkbox component builder
    pub fn new(name: impl Into<String>) -> CheckboxBuilder {
        CheckboxBuilder::new(name)
    }

    /// Create a Size1 (16px) checkbox
    pub fn size_1(name: impl Into<String>) -> CheckboxBuilder {
        Self::new(name).size_1()
    }

    /// Create a Size2 (20px) checkbox (default)
    pub fn size_2(name: impl Into<String>) -> CheckboxBuilder {
        Self::new(name).size_2()
    }

    /// Create a Size3 (24px) checkbox
    pub fn size_3(name: impl Into<String>) -> CheckboxBuilder {
        Self::new(name).size_3()
    }

    /// Create a checkbox with initial checked state
    pub fn checked(name: impl Into<String>) -> CheckboxBuilder {
        Self::new(name).checked()
    }
}

impl CheckboxBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        let checkbox_config = CheckboxComponent::default();
        let size_px = checkbox_config.size.to_pixels();

        Self {
            name: format!("{}_Checkbox", name.into()),
            checkbox_config: checkbox_config.clone(),
            node: Node {
                width: Val::Px(size_px),
                height: Val::Px(size_px),
                border: UiRect::all(Val::Px(checkbox_config.size.border_width())),
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        }
    }

    // =========================================================================
    // SIZE METHODS
    // =========================================================================

    /// Set checkbox size
    pub fn size(mut self, size: CheckboxSize) -> Self {
        self.checkbox_config.size = size;
        let size_px = size.to_pixels();
        self.node.width = Val::Px(size_px);
        self.node.height = Val::Px(size_px);
        self.node.border = UiRect::all(Val::Px(size.border_width()));
        self
    }

    /// Use Size1 (16px) - compact
    pub fn size_1(self) -> Self {
        self.size(CheckboxSize::Size1)
    }

    /// Use Size2 (20px) - default
    pub fn size_2(self) -> Self {
        self.size(CheckboxSize::Size2)
    }

    /// Use Size3 (24px) - large
    pub fn size_3(self) -> Self {
        self.size(CheckboxSize::Size3)
    }

    // =========================================================================
    // STATE METHODS
    // =========================================================================

    /// Set initial checked state
    pub fn checked(mut self) -> Self {
        self.checkbox_config.checked = true;
        self
    }

    /// Set checkbox as disabled
    pub fn disabled(mut self) -> Self {
        self.checkbox_config.disabled = true;
        self.checkbox_config.current_state = CheckboxState::Disabled;
        self
    }

    // =========================================================================
    // STYLING METHODS
    // =========================================================================

    /// Set color palette for theming
    pub fn color(mut self, palette: UiColorPalette) -> Self {
        self.checkbox_config.color_palette = palette;
        self
    }

    /// Use accent color palette
    pub fn accent(self) -> Self {
        self.color(accent_palette())
    }

    // =========================================================================
    // LAYOUT METHODS
    // =========================================================================

    /// Set margin around the checkbox
    pub fn margin(mut self, margin: Val) -> Self {
        self.node.margin = UiRect::all(margin);
        self
    }

    /// Set horizontal margin
    pub fn margin_x(mut self, margin: Val) -> Self {
        self.node.margin.left = margin;
        self.node.margin.right = margin;
        self
    }

    /// Set vertical margin
    pub fn margin_y(mut self, margin: Val) -> Self {
        self.node.margin.top = margin;
        self.node.margin.bottom = margin;
        self
    }

    // =========================================================================
    // BUILD HELPERS
    // =========================================================================

    /// Calculate background color based on state
    fn calculate_background_color(&self) -> BackgroundColor {
        let palette = &self.checkbox_config.color_palette;

        if self.checkbox_config.disabled {
            return BackgroundColor(palette.bg_subtle.with_alpha(0.5));
        }

        match (
            self.checkbox_config.checked,
            self.checkbox_config.current_state,
        ) {
            (true, CheckboxState::Normal) => BackgroundColor(palette.solid),
            (true, CheckboxState::Hover) => BackgroundColor(palette.solid_hover),
            (true, CheckboxState::Active) => BackgroundColor(palette.solid_hover),
            (false, CheckboxState::Normal) => BackgroundColor(Color::NONE),
            (false, CheckboxState::Hover) => BackgroundColor(palette.bg_hover),
            (false, CheckboxState::Active) => BackgroundColor(palette.bg_active),
            _ => BackgroundColor(Color::NONE),
        }
    }

    /// Calculate border color based on state
    fn calculate_border_color(&self) -> BorderColor {
        let palette = &self.checkbox_config.color_palette;

        if self.checkbox_config.disabled {
            return BorderColor(palette.border.with_alpha(0.5));
        }

        match (
            self.checkbox_config.checked,
            self.checkbox_config.current_state,
        ) {
            (true, _) => BorderColor(palette.solid),
            (false, CheckboxState::Normal) => BorderColor(palette.border),
            (false, CheckboxState::Hover) => BorderColor(palette.border_hover),
            (false, CheckboxState::Active) => BorderColor(palette.solid),
            _ => BorderColor(palette.border),
        }
    }

    /// Calculate border radius
    fn calculate_border_radius(&self) -> BorderRadius {
        let radius = UiLayout::default().radius.sm;
        BorderRadius::all(Val::Px(radius))
    }
}

impl CheckboxBuilder {
    pub fn build(self) -> impl Bundle {
        let background_color = self.calculate_background_color();
        let border_color = self.calculate_border_color();
        let border_radius = self.calculate_border_radius();

        (
            Name::new(self.name),
            self.checkbox_config,
            self.node,
            background_color,
            border_color,
            border_radius,
            Pickable::default(),
            Button,
            Interaction::None,
        )
    }
}

// =========================================================================
// INTERACTION SYSTEMS
// =========================================================================

/// System to handle checkbox interactions
pub fn handle_checkbox_interactions(
    mut interaction_query: Query<
        (
            Entity,
            &Interaction,
            &mut CheckboxComponent,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        (With<CheckboxComponent>, Changed<Interaction>),
    >,
    mut checkbox_events: EventWriter<CheckboxChangeEvent>,
) {
    for (entity, interaction, mut checkbox, mut bg_color, mut border_color) in
        &mut interaction_query
    {
        if checkbox.disabled {
            continue;
        }

        let old_state = checkbox.current_state;
        let old_checked = checkbox.checked;

        // Update state based on interaction
        checkbox.current_state = match *interaction {
            Interaction::Hovered => CheckboxState::Hover,
            Interaction::Pressed => CheckboxState::Active,
            Interaction::None => CheckboxState::Normal,
        };

        // Toggle checked state on click (when releasing click)
        // Check for both Active->Normal and Active->Hover transitions (both indicate release)
        if old_state == CheckboxState::Active
            && (checkbox.current_state == CheckboxState::Normal
                || checkbox.current_state == CheckboxState::Hover)
        {
            checkbox.checked = !checkbox.checked;

            // Send change event
            checkbox_events.write(CheckboxChangeEvent {
                checkbox_entity: entity,
                checked: checkbox.checked,
                size: checkbox.size,
            });
        }

        // Update visual appearance if state changed
        if old_state != checkbox.current_state || old_checked != checkbox.checked {
            let palette = &checkbox.color_palette;

            // Update background color
            *bg_color = match (checkbox.checked, checkbox.current_state) {
                (true, CheckboxState::Normal) => BackgroundColor(palette.solid),
                (true, CheckboxState::Hover) => BackgroundColor(palette.solid_hover),
                (true, CheckboxState::Active) => BackgroundColor(palette.solid_hover),
                (false, CheckboxState::Normal) => BackgroundColor(Color::NONE),
                (false, CheckboxState::Hover) => BackgroundColor(palette.bg_hover),
                (false, CheckboxState::Active) => BackgroundColor(palette.bg_active),
                _ => BackgroundColor(Color::NONE),
            };

            // Update border color
            *border_color = match (checkbox.checked, checkbox.current_state) {
                (true, _) => BorderColor(palette.solid),
                (false, CheckboxState::Normal) => BorderColor(palette.border),
                (false, CheckboxState::Hover) => BorderColor(palette.border_hover),
                (false, CheckboxState::Active) => BorderColor(palette.solid),
                _ => BorderColor(palette.border),
            };
        }
    }
}

/// System to spawn checkmarks for checked checkboxes
pub fn spawn_checkmarks(
    mut commands: Commands,
    checkbox_query: Query<(Entity, &CheckboxComponent), Added<CheckboxComponent>>,
) {
    for (entity, checkbox) in &checkbox_query {
        if checkbox.checked {
            spawn_checkmark_with_atlas(&mut commands, entity);
        }
    }
}

/// System to update checkmarks when checkbox state changes
pub fn update_checkmarks(
    mut commands: Commands,
    checkbox_query: Query<
        (Entity, &CheckboxComponent, Option<&Children>),
        Changed<CheckboxComponent>,
    >,
    checkmark_query: Query<Entity, With<CheckmarkComponent>>,
) {
    for (checkbox_entity, checkbox, children) in &checkbox_query {
        // Find existing checkmark among children
        let existing_checkmark = if let Some(children) = children {
            children
                .iter()
                .find(|&child| checkmark_query.contains(child))
        } else {
            None
        };

        if checkbox.checked {
            // Spawn checkmark if it doesn't exist
            if existing_checkmark.is_none() {
                spawn_checkmark_with_atlas(&mut commands, checkbox_entity);
            }
        } else {
            // Remove checkmark if it exists
            if let Some(checkmark_entity) = existing_checkmark {
                commands.entity(checkmark_entity).despawn();
            }
        }
    }
}

/// Marker component for checkmarks
#[derive(Component)]
pub struct CheckmarkComponent;

/// Helper function to spawn a checkmark icon from the texture atlas
fn spawn_checkmark_with_atlas(commands: &mut Commands, parent_entity: Entity) {
    // Use the Check icon from the interface font
    let icon = Interface::new(Check).color(crate::theme::color::TextColor::Accent); // White checkmark on colored background

    commands.entity(parent_entity).with_children(|parent| {
        // Create the icon bundle

        let mut entity_commands = parent.spawn((
            Name::new("Checkmark"),
            CheckmarkComponent,
            icon.build(),
            Pickable::IGNORE,
        ));

        // Update the existing Node from the icon bundle to position it
        entity_commands.insert(Node {
            position_type: PositionType::Absolute,
            top: Val::Px(0.0),
            left: Val::Px(0.0),
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            display: Display::Flex,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        });
    });
}

// Convenience type alias
pub type Checkbox = CheckboxComponent;
