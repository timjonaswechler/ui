use crate::components::text::Text;
use crate::theme::color::{
    theme_mode, ThemeMode, UiColorPalette, UiColorPalettes, UiColorPalettesName,
};
use crate::utilities::{ComponentBuilder, Portal};
use bevy::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SelectSize {
    Size1,
    Size2,
    Size3,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SelectVariant {
    Surface,
    Classic,
    Soft,
    Ghost,
    Outline,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SelectState {
    Closed,
    Open,
}

#[derive(Component, Debug, Clone)]
pub struct SelectComponent {
    pub open: bool,
    pub selected_value: Option<String>,
    pub placeholder: String,
    pub disabled: bool,
    pub size: SelectSize,
    pub variant: SelectVariant,
    pub color: UiColorPalettesName,
    pub state: SelectState,
}

impl Default for SelectComponent {
    fn default() -> Self {
        Self {
            open: false,
            selected_value: None,
            placeholder: "Select an option...".to_string(),
            disabled: false,
            size: SelectSize::Size2,
            variant: SelectVariant::Surface,
            color: UiColorPalettesName::Gray,
            state: SelectState::Closed,
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct SelectOptionComponent {
    pub value: String,
    pub label: String,
    pub disabled: bool,
    pub selected: bool,
    pub select_entity: Option<Entity>,
}

impl SelectOptionComponent {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            disabled: false,
            selected: false,
            select_entity: None,
        }
    }
}

#[derive(Component, Debug)]
pub struct SelectTrigger {
    pub select_entity: Entity,
}

#[derive(Component, Debug)]
pub struct SelectDropdownComponent {
    pub select_entity: Entity,
    pub is_open: bool,
}

#[derive(Component, Debug)]
pub struct SelectIndicator {
    pub select_entity: Entity,
}

#[derive(Component, Debug)]
pub struct RadixOffset(pub f32);

#[derive(Event, Debug, Clone)]
pub struct SelectOpenEvent {
    pub select_entity: Entity,
    pub open: bool,
}

#[derive(Event, Debug, Clone)]
pub struct SelectChangeEvent {
    pub select_entity: Entity,
    pub selected_value: String,
    pub previous_value: Option<String>,
    pub selected_label: String,
}

impl SelectComponent {
    pub fn calculate_trigger_height(&self) -> f32 {
        match self.size {
            SelectSize::Size1 => 32.0,
            SelectSize::Size2 => 40.0,
            SelectSize::Size3 => 48.0,
        }
    }

    pub fn calculate_padding(&self) -> (f32, f32) {
        match self.size {
            SelectSize::Size1 => (8.0, 12.0),
            SelectSize::Size2 => (10.0, 16.0),
            SelectSize::Size3 => (12.0, 20.0),
        }
    }

    pub fn calculate_option_height(&self) -> f32 {
        match self.size {
            SelectSize::Size1 => 32.0,
            SelectSize::Size2 => 36.0,
            SelectSize::Size3 => 40.0,
        }
    }

    pub fn find_selected_option_index(&self, options: &[String]) -> Option<usize> {
        if let Some(ref selected_value) = self.selected_value {
            options.iter().position(|opt| opt == selected_value)
        } else {
            None
        }
    }

    pub fn calculate_trigger_colors(&self) -> (Color, Color, Color) {
        let colors = self.get_color_palette();
        match self.variant {
            SelectVariant::Surface => (colors.surface, colors.border, colors.text),
            SelectVariant::Classic => (colors.bg, colors.border, colors.text),
            SelectVariant::Soft => (colors.bg_subtle, Color::NONE, colors.text),
            SelectVariant::Ghost => (Color::NONE, Color::NONE, colors.text),
            SelectVariant::Outline => (Color::NONE, colors.border, colors.text),
        }
    }

    pub fn calculate_hover_colors(&self) -> (Color, Color, Color) {
        let colors = self.get_color_palette();
        match self.variant {
            SelectVariant::Surface => (colors.bg_hover, colors.border_hover, colors.text),
            SelectVariant::Classic => (colors.bg_hover, colors.border_hover, colors.text),
            SelectVariant::Soft => (colors.bg_hover, Color::NONE, colors.text),
            SelectVariant::Ghost => (colors.bg_hover, Color::NONE, colors.text),
            SelectVariant::Outline => (colors.bg_hover, colors.border_hover, colors.text),
        }
    }

    fn get_color_palette(&self) -> UiColorPalette {
        let palettes = if theme_mode() == ThemeMode::Dark {
            UiColorPalettes::dark_mode()
        } else {
            UiColorPalettes::light_mode()
        };

        match self.color {
            UiColorPalettesName::Gray => palettes.gray,
            UiColorPalettesName::Red => palettes.red,
            UiColorPalettesName::Blue => palettes.blue,
            UiColorPalettesName::Green => palettes.green,
            UiColorPalettesName::Indigo => palettes.indigo,
            UiColorPalettesName::Purple => palettes.purple,
            UiColorPalettesName::Pink => palettes.pink,
            UiColorPalettesName::Cyan => palettes.cyan,
            UiColorPalettesName::Orange => palettes.orange,
            UiColorPalettesName::Yellow => palettes.yellow,
            _ => palettes.gray,
        }
    }
}

pub struct SelectBuilder {
    component: SelectComponent,
    node: Node,
}

impl SelectBuilder {
    pub fn new() -> Self {
        Self {
            component: SelectComponent::default(),
            node: Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Stretch,
                justify_content: JustifyContent::FlexStart,
                position_type: PositionType::Relative,
                ..default()
            },
        }
    }

    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.component.placeholder = placeholder.into();
        self
    }

    pub fn selected_value(mut self, value: impl Into<String>) -> Self {
        self.component.selected_value = Some(value.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.component.disabled = disabled;
        self
    }

    pub fn size(mut self, size: SelectSize) -> Self {
        self.component.size = size;
        self
    }

    pub fn size_1(self) -> Self {
        self.size(SelectSize::Size1)
    }

    pub fn size_2(self) -> Self {
        self.size(SelectSize::Size2)
    }

    pub fn size_3(self) -> Self {
        self.size(SelectSize::Size3)
    }

    pub fn variant(mut self, variant: SelectVariant) -> Self {
        self.component.variant = variant;
        self
    }

    pub fn surface(self) -> Self {
        self.variant(SelectVariant::Surface)
    }

    pub fn classic(self) -> Self {
        self.variant(SelectVariant::Classic)
    }

    pub fn soft(self) -> Self {
        self.variant(SelectVariant::Soft)
    }

    pub fn ghost(self) -> Self {
        self.variant(SelectVariant::Ghost)
    }

    pub fn outline(self) -> Self {
        self.variant(SelectVariant::Outline)
    }

    pub fn color(mut self, color: UiColorPalettesName) -> Self {
        self.component.color = color;
        self
    }

    pub fn gray(self) -> Self {
        self.color(UiColorPalettesName::Gray)
    }

    pub fn accent(self) -> Self {
        self.color(UiColorPalettesName::Indigo)
    }

    pub fn blue(self) -> Self {
        self.color(UiColorPalettesName::Blue)
    }

    pub fn green(self) -> Self {
        self.color(UiColorPalettesName::Green)
    }

    pub fn red(self) -> Self {
        self.color(UiColorPalettesName::Red)
    }

    pub fn width(mut self, width: Val) -> Self {
        self.node.width = width;
        self
    }

    pub fn min_width(mut self, min_width: Val) -> Self {
        self.node.min_width = min_width;
        self
    }

    pub fn max_width(mut self, max_width: Val) -> Self {
        self.node.max_width = max_width;
        self
    }
}

impl ComponentBuilder for SelectBuilder {
    type Output = (
        SelectComponent,
        Node,
        BackgroundColor,
        BorderColor,
        BorderRadius,
        SelectTrigger,
    );

    fn build(self) -> Self::Output {
        let (bg_color, border_color, _text_color) = self.component.calculate_trigger_colors();
        let height = self.component.calculate_trigger_height();
        let (vertical_padding, horizontal_padding) = self.component.calculate_padding();

        (
            self.component.clone(),
            Node {
                height: Val::Px(height),
                padding: UiRect::axes(Val::Px(horizontal_padding), Val::Px(vertical_padding)),
                border: UiRect::all(Val::Px(1.0)),
                ..self.node
            },
            BackgroundColor(bg_color),
            BorderColor(border_color),
            BorderRadius::all(Val::Px(6.0)),
            SelectTrigger {
                select_entity: Entity::PLACEHOLDER, // Will be updated by system
            },
        )
    }
}

pub struct SelectOptionBuilder {
    component: SelectOptionComponent,
    node: Node,
}

impl SelectOptionBuilder {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            component: SelectOptionComponent::new(value, label),
            node: Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexStart,
                padding: UiRect::axes(Val::Px(12.0), Val::Px(8.0)),
                width: Val::Percent(100.0),
                ..default()
            },
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.component.disabled = disabled;
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.component.selected = selected;
        self
    }
}

