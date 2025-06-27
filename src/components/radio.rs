use crate::{
    theme::{
        color::{accent_palette, UiColorPalette},
    },
};
use bevy::prelude::*;
use bevy_picking::prelude::Pickable;

/// Radio button click event
#[derive(Event, Debug, Clone)]
pub struct RadioChangeEvent {
    pub group_entity: Entity,
    pub group_name: String,
    pub selected_value: String,
    pub previous_value: Option<String>,
    pub radio_entity: Entity,
}

/// RadioGroup value change event
#[derive(Event, Debug, Clone)]
pub struct RadioGroupValueChangeEvent {
    pub group_entity: Entity,
    pub group_name: String,
    pub new_value: Option<String>,
    pub old_value: Option<String>,
}

/// Radio interaction state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RadioState {
    Normal,
    Hover,
    Active,
    Disabled,
}

/// Radio button orientation for groups
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RadioOrientation {
    Vertical,   // Default - column layout
    Horizontal, // Row layout
}

impl Default for RadioOrientation {
    fn default() -> Self {
        Self::Vertical
    }
}

/// Selection mode for radio groups
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionMode {
    /// Single selection - only one radio can be selected at a time, but can be deselected
    Single,
    /// Multiple selection - multiple radios can be selected and deselected independently
    Multiple,
}

impl Default for SelectionMode {
    fn default() -> Self {
        Self::Single
    }
}

/// Radio button visual variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum RadioVariant {
    /// Default background with gray border
    #[default]
    Surface,
    /// Gradient and shadow effects
    Classic,
    /// Accent-based background colors
    Soft,
}

/// Size variants for Radio component
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum RadioSize {
    /// 14px - Compact size for dense layouts
    Size1,
    /// 16px - Default size for most use cases
    #[default]
    Size2,
    /// 20px - Large size for better accessibility
    Size3,
}

impl RadioSize {
    /// Convert size to pixel dimensions
    pub fn to_pixels(self) -> f32 {
        match self {
            RadioSize::Size1 => 14.0,
            RadioSize::Size2 => 16.0,
            RadioSize::Size3 => 20.0,
        }
    }

    /// Get indicator dot size (40% of radio size)
    pub fn indicator_size(self) -> f32 {
        self.to_pixels() * 0.4
    }

    /// Get border width for the radio
    pub fn border_width(self) -> f32 {
        match self {
            RadioSize::Size1 => 1.0,
            RadioSize::Size2 => 1.5,
            RadioSize::Size3 => 2.0,
        }
    }
}

/// Individual Radio Component
#[derive(Component, Debug, Clone)]
pub struct RadioComponent {
    pub size: RadioSize,
    pub variant: RadioVariant,
    pub checked: bool,
    pub disabled: bool,
    pub color_palette: UiColorPalette,
    pub current_state: RadioState,
    pub value: String,
    pub group_entity: Option<Entity>,
    // Removed allow_deselect - ALL radios are toggle-able by default
}

impl Default for RadioComponent {
    fn default() -> Self {
        Self {
            size: RadioSize::Size2,
            variant: RadioVariant::Surface,
            checked: false,
            disabled: false,
            color_palette: accent_palette(),
            current_state: RadioState::Normal,
            value: String::new(),
            group_entity: None,
        }
    }
}

/// RadioGroup Container Component
#[derive(Component, Debug, Clone)]
pub struct RadioGroupComponent {
    pub name: String,
    pub selected_value: Option<String>,
    pub selected_values: Vec<String>, // For multiple selection mode
    pub default_value: Option<String>,
    pub disabled: bool,
    pub required: bool,
    pub orientation: RadioOrientation,
    pub selection_mode: SelectionMode,
    pub size: RadioSize,
    pub variant: RadioVariant,
    pub color_palette: UiColorPalette,
}

impl Default for RadioGroupComponent {
    fn default() -> Self {
        Self {
            name: String::new(),
            selected_value: None,
            selected_values: Vec::new(),
            default_value: None,
            disabled: false,
            required: false,
            orientation: RadioOrientation::Vertical,
            selection_mode: SelectionMode::Single,
            size: RadioSize::Size2,
            variant: RadioVariant::Surface,
            color_palette: accent_palette(),
        }
    }
}

