use bevy::prelude::*;

pub mod portal;
pub mod ui_root;

pub use portal::*;
pub use ui_root::*;

pub trait ComponentBuilder {
    type Output: Bundle;
    
    fn build(self) -> Self::Output;
}