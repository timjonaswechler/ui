use bevy::{color::palettes::css::*, prelude::*, winit::WinitSettings};
use ui::{
    components::{SwitchComponent, SwitchChangeEvent},
    plugin::ForgeUiPlugin,
    theme::color::{accent_palette, error_palette, success_palette, warning_palette},
    utilities::ComponentBuilder,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ForgeUiPlugin)
        .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, setup)
        .add_systems(Update, handle_switch_events)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            padding: UiRect::all(Val::Px(20.0)),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            row_gap: Val::Px(20.0),
            ..default()
        },
        BackgroundColor(GRAY.into()),
    )).with_children(|parent| {
        // Title
        parent.spawn((
            Text::new("Switch Component Test"),
            TextColor(BLACK.into()),
            TextFont {
                font_size: 32.0,
                ..default()
            },
        ));

        // Basic switches
        parent.spawn(SwitchComponent::new("basic").build());
        parent.spawn(SwitchComponent::new("checked").checked().build());
        parent.spawn(SwitchComponent::new("disabled").disabled().build());

        // Different sizes
        parent.spawn(SwitchComponent::new("size1").size_1().checked().build());
        parent.spawn(SwitchComponent::new("size2").size_2().checked().build());
        parent.spawn(SwitchComponent::new("size3").size_3().checked().build());

        // Different variants
        parent.spawn(SwitchComponent::new("surface").surface().checked().build());
        parent.spawn(SwitchComponent::new("classic").classic().checked().build());
        parent.spawn(SwitchComponent::new("soft").soft().checked().build());

        // Different colors
        parent.spawn(SwitchComponent::new("accent").color(accent_palette()).checked().build());
        parent.spawn(SwitchComponent::new("success").color(success_palette()).checked().build());
        parent.spawn(SwitchComponent::new("warning").color(warning_palette()).checked().build());
        parent.spawn(SwitchComponent::new("error").color(error_palette()).checked().build());
    });
}

fn handle_switch_events(mut switch_events: EventReader<SwitchChangeEvent>) {
    for event in switch_events.read() {
        println!(
            "Switch toggled! Entity: {:?}, Checked: {}, Size: {:?}",
            event.switch_entity, event.checked, event.size
        );
    }
}