//! Collection of icon textures loaded at startup.
//!
//! Icons are provided in multiple resolutions inside the `assets/16x16`,
//! `assets/32x32` and `assets/64x64` folders. All files are loaded via
//! [`bevy_asset_loader`](https://github.com/NiklasEi/bevy_asset_loader) and
//! stored in simple `HashMap`s. Builders which support icons receive a
//! [`Res<IconAssets>`](IconAssets) parameter so they can fetch handles by name:
//!
//! ```rust
//! use forge_ui::prelude::*;
//!
//! fn ui_system(mut commands: Commands, theme: Res<UiTheme>, icons: Res<IconAssets>) {
//!     let check = icons.0.get("check").expect("missing 'check' icon").clone();
//!     commands.spawn(NodeBundle::default()).with_children(|parent| {
//!         BadgeBuilder::new("Checked")
//!             .leading_icon(check)
//!             .spawn(parent, &theme, &theme.font.family.default);
//!     });
//! }
//! ```
//!
//! See the showcase modules for more examples.
//!
//! The tuple fields correspond to the three icon resolutions:
//! * `icons.0` – 16&times;16
//! * `icons.1` – 32&times;32
//! * `icons.2` – 64&times;64

use bevy::asset::{LoadedFolder, UntypedHandle};
use bevy::prelude::*;
use bevy_asset_loader::prelude::AssetCollection;
use std::collections::HashMap;

#[derive(Resource)]
pub struct IconAssets(
    /// 16x16 icons
    pub HashMap<String, Handle<Image>>,
    /// 32x32 icons
    pub HashMap<String, Handle<Image>>,
    /// 64x64 icons
    pub HashMap<String, Handle<Image>>,
);

fn load_folder(world: &World, folder: &str) -> HashMap<String, Handle<Image>> {
    let asset_server = world
        .get_resource::<AssetServer>()
        .expect("AssetServer missing");
    let loaded_folders = world
        .get_resource::<Assets<LoadedFolder>>()
        .expect("LoadedFolder assets missing");
    let handle = asset_server.load_folder(folder);
    if let Some(folder) = loaded_folders.get(&handle) {
        folder
            .handles
            .iter()
            .filter_map(|handle| {
                asset_server.get_path(handle).and_then(|path| {
                    path.path()
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .map(|stem| (stem.to_string(), handle.clone().typed()))
                })
            })
            .collect()
    } else {
        HashMap::new()
    }
}

impl AssetCollection for IconAssets {
    fn create(world: &mut World) -> Self {
        let map16 = load_folder(world, "16x16");
        let map32 = load_folder(world, "32x32");
        let map64 = load_folder(world, "64x64");
        IconAssets(map16, map32, map64)
    }

    fn load(world: &mut World) -> Vec<UntypedHandle> {
        let asset_server = world
            .get_resource::<AssetServer>()
            .expect("AssetServer missing");
        vec![
            asset_server.load_folder("16x16").untyped(),
            asset_server.load_folder("32x32").untyped(),
            asset_server.load_folder("64x64").untyped(),
        ]
    }
}