/// Builder for creating Radio components with fluent API
pub struct RadioBuilder {
    name: String,
    radio_config: RadioComponent,
    node: Node,
    label_text: Option<String>,
}

impl RadioComponent {
    /// Create a new Radio component builder
    pub fn new(value: impl Into<String>) -> RadioBuilder {
        RadioBuilder::new(value)
    }

    /// Create a Size1 (14px) radio
    pub fn size_1(value: impl Into<String>) -> RadioBuilder {
        Self::new(value).size_1()
    }

    /// Create a Size2 (16px) radio (default)
    pub fn size_2(value: impl Into<String>) -> RadioBuilder {
        Self::new(value).size_2()
    }

    /// Create a Size3 (20px) radio
    pub fn size_3(value: impl Into<String>) -> RadioBuilder {
        Self::new(value).size_3()
    }

    /// Create a radio with initial checked state
    pub fn checked(value: impl Into<String>) -> RadioBuilder {
        Self::new(value).checked()
    }
}

impl RadioBuilder {
    pub fn new(value: impl Into<String>) -> Self {
        let mut radio_config = RadioComponent::default();
        radio_config.value = value.into();
        let size_px = radio_config.size.to_pixels();

        Self {
            name: format!("{}_Radio", radio_config.value),
            radio_config,
            node: Node {
                width: Val::Px(size_px),
                height: Val::Px(size_px),
                border: UiRect::all(Val::Px(RadioSize::Size2.border_width())),
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            label_text: None,
        }
    }

    // =========================================================================
    // SIZE METHODS
    // =========================================================================

    /// Set radio size
    pub fn size(mut self, size: RadioSize) -> Self {
        self.radio_config.size = size;
        let size_px = size.to_pixels();
        self.node.width = Val::Px(size_px);
        self.node.height = Val::Px(size_px);
        self.node.border = UiRect::all(Val::Px(size.border_width()));
        self
    }

    /// Use Size1 (14px) - compact
    pub fn size_1(self) -> Self {
        self.size(RadioSize::Size1)
    }

    /// Use Size2 (16px) - default
    pub fn size_2(self) -> Self {
        self.size(RadioSize::Size2)
    }

    /// Use Size3 (20px) - large
    pub fn size_3(self) -> Self {
        self.size(RadioSize::Size3)
    }

    // =========================================================================
    // VARIANT METHODS
    // =========================================================================

    /// Set visual variant
    pub fn variant(mut self, variant: RadioVariant) -> Self {
        self.radio_config.variant = variant;
        self
    }

    /// Use Surface variant (default)
    pub fn surface(self) -> Self {
        self.variant(RadioVariant::Surface)
    }

    /// Use Classic variant with gradient
    pub fn classic(self) -> Self {
        self.variant(RadioVariant::Classic)
    }

    /// Use Soft variant with accent background
    pub fn soft(self) -> Self {
        self.variant(RadioVariant::Soft)
    }

    // =========================================================================
    // STATE METHODS
    // =========================================================================

    /// Set initial checked state
    pub fn checked(mut self) -> Self {
        self.radio_config.checked = true;
        self
    }

    /// Set radio as disabled
    pub fn disabled(mut self) -> Self {
        self.radio_config.disabled = true;
        self.radio_config.current_state = RadioState::Disabled;
        self
    }

    // Removed allow_deselect and toggle methods - ALL radios are toggle-able by default

    // =========================================================================
    // STYLING METHODS
    // =========================================================================

    /// Set color palette for theming
    pub fn color(mut self, palette: UiColorPalette) -> Self {
        self.radio_config.color_palette = palette;
        self
    }

    /// Use accent color palette
    pub fn accent(self) -> Self {
        self.color(accent_palette())
    }

    /// Add label text to the radio
    pub fn label(mut self, text: impl Into<String>) -> Self {
        self.label_text = Some(text.into());
        self
    }

    // =========================================================================
    // LAYOUT METHODS
    // =========================================================================

    /// Set margin around the radio
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
        let palette = &self.radio_config.color_palette;
        
        if self.radio_config.disabled {
            return BackgroundColor(palette.bg_subtle.with_alpha(0.5));
        }

        match (self.radio_config.checked, self.radio_config.variant, self.radio_config.current_state) {
            // Checked states
            (true, RadioVariant::Surface, RadioState::Normal) => BackgroundColor(palette.solid),
            (true, RadioVariant::Surface, RadioState::Hover) => BackgroundColor(palette.solid_hover),
            (true, RadioVariant::Surface, RadioState::Active) => BackgroundColor(palette.solid_hover),
            (true, RadioVariant::Classic, RadioState::Normal) => BackgroundColor(palette.solid),
            (true, RadioVariant::Classic, RadioState::Hover) => BackgroundColor(palette.solid_hover),
            (true, RadioVariant::Classic, RadioState::Active) => BackgroundColor(palette.solid_hover),
            (true, RadioVariant::Soft, RadioState::Normal) => BackgroundColor(palette.bg_active),
            (true, RadioVariant::Soft, RadioState::Hover) => BackgroundColor(palette.bg_hover),
            (true, RadioVariant::Soft, RadioState::Active) => BackgroundColor(palette.bg_active),
            
            // Unchecked states
            (false, RadioVariant::Surface, RadioState::Normal) => BackgroundColor(Color::NONE),
            (false, RadioVariant::Surface, RadioState::Hover) => BackgroundColor(palette.bg_hover),
            (false, RadioVariant::Surface, RadioState::Active) => BackgroundColor(palette.bg_active),
            (false, RadioVariant::Classic, RadioState::Normal) => BackgroundColor(Color::NONE),
            (false, RadioVariant::Classic, RadioState::Hover) => BackgroundColor(palette.bg_hover),
            (false, RadioVariant::Classic, RadioState::Active) => BackgroundColor(palette.bg_active),
            (false, RadioVariant::Soft, RadioState::Normal) => BackgroundColor(Color::NONE),
            (false, RadioVariant::Soft, RadioState::Hover) => BackgroundColor(palette.bg_hover),
            (false, RadioVariant::Soft, RadioState::Active) => BackgroundColor(palette.bg_active),
            
            _ => BackgroundColor(Color::NONE),
        }
    }

