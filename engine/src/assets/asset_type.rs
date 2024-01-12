use bevy_ecs::{entity::Entity, world::World};

use super::super::RenderType;

#[derive(Debug)]
pub enum AssetType {
    Render(RenderType),
    Entity,
}

// fn playground(mut world: World) {
//     // world.spawn_empty().id();
// }