impl ComponentBuilder for SelectOptionBuilder {
    type Output = (SelectOptionComponent, Node, BackgroundColor, BorderColor);

    fn build(self) -> Self::Output {
        (
            self.component,
            self.node,
            BackgroundColor(Color::NONE),
            BorderColor(Color::NONE),
        )
    }
}

pub struct Select;

impl Select {
    pub fn new() -> SelectBuilder {
        SelectBuilder::new()
    }

    pub fn option(value: impl Into<String>, label: impl Into<String>) -> SelectOptionBuilder {
        SelectOptionBuilder::new(value, label)
    }
}

// Systems for Select functionality

/// Sets up interaction observers for Select triggers and adds text children
pub fn setup_select_interactions(
    mut commands: Commands,
    selects_query: Query<(Entity, &SelectComponent), Added<SelectComponent>>,
) {
    for (entity, select) in &selects_query {
        // Add interaction observers
        commands
            .entity(entity)
            .observe(on_select_trigger_click)
            .observe(on_select_trigger_hover)
            .observe(on_select_trigger_hover_out);

        // Add placeholder text child
        let text_value = select
            .selected_value
            .as_ref()
            .unwrap_or(&select.placeholder)
            .clone();

        commands.entity(entity).with_children(|parent| {
            parent.spawn(Text::new(text_value).build());
        });
    }
}

