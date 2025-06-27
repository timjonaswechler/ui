use crate::{
    assets::InterfaceIconId,
    theme::color::{accent_palette, UiColorPalette},
};
use bevy::prelude::*;
use bevy_picking::prelude::{Click, Out, Over, Pickable, Pointer, Pressed, Released};

/// Event emitted when a toggle component's state changes.
///
/// This event is triggered whenever a user interacts with a toggle and changes
/// its pressed state. The event provides information about which toggle was
/// changed and its new state.
///
/// ### Example
///
/// ```rust,no_run
/// use bevy::prelude::*;
/// use ui::components::toggle::ToggleChangeEvent;
///
/// fn handle_toggle_events(mut toggle_events: EventReader<ToggleChangeEvent>) {
///     for event in toggle_events.read() {
///         info!(
///             "Toggle {:?} changed to pressed: {}",
///             event.toggle_entity,
///             event.pressed
///         );
///     }
/// }
/// ```
/// its pressed state. The event provides information about which toggle was
/// changed and its new state.
///
/// ### Example
///
/// ```rust
/// use bevy::prelude::*;
/// use ui::components::toggle::ToggleChangeEvent;
///
/// fn handle_toggle_events(mut toggle_events: EventReader<ToggleChangeEvent>) {
///     for event in toggle_events.read() {
///         info!(
///             "Toggle {:?} changed to pressed: {}",
///             event.toggle_entity,
///             event.pressed
///         );
///     }
/// }
/// ```
#[derive(Event, Debug, Clone)]
pub struct ToggleChangeEvent {
    /// The entity of the toggle component that changed
    pub toggle_entity: Entity,
    /// Whether the toggle is now in the pressed/active state
    pub pressed: bool,
    /// The size variant of the toggle that changed
    pub size: ToggleSize,
}

/// Visual states that a toggle component can be in.
///
/// These states determine the appearance of the toggle component and are used
/// internally to calculate appropriate colors and styling. State transitions
/// are handled automatically based on user interactions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToggleState {
    /// Default state when not being interacted with
    Normal,
    /// State when the mouse cursor is hovering over the toggle
    Hover,
    /// State when the toggle is being actively pressed/clicked
    Active,
    /// State when the toggle is disabled and cannot be interacted with
    Disabled,
}

/// Core toggle component that provides stateful button-like behavior.
///
/// A toggle component allows users to switch between two states: pressed (active)
/// and unpressed (inactive). Unlike regular buttons, toggles maintain their state
/// after being clicked, making them ideal for options, settings, and selections.
///
/// The component supports various visual variants, sizes, colors, and can include
/// both text labels and icons. It follows Radix UI design principles and provides
/// comprehensive accessibility support.
///
/// ### Example
///
/// ```rust,no_run
/// use bevy::prelude::*;
/// use ui::components::toggle::*;
/// use ui::theme::color::success_palette;
///
/// fn setup_ui(mut commands: Commands) {
///     // Basic toggle
///     commands.spawn(
///         ToggleComponent::new("basic")
///             .text("Toggle me")
///             .variant(ToggleVariant::Soft)
///             .build()
///     );
///     
///     // Pre-pressed toggle with custom color
///     commands.spawn(
///         ToggleComponent::new("active")
///             .text("Active")
///             .pressed()
///             .color(success_palette())
///             .build()
///     );
/// }
/// ```
#[derive(Component, Debug, Clone)]
pub struct ToggleComponent {
    /// The size variant determining dimensions and padding
    pub size: ToggleSize,
    /// The visual variant affecting styling approach
    pub variant: ToggleVariant,
    /// The color palette used for theming
    pub color: UiColorPalette,
    /// Whether the toggle is currently in the pressed/active state
    pub pressed: bool,
    /// Whether the toggle is disabled and non-interactive
    pub disabled: bool,
    /// Whether to use high contrast theming (accessibility feature)
    pub high_contrast: bool,
    /// Current visual state (normal, hover, active, disabled)
    pub current_state: ToggleState,
    /// Internal state tracking if the toggle is currently being clicked
    pub is_clicking: bool,
}

