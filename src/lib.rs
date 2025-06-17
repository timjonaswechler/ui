use bevy::prelude::*;

pub mod components;
pub mod theme;
pub mod utilities;

pub struct RadixUIPlugin;

impl Plugin for RadixUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(theme::ThemePlugin)
           .add_plugins(components::ComponentsPlugin);
    }
}

pub use components::*;
pub use theme::*;
pub use utilities::*;