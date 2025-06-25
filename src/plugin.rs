use crate::assets::{audio, AssetsPlugin};
use crate::components::ComponentsPlugin;
use crate::theme::typography::load_font_assets;
use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum UiState {
    #[default]
    LoadingAssets,
    LoadingTheme,
    Ready,
    HotReload,
}

pub(crate) const FONT_SIZE_BASE: f32 = 16.0;
pub(crate) const SPACING_FACTOR: f32 = 0.25;
pub(crate) const HIGH_CONTRAST: bool = false;
pub(crate) const SCALING: f32 = 1.0;

pub struct ForgeUiPlugin;

impl ForgeUiPlugin {
    pub fn new() -> Self {
        ForgeUiPlugin {}
    }
}

impl Plugin for ForgeUiPlugin {
    fn build(&self, app: &mut App) {
        app
            // Initialize state system - start with LoadingAssets
            .init_state::<UiState>()
            // Add asset loading in startup systems
            .add_systems(Startup, load_font_assets)
            // Add all plugin systems
            .add_plugins((AssetsPlugin, ComponentsPlugin, audio::plugin));

        info!("ForgeUiPlugin loaded. Starting with UiState={:?}", UiState::LoadingAssets);
    }
}