impl Default for ToggleComponent {
    fn default() -> Self {
        Self {
            size: ToggleSize::Size2,
            variant: ToggleVariant::Soft,
            color: accent_palette(),
            pressed: false,
            disabled: false,
            high_contrast: false,
            current_state: ToggleState::Normal,
            is_clicking: false,
        }
    }
}

/// Size variants for toggle components.
///
/// Each size variant provides different dimensions appropriate for various UI contexts.
/// The size affects height, padding, and font size to maintain visual harmony.
///
/// ### Example
///
/// ```rust,no_run
/// use ui::components::toggle::*;
///
/// // Small toggle for compact UIs
/// let small = ToggleComponent::new("small").size_1().build();
///
/// // Large toggle for prominent actions
/// let large = ToggleComponent::new("large").size_4().build();
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ToggleSize {
    /// Small size - 24px height, suitable for compact interfaces
    Size1,
    /// Default size - 32px height, recommended for most use cases
    #[default]
    Size2,
    /// Large size - 40px height, good for prominent toggles
    Size3,
    /// Extra large size - 48px height, ideal for touch interfaces
    Size4,
}

impl ToggleSize {
    /// Returns the height in pixels for this size variant.
    ///
    /// This is used internally for layout calculations and ensures consistent
    /// sizing across all toggle components.
    pub fn height(self) -> f32 {
        match self {
            ToggleSize::Size1 => 24.0,
            ToggleSize::Size2 => 32.0,
            ToggleSize::Size3 => 40.0,
            ToggleSize::Size4 => 48.0,
        }
    }

    /// Returns the internal padding in pixels for this size variant.
    ///
    /// Padding scales proportionally with size to maintain visual balance
    /// between the toggle border and its content.
    pub fn padding(self) -> f32 {
        match self {
            ToggleSize::Size1 => 4.0,
            ToggleSize::Size2 => 8.0,
            ToggleSize::Size3 => 12.0,
            ToggleSize::Size4 => 16.0,
        }
    }

    /// Returns the appropriate font size in pixels for text content.
    ///
    /// Font sizes are carefully chosen to ensure readability while maintaining
    /// proportional scaling with the toggle size.
    pub fn font_size(self) -> f32 {
        match self {
            ToggleSize::Size1 => 12.0,
            ToggleSize::Size2 => 14.0,
            ToggleSize::Size3 => 16.0,
            ToggleSize::Size4 => 18.0,
        }
    }
}

/// Visual style variants for toggle components.
///
/// Each variant provides a different visual approach to styling the toggle,
/// allowing for flexible integration into various design systems while
/// maintaining consistent behavior.
///
/// ### Example
///
/// ```rust,no_run
/// use ui::components::toggle::*;
///
/// // Subtle background variant (default)
/// let soft = ToggleComponent::new("soft").variant(ToggleVariant::Soft).build();
///
/// // Prominent solid variant
/// let solid = ToggleComponent::new("solid").variant(ToggleVariant::Solid).build();
///
/// // Minimal surface variant
/// let surface = ToggleComponent::new("surface").variant(ToggleVariant::Surface).build();
///
/// // Border-focused outline variant
/// let outline = ToggleComponent::new("outline").variant(ToggleVariant::Outline).build();
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ToggleVariant {
    /// Solid variant with pronounced styling and strong visual presence
    Solid,
    /// Default soft variant with subtle background styling
    #[default]
    Soft,
    /// Surface variant with minimal styling, nearly transparent
    Surface,
    /// Outline variant with visible border and transparent background
    Outline,
}

