use bevy::prelude::*;
use ui::{
    components::{BadgeBuilder, FlexComponent, Heading},
    plugin::{ForgeUiPlugin, UiState},
    theme::{
        color::{accent_palette, error_palette, success_palette, theme, warning_palette},
        typography::TextWeight,
    },
    utilities::ui_root,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ForgeUiPlugin)
        .add_systems(OnEnter(UiState::Ready), setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // Create main container
    let main_container = FlexComponent::new("BadgeDemo")
        .column()
        .gap(32.0)
        .padding(Val::Px(40.0))
        .build();

    let main_entity = commands.spawn(main_container).id();

    // Title
    let title = Heading::h1("Badge Component Demo")
        .weight(TextWeight::Bold)
        .build();
    let title_entity = commands.spawn(title).id();
    commands.entity(main_entity).add_child(title_entity);

    // Basic Examples Section
    let basic_section = create_section(
        &mut commands,
        "Basic Badges",
        vec![
            BadgeBuilder::new("New")
                .text("New")
                .color(theme().gray)
                .build(),
            BadgeBuilder::new("InProgress")
                .text("In Progress")
                .color(theme().orange)
                .build(),
            BadgeBuilder::new("Complete")
                .text("Complete")
                .color(theme().green)
                .build(),
        ],
    );
    commands.entity(main_entity).add_child(basic_section);

    // Size Variants Section
    let size_section = create_section(
        &mut commands,
        "Size Variants",
        vec![
            BadgeBuilder::new("Size1")
                .text("Size 1")
                .size_1()
                .color(accent_palette())
                .build(),
            BadgeBuilder::new("Size2")
                .text("Size 2")
                .size_2()
                .color(accent_palette())
                .build(),
            BadgeBuilder::new("Size3")
                .text("Size 3")
                .size_3()
                .color(accent_palette())
                .build(),
        ],
    );
    commands.entity(main_entity).add_child(size_section);

    // Visual Variants Section
    let variant_section = create_section(
        &mut commands,
        "Visual Variants",
        vec![
            BadgeBuilder::new("Solid")
                .text("Solid")
                .solid()
                .color(theme().blue)
                .build(),
            BadgeBuilder::new("Soft")
                .text("Soft")
                .soft()
                .color(theme().blue)
                .build(),
            BadgeBuilder::new("Surface")
                .text("Surface")
                .surface()
                .color(theme().blue)
                .build(),
            BadgeBuilder::new("Outline")
                .text("Outline")
                .outline()
                .color(theme().blue)
                .build(),
        ],
    );
    commands.entity(main_entity).add_child(variant_section);

    // Color Palette Section
    let color_section = create_section(
        &mut commands,
        "Color Palette",
        vec![
            BadgeBuilder::new("Gray")
                .text("Gray")
                .color(theme().gray)
                .build(),
            BadgeBuilder::new("Red")
                .text("Error")
                .color(error_palette())
                .build(),
            BadgeBuilder::new("Orange")
                .text("Warning")
                .color(warning_palette())
                .build(),
            BadgeBuilder::new("Green")
                .text("Success")
                .color(success_palette())
                .build(),
            BadgeBuilder::new("Blue")
                .text("Info")
                .color(theme().blue)
                .build(),
            BadgeBuilder::new("Cyan")
                .text("Cyan")
                .color(theme().cyan)
                .build(),
            BadgeBuilder::new("Crimson")
                .text("Crimson")
                .color(theme().crimson)
                .build(),
        ],
    );
    commands.entity(main_entity).add_child(color_section);

    // High Contrast Section
    let contrast_section = create_section(
        &mut commands,
        "High Contrast Mode",
        vec![
            BadgeBuilder::new("NormalContrast")
                .text("Normal")
                .color(theme().gray)
                .solid()
                .build(),
            BadgeBuilder::new("HighContrast")
                .text("High Contrast")
                .color(theme().gray)
                .solid()
                .high_contrast(true)
                .build(),
        ],
    );
    commands.entity(main_entity).add_child(contrast_section);

    // Border Radius Section
    let radius_section = create_section(
        &mut commands,
        "Border Radius Options",
        vec![
            BadgeBuilder::new("None")
                .text("None")
                .rounded_none()
                .color(accent_palette())
                .solid()
                .build(),
            BadgeBuilder::new("Small")
                .text("Small")
                .rounded_small()
                .color(accent_palette())
                .solid()
                .build(),
            BadgeBuilder::new("Medium")
                .text("Medium")
                .rounded_medium()
                .color(accent_palette())
                .solid()
                .build(),
            BadgeBuilder::new("Large")
                .text("Large")
                .rounded_large()
                .color(accent_palette())
                .solid()
                .build(),
            BadgeBuilder::new("Pill")
                .text("Pill (Full)")
                .pill()
                .color(accent_palette())
                .solid()
                .build(),
        ],
    );
    commands.entity(main_entity).add_child(radius_section);

    // Status Example Section
    let status_section = create_section(
        &mut commands,
        "Status Indicators",
        vec![
            BadgeBuilder::new("Online")
                .text("● Online")
                .color(success_palette())
                .soft()
                .size_1()
                .build(),
            BadgeBuilder::new("Away")
                .text("● Away")
                .color(warning_palette())
                .soft()
                .size_1()
                .build(),
            BadgeBuilder::new("Busy")
                .text("● Busy")
                .color(error_palette())
                .soft()
                .size_1()
                .build(),
            BadgeBuilder::new("Offline")
                .text("● Offline")
                .color(theme().gray)
                .soft()
                .size_1()
                .build(),
        ],
    );
    commands.entity(main_entity).add_child(status_section);

    // Setup UI Root
    commands
        .spawn(ui_root("badge_demo_root"))
        .add_child(main_entity);
}

fn create_section(commands: &mut Commands, title: &str, badges: Vec<impl Bundle>) -> Entity {
    // Section container
    let section = FlexComponent::new(format!("{}Section", title.replace(" ", "")))
        .column()
        .gap(12.0)
        .build();

    let section_entity = commands.spawn(section).id();

    // Section title
    let section_title = Heading::h3(title).weight(TextWeight::Medium).build();
    let title_entity = commands.spawn(section_title).id();
    commands.entity(section_entity).add_child(title_entity);

    // Badge container
    let badge_container = FlexComponent::new(format!("{}Badges", title.replace(" ", "")))
        .row()
        .gap(8.0)
        .wrap()
        .build();

    let container_entity = commands.spawn(badge_container).id();
    commands.entity(section_entity).add_child(container_entity);

    // Add badges to container
    for badge in badges {
        let badge_entity = commands.spawn(badge).id();
        commands.entity(container_entity).add_child(badge_entity);
    }

    section_entity
}
