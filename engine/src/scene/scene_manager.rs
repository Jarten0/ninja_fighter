use bevy_ecs::entity::Entity;
use bevy_ecs::system::Resource;
use bevy_reflect::TypeRegistry;

#[derive(Resource, Default)]
pub struct SceneManager {
    /// Contains every [`Entity`] with a [`Scene`]
    current_scenes: Vec<Entity>,
}
