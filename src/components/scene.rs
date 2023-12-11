use bevy_ecs::{bundle::Bundle, component::Component, query, system::Query};

use super::context::WorldInfo;

#[derive(Component)]
pub struct SceneProperties {}

#[derive(Bundle)]
pub struct Scene {
    scene_properties: SceneProperties,
}

fn new_entity(query: Query<(&mut SceneProperties, &mut WorldInfo)>) {}