    /// Calculate border color based on state
    fn calculate_border_color(&self) -> BorderColor {
        let palette = &self.radio_config.color_palette;

        if self.radio_config.disabled {
            return BorderColor(palette.border.with_alpha(0.5));
        }

        match (self.radio_config.checked, self.radio_config.current_state) {
            (true, _) => BorderColor(palette.solid),
            (false, RadioState::Normal) => BorderColor(palette.border),
            (false, RadioState::Hover) => BorderColor(palette.border_hover),
            (false, RadioState::Active) => BorderColor(palette.solid),
            _ => BorderColor(palette.border),
        }
    }

    /// Calculate border radius (always circular)
    fn calculate_border_radius(&self) -> BorderRadius {
        BorderRadius::all(Val::Percent(50.0)) // Always circular
    }
}

impl RadioBuilder {
    pub fn build(self) -> impl Bundle {
        let background_color = self.calculate_background_color();
        let border_color = self.calculate_border_color();
        let border_radius = self.calculate_border_radius();

        (
            Name::new(self.name),
            self.radio_config,
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

/// Builder for creating RadioGroup components with fluent API
pub struct RadioGroupBuilder {
    name: String,
    group_config: RadioGroupComponent,
    node: Node,
}

impl RadioGroupComponent {
    /// Create a new RadioGroup component builder
    pub fn new(name: impl Into<String>) -> RadioGroupBuilder {
        RadioGroupBuilder::new(name)
    }

    /// Create a vertical radio group (default)
    pub fn vertical(name: impl Into<String>) -> RadioGroupBuilder {
        Self::new(name).vertical()
    }

    /// Create a horizontal radio group
    pub fn horizontal(name: impl Into<String>) -> RadioGroupBuilder {
        Self::new(name).horizontal()
    }

    /// Create a radio group with default value
    pub fn with_default(name: impl Into<String>, default_value: impl Into<String>) -> RadioGroupBuilder {
        Self::new(name).default_value(default_value)
    }

    /// Create a single-choice radio group (default) - only one can be selected
    pub fn single_choice(name: impl Into<String>) -> RadioGroupBuilder {
        Self::new(name).single_choice()
    }

    /// Create a multi-choice radio group - multiple can be selected
    pub fn multi_choice(name: impl Into<String>) -> RadioGroupBuilder {
        Self::new(name).multi_choice()
    }
}

impl RadioGroupBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        let group_name = name.into();
        let mut group_config = RadioGroupComponent::default();
        group_config.name = group_name.clone();

        Self {
            name: format!("{}_RadioGroup", group_name),
            group_config,
            node: Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column, // Default vertical
                ..default()
            },
        }
    }

