use bevy::prelude::*;

pub trait ComponentBuilder {
    type Output: Bundle;
    
    fn build(self) -> Self::Output;
}