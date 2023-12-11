use bevy_ecs::prelude::*;

use crate::components::Transform;

use super::Renderer;

#[derive(Default, Component)]
pub struct Protag {
    
}

#[derive(Bundle)]
pub struct ProtagBundle {
    pub protag: Protag,
    pub transform: Transform,
    pub renderer: Renderer,
}
