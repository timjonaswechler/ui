use crate::assets::{audio::plugin, IconAssets};
use crate::components::ComponentsPlugin;
use crate::theme::typography::TypographyAssets;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

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
            // region: 1) Initialize UiState and set starting variant
            .init_state::<UiState>()
            .insert_state(UiState::LoadingAssets)
            // endregion
            // region: 2) Asset-Loading: load FontAssets and IconAssets, then go to LoadingTheme
            .add_loading_state(
                LoadingState::new(UiState::LoadingAssets)
                    .continue_to_state(UiState::LoadingTheme)
                    .load_collection::<TypographyAssets>(),
            )
            // endregion
            // 3) Theme initialization: check if assets are loaded and go to Ready
            .add_systems(
                Update,
                check_assets_loaded.run_if(in_state(UiState::LoadingTheme)),
            )
            // 6) HotReload cycle: detect & trigger in Ready, process in HotReload
            .add_systems(
                Update,
                (|mut _next: ResMut<NextState<UiState>>| {}).run_if(in_state(UiState::Ready)),
            )
            .add_plugins((ComponentsPlugin, crate::assets::audio::plugin));

        info!("ForgeUiPlugin loaded. UiState={:?}", app.plugins_state());
    }
}

/// System to check if assets are loaded and transition to Ready state
fn check_assets_loaded(
    typography_assets: Option<Res<TypographyAssets>>,
    mut next_state: ResMut<NextState<UiState>>,
) {
    if typography_assets.is_some() {
        info!("Assets loaded, transitioning to Ready state");
        next_state.set(UiState::Ready);
    }
}