/// Builder for creating toggle components with a fluent API.
///
/// The `ToggleBuilder` provides a convenient and type-safe way to construct
/// toggle components with various options. It follows the builder pattern,
/// allowing method chaining for readable configuration.
///
/// Create a new builder using [`ToggleComponent::new()`], then chain configuration
/// methods before calling [`build()`](ToggleBuilder::build) to create the final component bundle.
///
/// ### Example
///
/// ```rust,no_run
/// use bevy::prelude::*;
/// use ui::components::toggle::*;
/// use ui::theme::color::success_palette;
///
/// fn setup_toggle(mut commands: Commands) {
///     commands.spawn(
///         ToggleComponent::new("settings_toggle")
///             .text("Enable notifications")
///             .size_3()
///             .color(success_palette())
///             .pressed()
///             .build()
///     );
/// }
/// ```
pub struct ToggleBuilder {
    name: String,
    toggle: ToggleComponent,
    text: Option<String>,
    icon: Option<InterfaceIconId>,
}

impl ToggleBuilder {
    /// Creates a new toggle builder with the specified name.
    ///
    /// The name is used for debugging and entity identification. It will be
    /// prefixed with "_Toggle" to create the full entity name.
    ///
    /// # Arguments
    ///
    /// * `name` - A unique identifier for this toggle component
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: format!("{}_Toggle", name.into()),
            toggle: ToggleComponent::default(),
            text: None,
            icon: None,
        }
    }

    /// Sets the size variant for the toggle.
    ///
    /// # Arguments
    ///
    /// * `size` - The size variant to use
    pub fn size(mut self, size: ToggleSize) -> Self {
        self.toggle.size = size;
        self
    }

    /// Sets the toggle to use Size1 (24px height) - compact size.
    pub fn size_1(self) -> Self {
        self.size(ToggleSize::Size1)
    }

    /// Sets the toggle to use Size2 (32px height) - default size.
    pub fn size_2(self) -> Self {
        self.size(ToggleSize::Size2)
    }

    /// Sets the toggle to use Size3 (40px height) - large size.
    pub fn size_3(self) -> Self {
        self.size(ToggleSize::Size3)
    }

    /// Sets the toggle to use Size4 (48px height) - extra large size.
    pub fn size_4(self) -> Self {
        self.size(ToggleSize::Size4)
    }

    /// Sets the visual variant for the toggle.
    ///
    /// # Arguments
    ///
    /// * `variant` - The visual variant to use
    pub fn variant(mut self, variant: ToggleVariant) -> Self {
        self.toggle.variant = variant;
        self
    }

    /// Sets the toggle to use the solid variant with pronounced styling.
    pub fn solid(self) -> Self {
        self.variant(ToggleVariant::Solid)
    }

    /// Sets the toggle to use the soft variant with subtle background styling (default).
    pub fn soft(self) -> Self {
        self.variant(ToggleVariant::Soft)
    }

    /// Sets the toggle to use the surface variant with minimal styling.
    pub fn surface(self) -> Self {
        self.variant(ToggleVariant::Surface)
    }

    /// Sets the toggle to use the outline variant with visible border.
    pub fn outline(self) -> Self {
        self.variant(ToggleVariant::Outline)
    }

    /// Sets the color palette for the toggle.
    ///
    /// The color palette defines all colors used for different states and variants.
    /// Use predefined palettes like `accent_palette()`, `success_palette()`, etc.
    ///
    /// # Arguments
    ///
    /// * `color` - The color palette to use for theming
    pub fn color(mut self, color: UiColorPalette) -> Self {
        self.toggle.color = color;
        self
    }

    /// Sets the toggle to start in the pressed/active state.
    ///
    /// By default, toggles start in the unpressed state. This method
    /// allows you to create toggles that are initially active.
    pub fn pressed(mut self) -> Self {
        self.toggle.pressed = true;
        self
    }

    /// Sets the toggle to be disabled and non-interactive.
    ///
    /// Disabled toggles cannot be clicked and are visually distinguished
    /// with reduced opacity and muted colors.
    pub fn disabled(mut self) -> Self {
        self.toggle.disabled = true;
        self.toggle.current_state = ToggleState::Disabled;
        self
    }

    /// Enables high contrast mode for improved accessibility.
    ///
    /// High contrast mode uses stronger color differences to improve
    /// visibility for users with visual impairments.
    pub fn high_contrast(mut self) -> Self {
        self.toggle.high_contrast = true;
        self
    }

    /// Sets the text label for the toggle.
    ///
    /// The text will be displayed inside the toggle button. Text can be
    /// combined with icons to create rich toggle controls.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to display in the toggle
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    /// Sets an icon for the toggle.
    ///
    /// Icons can be used alone or combined with text. The icon will be
    /// tinted according to the current theme colors.
    ///
    /// # Arguments
    ///
    /// * `icon` - The interface icon to display
    ///
    /// # Note
    ///
    /// Icon rendering is currently being implemented and may not be fully functional.
    pub fn icon(mut self, icon: InterfaceIconId) -> Self {
        self.icon = Some(icon);
        // TODO: Implement icon support
        self
    }
}

