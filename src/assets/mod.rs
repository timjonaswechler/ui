//! Asset collections used by Forge UI.
//!
//! The [`ForgeUiPlugin`](crate::plugin::ForgeUiPlugin) loads both fonts and
//! icons at startup using `bevy_asset_loader`. Icons are stored in the
//! `assets/16x16`, `assets/32x32` and `assets/64x64` folders and can then be accessed via
//! [`IconAssets`], e.g. `icons.0.get("check")`.
//!
//! ```rust
//! use forge_ui::prelude::*;
//!
//! fn system(icons: Res<IconAssets>) {
//!     if let Some(handle) = icons.0.get("cross_1") {
//!         // use `handle` with a builder
//!     }
//! }
//! ```
//
mod icon_assets;
pub use icon_assets::IconAssets;
pub mod audio;
