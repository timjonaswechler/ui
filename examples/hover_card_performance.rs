use bevy::prelude::*;
use ui::{
    components::{
        hover_card::{
            HoverCardBuilder, HoverCardContentBuilder, HoverCardSide, HoverCardTriggerBuilder,
        },
        text::Text,
    },
    plugin::ForgeUiPlugin,
    theme::color::{accent_palette, error_palette, success_palette},
    utilities::ui_root::ui_root,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ForgeUiPlugin)
        .add_systems(Startup, setup_performance_test)
        .add_systems(
            Update,
            (performance_monitor_system, hover_card_stress_test_system),
        )
        .run();
}

#[derive(Component)]
struct PerformanceTestCard {
    index: usize,
    creation_time: f32,
}

#[derive(Resource)]
struct PerformanceMetrics {
    total_cards: usize,
    active_cards: usize,
    frame_time: f32,
    last_report_time: f32,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            total_cards: 0,
            active_cards: 0,
            frame_time: 0.0,
            last_report_time: 0.0,
        }
    }
}

fn setup_performance_test(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn(ui_root("main"));
    commands.init_resource::<PerformanceMetrics>();

    // Create main container
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
        ))
        .with_children(|parent| {
            // Title
            parent.spawn(
                Text::title("HoverCard Performance Test")
                    .size(ui::theme::typography::TextSize::X3l)
                    .build(),
            );

            // Instructions
            parent.spawn(
                Text::body("Stress testing HoverCard performance with multiple cards")
                    .size(ui::theme::typography::TextSize::Base)
                    .build(),
            );

            // Performance metrics display
            parent.spawn((
                Text::body("Performance: Initializing...")
                    .size(ui::theme::typography::TextSize::Sm)
                    .build(),
                Name::new("PerformanceDisplay"),
            ));

            // Grid of hover cards for stress testing
            parent.spawn((
                Node {
                    display: Display::Grid,
                    grid_template_columns: RepeatedGridTrack::flex(10, 1.0),
                    grid_template_rows: RepeatedGridTrack::flex(8, 1.0),
                    column_gap: Val::Px(5.0),
                    row_gap: Val::Px(5.0),
                    margin: UiRect::top(Val::Px(20.0)),
                    ..default()
                },
                Name::new("CardGrid"),
            ));
        });
}

// This would be called in a setup system to create test cards
fn create_performance_test_cards(mut commands: Commands) {
    // This is a simplified version for demonstration
    info!("Performance test setup would create cards here");
}

fn performance_monitor_system(
    time: Res<Time>,
    mut metrics: ResMut<PerformanceMetrics>,
    test_cards: Query<&PerformanceTestCard>,
    hover_cards: Query<&ui::components::hover_card::HoverCard>,
    mut text_query: Query<&mut Text2d>,
    display_query: Query<Entity, With<Name>>,
) {
    metrics.frame_time = time.delta_secs();
    metrics.total_cards = test_cards.iter().count();
    metrics.active_cards = hover_cards
        .iter()
        .filter(|card| card.state == ui::components::hover_card::HoverCardState::Open)
        .count();

    // Update performance display every second
    if time.elapsed_secs() - metrics.last_report_time > 1.0 {
        metrics.last_report_time = time.elapsed_secs();

        let fps = if metrics.frame_time > 0.0 {
            1.0 / metrics.frame_time
        } else {
            0.0
        };

        let performance_text = format!(
            "Performance: {} cards total | {} active | {:.1} FPS | {:.2}ms frame time",
            metrics.total_cards,
            metrics.active_cards,
            fps,
            metrics.frame_time * 1000.0
        );

        // Find and update the performance display
        for entity in display_query.iter() {
            if let Ok(mut text) = text_query.get_mut(entity) {
                *text = Text2d::new(performance_text.clone());
                break;
            }
        }
    }
}

fn hover_card_stress_test_system(time: Res<Time>, mut test_cards: Query<&mut PerformanceTestCard>) {
    // Update creation times for cards
    for mut card in test_cards.iter_mut() {
        if card.creation_time == 0.0 {
            card.creation_time = time.elapsed_secs();
        }
    }
}
