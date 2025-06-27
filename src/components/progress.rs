use crate::{
    theme::{
        color::{accent_palette, UiColorPalette},
    },
};
use bevy::{ecs::spawn::SpawnWith, prelude::*};

#[derive(Component, Debug, Clone)]
pub struct ProgressComponent {
    pub value: f32,
    pub max: f32,
    pub size: ProgressSize,
    pub color: UiColorPalette,
    pub indeterminate: bool,
    pub label: Option<String>,
}

impl Default for ProgressComponent {
    fn default() -> Self {
        Self {
            value: 0.0,
            max: 1.0,
            size: ProgressSize::Size2,
            color: accent_palette(),
            indeterminate: false,
            label: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProgressSize {
    Size1, // Small - 4px height
    #[default]
    Size2, // Medium - 6px height
    Size3, // Large - 8px height
}

#[derive(Component, Debug)]
pub struct ProgressAnimation {
    pub speed: f32,
    pub offset: f32,
}

impl Default for ProgressAnimation {
    fn default() -> Self {
        Self {
            speed: 2.0, // Cycles per second for indeterminate animation
            offset: 0.0,
        }
    }
}

pub struct ProgressBuilder {
    name: String,
    progress: ProgressComponent,
}

impl ProgressBuilder {
    pub fn new() -> Self {
        Self {
            name: "Progress".to_string(),
            progress: ProgressComponent::default(),
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    pub fn value(mut self, value: f32) -> Self {
        self.progress.value = value;
        self.progress.indeterminate = false;
        self
    }

    pub fn max(mut self, max: f32) -> Self {
        self.progress.max = max.max(0.001); // Prevent division by zero
        self
    }

    pub fn percentage(mut self, percentage: f32) -> Self {
        self.progress.max = 100.0;
        self.progress.value = percentage.clamp(0.0, 100.0);
        self.progress.indeterminate = false;
        self
    }

    pub fn progress(mut self, current: f32, total: f32) -> Self {
        self.progress.max = total.max(0.001);
        self.progress.value = current;
        self.progress.indeterminate = false;
        self
    }

    pub fn indeterminate(mut self) -> Self {
        self.progress.indeterminate = true;
        self
    }

    pub fn determinate(mut self) -> Self {
        self.progress.indeterminate = false;
        self
    }

    pub fn size(mut self, size: ProgressSize) -> Self {
        self.progress.size = size;
        self
    }

    pub fn size1(mut self) -> Self {
        self.progress.size = ProgressSize::Size1;
        self
    }

    pub fn size2(mut self) -> Self {
        self.progress.size = ProgressSize::Size2;
        self
    }

    pub fn size3(mut self) -> Self {
        self.progress.size = ProgressSize::Size3;
        self
    }

    pub fn color(mut self, color: UiColorPalette) -> Self {
        self.progress.color = color;
        self
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.progress.label = Some(label.into());
        self
    }

    pub fn build(self) -> impl Bundle {
        // Clamp the final values
        let mut final_progress = self.progress.clone();
        final_progress.value = final_progress.value.clamp(0.0, final_progress.max);
        
        let track_node = self.calculate_track_style();
        let track_background = self.calculate_track_background();
        let track_border_radius = self.calculate_track_border_radius();

        let progress_percentage = if final_progress.indeterminate {
            0.0 // Will be animated
        } else {
            final_progress.value / final_progress.max
        };

        let bundle = (
            Name::new(self.name.clone()),
            final_progress.clone(),
            track_node,
            track_background,
            track_border_radius,
            Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
                // Progress indicator (filled portion)
                let indicator_node = self.calculate_indicator_style(progress_percentage);
                let indicator_background = self.calculate_indicator_background();

                let indicator_bundle = (
                    Name::new(format!("{}_Indicator", self.name)),
                    ProgressIndicator,
                    indicator_node,
                    indicator_background,
                );

                if self.progress.indeterminate {
                    parent.spawn((
                        indicator_bundle,
                        ProgressAnimation::default(),
                    ));
                } else {
                    parent.spawn(indicator_bundle);
                }
            })),
        );

        bundle
    }
}

#[derive(Component, Debug)]
pub struct ProgressIndicator;

impl ProgressBuilder {
    fn calculate_track_style(&self) -> Node {
        let height = match self.progress.size {
            ProgressSize::Size1 => Val::Px(4.0),
            ProgressSize::Size2 => Val::Px(6.0), 
            ProgressSize::Size3 => Val::Px(8.0),
        };

        Node {
            width: Val::Percent(100.0),
            height,
            overflow: Overflow::clip(),
            position_type: PositionType::Relative,
            ..default()
        }
    }

    fn calculate_track_background(&self) -> BackgroundColor {
        BackgroundColor(self.progress.color.bg)
    }

    fn calculate_track_border_radius(&self) -> BorderRadius {
        let radius = match self.progress.size {
            ProgressSize::Size1 => Val::Px(2.0),
            ProgressSize::Size2 => Val::Px(3.0), 
            ProgressSize::Size3 => Val::Px(4.0),
        };
        BorderRadius::all(radius)
    }

    fn calculate_indicator_style(&self, percentage: f32) -> Node {
        let width = if self.progress.indeterminate {
            Val::Percent(30.0) // Fixed width for animation
        } else {
            Val::Percent(percentage * 100.0)
        };

        Node {
            width,
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            left: if self.progress.indeterminate {
                Val::Percent(0.0) // Will be animated
            } else {
                Val::Px(0.0)
            },
            ..default()
        }
    }

    fn calculate_indicator_background(&self) -> BackgroundColor {
        BackgroundColor(self.progress.color.solid)
    }
}

impl Default for ProgressBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// Note: ProgressBuilder already has a direct build() method that returns impl Bundle

// Animation system for indeterminate progress
pub fn animate_indeterminate_progress(
    time: Res<Time>,
    mut query: Query<(&mut Node, &mut ProgressAnimation), With<ProgressIndicator>>,
) {
    for (mut node, mut animation) in query.iter_mut() {
        animation.offset += animation.speed * time.delta_secs();
        
        // Create a smooth back-and-forth animation
        let cycle = (animation.offset * std::f32::consts::PI).sin();
        let position = (cycle + 1.0) / 2.0; // Normalize to 0.0-1.0
        
        // Move the indicator across the track
        let max_position = 70.0; // 100% - 30% width = 70% max left position
        node.left = Val::Percent(position * max_position);
    }
}

// System to setup progress components
pub fn setup_progress_components(
    mut indicator_query: Query<&mut Node, (With<ProgressIndicator>, Without<ProgressAnimation>)>,
    progress_query: Query<(Entity, &ProgressComponent), Added<ProgressComponent>>,
    children_query: Query<&Children>,
) {
    for (progress_entity, progress) in &progress_query {
        if !progress.indeterminate {
            let percentage = progress.value / progress.max;
            
            // Find the indicator child of this specific progress entity
            if let Ok(children) = children_query.get(progress_entity) {
                for child_entity in children.iter() {
                    // Check if this child is a progress indicator
                    if let Ok(mut indicator_node) = indicator_query.get_mut(child_entity) {
                        indicator_node.width = Val::Percent(percentage * 100.0);
                    }
                }
            }
        }
    }
}

// System to update progress values dynamically
pub fn update_progress_values(
    mut indicator_query: Query<&mut Node, (With<ProgressIndicator>, Without<ProgressAnimation>)>,
    progress_query: Query<(Entity, &ProgressComponent), Changed<ProgressComponent>>,
    children_query: Query<&Children>,
) {
    for (progress_entity, progress) in &progress_query {
        if !progress.indeterminate {
            let percentage = progress.value / progress.max;
            
            // Find the indicator child of this specific progress entity
            if let Ok(children) = children_query.get(progress_entity) {
                for child_entity in children.iter() {
                    // Check if this child is a progress indicator
                    if let Ok(mut indicator_node) = indicator_query.get_mut(child_entity) {
                        indicator_node.width = Val::Percent(percentage * 100.0);
                    }
                }
            }
        }
    }
}