    // =========================================================================
    // VALUE METHODS
    // =========================================================================

    /// Set default selected value
    pub fn default_value(mut self, value: impl Into<String>) -> Self {
        let value = value.into();
        self.group_config.default_value = Some(value.clone());
        self.group_config.selected_value = Some(value);
        self
    }

    /// Set current selected value
    pub fn selected_value(mut self, value: impl Into<String>) -> Self {
        self.group_config.selected_value = Some(value.into());
        self
    }

    // =========================================================================
    // STATE METHODS
    // =========================================================================

    /// Set radio group as disabled
    pub fn disabled(mut self) -> Self {
        self.group_config.disabled = true;
        self
    }

    /// Set radio group as required for forms
    pub fn required(mut self) -> Self {
        self.group_config.required = true;
        self
    }

    // =========================================================================
    // SELECTION MODE METHODS
    // =========================================================================

    /// Set selection mode to single choice (only one selected, but deselectable)
    pub fn single_choice(mut self) -> Self {
        self.group_config.selection_mode = SelectionMode::Single;
        self
    }

    /// Set selection mode to multiple choice (multiple selected independently)
    pub fn multi_choice(mut self) -> Self {
        self.group_config.selection_mode = SelectionMode::Multiple;
        self
    }

    /// Set custom selection mode
    pub fn selection_mode(mut self, mode: SelectionMode) -> Self {
        self.group_config.selection_mode = mode;
        self
    }

    // =========================================================================
    // LAYOUT METHODS
    // =========================================================================

    /// Set orientation to vertical (column layout)
    pub fn vertical(mut self) -> Self {
        self.group_config.orientation = RadioOrientation::Vertical;
        self.node.flex_direction = FlexDirection::Column;
        self
    }

    /// Set orientation to horizontal (row layout)
    pub fn horizontal(mut self) -> Self {
        self.group_config.orientation = RadioOrientation::Horizontal;
        self.node.flex_direction = FlexDirection::Row;
        self
    }

    /// Set gap between radio items
    pub fn gap(mut self, gap: f32) -> Self {
        // For now, we'll use padding since Bevy's Node doesn't support gap directly
        // In a real implementation, you'd use FlexComponent for gap support
        self.node.padding = UiRect::all(Val::Px(gap / 2.0));
        self
    }

    /// Set margin around the radio group
    pub fn margin(mut self, margin: Val) -> Self {
        self.node.margin = UiRect::all(margin);
        self
    }

    /// Set padding inside the radio group
    pub fn padding(mut self, padding: Val) -> Self {
        self.node.padding = UiRect::all(padding);
        self
    }

    // =========================================================================
    // STYLING METHODS
    // =========================================================================

    /// Set size for all radio items in group
    pub fn size(mut self, size: RadioSize) -> Self {
        self.group_config.size = size;
        self
    }

    /// Set variant for all radio items in group
    pub fn variant(mut self, variant: RadioVariant) -> Self {
        self.group_config.variant = variant;
        self
    }

    /// Set color palette for all radio items in group
    pub fn color(mut self, palette: UiColorPalette) -> Self {
        self.group_config.color_palette = palette;
        self
    }

