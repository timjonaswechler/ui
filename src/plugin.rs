use crate::assets::IconAssets;
use crate::components::ComponentsPlugin;
use crate::theme::color::{UiColorPalette, UiColorPalettes};
use crate::theme::typography::TypographyAssets;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use once_cell::sync::Lazy;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum UiState {
    #[default]
    LoadingAssets,
    LoadingTheme,
    Ready,
    HotReload,
}

pub static ACCENT_COLOR_PALETTE: Lazy<UiColorPalette> =
    Lazy::new(|| UiColorPalettes::default().indigo);
pub(crate) const FONT_SIZE_BASE: f32 = 16.0;
const SPACING_FACTOR: f32 = 0.25;
const HIGH_CONTRAST: bool = false;
const SCALING: f32 = 1.0;

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
                    .load_collection::<TypographyAssets>()
                    .load_collection::<IconAssets>(),
            )
            // endregion
            // 6) HotReload cycle: detect & trigger in Ready, process in HotReload
            .add_systems(
                Update,
                (|mut _next: ResMut<NextState<UiState>>| {}).run_if(in_state(UiState::Ready)),
            )
            .add_plugins(ComponentsPlugin);

        info!("ForgeUiPlugin loaded. UiState={:?}", app.plugins_state());
    }
}
