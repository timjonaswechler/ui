use crate::{
    components::text::{Text, TextBuilder},
    theme::{
        color::{TextColor as TextColorEnum, TextContrastLevel, UiColorPalette},
        typography::{FontFamily, TextSize, TextWeight},
    },
};
use bevy::prelude::*;

use super::core::{Button, ButtonRadius, ButtonSize, ButtonVariant};

/// Builder for constructing Button components using a fluent API.
///
/// The ButtonBuilder provides a convenient way to create and customize Button components
/// with method chaining. It handles the complex logic of mapping button properties
/// to Bevy UI components, styling, interaction handling, and accessibility features.
///
/// # Example
/// ```text
/// let button = Button::builder("my-button")
///     .text("Click Me")
///     .solid()
///     .accent()
///     .size_large()
///     .pill()
///     .build();
/// ```
///
/// # Builder Pattern
///
/// The builder follows a fluent interface pattern where methods can be chained
/// together to configure the button. Methods are organized into categories:
/// - **Appearance**: `variant()`, `color()`, `radius()`
/// - **Size**: `size()`, convenience methods like `size_large()`
/// - **Text**: `text()`, `text_builder()`, text styling methods
/// - **State**: `loading()`, `disabled()`, `high_contrast()`
/// - **Content**: `child()`, `children()`
pub struct ButtonBuilder {
    /// Component name for debugging and identification
    name: String,
    /// Button configuration
    button: Button,
    /// Simple text content (alternative to text_builder)
    text: Option<String>,
    /// Advanced text configuration (overrides simple text)
    text_builder: Option<TextBuilder>,
    /// Additional child entities to include in the button
    children: Vec<Entity>,
}