    /// Use accent color palette
    pub fn accent(self) -> Self {
        self.color(accent_palette())
    }

    // =========================================================================
    // ALIGNMENT METHODS
    // =========================================================================

    /// Align items to start
    pub fn align_start(mut self) -> Self {
        self.node.align_items = AlignItems::Start;
        self
    }

    /// Align items to center
    pub fn align_center(mut self) -> Self {
        self.node.align_items = AlignItems::Center;
        self
    }

    /// Align items to end
    pub fn align_end(mut self) -> Self {
        self.node.align_items = AlignItems::End;
        self
    }

    /// Justify content to start
    pub fn justify_start(mut self) -> Self {
        self.node.justify_content = JustifyContent::Start;
        self
    }

    /// Justify content to center
    pub fn justify_center(mut self) -> Self {
        self.node.justify_content = JustifyContent::Center;
        self
    }

    /// Justify content to end
    pub fn justify_end(mut self) -> Self {
        self.node.justify_content = JustifyContent::End;
        self
    }
}

impl RadioGroupBuilder {
    pub fn build(self) -> impl Bundle {
        (
            Name::new(self.name),
            self.group_config,
            self.node,
        )
    }
}

/// Marker component for radio indicators
#[derive(Component)]
pub struct RadioIndicatorComponent;

// =========================================================================
// INTERACTION SYSTEMS
// =========================================================================