impl ToggleComponent {
    /// Creates a new toggle builder with the specified name.
    ///
    /// This is the primary entry point for creating toggle components.
    /// Use the returned builder to configure the toggle before calling
    /// [`build()`](ToggleBuilder::build).
    ///
    /// # Arguments
    ///
    /// * `name` - A unique identifier for this toggle component
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use ui::components::toggle::ToggleComponent;
    ///
    /// let toggle = ToggleComponent::new("my_toggle")
    ///     .text("Click me")
    ///     .build();
    /// ```
    pub fn new(name: impl Into<String>) -> ToggleBuilder {
        ToggleBuilder::new(name)
    }

    /// Calculates the current styling for the toggle based on its state.
    ///
    /// This method computes the appropriate colors for background, border,
    /// and text based on the current configuration and interaction state.
    ///
    /// # Arguments
    ///
    /// * `state` - The visual state to calculate styling for
    ///
    /// # Returns
    ///
    /// A `ToggleStyling` struct containing all computed visual properties.
    pub fn get_styling(&self, state: ToggleState) -> ToggleStyling {
        let current_state = if self.disabled {
            ToggleState::Disabled
        } else {
            state
        };

        ToggleStyling {
            background_color: self.calculate_background_color(current_state),
            border_color: self.calculate_border_color(current_state),
            text_color: self.calculate_text_color(current_state),
        }
    }

