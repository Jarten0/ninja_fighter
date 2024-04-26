mod component;
mod converter;
mod error;
mod object_data;
mod object_id;
mod scene_manager;
mod serialized_scene;
#[cfg(test)]
mod test;

use bevy_ecs::reflect::ReflectFromWorld;
use bevy_ecs::world::FromWorld;
use bevy_ecs::{component::Component, world::Mut};
use bevy_reflect::{GetTypeRegistration, Reflect};
use bevy_trait_query::RegisterExt;

use crate::space;
pub use component::Scene;
pub use component::{
    add_entity_to_scene, load_scene, new_scene, save_scene, to_serializable_scene_data,
    unload_scene, validate_name,
};
pub use converter::*;
pub use error::SceneError;
pub use object_data::ReflectTestSuperTrait;
pub use object_data::SceneData;
pub use object_data::TestSuperTrait;
pub use object_id::CounterType;
pub use object_id::ObjectID;
pub use object_id::{ComponentInstanceID, Counter, IDCounter};
pub use scene_manager::SceneManager;
pub use serialized_scene::ToReflect;

pub fn register_scene_types(world: &mut bevy_ecs::world::World) {
    world.init_resource::<scene_manager::SceneManager>();
    world.resource_scope(|world, mut res: Mut<SceneManager>| {
        let mut registry = &mut res.type_registry;
        register_component::<space::Position>(world, registry);
        register_component::<space::Rotation>(world, registry);
        register_component::<space::Scale>(world, registry);
        register_component::<space::TransformSettings>(world, registry);
        register_component::<space::Velocity>(world, registry);
    });
}

/// Registers type data in the registry for the componenets.
pub fn register_component<
    T: bevy_ecs::component::Component
        + bevy_reflect::Reflect
        + bevy_reflect::GetTypeRegistration
        + bevy_reflect::FromReflect
        + bevy_reflect::TypePath
        + serde::Serialize
        + Default
        + TestSuperTrait
        + FromWorld,
>(
    world: &mut bevy_ecs::prelude::World,
    type_registry: &mut bevy_reflect::TypeRegistry,
) {
    type_registry.register::<T>();
    type_registry.register_type_data::<T, ReflectFromWorld>();
    type_registry.register_type_data::<T, ReflectTestSuperTrait>();
    world.init_component::<T>(); // Registers the component id
    world.register_component_as::<dyn TestSuperTrait, T>(); // TestSuperTrait is used in world queries for iterating over types dynamically
}