/// Handles trigger click to open/close dropdown
fn on_select_trigger_click(
    trigger: Trigger<Pointer<Click>>,
    mut select_query: Query<&mut SelectComponent>,
    mut commands: Commands,
    dropdown_query: Query<Entity, With<SelectDropdownComponent>>,
    mut select_open_events: EventWriter<SelectOpenEvent>,
) {
    if let Ok(mut select) = select_query.get_mut(trigger.target()) {
        select.open = !select.open;
        select.state = if select.open {
            SelectState::Open
        } else {
            SelectState::Closed
        };

        // Send open event
        select_open_events.write(SelectOpenEvent {
            select_entity: trigger.target(),
            open: select.open,
        });

        if select.open {
            // Spawn dropdown
            spawn_select_dropdown(&mut commands, trigger.target(), &select);
        } else {
            // Close existing dropdown
            for dropdown_entity in &dropdown_query {
                commands.entity(dropdown_entity).despawn();
            }
        }
    }
}

/// Handles trigger hover for visual feedback
fn on_select_trigger_hover(
    trigger: Trigger<Pointer<Over>>,
    mut select_query: Query<(&SelectComponent, &mut BackgroundColor, &mut BorderColor)>,
) {
    if let Ok((select, mut bg_color, mut border_color)) = select_query.get_mut(trigger.target()) {
        if !select.disabled {
            let (hover_bg, hover_border, _) = select.calculate_hover_colors();
            *bg_color = BackgroundColor(hover_bg);
            *border_color = BorderColor(hover_border);
        }
    }
}

/// Handles trigger hover out to reset colors
fn on_select_trigger_hover_out(
    trigger: Trigger<Pointer<Out>>,
    mut select_query: Query<(&SelectComponent, &mut BackgroundColor, &mut BorderColor)>,
) {
    if let Ok((select, mut bg_color, mut border_color)) = select_query.get_mut(trigger.target()) {
        if !select.disabled {
            let (normal_bg, normal_border, _) = select.calculate_trigger_colors();
            *bg_color = BackgroundColor(normal_bg);
            *border_color = BorderColor(normal_border);
        }
    }
}

