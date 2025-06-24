use crate::{
    theme::color::{accent_palette, theme, UiColorPalette},
    utilities::ComponentBuilder,
};
use bevy::{ecs::spawn::SpawnWith, prelude::*};
use bevy_picking::prelude::{Click, Drag, DragEnd, DragStart, Pickable, Pointer};

#[derive(Event, Debug, Clone)]
pub struct SliderValueChangeEvent {
    pub slider_entity: Entity,
    pub value: f32,
    pub thumb_index: usize,
}

#[derive(Event, Debug, Clone)]
pub struct SliderValueCommitEvent {
    pub slider_entity: Entity,
    pub value: f32,
    pub thumb_index: usize,
}

#[derive(Component, Debug, Clone)]
pub struct SliderComponent {
    pub value: f32,
    pub min: f32,
    pub max: f32,
    pub step: f32,
    pub size: SliderSize,
    pub color: UiColorPalette,
    pub orientation: SliderOrientation,
    pub disabled: bool,
    pub name: Option<String>,
}

impl Default for SliderComponent {
    fn default() -> Self {
        Self {
            value: 0.0,
            min: 0.0,
            max: 100.0,
            step: 1.0,
            size: SliderSize::Size2,
            color: accent_palette(),
            orientation: SliderOrientation::Horizontal,
            disabled: false,
            name: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SliderSize {
    Size1, // Small - 8px track height, 16px thumb
    #[default]
    Size2, // Medium - 12px track height, 20px thumb
    Size3, // Large - 16px track height, 24px thumb
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SliderOrientation {
    #[default]
    Horizontal,
    Vertical,
}

#[derive(Component, Debug)]
pub struct SliderTrack;

#[derive(Component, Debug)]
pub struct SliderRange;

#[derive(Component, Debug)]
pub struct SliderThumb {
    pub index: usize,
    pub is_dragging: bool,
}

impl SliderThumb {
    pub fn new(index: usize) -> Self {
        Self {
            index,
            is_dragging: false,
        }
    }
}

#[derive(Component, Debug)]
pub struct TrackRef(pub Entity);

#[derive(Component, Debug)]
pub struct ThumbRef(pub Entity);

pub struct SliderBuilder {
    name: String,
    slider: SliderComponent,
}

impl SliderBuilder {
    pub fn new() -> Self {
        Self {
            name: "Slider".to_string(),
            slider: SliderComponent::default(),
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self.slider.name = Some(self.name.clone());
        self
    }

    pub fn value(mut self, value: f32) -> Self {
        self.slider.value = value.clamp(self.slider.min, self.slider.max);
        self
    }

    pub fn min(mut self, min: f32) -> Self {
        self.slider.min = min;
        self.slider.value = self.slider.value.clamp(min, self.slider.max);
        self
    }

    pub fn max(mut self, max: f32) -> Self {
        self.slider.max = max;
        self.slider.value = self.slider.value.clamp(self.slider.min, max);
        self
    }

    pub fn step(mut self, step: f32) -> Self {
        self.slider.step = step.max(0.001);
        self
    }

    pub fn range(mut self, min: f32, max: f32) -> Self {
        self.slider.min = min;
        self.slider.max = max;
        self.slider.value = self.slider.value.clamp(min, max);
        self
    }

    pub fn size(mut self, size: SliderSize) -> Self {
        self.slider.size = size;
        self
    }

    pub fn size1(mut self) -> Self {
        self.slider.size = SliderSize::Size1;
        self
    }

    pub fn size2(mut self) -> Self {
        self.slider.size = SliderSize::Size2;
        self
    }

    pub fn size3(mut self) -> Self {
        self.slider.size = SliderSize::Size3;
        self
    }

    pub fn color(mut self, color: UiColorPalette) -> Self {
        self.slider.color = color;
        self
    }

    pub fn orientation(mut self, orientation: SliderOrientation) -> Self {
        self.slider.orientation = orientation;
        self
    }

    pub fn horizontal(mut self) -> Self {
        self.slider.orientation = SliderOrientation::Horizontal;
        self
    }

    pub fn vertical(mut self) -> Self {
        self.slider.orientation = SliderOrientation::Vertical;
        self
    }

    pub fn disabled(mut self) -> Self {
        self.slider.disabled = true;
        self
    }

    pub fn build(self) -> impl Bundle {
        let mut final_slider = self.slider.clone();
        final_slider.value = final_slider.value.clamp(final_slider.min, final_slider.max);

        let root_node = self.calculate_root_style();
        let root_background = BackgroundColor(Color::NONE);

        let percentage =
            (final_slider.value - final_slider.min) / (final_slider.max - final_slider.min);

        let name_clone = self.name.clone();
        let track_name = format!("{}_Track", &name_clone);
        let range_name = format!("{}_Range", &name_clone);
        let thumb_name = format!("{}_Thumb", &name_clone);

        let track_node = self.calculate_track_style();
        let track_background = self.calculate_track_background();
        let track_border_radius = self.calculate_track_border_radius();
        let range_node = self.calculate_range_style(percentage);
        let range_background = self.calculate_range_background();
        let range_border_radius = self.calculate_track_border_radius();
        let thumb_node = self.calculate_thumb_style(percentage);
        let thumb_background = self.calculate_thumb_background();
        let thumb_border_radius = self.calculate_thumb_border_radius();

        let bundle = (
            Name::new(name_clone),
            final_slider.clone(),
            root_node,
            root_background,
            BorderColor(theme().red.solid),
            Pickable::default(),
            Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
                // Track container
                let slider_entity = parent.target_entity();
                parent.spawn((
                    Name::new(track_name),
                    SliderTrack,
                    TrackRef(slider_entity),
                    track_node,
                    track_background,
                    track_border_radius,
                    Pickable::default(),
                    Children::spawn(SpawnWith(move |track_parent: &mut ChildSpawner| {
                        // Range (filled portion)
                        track_parent.spawn((
                            Name::new(range_name),
                            SliderRange,
                            range_node,
                            range_background,
                            range_border_radius,
                        ));
                    })),
                ));

                // Thumb
                let slider_entity = parent.target_entity();
                parent.spawn((
                    Name::new(thumb_name),
                    SliderThumb::new(0),
                    ThumbRef(slider_entity),
                    thumb_node,
                    thumb_background,
                    thumb_border_radius,
                    Pickable::default(),
                ));
            })),
        );

        bundle
    }
}

impl SliderBuilder {
    fn calculate_root_style(&self) -> Node {
        match self.slider.orientation {
            SliderOrientation::Horizontal => Node {
                width: Val::Percent(100.0),
                height: self.get_root_height(),
                position_type: PositionType::Relative,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            SliderOrientation::Vertical => Node {
                width: self.get_root_height(),
                height: Val::Percent(100.0),
                position_type: PositionType::Relative,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        }
    }

    fn calculate_track_style(&self) -> Node {
        let (track_width, track_height) = self.get_track_dimensions();

        Node {
            width: track_width,
            height: track_height,
            position_type: PositionType::Relative,
            overflow: Overflow::clip(),
            ..default()
        }
    }

    fn calculate_range_style(&self, percentage: f32) -> Node {
        match self.slider.orientation {
            SliderOrientation::Horizontal => Node {
                width: Val::Percent(percentage * 100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                ..default()
            },
            SliderOrientation::Vertical => Node {
                width: Val::Percent(100.0),
                height: Val::Percent(percentage * 100.0),
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                bottom: Val::Px(0.0),
                ..default()
            },
        }
    }

    fn calculate_thumb_style(&self, percentage: f32) -> Node {
        let thumb_size = self.get_thumb_size();
        let thumb_size_px = match self.slider.size {
            SliderSize::Size1 => 16.0,
            SliderSize::Size2 => 20.0,
            SliderSize::Size3 => 24.0,
        };
        let half_thumb = thumb_size_px / 2.0;

        match self.slider.orientation {
            SliderOrientation::Horizontal => {
                // Position thumb precisely on the track
                let thumb_center_offset = percentage * 100.0;
                
                Node {
                    width: thumb_size,
                    height: thumb_size,
                    position_type: PositionType::Absolute,
                    // Position horizontally at the percentage point
                    left: Val::Percent(thumb_center_offset),
                    // Center vertically within the root container
                    top: Val::Percent(50.0),
                    // Use negative margin to center the thumb precisely
                    margin: UiRect {
                        left: Val::Px(-half_thumb),  // Half of thumb width
                        top: Val::Px(-half_thumb),   // Half of thumb height
                        ..default()
                    },
                    ..default()
                }
            }
            SliderOrientation::Vertical => {
                let thumb_center_offset = percentage * 100.0;

                Node {
                    width: thumb_size,
                    height: thumb_size,
                    position_type: PositionType::Absolute,
                    left: Val::Percent(50.0),
                    bottom: Val::Percent(thumb_center_offset),
                    margin: UiRect {
                        left: Val::Px(-half_thumb),
                        bottom: Val::Px(-half_thumb),
                        ..default()
                    },
                    ..default()
                }
            }
        }
    }

    fn get_root_height(&self) -> Val {
        let thumb_size = match self.slider.size {
            SliderSize::Size1 => 16.0,
            SliderSize::Size2 => 20.0,
            SliderSize::Size3 => 24.0,
        };
        Val::Px(thumb_size + 20.0) // Add more padding for proper spacing
    }

    fn get_track_dimensions(&self) -> (Val, Val) {
        let track_thickness = match self.slider.size {
            SliderSize::Size1 => 8.0,
            SliderSize::Size2 => 12.0,
            SliderSize::Size3 => 16.0,
        };

        match self.slider.orientation {
            SliderOrientation::Horizontal => (Val::Percent(100.0), Val::Px(track_thickness)),
            SliderOrientation::Vertical => (Val::Px(track_thickness), Val::Percent(100.0)),
        }
    }

    fn get_thumb_size(&self) -> Val {
        let size = match self.slider.size {
            SliderSize::Size1 => 16.0,
            SliderSize::Size2 => 20.0,
            SliderSize::Size3 => 24.0,
        };
        Val::Px(size)
    }

    fn get_thumb_offset(&self, percentage: f32) -> Val {
        // Simple approach: position the thumb so its left edge is at the percentage point
        // minus half the thumb width to center it on that point
        // But since we can't easily convert pixel thumb size to percentage of unknown container width,
        // let's use a simpler approach that works better in practice

        let thumb_size = match self.slider.size {
            SliderSize::Size1 => 16.0,
            SliderSize::Size2 => 20.0,
            SliderSize::Size3 => 24.0,
        };

        // Position thumb with a simple offset that accounts for thumb size
        // This is an approximation that works better in practice
        let adjusted_percentage = percentage * 100.0;

        // Adjust for thumb size by reducing the effective range slightly
        let size_adjustment = (thumb_size / 400.0) * 100.0; // Rough approximation
        let final_percentage = (adjusted_percentage * (100.0 - size_adjustment) / 100.0);

        Val::Percent(final_percentage)
    }

    fn calculate_track_background(&self) -> BackgroundColor {
        BackgroundColor(self.slider.color.bg)
    }

    fn calculate_range_background(&self) -> BackgroundColor {
        BackgroundColor(self.slider.color.solid)
    }

    fn calculate_thumb_background(&self) -> BackgroundColor {
        BackgroundColor(self.slider.color.solid)
    }

    fn calculate_track_border_radius(&self) -> BorderRadius {
        let radius = match self.slider.size {
            SliderSize::Size1 => Val::Px(4.0),
            SliderSize::Size2 => Val::Px(6.0),
            SliderSize::Size3 => Val::Px(8.0),
        };
        BorderRadius::all(radius)
    }

    fn calculate_thumb_border_radius(&self) -> BorderRadius {
        let radius = match self.slider.size {
            SliderSize::Size1 => Val::Px(8.0),
            SliderSize::Size2 => Val::Px(10.0),
            SliderSize::Size3 => Val::Px(12.0),
        };
        BorderRadius::all(radius)
    }
}

impl Default for SliderBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ComponentBuilder for SliderBuilder {
    type Output = (Name, SliderComponent, Node, BackgroundColor);

    fn build(self) -> Self::Output {
        panic!("Use the build() method directly on SliderBuilder")
    }
}

#[derive(Debug, Clone)]
pub struct Rect {
    pub min: Vec2,
    pub max: Vec2,
}

impl Rect {
    pub fn width(&self) -> f32 {
        self.max.x - self.min.x
    }
    pub fn height(&self) -> f32 {
        self.max.y - self.min.y
    }
    pub fn contains(&self, point: Vec2) -> bool {
        point.x >= self.min.x
            && point.x <= self.max.x
            && point.y >= self.min.y
            && point.y <= self.max.y
    }
}

fn calculate_track_rect(transform: &GlobalTransform, node: &Node) -> Rect {
    let position = transform.translation().truncate();

    let width = match node.width {
        Val::Px(w) => w,
        Val::Percent(p) => p * 200.0, // Fallback
        _ => 200.0,
    };

    let height = match node.height {
        Val::Px(h) => h,
        Val::Percent(p) => p * 20.0, // Fallback
        _ => 20.0,
    };

    Rect {
        min: position - Vec2::new(width / 2.0, height / 2.0),
        max: position + Vec2::new(width / 2.0, height / 2.0),
    }
}

fn calculate_slider_value_from_computed_size(
    mouse_position: Vec2,
    track_transform: &GlobalTransform,
    computed_node: &ComputedNode,
    slider: &SliderComponent,
) -> f32 {
    // Get computed track dimensions
    let track_width = computed_node.size().x;
    let track_height = computed_node.size().y;

    // Get track center position in world coordinates
    let track_center = track_transform.translation().truncate();

    // Calculate relative position from track start
    let percentage = match slider.orientation {
        SliderOrientation::Horizontal => {
            let track_left = track_center.x - track_width / 2.0;
            let relative_x = mouse_position.x - track_left;
            (relative_x / track_width).clamp(0.0, 1.0)
        }
        SliderOrientation::Vertical => {
            let track_bottom = track_center.y - track_height / 2.0;
            let relative_y = mouse_position.y - track_bottom;
            (relative_y / track_height).clamp(0.0, 1.0)
        }
    };

    slider.min + percentage * (slider.max - slider.min)
}

fn calculate_slider_value_from_position(
    mouse_position: Vec2,
    track_transform: &GlobalTransform,
    track_node: &Node,
    slider: &SliderComponent,
) -> f32 {
    // Get track dimensions
    let track_width = match track_node.width {
        Val::Px(w) => w,
        Val::Percent(_) => 200.0, // Default fallback
        _ => 200.0,
    };

    let track_height = match track_node.height {
        Val::Px(h) => h,
        Val::Percent(_) => 20.0, // Default fallback
        _ => 20.0,
    };

    // Get track center position in world coordinates
    let track_center = track_transform.translation().truncate();

    // Calculate relative position from track start
    let percentage = match slider.orientation {
        SliderOrientation::Horizontal => {
            let track_left = track_center.x - track_width / 2.0;
            let relative_x = mouse_position.x - track_left;
            (relative_x / track_width).clamp(0.0, 1.0)
        }
        SliderOrientation::Vertical => {
            let track_bottom = track_center.y - track_height / 2.0;
            let relative_y = mouse_position.y - track_bottom;
            (relative_y / track_height).clamp(0.0, 1.0)
        }
    };

    slider.min + percentage * (slider.max - slider.min)
}

fn update_thumb_for_slider(
    slider_entity: Entity,
    slider: &SliderComponent,
    thumbs: &mut Query<(&ThumbRef, &mut Node), With<SliderThumb>>,
) {
    let percentage = (slider.value - slider.min) / (slider.max - slider.min);

    for (thumb_ref, mut thumb_node) in thumbs.iter_mut() {
        if thumb_ref.0 == slider_entity {
            update_thumb_position(&mut thumb_node, percentage, slider);
            break;
        }
    }
}

fn send_slider_events(
    slider_entity: Entity,
    value: f32,
    events: &mut EventWriter<SliderValueChangeEvent>,
    commit_events: &mut EventWriter<SliderValueCommitEvent>,
) {
    events.write(SliderValueChangeEvent {
        slider_entity,
        value,
        thumb_index: 0,
    });

    commit_events.write(SliderValueCommitEvent {
        slider_entity,
        value,
        thumb_index: 0,
    });
}

// System to handle thumb dragging
pub fn handle_slider_drag(
    mut sliders: Query<&mut SliderComponent>,
    mut thumbs: Query<(&mut SliderThumb, &ThumbRef), With<SliderThumb>>,
    _thumb_transforms: Query<&GlobalTransform, With<SliderThumb>>,
    tracks: Query<(&TrackRef, &Node, &GlobalTransform, &ComputedNode), With<SliderTrack>>,
    mut events: EventWriter<SliderValueChangeEvent>,
    mut commit_events: EventWriter<SliderValueCommitEvent>,
    mut drag_start_events: EventReader<Pointer<DragStart>>,
    mut drag_events: EventReader<Pointer<Drag>>,
    mut drag_end_events: EventReader<Pointer<DragEnd>>,
) {
    // Handle drag start
    for event in drag_start_events.read() {
        if let Ok((mut thumb, _thumb_ref)) = thumbs.get_mut(event.target) {
            thumb.is_dragging = true;
        }
    }

    // Handle drag
    for event in drag_events.read() {
        if let Ok((thumb, thumb_ref)) = thumbs.get(event.target) {
            if thumb.is_dragging {
                let slider_entity = thumb_ref.0;

                // Find the corresponding track
                let track_result = tracks
                    .iter()
                    .find(|(track_ref, _node, _transform, _computed)| track_ref.0 == slider_entity);

                if let Some((_track_ref, _track_node, track_transform, computed_node)) =
                    track_result
                {
                    if let Ok(mut slider) = sliders.get_mut(slider_entity) {
                        let new_value = calculate_slider_value_from_computed_size(
                            event.pointer_location.position,
                            track_transform,
                            computed_node,
                            &slider,
                        );

                        // Only snap to step if step > 0, otherwise allow continuous movement
                        slider.value = if slider.step > 0.0 {
                            snap_to_step(new_value, slider.step, slider.min, slider.max)
                        } else {
                            new_value.clamp(slider.min, slider.max)
                        };

                        events.write(SliderValueChangeEvent {
                            slider_entity,
                            value: slider.value,
                            thumb_index: thumb.index,
                        });
                    }
                }
            }
        }
    }

    // Handle drag end
    for event in drag_end_events.read() {
        if let Ok((mut thumb, thumb_ref)) = thumbs.get_mut(event.target) {
            if thumb.is_dragging {
                thumb.is_dragging = false;

                // Send commit event
                let slider_entity = thumb_ref.0;
                if let Ok(slider) = sliders.get(slider_entity) {
                    commit_events.write(SliderValueCommitEvent {
                        slider_entity,
                        value: slider.value,
                        thumb_index: thumb.index,
                    });
                }
            }
        }
    }
}

fn snap_to_step(value: f32, step: f32, min: f32, max: f32) -> f32 {
    if step <= 0.0 {
        return value.clamp(min, max);
    }

    let stepped = ((value - min) / step).round() * step + min;
    stepped.clamp(min, max)
}

fn update_thumb_position(thumb_node: &mut Node, percentage: f32, slider: &SliderComponent) {
    let thumb_size_px = match slider.size {
        SliderSize::Size1 => 16.0,
        SliderSize::Size2 => 20.0,
        SliderSize::Size3 => 24.0,
    };
    let half_thumb = thumb_size_px / 2.0;

    match slider.orientation {
        SliderOrientation::Horizontal => {
            // Position the thumb center at the percentage point
            let center_percentage = percentage * 100.0;
            thumb_node.left = Val::Percent(center_percentage);
            thumb_node.top = Val::Percent(50.0);
            
            // Use pixel-based margins for precise centering
            thumb_node.margin.left = Val::Px(-half_thumb);
            thumb_node.margin.top = Val::Px(-half_thumb);
        }
        SliderOrientation::Vertical => {
            let center_percentage = percentage * 100.0;
            thumb_node.bottom = Val::Percent(center_percentage);
            thumb_node.left = Val::Percent(50.0);
            
            // Use pixel-based margins for precise centering
            thumb_node.margin.left = Val::Px(-half_thumb);
            thumb_node.margin.bottom = Val::Px(-half_thumb);
        }
    }
}

// System to handle click on track to jump to position
pub fn handle_track_click(
    mut sliders: Query<&mut SliderComponent>,
    tracks: Query<
        (&TrackRef, &Node, &GlobalTransform, &ComputedNode),
        (With<SliderTrack>, Without<SliderThumb>),
    >,
    mut thumbs: Query<(&ThumbRef, &mut Node), (With<SliderThumb>, Without<SliderTrack>)>,
    mut events: EventWriter<SliderValueChangeEvent>,
    mut commit_events: EventWriter<SliderValueCommitEvent>,
    mut click_events: EventReader<Pointer<Click>>,
) {
    for event in click_events.read() {
        if let Ok((track_ref, _track_node, track_transform, computed_node)) =
            tracks.get(event.target)
        {
            let slider_entity = track_ref.0;
            if let Ok(mut slider) = sliders.get_mut(slider_entity) {
                let new_value = calculate_slider_value_from_computed_size(
                    event.pointer_location.position,
                    track_transform,
                    computed_node,
                    &slider,
                );

                // Only snap to step if step > 0, otherwise allow continuous movement
                slider.value = if slider.step > 0.0 {
                    snap_to_step(new_value, slider.step, slider.min, slider.max)
                } else {
                    new_value.clamp(slider.min, slider.max)
                };

                // Update only the thumb for this specific slider
                let percentage = (slider.value - slider.min) / (slider.max - slider.min);
                for (thumb_ref, mut thumb_node) in thumbs.iter_mut() {
                    if thumb_ref.0 == slider_entity {
                        update_thumb_position(&mut thumb_node, percentage, &slider);
                        break;
                    }
                }

                send_slider_events(slider_entity, slider.value, &mut events, &mut commit_events);
            }
        }
    }
}

fn calculate_value_from_click(
    _event: &Pointer<Click>,
    slider: &SliderComponent,
    _track_node: &Node,
) -> f32 {
    // This is a simplified calculation - in a real implementation,
    // you'd need to convert from screen coordinates to track-relative coordinates
    match slider.orientation {
        SliderOrientation::Horizontal => {
            let percentage = 0.5; // Placeholder - would calculate from event.hit.position
            slider.min + percentage * (slider.max - slider.min)
        }
        SliderOrientation::Vertical => {
            let percentage = 0.5; // Placeholder - would calculate from event.hit.position
            slider.min + percentage * (slider.max - slider.min)
        }
    }
}

// System to update range and thumb when slider value changes
pub fn update_slider_visuals(
    sliders: Query<Entity, (With<SliderComponent>, Changed<SliderComponent>)>,
    slider_components: Query<&SliderComponent>,
    mut ranges: Query<
        &mut Node,
        (
            With<SliderRange>,
            Without<SliderThumb>,
            Without<SliderTrack>,
        ),
    >,
    mut thumbs: Query<
        (&ThumbRef, &mut Node),
        (
            With<SliderThumb>,
            Without<SliderRange>,
            Without<SliderTrack>,
        ),
    >,
    tracks: Query<(&TrackRef, &Children), With<SliderTrack>>,
) {
    for changed_slider_entity in sliders.iter() {
        if let Ok(slider) = slider_components.get(changed_slider_entity) {
            let percentage = (slider.value - slider.min) / (slider.max - slider.min);

            // Find and update the specific range for this slider
            for (track_ref, children) in tracks.iter() {
                if track_ref.0 == changed_slider_entity {
                    // Update range within this track
                    for child in children.iter() {
                        if let Ok(mut range_node) = ranges.get_mut(child) {
                            match slider.orientation {
                                SliderOrientation::Horizontal => {
                                    range_node.width = Val::Percent(percentage * 100.0);
                                }
                                SliderOrientation::Vertical => {
                                    range_node.height = Val::Percent(percentage * 100.0);
                                }
                            }
                        }
                    }
                }
            }

            // Update the specific thumb for this slider
            for (thumb_ref, mut thumb_node) in thumbs.iter_mut() {
                if thumb_ref.0 == changed_slider_entity {
                    update_thumb_position(&mut thumb_node, percentage, slider);
                }
            }
        }
    }
}