    /// Calculates the background color for a given state.
    ///
    /// This internal method determines the appropriate background color based on
    /// the toggle's variant, pressed state, and current interaction state.
    fn calculate_background_color(&self, state: ToggleState) -> BackgroundColor {
        let base_color = match (self.pressed, self.variant, state) {
            // Pressed states
            (true, ToggleVariant::Solid, ToggleState::Normal) => self.color.solid,
            (true, ToggleVariant::Solid, ToggleState::Hover) => self.color.solid_hover,
            (true, ToggleVariant::Solid, ToggleState::Active) => self.color.solid_hover,
            (true, ToggleVariant::Soft, ToggleState::Normal) => self.color.bg_active,
            (true, ToggleVariant::Soft, ToggleState::Hover) => self.color.bg_hover,
            (true, ToggleVariant::Soft, ToggleState::Active) => self.color.bg_hover,
            (true, ToggleVariant::Surface, ToggleState::Normal) => self.color.bg_subtle,
            (true, ToggleVariant::Surface, ToggleState::Hover) => self.color.bg_hover,
            (true, ToggleVariant::Surface, ToggleState::Active) => self.color.bg_active,
            (true, ToggleVariant::Outline, ToggleState::Normal) => self.color.bg_subtle,
            (true, ToggleVariant::Outline, ToggleState::Hover) => self.color.bg_hover,
            (true, ToggleVariant::Outline, ToggleState::Active) => self.color.bg_active,

            // Unpressed states
            (false, ToggleVariant::Solid, ToggleState::Normal) => Color::NONE,
            (false, ToggleVariant::Solid, ToggleState::Hover) => self.color.bg_hover,
            (false, ToggleVariant::Solid, ToggleState::Active) => self.color.bg_active,
            (false, ToggleVariant::Soft, ToggleState::Normal) => Color::NONE,
            (false, ToggleVariant::Soft, ToggleState::Hover) => self.color.bg_hover,
            (false, ToggleVariant::Soft, ToggleState::Active) => self.color.bg_active,
            (false, ToggleVariant::Surface, ToggleState::Normal) => Color::NONE,
            (false, ToggleVariant::Surface, ToggleState::Hover) => self.color.bg_hover,
            (false, ToggleVariant::Surface, ToggleState::Active) => self.color.bg_active,
            (false, ToggleVariant::Outline, ToggleState::Normal) => Color::NONE,
            (false, ToggleVariant::Outline, ToggleState::Hover) => self.color.bg_hover,
            (false, ToggleVariant::Outline, ToggleState::Active) => self.color.bg_active,

            // Disabled state
            (_, _, ToggleState::Disabled) => {
                if self.pressed {
                    self.color.bg_subtle
                } else {
                    Color::NONE
                }
            }
        };

        let mut bg_color = BackgroundColor(base_color);

        if state == ToggleState::Disabled {
            let srgba = bg_color.0.to_srgba();
            bg_color.0 = Color::srgba(srgba.red, srgba.green, srgba.blue, 0.5);
        }

        bg_color
    }

    /// Calculates the border color for a given state.
    ///
    /// Border colors are primarily used by the Outline variant to create
    /// visible borders. Other variants typically use transparent borders.
    fn calculate_border_color(&self, state: ToggleState) -> BorderColor {
        match (self.variant, state) {
            (ToggleVariant::Outline, ToggleState::Normal) => {
                if self.pressed {
                    BorderColor(self.color.solid)
                } else {
                    BorderColor(self.color.border)
                }
            }
            (ToggleVariant::Outline, ToggleState::Hover) => {
                if self.pressed {
                    BorderColor(self.color.solid_hover)
                } else {
                    BorderColor(self.color.border_hover)
                }
            }
            (ToggleVariant::Outline, ToggleState::Active) => BorderColor(self.color.solid),
            (ToggleVariant::Outline, ToggleState::Disabled) => {
                let color = if self.pressed {
                    self.color.solid
                } else {
                    self.color.border
                };
                BorderColor(color.with_alpha(0.5))
            }
            _ => BorderColor(Color::NONE),
        }
    }

    /// Calculates the text color for a given state.
    ///
    /// Text colors are adjusted based on the background to ensure proper
    /// contrast and readability across all variants and states.
    fn calculate_text_color(&self, state: ToggleState) -> Color {
        match (self.pressed, self.variant, state) {
            // Pressed states
            (true, ToggleVariant::Solid, _) => self.color.text_contrast,
            (true, ToggleVariant::Soft, _) => self.color.text,
            (true, ToggleVariant::Surface, _) => self.color.text,
            (true, ToggleVariant::Outline, _) => self.color.text,

            // Unpressed states
            (false, _, _) => self.color.text,
            // Disabled state overrides
        }
        .with_alpha(if state == ToggleState::Disabled {
            0.5
        } else {
            1.0
        })
    }
}

/// Complete styling information for a toggle component.
///
/// This struct contains all the computed visual properties needed to render
/// a toggle in its current state. It's generated by the toggle component
/// based on its configuration and current interaction state.
#[derive(Debug, Clone)]
pub struct ToggleStyling {
    /// The background color for the toggle
    pub background_color: BackgroundColor,
    /// The border color (used primarily by outline variant)
    pub border_color: BorderColor,
    /// The color for text content within the toggle
    pub text_color: Color,
}

