use std::borrow::Cow;

use bevy::{prelude::*, ui::Val::*};

/// A root UI container component that serves as the foundation for UI hierarchies.
///
/// Similar to `document.body` in web development, UIRoot provides a base container
/// that fills the entire screen and can serve as a parent for other UI elements.
/// This is particularly useful for portal systems and global UI positioning.
///
/// # Example
/// ```rust
/// use bevy::prelude::*;
/// use your_crate::utilities::ui_root;
///
/// fn setup_ui(mut commands: Commands) {
///     // Create a main UI root that fills the screen
///     commands.spawn(ui_root("main_ui"));
/// }
/// ```
#[derive(Component, Debug, Clone)]
pub struct UIRoot {
    pub name: String,
}

impl UIRoot {
    /// Creates a new UIRoot with the given name.
    ///
    /// # Arguments
    /// * `name` - A unique identifier for this UI root
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

/// Creates a UI root bundle that fills the window and centers its content.
///
/// This function creates a complete UI root setup with:
/// - Absolute positioning that fills 100% of the window
/// - Centered content alignment (both horizontal and vertical)
/// - Column flex direction with 20px row gap
/// - Pickable::IGNORE to prevent blocking other UI interactions
///
/// # Arguments
/// * `name` - A name for the UI root, used for identification and portal targeting
///
/// # Returns
/// A bundle containing all necessary components for a UI root
///
/// # Example
/// ```rust
/// use bevy::prelude::*;
/// use your_crate::utilities::ui_root;
///
/// fn setup_main_ui(mut commands: Commands) {
///     // Create the main UI container
///     commands.spawn(ui_root("main_ui"));
///     
///     // Create a modal overlay container
///     commands.spawn(ui_root("modal_overlay"));
/// }
/// ```
pub fn ui_root(name: impl Into<Cow<'static, str>>) -> impl Bundle {
    let name_cow = name.into();
    let name_str = name_cow.clone();
    info!("Creating UI root with name: {}", name_str);
    (
        Name::new(name_cow),
        UIRoot::new(name_str.into_owned()),
        Node {
            position_type: PositionType::Absolute,
            width: Percent(100.0),
            height: Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Px(20.0),
            ..default()
        },
        BackgroundColor(Color::WHITE),
        Pickable::IGNORE,
    )
}

/// Finds a UI root entity by name or returns the first available one.
///
/// This helper function is used internally by the portal system to locate
/// target containers for portal content.
///
/// # Arguments
/// * `query` - A query for entities with UIRoot components
/// * `name` - Optional name to search for; if None, returns the first UIRoot found
///
/// # Returns
/// The entity of the matching UIRoot, or None if no UIRoot is found
///
/// # Note
/// Currently returns the first UIRoot when searching by name due to query limitations.
/// This will be improved in future versions to properly match by name.
pub fn find_ui_root(query: &Query<Entity, With<UIRoot>>, name: Option<&str>) -> Option<Entity> {
    if let Some(target_name) = name {
        query.iter().find(|&entity| {
            // Would need access to UIRoot component to check name
            // For now, return first match
            true
        })
    } else {
        query.iter().next()
    }
}