/// System to handle radio interactions
pub fn handle_radio_interactions(
    mut interaction_query: Query<
        (
            Entity,
            &Interaction,
            &mut RadioComponent,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        (With<RadioComponent>, Changed<Interaction>),
    >,
    mut radio_events: EventWriter<RadioChangeEvent>,
    radio_group_query: Query<&RadioGroupComponent>,
) {
    for (entity, interaction, mut radio, mut bg_color, mut border_color) in &mut interaction_query {
        if radio.disabled {
            continue;
        }

        let old_state = radio.current_state;
        let old_checked = radio.checked;

        // Update state based on interaction
        radio.current_state = match *interaction {
            Interaction::Hovered => RadioState::Hover,
            Interaction::Pressed => RadioState::Active,
            Interaction::None => RadioState::Normal,
        };

        // Handle click (when releasing click)
        if old_state == RadioState::Active && 
           (radio.current_state == RadioState::Normal || radio.current_state == RadioState::Hover) {
            
            // Determine if radio should toggle based on group selection mode
            let should_toggle = if let Some(group_entity) = radio.group_entity {
                if let Ok(group) = radio_group_query.get(group_entity) {
                    match group.selection_mode {
                        SelectionMode::Single => {
                            // In single-choice mode, only allow toggling ON (not OFF if already selected)
                            !radio.checked
                        }
                        SelectionMode::Multiple => {
                            // In multi-choice mode, always allow toggling
                            true
                        }
                    }
                } else {
                    // If group not found, default to toggle behavior
                    true
                }
            } else {
                // Standalone radios can always toggle
                true
            };

            if should_toggle {
                radio.checked = !radio.checked;
                
                // Get group information for event
                if let Some(group_entity) = radio.group_entity {
                    if let Ok(group) = radio_group_query.get(group_entity) {
                        radio_events.write(RadioChangeEvent {
                            group_entity,
                            group_name: group.name.clone(),
                            selected_value: radio.value.clone(),
                            previous_value: group.selected_value.clone(),
                            radio_entity: entity,
                        });
                    }
                } else {
                    // Send event even without group for standalone radios
                    radio_events.write(RadioChangeEvent {
                        group_entity: Entity::PLACEHOLDER,
                        group_name: "standalone".to_string(),
                        selected_value: radio.value.clone(),
                        previous_value: None,
                        radio_entity: entity,
                    });
                }
            }
        }

        // Update visual appearance if state changed
        if old_state != radio.current_state || old_checked != radio.checked {
            update_radio_appearance(&radio, &mut bg_color, &mut border_color);
        }
    }
}

/// System to enforce selection rules within radio groups
pub fn update_radio_groups(
    mut radio_events: EventReader<RadioChangeEvent>,
    mut radio_group_query: Query<&mut RadioGroupComponent>,
    mut radio_query: Query<(&mut RadioComponent, &mut BackgroundColor, &mut BorderColor, Option<&Children>)>,
    children_query: Query<&Children>,
    mut group_events: EventWriter<RadioGroupValueChangeEvent>,
) {
    for event in radio_events.read() {
        // Skip standalone radios
        if event.group_entity == Entity::PLACEHOLDER {
            continue;
        }

        // Update the radio group based on selection mode
        if let Ok(mut group) = radio_group_query.get_mut(event.group_entity) {
            // Get current radio state
            let radio_is_checked = if let Ok((radio, _, _, _)) = radio_query.get(event.radio_entity) {
                radio.checked
            } else {
                continue;
            };

            match group.selection_mode {
                SelectionMode::Single => {
                    // Single choice: when selecting one, deselect others
                    if radio_is_checked {
                        let old_value = group.selected_value.clone();
                        group.selected_value = Some(event.selected_value.clone());
                        
                        // Send group value change event
                        group_events.write(RadioGroupValueChangeEvent {
                            group_entity: event.group_entity,
                            group_name: event.group_name.clone(),
                            new_value: Some(event.selected_value.clone()),
                            old_value,
                        });
                        
                        // Uncheck all other radios in the same group recursively
                        if let Ok(children) = children_query.get(event.group_entity) {
                            uncheck_other_radios_recursive(
                                &mut radio_query,
                                &children_query,
                                children.iter(),
                                event.radio_entity,
                            );
                        }
                    } else {
                        // Radio was deselected - clear group selection
                        let old_value = group.selected_value.clone();
                        group.selected_value = None;
                        
                        group_events.write(RadioGroupValueChangeEvent {
                            group_entity: event.group_entity,
                            group_name: event.group_name.clone(),
                            new_value: None,
                            old_value,
                        });
                    }
                }
                SelectionMode::Multiple => {
                    // Multiple choice: manage list of selected values
                    if radio_is_checked {
                        // Add to selected values if not already present
                        if !group.selected_values.contains(&event.selected_value) {
                            group.selected_values.push(event.selected_value.clone());
                        }
                    } else {
                        // Remove from selected values
                        group.selected_values.retain(|v| v != &event.selected_value);
                    }
                    
                    // For compatibility, set selected_value to the first selected
                    let old_value = group.selected_value.clone();
                    group.selected_value = group.selected_values.first().cloned();
                    
                    group_events.write(RadioGroupValueChangeEvent {
                        group_entity: event.group_entity,
                        group_name: event.group_name.clone(),
                        new_value: group.selected_value.clone(),
                        old_value,
                    });
                }
            }
        }
    }
}

/// System to spawn radio indicators for checked radios
pub fn spawn_radio_indicators(
    mut commands: Commands,
    radio_query: Query<(Entity, &RadioComponent), Added<RadioComponent>>,
) {
    for (entity, radio) in &radio_query {
        if radio.checked {
            spawn_radio_indicator(&mut commands, entity, radio);
        }
    }
}

/// System to update radio indicators when radio state changes
pub fn update_radio_indicators(
    mut commands: Commands,
    radio_query: Query<(Entity, &RadioComponent, Option<&Children>), Changed<RadioComponent>>,
    indicator_query: Query<Entity, With<RadioIndicatorComponent>>,
) {
    for (radio_entity, radio, children) in &radio_query {
        // Find existing indicator among children
        let existing_indicator = if let Some(children) = children {
            children
                .iter()
                .find(|&child| indicator_query.contains(child))
        } else {
            None
        };

        if radio.checked {
            // Spawn indicator if it doesn't exist
            if existing_indicator.is_none() {
                spawn_radio_indicator(&mut commands, radio_entity, radio);
            }
        } else {
            // Remove indicator if it exists
            if let Some(indicator_entity) = existing_indicator {
                commands.entity(indicator_entity).despawn();
            }
        }
    }
}

/// System to setup radio interactions with observer pattern
pub fn setup_radio_interactions(
    radios: Query<Entity, Added<RadioComponent>>,
    radio_query: Query<&RadioComponent>,
    mut bg_colors: Query<&mut BackgroundColor>,
    mut border_colors: Query<&mut BorderColor>,
) {
    for entity in &radios {
        // Apply initial styling
        if let Ok(radio) = radio_query.get(entity) {
            if let (Ok(mut bg_color), Ok(mut border_color)) = 
                (bg_colors.get_mut(entity), border_colors.get_mut(entity)) {
                update_radio_appearance(radio, &mut bg_color, &mut border_color);
            }
        }
    }
}

/// System to sync radio groups with their child radios and apply default values
pub fn link_radios_to_groups(
    mut radio_query: Query<(&mut RadioComponent, &mut BackgroundColor, &mut BorderColor)>,
    group_query: Query<(Entity, &RadioGroupComponent, &Children)>,
    children_query: Query<&Children>,
) {
    // Iterate through all radio groups
    for (group_entity, group, children) in &group_query {
        // Recursively search for radio components in the hierarchy
        find_and_link_radios_recursive(
            &mut radio_query,
            &children_query,
            children.iter(),
            group_entity,
            group,
        );
    }
}

/// Recursively uncheck other radio buttons in a single-choice group
fn uncheck_other_radios_recursive(
    radio_query: &mut Query<(&mut RadioComponent, &mut BackgroundColor, &mut BorderColor, Option<&Children>)>,
    children_query: &Query<&Children>,
    child_entities: impl Iterator<Item = Entity>,
    selected_radio_entity: Entity,
) {
    for child_entity in child_entities {
        // Check if this child is a radio component
        if let Ok((mut radio, mut bg_color, mut border_color, _radio_children)) = radio_query.get_mut(child_entity) {
            if child_entity != selected_radio_entity && radio.checked {
                radio.checked = false;
                // Update visual appearance
                update_radio_appearance(&radio, &mut bg_color, &mut border_color);
                
                // Let update_radio_indicators system handle indicator removal via Change Detection
                // This prevents double-despawn warnings
            }
        }
        
        // Recursively search children of this entity
        if let Ok(grandchildren) = children_query.get(child_entity) {
            uncheck_other_radios_recursive(
                radio_query,
                children_query,
                grandchildren.iter(),
                selected_radio_entity,
            );
        }
    }
}

/// Recursively find and link radio components to their group
fn find_and_link_radios_recursive(
    radio_query: &mut Query<(&mut RadioComponent, &mut BackgroundColor, &mut BorderColor)>,
    children_query: &Query<&Children>,
    child_entities: impl Iterator<Item = Entity>,
    group_entity: Entity,
    group: &RadioGroupComponent,
) {
    for child_entity in child_entities {
        // Check if this child is a radio component
        if let Ok((mut radio, mut bg_color, mut border_color)) = radio_query.get_mut(child_entity) {
            // Link radio to this group if not already linked
            if radio.group_entity.is_none() {
                radio.group_entity = Some(group_entity);
                
                // Inherit group settings
                if radio.size == RadioSize::Size2 && group.size != RadioSize::Size2 {
                    radio.size = group.size;
                }
                if radio.variant == RadioVariant::Surface && group.variant != RadioVariant::Surface {
                    radio.variant = group.variant;
                }
                if radio.color_palette.solid == accent_palette().solid && group.color_palette.solid != accent_palette().solid {
                    radio.color_palette = group.color_palette.clone();
                }
                
                // Set initial checked state based on group's default_value
                if let Some(default_value) = &group.default_value {
                    if radio.value == *default_value {
                        radio.checked = true;
                        // Update visual appearance immediately
                        update_radio_appearance(&radio, &mut bg_color, &mut border_color);
                    }
                }
            }
        }
        
        // Recursively search children of this entity
        if let Ok(grandchildren) = children_query.get(child_entity) {
            find_and_link_radios_recursive(
                radio_query,
                children_query,
                grandchildren.iter(),
                group_entity,
                group,
            );
        }
    }
}

/// Helper function to update radio visual appearance
fn update_radio_appearance(
    radio: &RadioComponent,
    bg_color: &mut BackgroundColor,
    border_color: &mut BorderColor,
) {
    let palette = &radio.color_palette;
    
    // Update background color
    *bg_color = if radio.disabled {
        BackgroundColor(palette.bg_subtle.with_alpha(0.5))
    } else {
        match (radio.checked, radio.variant, radio.current_state) {
            // Checked states
            (true, RadioVariant::Surface, RadioState::Normal) => BackgroundColor(palette.solid),
            (true, RadioVariant::Surface, RadioState::Hover) => BackgroundColor(palette.solid_hover),
            (true, RadioVariant::Surface, RadioState::Active) => BackgroundColor(palette.solid_hover),
            (true, RadioVariant::Classic, RadioState::Normal) => BackgroundColor(palette.solid),
            (true, RadioVariant::Classic, RadioState::Hover) => BackgroundColor(palette.solid_hover),
            (true, RadioVariant::Classic, RadioState::Active) => BackgroundColor(palette.solid_hover),
            (true, RadioVariant::Soft, RadioState::Normal) => BackgroundColor(palette.bg_active),
            (true, RadioVariant::Soft, RadioState::Hover) => BackgroundColor(palette.bg_hover),
            (true, RadioVariant::Soft, RadioState::Active) => BackgroundColor(palette.bg_active),
            
            // Unchecked states
            (false, RadioVariant::Surface, RadioState::Normal) => BackgroundColor(Color::NONE),
            (false, RadioVariant::Surface, RadioState::Hover) => BackgroundColor(palette.bg_hover),
            (false, RadioVariant::Surface, RadioState::Active) => BackgroundColor(palette.bg_active),
            (false, RadioVariant::Classic, RadioState::Normal) => BackgroundColor(Color::NONE),
            (false, RadioVariant::Classic, RadioState::Hover) => BackgroundColor(palette.bg_hover),
            (false, RadioVariant::Classic, RadioState::Active) => BackgroundColor(palette.bg_active),
            (false, RadioVariant::Soft, RadioState::Normal) => BackgroundColor(Color::NONE),
            (false, RadioVariant::Soft, RadioState::Hover) => BackgroundColor(palette.bg_hover),
            (false, RadioVariant::Soft, RadioState::Active) => BackgroundColor(palette.bg_active),
            
            _ => BackgroundColor(Color::NONE),
        }
    };

    // Update border color
    *border_color = if radio.disabled {
        BorderColor(palette.border.with_alpha(0.5))
    } else {
        match (radio.checked, radio.current_state) {
            (true, _) => BorderColor(palette.solid),
            (false, RadioState::Normal) => BorderColor(palette.border),
            (false, RadioState::Hover) => BorderColor(palette.border_hover),
            (false, RadioState::Active) => BorderColor(palette.solid),
            _ => BorderColor(palette.border),
        }
    };
}

/// Helper function to spawn a radio indicator
fn spawn_radio_indicator(
    commands: &mut Commands,
    parent_entity: Entity,
    radio: &RadioComponent,
) {
    let indicator_size = radio.size.indicator_size();
    let indicator_color = match radio.variant {
        RadioVariant::Surface | RadioVariant::Classic => Color::WHITE,
        RadioVariant::Soft => radio.color_palette.text_contrast,
    };

    commands.entity(parent_entity).with_children(|parent| {
        parent.spawn((
            Name::new("RadioIndicator"),
            RadioIndicatorComponent,
            Node {
                position_type: PositionType::Absolute,
                width: Val::Px(indicator_size),
                height: Val::Px(indicator_size),
                top: Val::Percent(50.0),
                left: Val::Percent(50.0),
                margin: UiRect {
                    top: Val::Px(-indicator_size / 2.0),
                    left: Val::Px(-indicator_size / 2.0),
                    ..default()
                },
                ..default()
            },
            BackgroundColor(indicator_color),
            BorderRadius::all(Val::Percent(50.0)), // Circular indicator
            Pickable::IGNORE,
        ));
    });
}

// Convenience type aliases
pub type Radio = RadioComponent;
pub type RadioGroup = RadioGroupComponent;