impl ToggleBuilder {
    /// Calculates the base layout style for the toggle.
    ///
    /// This internal method determines dimensions, padding, and layout properties
    /// based on the configured size and variant.
    fn calculate_style(&self) -> Node {
        let size = self.toggle.size;
        let padding = size.padding();

        Node {
            height: Val::Px(size.height()),
            padding: UiRect::all(Val::Px(padding)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: match self.toggle.variant {
                ToggleVariant::Outline => UiRect::all(Val::Px(1.0)),
                _ => UiRect::all(Val::Px(0.0)),
            },
            ..default()
        }
    }

    /// Calculates the initial background color for the toggle.
    fn calculate_background_color(&self) -> BackgroundColor {
        self.toggle
            .get_styling(ToggleState::Normal)
            .background_color
    }

    /// Calculates the initial border color for the toggle.
    fn calculate_border_color(&self) -> BorderColor {
        self.toggle.get_styling(ToggleState::Normal).border_color
    }

    /// Calculates the border radius for the toggle.
    fn calculate_border_radius(&self) -> BorderRadius {
        BorderRadius::all(Val::Px(4.0))
    }
}

impl ToggleBuilder {
    /// Builds the final toggle component with all configured options.
    ///
    /// This method consumes the builder and creates a complete Bevy entity bundle
    /// that can be spawned into the world. The bundle includes all necessary
    /// components for rendering, interaction, and state management.
    ///
    /// # Returns
    ///
    /// A bundle containing all components needed for the toggle to function properly.
    /// This includes the toggle component itself, styling components, and interaction
    /// handlers.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use bevy::prelude::*;
    /// use ui::components::toggle::ToggleComponent;
    ///
    /// fn spawn_toggle(mut commands: Commands) {
    ///     commands.spawn(
    ///         ToggleComponent::new("example")
    ///             .text("Toggle me")
    ///             .size_2()
    ///             .build()
    ///     );
    /// }
    /// ```
    pub fn build(self) -> impl Bundle {
        let node = self.calculate_style();
        let background_color = self.calculate_background_color();
        let border_color = self.calculate_border_color();
        let border_radius = self.calculate_border_radius();

        let bundle = (
            self.toggle.clone(),
            node,
            background_color,
            border_color,
            border_radius,
            Pickable::default(),
            Button,
            Interaction::None,
        );

        // Always add content spawner for icons and/or text
        (
            bundle,
            ToggleContentSpawner {
                text: self.text,
                icon: self.icon,
                font_size: self.toggle.size.font_size(),
                text_color: self.toggle.get_styling(ToggleState::Normal).text_color,
            },
        )
    }
}

/// Internal component used to spawn toggle content (text and icons).
///
/// This component is temporarily attached to toggle entities to coordinate
/// the spawning of child entities for text and icon content. It's automatically
/// removed after processing.
#[derive(Component)]
pub struct ToggleContentSpawner {
    text: Option<String>,
    icon: Option<InterfaceIconId>,
    font_size: f32,
    text_color: Color,
}

/// Marker component for toggle icons during the creation process.
///
/// This component temporarily holds icon information while the icon
/// system sets up the proper icon rendering components.
#[derive(Component)]
pub struct ToggleIconMarker {
    icon: InterfaceIconId,
    color: Color,
}

/// System that spawns child entities for toggle text and icon content.
///
/// This system processes newly created toggles with `ToggleContentSpawner` components
/// and creates the appropriate child entities for text labels and icons. It runs
/// automatically as part of the UI update loop.
///
/// The system handles:
/// - Creating text entities with proper styling
/// - Setting up icon marker components for later processing
/// - Cleaning up temporary spawner components
pub fn spawn_toggle_children(
    mut commands: Commands,
    query: Query<(Entity, &ToggleContentSpawner), Added<ToggleContentSpawner>>,
) {
    for (entity, spawner) in &query {
        commands.entity(entity).with_children(|parent| {
            // Create a container for icon + text if both exist
            let has_text = spawner.text.as_ref().map_or(false, |t| !t.is_empty());
            let has_icon = spawner.icon.is_some();

            // For now, only handle text. Icon support coming later
            if has_text {
                if let Some(text) = &spawner.text {
                    parent.spawn((
                        Name::new("ToggleText"),
                        Text::new(text),
                        TextColor(spawner.text_color),
                        TextFont {
                            font_size: spawner.font_size,
                            ..default()
                        },
                        Pickable::IGNORE,
                    ));
                }
            }

            // Add icon support
            if has_icon {
                if let Some(icon_id) = spawner.icon {
                    parent.spawn((
                        Name::new("ToggleIcon"),
                        ToggleIconMarker {
                            icon: icon_id,
                            color: spawner.text_color,
                        },
                        Pickable::IGNORE,
                    ));
                }
            }
        });

        // Remove the spawner component
        commands.entity(entity).remove::<ToggleContentSpawner>();
    }
}

/// System that converts icon markers into actual icon components.
///
/// This system processes `ToggleIconMarker` components and replaces them with
/// the appropriate icon rendering components from the interface atlas system.
/// It requires the interface atlases to be loaded before it can function.
///
/// # Note
///
/// Icon support is currently being implemented and may not be fully functional.
pub fn spawn_toggle_icons(
    mut commands: Commands,
    query: Query<(Entity, &ToggleIconMarker), Added<ToggleIconMarker>>,
    interface_atlases: Option<Res<crate::assets::icons::interface::InterfaceAtlases>>,
) {
    use crate::assets::icons::interface::InterfaceIcon;

    if let Some(atlases) = interface_atlases {
        for (entity, marker) in &query {
            // Replace the marker with the actual icon
            commands.entity(entity).remove::<ToggleIconMarker>();
            commands.entity(entity).insert(
                InterfaceIcon::new(marker.icon)
                    .tint(marker.color)
                    .bundle(&atlases),
            );
        }
    }
}

/// System that sets up interaction observers for toggle components.
///
/// This system runs when new toggle components are added to the world and
/// attaches the necessary event observers for handling user interactions
/// like clicks, hovers, and presses.
///
/// The interaction system enables:
/// - Toggle state changes on click
/// - Visual feedback during hover and press states
/// - Proper disabled state handling
/// - Event emission for toggle state changes
pub fn setup_toggle_interactions(
    mut commands: Commands,
    toggles: Query<Entity, Added<ToggleComponent>>,
) {
    for entity in &toggles {
        commands
            .entity(entity)
            .observe(on_toggle_click)
            .observe(on_toggle_hover_start)
            .observe(on_toggle_hover_end)
            .observe(on_toggle_press)
            .observe(on_toggle_release);
    }
}

// Observer functions for toggle interactions
fn on_toggle_click(_trigger: Trigger<Pointer<Click>>) {
    // Empty - we handle toggle on click
}

fn on_toggle_hover_start(
    trigger: Trigger<Pointer<Over>>,
    mut toggles: Query<&mut ToggleComponent>,
) {
    let entity = trigger.target();
    if let Ok(mut toggle) = toggles.get_mut(entity) {
        if !toggle.disabled {
            toggle.current_state = ToggleState::Hover;
        }
    }
}

fn on_toggle_hover_end(trigger: Trigger<Pointer<Out>>, mut toggles: Query<&mut ToggleComponent>) {
    let entity = trigger.target();
    if let Ok(mut toggle) = toggles.get_mut(entity) {
        if !toggle.disabled {
            toggle.current_state = ToggleState::Normal;
            toggle.is_clicking = false;
        }
    }
}

fn on_toggle_press(trigger: Trigger<Pointer<Pressed>>, mut toggles: Query<&mut ToggleComponent>) {
    let entity = trigger.target();
    if let Ok(mut toggle) = toggles.get_mut(entity) {
        if !toggle.disabled {
            toggle.current_state = ToggleState::Active;
            toggle.is_clicking = true;
        }
    }
}

fn on_toggle_release(
    trigger: Trigger<Pointer<Released>>,
    mut toggles: Query<&mut ToggleComponent>,
    mut events: EventWriter<ToggleChangeEvent>,
) {
    let entity = trigger.target();
    if let Ok(mut toggle) = toggles.get_mut(entity) {
        if !toggle.disabled && toggle.is_clicking {
            // Toggle the pressed state
            toggle.pressed = !toggle.pressed;
            toggle.is_clicking = false;
            toggle.current_state = ToggleState::Hover;

            // Send change event
            events.write(ToggleChangeEvent {
                toggle_entity: entity,
                pressed: toggle.pressed,
                size: toggle.size,
            });
        }
    }
}

/// System that updates visual styling when toggle state changes.
///
/// This system monitors toggle components for changes and updates their visual
/// appearance accordingly. It handles background colors, border colors, and
/// text/icon colors for the toggle and all its child elements.
///
/// The system automatically:
/// - Updates colors based on current state (normal, hover, active, disabled)
/// - Applies theme colors from the configured palette
/// - Propagates color changes to child text and icon elements
/// - Maintains visual consistency across state transitions
pub fn update_toggle_styling(
    toggles_query: Query<(Entity, &ToggleComponent), Changed<ToggleComponent>>,
    mut bg_colors: Query<&mut BackgroundColor, (With<ToggleComponent>, Without<Text>)>,
    mut border_colors: Query<&mut BorderColor, With<ToggleComponent>>,
    children_query: Query<&Children>,
    mut text_colors: Query<&mut TextColor, With<Text>>,
    mut icon_markers: Query<&mut ToggleIconMarker>,
) {
    for (entity, toggle) in &toggles_query {
        let styling = toggle.get_styling(toggle.current_state);

        // Update toggle background color
        if let Ok(mut bg_color) = bg_colors.get_mut(entity) {
            *bg_color = styling.background_color;
        }

        // Update border color
        if let Ok(mut border_color) = border_colors.get_mut(entity) {
            *border_color = styling.border_color;
        }

        // Update text and icon colors recursively
        update_children_colors(
            &children_query,
            entity,
            styling.text_color,
            &mut text_colors,
            &mut icon_markers,
        );
    }
}

/// Helper function to recursively update colors of child elements.
///
/// This internal function traverses the entity hierarchy and updates text
/// and icon colors to match the parent toggle's current styling.
fn update_children_colors(
    children_query: &Query<&Children>,
    entity: Entity,
    color: Color,
    text_colors: &mut Query<&mut TextColor, With<Text>>,
    icon_markers: &mut Query<&mut ToggleIconMarker>,
) {
    if let Ok(children) = children_query.get(entity) {
        for child in children.iter() {
            // Update text color
            if let Ok(mut text_color) = text_colors.get_mut(child) {
                *text_color = TextColor(color);
            }

            // Update icon marker color for later processing
            if let Ok(mut icon_marker) = icon_markers.get_mut(child) {
                icon_marker.color = color;
            }

            // Recursively update grandchildren
            update_children_colors(children_query, child, color, text_colors, icon_markers);
        }
    }
}

/// Type alias for `ToggleComponent` providing a shorter name.
///
/// This alias can be used as a more concise alternative to `ToggleComponent`
/// in contexts where brevity is preferred.
///
/// # Example
///
/// ```rust,no_run
/// use ui::components::toggle::{Toggle, ToggleComponent};
///
/// // These are equivalent:
/// let toggle1 = Toggle::new("short").build();
/// let toggle2 = ToggleComponent::new("long").build();
/// ```
pub type Toggle = ToggleComponent;
