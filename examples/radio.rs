use bevy::prelude::*;
use ui::{
    components::{
        radio::{
            RadioChangeEvent, RadioComponent, RadioGroupComponent, RadioGroupValueChangeEvent,
        },
        text::Text,
        BoxComponent, FlexComponent,
    },
    plugin::{ForgeUiPlugin, UiState},
    theme::typography::TextSize,
    utilities::{ui_root, ComponentBuilder},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ForgeUiPlugin)
        .add_systems(OnEnter(UiState::Ready), setup_ui)
        .add_systems(Update, (handle_radio_events, handle_group_events))
        .run();
}

fn setup_ui(mut commands: Commands) {
    // Create camera
    commands.spawn(Camera2d);

    // Create UI root
    commands
        .spawn(ui_root("Radio Example"))
        .with_children(|parent| {
            // Title
            parent.spawn(
                Text::title("Radio Component Examples")
                    .size(TextSize::X3l)
                    .build(),
            );

            // Section spacing
            parent.spawn(BoxComponent::new("spacing-1").height(Val::Px(32.0)).build());

            // Single Choice Examples Section
            parent.spawn(
                Text::title("Single Choice Radio Groups")
                    .size(TextSize::Xl)
                    .build(),
            );

            // Theme Selection (with pre-selected option)
            parent
                .spawn(
                    BoxComponent::new("theme-section")
                        .panel()
                        .padding(Val::Px(20.0))
                        .margin_y(Val::Px(16.0))
                        .build(),
                )
                .with_children(|section| {
                    section.spawn(Text::title("Theme Selection").size(TextSize::Lg).build());

                    section.spawn(
                        BoxComponent::new("theme-spacing")
                            .height(Val::Px(12.0))
                            .build(),
                    );

                    // Single choice radio group with pre-selected "dark"
                    section
                        .spawn(
                            RadioGroupComponent::single_choice("theme")
                                .default_value("dark")
                                .vertical()
                                .gap(12.0)
                                .build(),
                        )
                        .with_children(|group| {
                            let theme_options = [
                                ("light", "Light Theme"),
                                ("dark", "Dark Theme"),
                                ("auto", "Auto (System)"),
                            ];

                            for (value, label) in theme_options {
                                group
                                    .spawn(
                                        FlexComponent::row(&format!("theme-{}", value))
                                            .gap(12.0)
                                            .align_center()
                                            .margin_y(Val::Px(4.0))
                                            .build(),
                                    )
                                    .with_children(|row| {
                                        row.spawn(RadioComponent::new(value).build());
                                        row.spawn(Text::body(label).build());
                                    });
                            }
                        });
                });

            // Language Selection (horizontal layout)
            parent
                .spawn(
                    BoxComponent::new("language-section")
                        .panel()
                        .padding(Val::Px(20.0))
                        .margin_y(Val::Px(16.0))
                        .build(),
                )
                .with_children(|section| {
                    section.spawn(Text::title("Language Selection").size(TextSize::Lg).build());

                    section.spawn(
                        BoxComponent::new("language-spacing")
                            .height(Val::Px(12.0))
                            .build(),
                    );

                    // Horizontal single choice radio group with pre-selected "en"
                    section
                        .spawn(
                            RadioGroupComponent::single_choice("language")
                                .default_value("en")
                                .horizontal()
                                .gap(24.0)
                                .build(),
                        )
                        .with_children(|group| {
                            let language_options = [
                                ("en", "English"),
                                ("de", "Deutsch"),
                                ("fr", "Français"),
                                ("es", "Español"),
                            ];

                            for (value, label) in language_options {
                                group
                                    .spawn(
                                        FlexComponent::row(&format!("lang-{}", value))
                                            .gap(8.0)
                                            .align_center()
                                            .build(),
                                    )
                                    .with_children(|row| {
                                        row.spawn(RadioComponent::new(value).build());
                                        row.spawn(Text::body(label).build());
                                    });
                            }
                        });
                });

            // Section spacing
            parent.spawn(BoxComponent::new("spacing-2").height(Val::Px(32.0)).build());

            // Multi Choice Examples Section
            parent.spawn(
                Text::title("Multi Choice Radio Groups")
                    .size(TextSize::Xl)
                    .build(),
            );

            // Features Selection (multiple choice with pre-selected options)
            // parent
            //     .spawn(
            //         BoxComponent::new("features-section")
            //             .panel()
            //             .padding(Val::Px(20.0))
            //             .margin_y(Val::Px(16.0))
            //             .build(),
            //     )
            //     .with_children(|section| {
            //         section.spawn(
            //             Text::title("Enable Features")
            //                 .size(TextSize::Lg)
            //                 .build(),
            //         );

            //         section.spawn(
            //             Text::body("Multiple selections allowed - each can be toggled independently")
            //                 .size(TextSize::Sm)
            //                 .build(),
            //         );

            //         section.spawn(
            //             BoxComponent::new("features-spacing")
            //                 .height(Val::Px(12.0))
            //                 .build(),
            //         );

            //         // Multi choice radio group
            //         section
            //             .spawn(
            //                 RadioGroupComponent::multi_choice("features")
            //                     .vertical()
            //                     .gap(12.0)
            //                     .build(),
            //             )
            //             .with_children(|group| {
            //                 let feature_options = [
            //                     ("notifications", "Push Notifications", true),
            //                     ("auto-save", "Auto Save", true),
            //                     ("sync", "Cloud Sync", false),
            //                     ("analytics", "Usage Analytics", false),
            //                     ("beta", "Beta Features", false),
            //                 ];

            //                 for (value, label, pre_selected) in feature_options {
            //                     group.spawn(
            //                         FlexComponent::row(&format!("feature-{}", value))
            //                             .gap(12.0)
            //                             .align_center()
            //                             .margin_y(Val::Px(4.0))
            //                             .build(),
            //                     )
            //                     .with_children(|row| {
            //                         let radio = if pre_selected {
            //                             RadioComponent::checked(value)
            //                         } else {
            //                             RadioComponent::new(value)
            //                         };
            //                         row.spawn(radio.build());
            //                         row.spawn(Text::body(label).build());
            //                     });
            //                 }
            //             });
            //     });

            // // Permissions Selection (multiple choice)
            // parent
            //     .spawn(
            //         BoxComponent::new("permissions-section")
            //             .panel()
            //             .padding(Val::Px(20.0))
            //             .margin_y(Val::Px(16.0))
            //             .build(),
            //     )
            //     .with_children(|section| {
            //         section.spawn(
            //             Text::title("App Permissions")
            //                 .size(TextSize::Lg)
            //                 .build(),
            //         );

            //         section.spawn(
            //             Text::body("Grant permissions as needed")
            //                 .size(TextSize::Sm)
            //                 .build(),
            //         );

            //         section.spawn(
            //             BoxComponent::new("permissions-spacing")
            //                 .height(Val::Px(12.0))
            //                 .build(),
            //         );

            //         // Multi choice radio group for permissions
            //         section
            //             .spawn(
            //                 RadioGroupComponent::multi_choice("permissions")
            //                     .vertical()
            //                     .gap(12.0)
            //                     .build(),
            //             )
            //             .with_children(|group| {
            //                 let permission_options = [
            //                     ("camera", "Camera Access", false),
            //                     ("microphone", "Microphone Access", false),
            //                     ("location", "Location Services", true),
            //                     ("contacts", "Contacts Access", false),
            //                     ("storage", "File Storage", true),
            //                 ];

            //                 for (value, label, pre_selected) in permission_options {
            //                     group.spawn(
            //                         FlexComponent::row(&format!("perm-{}", value))
            //                             .gap(12.0)
            //                             .align_center()
            //                             .margin_y(Val::Px(4.0))
            //                             .build(),
            //                     )
            //                     .with_children(|row| {
            //                         let radio = if pre_selected {
            //                             RadioComponent::checked(value)
            //                         } else {
            //                             RadioComponent::new(value)
            //                         };
            //                         row.spawn(radio.build());
            //                         row.spawn(Text::body(label).build());
            //                     });
            //                 }
            //             });
            //     });

            // // Section spacing
            // parent.spawn(
            //     BoxComponent::new("spacing-3")
            //         .height(Val::Px(32.0))
            //         .build(),
            // );

            // // Size and Variant Examples Section
            // parent.spawn(
            //     Text::title("Radio Sizes & Variants")
            //         .size(TextSize::Xl)
            //         .build(),
            // );

            // parent
            //     .spawn(
            //         FlexComponent::row("sizes-demo")
            //             .gap(32.0)
            //             .align_start()
            //             .margin_y(Val::Px(16.0))
            //             .build(),
            //     )
            //     .with_children(|row| {
            //         // Size examples
            //         row.spawn(
            //             FlexComponent::column("sizes")
            //                 .gap(16.0)
            //                 .build(),
            //         )
            //         .with_children(|col| {
            //             col.spawn(
            //                 Text::title("Sizes")
            //                     .size(TextSize::Lg)
            //                     .build(),
            //             );

            //             // Size 1 - Compact
            //             col.spawn(
            //                 FlexComponent::row("size1-row")
            //                     .gap(8.0)
            //                     .align_center()
            //                     .build(),
            //             )
            //             .with_children(|size_row| {
            //                 size_row.spawn(RadioComponent::size_1("size1").checked().build());
            //                 size_row.spawn(Text::body("Size 1 (14px)").build());
            //             });

            //             // Size 2 - Default
            //             col.spawn(
            //                 FlexComponent::row("size2-row")
            //                     .gap(8.0)
            //                     .align_center()
            //                     .build(),
            //             )
            //             .with_children(|size_row| {
            //                 size_row.spawn(RadioComponent::size_2("size2").checked().build());
            //                 size_row.spawn(Text::body("Size 2 (16px)").build());
            //             });

            //             // Size 3 - Large
            //             col.spawn(
            //                 FlexComponent::row("size3-row")
            //                     .gap(8.0)
            //                     .align_center()
            //                     .build(),
            //             )
            //             .with_children(|size_row| {
            //                 size_row.spawn(RadioComponent::size_3("size3").checked().build());
            //                 size_row.spawn(Text::body("Size 3 (20px)").build());
            //             });
            //         });

            //         // Variant examples
            //         row.spawn(
            //             FlexComponent::column("variants")
            //                 .gap(16.0)
            //                 .build(),
            //         )
            //         .with_children(|col| {
            //             col.spawn(
            //                 Text::title("Variants")
            //                     .size(TextSize::Lg)
            //                     .build(),
            //             );

            //             // Surface variant
            //             col.spawn(
            //                 FlexComponent::row("surface-row")
            //                     .gap(8.0)
            //                     .align_center()
            //                     .build(),
            //             )
            //             .with_children(|var_row| {
            //                 var_row.spawn(RadioComponent::new("surface").surface().checked().build());
            //                 var_row.spawn(Text::body("Surface").build());
            //             });

            //             // Classic variant
            //             col.spawn(
            //                 FlexComponent::row("classic-row")
            //                     .gap(8.0)
            //                     .align_center()
            //                     .build(),
            //             )
            //             .with_children(|var_row| {
            //                 var_row.spawn(RadioComponent::new("classic").classic().checked().build());
            //                 var_row.spawn(Text::body("Classic").build());
            //             });

            //             // Soft variant
            //             col.spawn(
            //                 FlexComponent::row("soft-row")
            //                     .gap(8.0)
            //                     .align_center()
            //                     .build(),
            //             )
            //             .with_children(|var_row| {
            //                 var_row.spawn(RadioComponent::new("soft").soft().checked().build());
            //                 var_row.spawn(Text::body("Soft").build());
            //             });
            //         });
            //     });

            // // Instructions
            // parent.spawn(
            //     BoxComponent::new("spacing-4")
            //         .height(Val::Px(32.0))
            //         .build(),
            // );

            // parent.spawn(
            //     Text::body("Click any radio button to toggle its state. Single-choice groups allow only one selection, multi-choice groups allow multiple. Check the console for change events.")
            //         .size(TextSize::Sm)
            //         .build(),
            // );
        });
}

fn handle_radio_events(mut radio_events: EventReader<RadioChangeEvent>) {
    for event in radio_events.read() {
        println!(
            "🔘 Radio '{}' in group '{}' changed - selected: {} (previous: {:?})",
            event.selected_value, event.group_name, event.selected_value, event.previous_value
        );
    }
}

fn handle_group_events(mut group_events: EventReader<RadioGroupValueChangeEvent>) {
    for event in group_events.read() {
        println!(
            "📻 Group '{}' value changed - new: {:?}, old: {:?}",
            event.group_name, event.new_value, event.old_value
        );
    }
}