impl ButtonBuilder {
    /// Creates a new ButtonBuilder with the specified name.
    ///
    /// # Arguments
    /// * `name` - A name for the button component (used for debugging and identification)
    ///
    /// # Returns
    /// A new ButtonBuilder with default settings
    ///
    /// # Example
    /// ```text
    /// let builder = ButtonBuilder::new("submit-button");
    /// ```
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: format!("{}_Button", name.into()),
            button: Button::default(),
            text: None,
            text_builder: None,
            children: Vec::new(),
        }
    }

    /// Sets the visual variant of the button.
    ///
    /// # Arguments
    /// * `variant` - The ButtonVariant to use (Solid, Soft, Outline, Ghost)
    ///
    /// # Example
    /// ```rust
    /// let button = Button::builder("my-button")
    ///     .variant(ButtonVariant::Outline)
    ///     .build();
    /// ```
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.button.variant = variant;
        self
    }

    /// Sets the size of the button.
    ///
    /// # Arguments
    /// * `size` - The ButtonSize to use (Small, Default, Large)
    ///
    /// # Example
    /// ```rust
    /// let button = Button::builder("my-button")
    ///     .size(ButtonSize::Large)
    ///     .build();
    /// ```
    pub fn size(mut self, size: ButtonSize) -> Self {
        self.button.size = size;
        self
    }

    /// Sets the color palette for the button.
    ///
    /// # Arguments
    /// * `color` - A UiColorPalette from the theme system
    ///
    /// # Example
    /// ```rust
    /// let button = Button::builder("my-button")
    ///     .color(theme().blue)
    ///     .build();
    /// ```
    pub fn color(mut self, color: UiColorPalette) -> Self {
        self.button.color = color;
        self
    }

    /// Enables high contrast mode for better accessibility.
    ///
    /// High contrast mode increases the color contrast between
    /// text and background for improved readability.
    ///
    /// # Example
    /// ```rust
    /// let accessible_button = Button::builder("accessible")
    ///     .text("Submit")
    ///     .high_contrast()
    ///     .build();
    /// ```
    pub fn high_contrast(mut self) -> Self {
        self.button.high_contrast = true;
        self
    }

    /// Sets automatic contrast calculation (default behavior).
    ///
    /// This method serves for clarity and future extensibility.
    /// Automatic contrast is enabled by default.
    ///
    /// # Example
    /// ```rust
    /// let button = Button::builder("my-button")
    ///     .auto_contrast()
    ///     .build();
    /// ```
    pub fn auto_contrast(self) -> Self {
        // Activates automatic contrast calculation (enabled by default)
        // This method serves for clarity and can be extended in the future
        self
    }

    /// Sets the border radius of the button.
    ///
    /// # Arguments
    /// * `radius` - The ButtonRadius to use
    ///
    /// # Example
    /// ```rust
    /// let rounded_button = Button::builder("rounded")
    ///     .radius(ButtonRadius::Large)
    ///     .build();
    /// ```
    pub fn radius(mut self, radius: ButtonRadius) -> Self {
        self.button.radius = radius;
        self
    }

    /// Sets the button to loading state.
    ///
    /// Loading state typically shows a spinner and prevents interaction
    /// while an asynchronous operation is in progress.
    ///
    /// # Example
    /// ```rust
    /// let loading_button = Button::builder("submit")
    ///     .text("Submitting...")
    ///     .loading()
    ///     .build();
    /// ```
    pub fn loading(mut self) -> Self {
        self.button.loading = true;
        self
    }

    /// Sets the button to disabled state.
    ///
    /// Disabled buttons cannot be interacted with and typically
    /// appear with reduced opacity and different styling.
    ///
    /// # Example
    /// ```rust
    /// let disabled_button = Button::builder("unavailable")
    ///     .text("Not Available")
    ///     .disabled()
    ///     .build();
    /// ```
    pub fn disabled(mut self) -> Self {
        self.button.disabled = true;
        self
    }

    /// Sets simple text content for the button.
    ///
    /// This is a convenience method for basic text. For more complex text
    /// styling, use `text_builder()` instead.
    ///
    /// # Arguments
    /// * `text` - The text content to display
    ///
    /// # Example
    /// ```rust
    /// let button = Button::builder("action")
    ///     .text("Click Me")
    ///     .build();
    /// ```
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    /// Sets the text size for the button text.
    ///
    /// This method configures a TextBuilder for advanced text styling.
    /// If no TextBuilder exists, one will be created with the current text content.
    ///
    /// # Arguments
    /// * `size` - The TextSize to use for the button text
    ///
    /// # Example
    /// ```rust
    /// let button = Button::builder("large-text")
    ///     .text("Big Button")
    ///     .text_size(TextSize::Lg)
    ///     .build();
    /// ```
    pub fn text_size(mut self, size: TextSize) -> Self {
        let text_content = self.text.clone().unwrap_or_default();
        let builder = self
            .text_builder
            .take()
            .unwrap_or_else(|| Text::new(text_content));
        self.text_builder = Some(builder.size(size));
        self
    }

    /// Sets the text weight (boldness) for the button text.
    ///
    /// # Arguments
    /// * `weight` - The TextWeight to use (Light, Regular, Medium, SemiBold, Bold, ExtraBold)
    ///
    /// # Example
    /// ```rust
    /// let bold_button = Button::builder("bold")
    ///     .text("Important Action")
    ///     .text_weight(TextWeight::Bold)
    ///     .build();
    /// ```
    pub fn text_weight(mut self, weight: TextWeight) -> Self {
        let text_content = self.text.clone().unwrap_or_default();
        let builder = self
            .text_builder
            .take()
            .unwrap_or_else(|| Text::new(text_content));
        self.text_builder = Some(builder.weight(weight));
        self
    }

    /// Sets the font family for the button text.
    ///
    /// # Arguments
    /// * `family` - The FontFamily to use (Default, Mono, Serif, Sans)
    ///
    /// # Example
    /// ```rust
    /// let mono_button = Button::builder("code")
    ///     .text("Execute")
    ///     .text_family(FontFamily::Mono)
    ///     .build();
    /// ```
    pub fn text_family(mut self, family: FontFamily) -> Self {
        let text_content = self.text.clone().unwrap_or_default();
        let builder = self
            .text_builder
            .take()
            .unwrap_or_else(|| Text::new(text_content));
        self.text_builder = Some(builder.family(family));
        self
    }

    pub fn text_italic(mut self) -> Self {
        let text_content = self.text.clone().unwrap_or_default();
        let builder = self
            .text_builder
            .take()
            .unwrap_or_else(|| Text::new(text_content));
        self.text_builder = Some(builder.italic());
        self
    }

    pub fn text_align(mut self, align: JustifyText) -> Self {
        let text_content = self.text.clone().unwrap_or_default();
        let builder = self
            .text_builder
            .take()
            .unwrap_or_else(|| Text::new(text_content));
        self.text_builder = Some(builder.align(align));
        self
    }

    pub fn text_center(self) -> Self {
        self.text_align(JustifyText::Center)
    }

    pub fn text_right(self) -> Self {
        self.text_align(JustifyText::Right)
    }

    pub fn text_on_background(mut self, background_color: Color) -> Self {
        let text_content = self.text.clone().unwrap_or_default();
        let builder = self
            .text_builder
            .take()
            .unwrap_or_else(|| Text::new(text_content));
        self.text_builder = Some(builder.on_background(background_color));
        self
    }

    pub fn text_contrast_level(mut self, level: TextContrastLevel) -> Self {
        let text_content = self.text.clone().unwrap_or_default();
        let builder = self
            .text_builder
            .take()
            .unwrap_or_else(|| Text::new(text_content));
        self.text_builder = Some(builder.contrast_level(level));
        self
    }

    pub fn text_high_contrast(mut self) -> Self {
        let text_content = self.text.clone().unwrap_or_default();
        let builder = self
            .text_builder
            .take()
            .unwrap_or_else(|| Text::new(text_content));
        self.text_builder = Some(builder.high_contrast());
        self
    }

    pub fn text_accessible(mut self) -> Self {
        let text_content = self.text.clone().unwrap_or_default();
        let builder = self
            .text_builder
            .take()
            .unwrap_or_else(|| Text::new(text_content));
        self.text_builder = Some(builder.accessible());
        self
    }

    pub fn text_auto_contrast(mut self) -> Self {
        let text_content = self.text.clone().unwrap_or_default();
        let builder = self
            .text_builder
            .take()
            .unwrap_or_else(|| Text::new(text_content));
        self.text_builder = Some(builder.auto_contrast());
        self
    }

    pub fn text_manual_color(mut self) -> Self {
        let text_content = self.text.clone().unwrap_or_default();
        let builder = self
            .text_builder
            .take()
            .unwrap_or_else(|| Text::new(text_content));
        self.text_builder = Some(builder.manual_color());
        self
    }

    pub fn text_color(mut self, color: TextColorEnum) -> Self {
        let text_content = self.text.clone().unwrap_or_default();
        let builder = self
            .text_builder
            .take()
            .unwrap_or_else(|| Text::new(text_content));
        self.text_builder = Some(builder.color(color));
        self
    }

    pub fn child(mut self, entity: Entity) -> Self {
        self.children.push(entity);
        self
    }

    /// Check if explicit text color was set (this affects whether button should manage text color)
    fn has_explicit_text_color(&self) -> bool {
        // If text_builder exists and has explicit color methods called, it should not be managed
        // For now, we'll use a simple heuristic: if any text_color method was called, it's explicit
        // This is a simplified check since we can't access TextBuilder internals
        false // For now, always allow button to manage text color
    }

    /// Get appropriate text size for button size (only if no explicit size set)
    fn get_button_text_size(&self) -> TextSize {
        // If text_builder exists and has explicit size, don't override it
        if self.text_builder.is_some() {
            // Return a default - the actual size will be preserved from TextBuilder
            TextSize::Base
        } else {
            match self.button.size {
                ButtonSize::Small => TextSize::Xs,
                ButtonSize::Default => TextSize::Base,
                ButtonSize::Large => TextSize::X2l,
            }
        }
    }

    /// Get appropriate text weight for button variant
    fn get_button_text_weight(&self) -> TextWeight {
        match self.button.variant {
            ButtonVariant::Solid => TextWeight::Medium,
            ButtonVariant::Soft => TextWeight::Regular,
            ButtonVariant::Outline => TextWeight::Regular,
            ButtonVariant::Ghost => TextWeight::Regular,
        }
    }

    /// Convert button text color to Text component color enum
    fn get_text_color_enum(&self) -> TextColorEnum {
        let calculated_color = self.calculate_text_color();
        TextColorEnum::Custom(calculated_color.0)
    }

    fn calculate_style(&self) -> Node {
        super::styling::calculate_button_style(&self.button)
    }

    fn calculate_background_color(&self) -> BackgroundColor {
        self.button
            .get_styling(super::core::ButtonState::Normal)
            .background_color
    }

    fn calculate_border_color(&self) -> BorderColor {
        self.button
            .get_styling(super::core::ButtonState::Normal)
            .border_color
    }

    fn calculate_text_color(&self) -> TextColor {
        self.button
            .get_styling(super::core::ButtonState::Normal)
            .text_color
    }
}

