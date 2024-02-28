mod component;
mod error;
mod scene_manager;
mod scene_object;
mod serialized_scene;
#[cfg(test)]
mod test;
mod traits;

pub(self) use component::{
    add_entity_to_scene, load_scene, new_scene, save_scene, to_serialized_scene, unload_scene,
    validate_name,
};

pub use component::Scene;
pub use error::SceneError;
pub use scene_manager::SceneManager;
pub use traits::SceneData;

pub fn register_scene_types(world: &mut bevy_ecs::world::World) {
    world.init_resource::<scene_manager::SceneManager>();
    world.init_component::<traits::SceneData>();
}
