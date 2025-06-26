// Re-export aller Loading-Funktionen für direkten Zugriff
pub use super::controllers::{
    load_generic_atlases, load_indicators_atlases, load_keyboardmouse_atlases,
    load_nintendoswitch_atlases, load_playstation_atlases, load_steamdeck_atlases,
    load_xbox_atlases,
};
pub use super::interface::load_interface_atlases;

/// Lädt alle Icon-Atlases für alle Kategorien
///
/// Verwendung in der App:
/// ```
/// app.add_systems(Startup, (
///     load_interface_atlases,
///     load_generic_atlases,
///     load_input_atlases,
///     load_nintendoswitch_atlases,
///     load_playstation_atlases,
///     load_xbox_atlases,
///     load_steamdeck_atlases,
///     load_indicators_atlases,
/// ));
/// ```
pub struct IconLoading;
