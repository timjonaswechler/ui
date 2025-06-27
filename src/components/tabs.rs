use bevy::prelude::*;

use crate::{
    components::{
        button::{ButtonBuilder, ButtonVariant, ButtonSize, ButtonClickEvent},
    },
    theme::color::accent_palette,
    utilities::ComponentBuilder,
};

// A marker component for the root of a tabs system.
#[derive(Component, Default, Debug)]
pub struct TabsRoot;

// A marker component for the list of tab triggers.
#[derive(Component, Default, Debug)]
pub struct TabsList;

// A component for a tab trigger button.
#[derive(Component, Debug)]
pub struct TabTrigger {
    // The entity of the content panel this trigger controls.
    pub content: Entity,
}

// A marker component for a tab content panel.
#[derive(Component, Default, Debug)]
pub struct TabContent;

// A marker component for the currently active trigger and content.
#[derive(Component, Default, Debug)]
pub struct ActiveTab;

#[derive(Default)]
pub struct TabTriggerBuilder {
    label: String,
}

impl TabTriggerBuilder {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
        }
    }

    pub fn build(self, commands: &mut Commands) -> Entity {
        let button = ButtonBuilder::new(&self.label)
            .variant(ButtonVariant::Ghost);

        commands.spawn(button.build()).id()
    }
}

#[derive(Default)]
pub struct TabsBuilder {
    triggers: Vec<String>,
    contents: Vec<Entity>,
}

impl TabsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn triggers(mut self, triggers: Vec<&str>) -> Self {
        self.triggers = triggers.into_iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn contents(mut self, contents: Vec<Entity>) -> Self {
        self.contents = contents;
        self
    }

    pub fn build(self, commands: &mut Commands) -> Entity {
        let content_entities = self.contents.clone();

        // Create wrapper containers for each content panel to ensure consistent positioning
        let wrapped_content_entities: Vec<Entity> = content_entities
            .iter()
            .enumerate()
            .map(|(i, &content_entity)| {
                let wrapper = commands
                    .spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Px(0.0),
                            top: Val::Px(0.0),
                            width: Val::Percent(100.0),
                            height: Val::Auto,
                            ..default()
                        },
                        TabContent,
                    ))
                    .add_child(content_entity)
                    .id();

                // Apply visibility and ActiveTab to the wrapper instead of the content
                if i == 0 {
                    commands.entity(wrapper).insert(ActiveTab);
                } else {
                    commands.entity(wrapper).insert(Visibility::Hidden);
                }

                wrapper
            })
            .collect();

        let trigger_entities: Vec<Entity> = self
            .triggers
            .iter()
            .enumerate()
            .map(|(i, label)| {
                let wrapper_entity = wrapped_content_entities[i];
                let trigger_button = ButtonBuilder::new(label)
                    .variant(ButtonVariant::Soft)
                    .size(ButtonSize::Default)
                    .text(label);

                let trigger = commands
                    .spawn((
                        trigger_button.build(),
                        TabTrigger {
                            content: wrapper_entity,
                        },
                    ))
                    .id();

                if i == 0 {
                    commands.entity(trigger).insert(ActiveTab);
                }

                trigger
            })
            .collect();

        let tabs_list = commands
            .spawn((
                TabsList,
                Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(8.0),
                    align_items: AlignItems::Center,
                    min_height: Val::Px(40.0),
                    padding: UiRect::all(Val::Px(4.0)),
                    ..default()
                },
            ))
            .add_children(&trigger_entities)
            .id();

        // Create a content container that will hold all wrapped content panels
        let content_container = commands
            .spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Auto,
                    position_type: PositionType::Relative,
                    min_height: Val::Px(200.0), // Ensure minimum height for consistent layout
                    ..default()
                },
            ))
            .add_children(&wrapped_content_entities)
            .id();

        let root = commands
            .spawn((
                TabsRoot,
                Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(20.0),
                    align_items: AlignItems::Start,
                    ..default()
                },
            ))
            .add_child(tabs_list)
            .add_child(content_container)
            .id();

        root
    }
}

pub fn handle_trigger_clicks(
    mut commands: Commands,
    mut button_events: EventReader<ButtonClickEvent>,
    trigger_query: Query<&TabTrigger>,
    mut tabs_query: Query<&mut Visibility, With<TabContent>>,
    active_query: Query<(Entity, &TabTrigger), With<ActiveTab>>,
) {
    for event in button_events.read() {
        // Check if this button is a tab trigger
        if let Ok(trigger) = trigger_query.get(event.button_entity) {
            info!("Tab trigger clicked: {:?}", event.button_entity);
            
            // Deactivate all old tabs
            for (active_trigger_entity, active_trigger) in &active_query {
                commands.entity(active_trigger_entity).remove::<ActiveTab>();
                commands.entity(active_trigger.content).remove::<ActiveTab>();
                if let Ok(mut visibility) = tabs_query.get_mut(active_trigger.content) {
                    *visibility = Visibility::Hidden;
                }
            }

            // Activate the new tab
            commands.entity(event.button_entity).insert(ActiveTab);
            commands.entity(trigger.content).insert(ActiveTab);
            if let Ok(mut visibility) = tabs_query.get_mut(trigger.content) {
                *visibility = Visibility::Inherited;
            }
            
            info!("Tab switched to content: {:?}", trigger.content);
        }
    }
}

pub fn style_active_triggers(
    mut triggers_query: Query<(&mut BackgroundColor, &mut BorderColor, Option<&ActiveTab>), With<TabTrigger>>,
) {
    for (mut bg_color, mut border_color, is_active) in &mut triggers_query {
        if is_active.is_some() {
            *bg_color = BackgroundColor(accent_palette().solid);
            *border_color = BorderColor(accent_palette().solid);
        } else {
            *bg_color = BackgroundColor(bevy::color::Color::NONE);
            *border_color = BorderColor(accent_palette().border);
        }
    }
}

pub struct TabsPlugin;

impl Plugin for TabsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_trigger_clicks, style_active_triggers));
    }
}