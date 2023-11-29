use bevy_ecs::prelude::*;
// use ggez::graphics::Transform;

use crate::transform::Transform;

#[derive(Default, Component)]
pub struct Protag {}

#[derive(Bundle, Default)]
pub struct ProtagBundle {
    pub protag: Protag,
    pub transform: Transform,
}
