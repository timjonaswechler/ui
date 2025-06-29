use crate::plugin::UiState;
use crate::theme::typography::FontAssets;
use bevy::app::Update;
use bevy::prelude::*;

pub mod audio;
pub mod icons;

pub use icons::*;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                check_assets_loaded.run_if(in_state(UiState::LoadingAssets)),
                check_theme_ready.run_if(in_state(UiState::LoadingTheme)),
            ),
        );
        info!("AssetPlugin loaded. Font-based Interface icons and Atlas-based Controller icons will be available after startup.");
    }
}

/// System to check if all required assets are loaded
pub fn check_assets_loaded(
    font_assets: Option<Res<FontAssets>>,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<UiState>>,
) {
    // Check if FontAssets resource exists
    let Some(font_assets) = font_assets else {
        return;
    };

    // Check if all font assets are loaded
    let fonts_loaded = all_fonts_loaded(&font_assets, &asset_server);

    if fonts_loaded {
        info!("All assets loaded successfully! Transitioning to LoadingTheme state.");
        next_state.set(UiState::LoadingTheme);
    }
}

/// System to check if theme is ready and transition to Ready state
pub fn check_theme_ready(mut next_state: ResMut<NextState<UiState>>) {
    // For now, immediately transition to Ready after LoadingTheme
    // TODO: Add actual theme initialization checks here
    info!("Theme loading complete! Transitioning to Ready state.");
    next_state.set(UiState::Ready);
}

/// Check if all font assets are loaded
fn all_fonts_loaded(font_assets: &FontAssets, asset_server: &AssetServer) -> bool {
    let font_handles = [
        &font_assets.sans_light,
        &font_assets.sans_regular,
        &font_assets.sans_medium,
        &font_assets.sans_bold,
        &font_assets.serif_regular,
        &font_assets.serif_bold,
        &font_assets.mono_regular,
        &font_assets.mono_bold,
    ];

    font_handles.iter().all(|handle| {
        matches!(
            asset_server.get_load_state(handle.id()),
            Some(bevy::asset::LoadState::Loaded)
        )
    })
}
