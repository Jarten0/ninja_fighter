mod component;
mod converter;
mod error;
mod object_data;
mod object_id;
mod scene_manager;
mod serialized_scene;
#[cfg(test)]
mod test;

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
        register::<space::Position>(world, &mut res);
        register::<space::Rotation>(world, &mut res);
        register::<space::Scale>(world, &mut res);
        register::<space::TransformSettings>(world, &mut res);
        register::<space::Velocity>(world, &mut res);
        // register::<space::Vector2>(world, &mut res);
    });
}

pub fn register<T>(world: &mut bevy_ecs::world::World, res: &mut Mut<SceneManager>)
where
    T: Component
        + Reflect
        + GetTypeRegistration
        + bevy_reflect::TypePath
        + bevy_reflect::TypePath
        + serde::Serialize
        + Default
        + TestSuperTrait,
{
    world.init_component::<T>();
    res.type_registry.register::<T>();
    res.type_registry
        .register_type_data::<T, ReflectTestSuperTrait>();
    world.register_component_as::<dyn TestSuperTrait, T>();
}

/// Enables the component to be serialized
pub fn serialize_component<
    T: bevy_ecs::component::Component
        + bevy_reflect::GetTypeRegistration
        + bevy_reflect::Reflect
        + serde::Serialize
        + bevy_reflect::TypePath
        + Default
        + TestSuperTrait,
>(
    world: &mut bevy_ecs::prelude::World,
    register: &mut bevy_reflect::TypeRegistry,
) {
    world.init_component::<T>();
    bevy_trait_query::RegisterExt::register_component_as::<dyn TestSuperTrait, T>(world);
    register.register::<T>();
    register.register_type_data::<T, ReflectTestSuperTrait>()
}