impl Button {
    /// Creates a new ButtonBuilder for constructing a Button component.
    ///
    /// This is the primary entry point for creating buttons using the builder pattern.
    /// The builder provides a fluent API for configuring all aspects of button appearance,
    /// behavior, and content.
    ///
    /// # Arguments
    /// * `name` - A name for the button component (used for debugging and identification)
    ///
    /// # Returns
    /// A new ButtonBuilder with default settings
    ///
    /// # Example
    /// ```rust
    /// let button = Button::builder("submit-button")
    ///     .text("Submit Form")
    ///     .solid()
    ///     .accent()
    ///     .size_large()
    ///     .build();
    /// ```
    ///
    /// # Builder Pattern Benefits
    ///
    /// - **Type Safety**: Compile-time verification of configuration
    /// - **Discoverability**: IDE autocomplete reveals available options
    /// - **Flexibility**: Mix and match any combination of properties
    /// - **Readability**: Self-documenting code through method names
    /// - **Extensibility**: Easy to add new configuration options
    pub fn builder(name: impl Into<String>) -> ButtonBuilder {
        ButtonBuilder::new(name)
    }
}

impl ButtonBuilder {
    pub fn build(self) -> impl Bundle {
        use super::{
            animations::SpinnerAnimation, interactions::ButtonManagedText,
            styling::calculate_border_radius,
        };
        use crate::components::text::Text;
        use bevy::{ecs::spawn::SpawnWith, prelude::*};

        let node = self.calculate_style();
        let background_color = self.calculate_background_color();
        let border_color = self.calculate_border_color();
        let border_radius = calculate_border_radius(self.button.radius);
        let display_text = self.text.clone().unwrap_or_default();
        let is_loading = self.button.loading;
        let text_size = self.get_button_text_size();
        let text_weight = self.get_button_text_weight();
        let text_color_enum = self.get_text_color_enum();

        // Prepare TextBuilder with automatic contrast optimization if text_builder is used
        let text_builder = if let Some(builder) = self.text_builder.clone() {
            // Apply button background context for intelligent contrast
            let button_bg = self.calculate_background_color().0;
            let contrast_level = if self.button.high_contrast {
                TextContrastLevel::Accessible
            } else {
                TextContrastLevel::High
            };

            Some(
                builder
                    .on_background(button_bg)
                    .contrast_level(contrast_level),
            )
        } else {
            None
        };

        (
            Name::new(format!("{}_Button", self.name)),
            self.button,
            node,
            border_color,
            border_radius,
            background_color,
            bevy_picking::prelude::Pickable::default(),
            Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
                if is_loading {
                    // Spawn rotating spinner image
                    parent.spawn((
                        Name::new("Button Spinner"),
                        Node {
                            width: Val::Px(16.0),
                            height: Val::Px(16.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        SpinnerAnimation::default(),
                    ));
                } else {
                    // Use advanced TextBuilder if available, otherwise fallback to simple text
                    if let Some(builder) = text_builder {
                        parent.spawn((
                            builder.center().build(),
                            ButtonManagedText, // Always add marker for now - will be refined later
                        ));
                    } else {
                        // Fallback text is always managed by button
                        parent.spawn((
                            Text::label(display_text.clone())
                                .color(text_color_enum)
                                .size(text_size)
                                .weight(text_weight)
                                .center()
                                .build(),
                            ButtonManagedText,
                        ));
                    }
                }
            })),
        )
    }
}
