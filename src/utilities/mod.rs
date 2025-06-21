use bevy::prelude::*;

pub mod portal;
pub mod text_styling;
pub mod ui_root;

pub use portal::*;
pub use text_styling::*;
pub use ui_root::*;

pub trait ComponentBuilder {
    type Output: Bundle;
    
    fn build(self) -> Self::Output;
}