/// Spawns a dropdown directly as child of the select (simpler approach)
fn spawn_select_dropdown(commands: &mut Commands, select_entity: Entity, select: &SelectComponent) {
    info!("Spawning dropdown for select entity: {:?}", select_entity);

    // Calculate option positioning for Radix-style behavior
    let option_height = select.calculate_option_height();
    let option_labels = vec![
        "Apple".to_string(),
        "Orange".to_string(),
        "Grape".to_string(),
        "Carrot".to_string(),
        "Potato".to_string(),
    ];

    // Find the index of the selected option
    let selected_index = if let Some(selected_value) = &select.selected_value {
        // Try to find by direct value match first
        option_labels
            .iter()
            .position(|label| label == selected_value)
            .or_else(|| {
                // Try to find by option-{i} format
                if selected_value.starts_with("option-") {
                    selected_value
                        .strip_prefix("option-")
                        .and_then(|s| s.parse::<usize>().ok())
                        .filter(|&i| i < option_labels.len())
                } else {
                    None
                }
            })
    } else {
        None
    };

    // Calculate Y offset so selected option aligns with trigger
    let y_offset = if let Some(index) = selected_index {
        // Negative offset to move dropdown up so selected option aligns with trigger
        -(index as f32 * option_height)
    } else {
        // No selection - default positioning (first option aligns with trigger)
        0.0
    };

    // Create dropdown with simple positioning - no Portal for now
    let dropdown_entity = commands
        .spawn((
            SelectDropdownComponent {
                select_entity,
                is_open: true,
            },
            Node {
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(4.0)),
                border: UiRect::all(Val::Px(1.0)),
                width: Val::Px(200.0),
                height: Val::Auto,
                top: Val::Px(40.0 + y_offset - 40.0), // Position below trigger with offset
                left: Val::Px(0.0),
                ..default()
            },
            BackgroundColor(select.get_color_palette().surface),
            BorderColor(select.get_color_palette().border),
            BorderRadius::all(Val::Px(6.0)),
            GlobalZIndex(1), // Very high Z-index
        ))
        .id();

    info!(
        "Created dropdown with Radix-style offset {} for selected index {:?} (selected_value: {:?})",
        y_offset, selected_index, select.selected_value
    );

    // Add sample options for now (Phase 3 will make this dynamic)
    let option_labels = ["Apple", "Orange", "Grape", "Carrot", "Potato"];
    let option_entities = option_labels
        .iter()
        .enumerate()
        .map(|(i, option_text)| {
            let is_selected = select.selected_value.as_ref().map_or(false, |selected| {
                selected == &format!("option-{}", i) || selected == option_text
            });

            let option_entity = commands
                .spawn((
                    SelectOptionComponent::new(format!("option-{}", i), option_text.to_string()),
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(option_height),
                        padding: UiRect::axes(Val::Px(12.0), Val::Px(8.0)),
                        ..default()
                    },
                    BackgroundColor(if is_selected {
                        select.get_color_palette().bg_subtle
                    } else {
                        Color::NONE
                    }),
                    BorderColor(Color::NONE),
                ))
                .id();

            // Add interaction observers for option clicks
            commands
                .entity(option_entity)
                .observe(on_select_option_click)
                .observe(on_select_option_hover)
                .observe(on_select_option_hover_out);

            // Add text child for option label
            commands.entity(option_entity).with_children(|option| {
                option.spawn(Text::new(option_text.to_string()).build());
            });

            option_entity
        })
        .collect::<Vec<_>>();

    // Add all option entities as children to the dropdown
    commands
        .entity(dropdown_entity)
        .add_children(&option_entities);

    // Add as child to select for simple relative positioning
    commands.entity(select_entity).add_child(dropdown_entity);
}

/// Simple positioning system - not needed anymore with relative positioning
pub fn position_select_dropdowns(
    _select_query: Query<
        (&GlobalTransform, &SelectComponent, &Node),
        (With<SelectComponent>, Without<SelectDropdownComponent>),
    >,
    _dropdown_query: Query<(&mut Node, &SelectDropdownComponent)>,
    _window_query: Query<&Window>,
) {
    // Using relative positioning now - no coordinate conversion needed
}

