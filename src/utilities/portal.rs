use bevy::prelude::*;

use crate::utilities::ui_root::UIRoot;

/// A portal component that renders its children in a different part of the UI hierarchy.
///
/// Similar to React portals, this allows you to render UI elements outside of their
/// normal parent-child relationship, which is useful for modals, tooltips, dropdowns,
/// and overlays that need to appear above other content.
///
/// The portal automatically moves its children to a target container (UIRoot) when spawned.
/// By default, it targets the first available UIRoot, but you can specify a named container.
///
/// # Example
/// ```rust
/// use bevy::prelude::*;
/// use your_crate::utilities::{Portal, ui_root};
///
/// fn setup_modal(mut commands: Commands) {
///     // Create a UI root for modals
///     commands.spawn(ui_root("modal_layer"));
///     
///     // Create a portal that renders to the modal layer
///     commands.spawn((
///         Portal::new()
///             .container("modal_layer")
///             .build(),
///         // Add your modal content as children
///     )).with_children(|parent| {
///         parent.spawn((
///             Node {
///                 width: Val::Px(300.0),
///                 height: Val::Px(200.0),
///                 background_color: Color::WHITE.into(),
///                 ..default()
///             },
///             // Modal content
///         ));
///     });
/// }
/// ```
#[derive(Component, Debug, Clone)]
pub struct Portal {
    pub container: Option<String>,
}

impl Portal {
    /// Creates a new PortalBuilder for configuring the portal.
    ///
    /// # Returns
    /// A PortalBuilder that can be used to configure the portal before building
    ///
    /// # Example
    /// ```rust
    /// use your_crate::utilities::Portal;
    ///
    /// let portal_bundle = Portal::new()
    ///     .container("overlay_layer")
    ///     .build();
    /// ```
    pub fn new() -> PortalBuilder {
        PortalBuilder::default()
    }
}

/// Builder for configuring Portal components using the builder pattern.
///
/// This allows for fluent configuration of portal behavior before spawning.
#[derive(Default)]
pub struct PortalBuilder {
    container: Option<String>,
}

impl PortalBuilder {
    /// Specifies the target container for the portal content.
    ///
    /// The container should be the name of a UIRoot entity. If not specified,
    /// the portal will target the first available UIRoot.
    ///
    /// # Arguments
    /// * `container` - The name of the target UIRoot container
    ///
    /// # Returns
    /// Self for method chaining
    ///
    /// # Example
    /// ```rust
    /// use your_crate::utilities::Portal;
    ///
    /// let modal_portal = Portal::new()
    ///     .container("modal_overlay")
    ///     .build();
    /// ```
    pub fn container(mut self, container: impl Into<String>) -> Self {
        self.container = Some(container.into());
        self
    }

    /// Builds the portal bundle with the configured settings.
    ///
    /// # Returns
    /// A bundle containing the Portal component and necessary UI components
    pub fn build(self) -> impl Bundle {
        (
            Portal {
                container: self.container,
            },
            Node::default(),
        )
    }
}

/// Marker component for entities that have been moved by a portal.
///
/// This component tracks which portal moved the entity, allowing for proper
/// cleanup and management of portal content.
#[derive(Component, Debug)]
pub struct PortalContent {
    pub portal_entity: Entity,
}

/// System that handles the portal logic, moving children to target containers.
///
/// This system runs when Portal components change and automatically:
/// 1. Finds the target container (UIRoot) based on the portal configuration
/// 2. Moves all children of the portal entity to the target container
/// 3. Marks moved entities with PortalContent for tracking
///
/// The system ensures that portal content appears in the correct UI layer
/// while maintaining the logical parent-child relationships for updates.
pub fn portal_system(
    mut commands: Commands,
    portal_query: Query<(Entity, &Portal, &Children), Changed<Portal>>,
    ui_root_query: Query<Entity, With<UIRoot>>,
    name_query: Query<&Name>,
    mut portal_content_query: Query<(Entity, &mut PortalContent)>,
) {
    for (portal_entity, portal, children) in portal_query.iter() {
        let target_entity = if let Some(container_name) = &portal.container {
            // Try to find the named container first
            ui_root_query
                .iter()
                .find(|&entity| {
                    name_query
                        .get(entity)
                        .map(|name| name.as_str() == container_name)
                        .unwrap_or(false)
                })
                // Fall back to the first available UIRoot
                .or_else(|| ui_root_query.iter().next())
        } else {
            // Use the first available UIRoot
            ui_root_query.iter().next()
        };

        if let Some(target) = target_entity {
            for &child in children.iter() {
                // Only move children that haven't been processed yet
                if portal_content_query.get(child).is_err() {
                    commands.entity(child).insert(PortalContent {
                        portal_entity,
                    });
                    
                    // Move child to the target container
                    commands.entity(target).add_child(child);
                    commands.entity(portal_entity).remove_children(&[child]);
                }
            }
        }
    }
}

/// Plugin that adds portal functionality to the Bevy app.
///
/// Add this plugin to enable portal rendering in your application.
///
/// # Example
/// ```rust
/// use bevy::prelude::*;
/// use your_crate::utilities::PortalPlugin;
///
/// fn main() {
///     App::new()
///         .add_plugins(DefaultPlugins)
///         .add_plugins(PortalPlugin)
///         .run();
/// }
/// ```
pub struct PortalPlugin;

impl Plugin for PortalPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, portal_system);
    }
}