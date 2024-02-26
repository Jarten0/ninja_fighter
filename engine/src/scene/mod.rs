mod component;
mod scene_object;
mod serialized_scene;

#[cfg(test)]
mod test;

mod scene_manager;
mod traits;

#[derive(Debug)]
pub enum SceneError {
    /// No scene was selected as the target when saving.
    NoTargetScene,
    /// Something went wrong while parsing a file.
    /// [`String`] is the IO error message, formatted as a string
    IOError(String),
    /// A scene that was loaded contained a component that has not been registered by the directory.
    /// [`String`] is the path of the missing component.
    MissingTypeRegistry(String),
    /// Failed to instantiate a scene, though not because of an IO error.
    /// Might be because of an ECS failure somewhere.
    LoadFailure(&'static str),
}

pub(self) use component::{
    add_entity_to_scene, load_scene, new_scene, save_scene, to_serialized_scene, unload_scene,
    validate_name,
};

pub use scene_manager::SceneManager;
pub use traits::SceneData;

pub fn register_scene_types(world: &mut bevy_ecs::world::World) {
    world.init_resource::<scene_manager::SceneManager>();
    world.init_component::<traits::SceneData>();
}