/// Handles option click to select value
fn on_select_option_click(
    trigger: Trigger<Pointer<Click>>,
    option_query: Query<&SelectOptionComponent>,
    mut select_query: Query<&mut SelectComponent>,
    mut commands: Commands,
    dropdown_query: Query<(Entity, &SelectDropdownComponent)>,
    mut select_change_events: EventWriter<SelectChangeEvent>,
    mut select_open_events: EventWriter<SelectOpenEvent>,
) {
    if let Ok(option) = option_query.get(trigger.target()) {
        // Find the dropdown entity and its associated select entity
        let mut select_entity_opt = None;
        let mut dropdown_to_despawn = None;

        for (dropdown_entity, dropdown_comp) in &dropdown_query {
            select_entity_opt = Some(dropdown_comp.select_entity);
            dropdown_to_despawn = Some(dropdown_entity);
            break; // For now, handle first dropdown found
        }

        if let (Some(select_entity), Some(dropdown_entity)) =
            (select_entity_opt, dropdown_to_despawn)
        {
            if let Ok(mut select) = select_query.get_mut(select_entity) {
                let previous_value = select.selected_value.clone();

                // Update select state
                select.selected_value = Some(option.value.clone());
                select.open = false;
                select.state = SelectState::Closed;

                // Send change event
                select_change_events.write(SelectChangeEvent {
                    select_entity,
                    selected_value: option.value.clone(),
                    previous_value,
                    selected_label: option.label.clone(),
                });

                // Send close event
                select_open_events.write(SelectOpenEvent {
                    select_entity,
                    open: false,
                });

                // Close dropdown
                commands.entity(dropdown_entity).despawn();

                info!("Selected option: {} ({})", option.label, option.value);
            }
        }
    }
}

/// Handles option hover for visual feedback
fn on_select_option_hover(
    trigger: Trigger<Pointer<Over>>,
    mut option_query: Query<&mut BackgroundColor, With<SelectOptionComponent>>,
    select_query: Query<&SelectComponent>,
) {
    if let Ok(mut bg_color) = option_query.get_mut(trigger.target()) {
        // Get hover color from theme (using gray palette as default)
        let hover_color = if theme_mode() == ThemeMode::Dark {
            UiColorPalettes::dark_mode().gray.bg_hover
        } else {
            UiColorPalettes::light_mode().gray.bg_hover
        };

        *bg_color = BackgroundColor(hover_color);
    }
}

/// Handles option hover out to reset colors
fn on_select_option_hover_out(
    trigger: Trigger<Pointer<Out>>,
    mut option_query: Query<&mut BackgroundColor, With<SelectOptionComponent>>,
) {
    if let Ok(mut bg_color) = option_query.get_mut(trigger.target()) {
        *bg_color = BackgroundColor(Color::NONE);
    }
}

/// Updates trigger text when selection changes
pub fn update_select_trigger_text(
    mut select_change_events: EventReader<SelectChangeEvent>,
    select_query: Query<&SelectComponent>,
    children_query: Query<&Children>,
    mut text_query: Query<&mut bevy::ui::widget::Text>,
) {
    for event in select_change_events.read() {
        if let Ok(select) = select_query.get(event.select_entity) {
            if let Ok(children) = children_query.get(event.select_entity) {
                for child in children {
                    if let Ok(mut text) = text_query.get_mut(*child) {
                        text.0 = event.selected_label.clone();
                        break;
                    }
                }
            }
        }
    }
}

/// Detects clicks outside of open dropdowns to close them
pub fn handle_click_outside_select(
    mut commands: Commands,
    dropdown_query: Query<(Entity, &SelectDropdownComponent)>,
    mut select_query: Query<&mut SelectComponent>,
    mut select_open_events: EventWriter<SelectOpenEvent>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    // TODO: Add cursor position and bounds checking in Phase 3
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        // For now, we'll implement a simple version
        // In Phase 3, we'll add proper bounds checking
        for (dropdown_entity, dropdown) in &dropdown_query {
            if let Ok(mut select) = select_query.get_mut(dropdown.select_entity) {
                // Simple click-anywhere-to-close for now
                // TODO: Add proper bounds checking to detect clicks outside dropdown
                select.open = false;
                select.state = SelectState::Closed;

                select_open_events.write(SelectOpenEvent {
                    select_entity: dropdown.select_entity,
                    open: false,
                });

                commands.entity(dropdown_entity).despawn();
                info!("Closed dropdown due to outside click");
            }
        }
    }
}
