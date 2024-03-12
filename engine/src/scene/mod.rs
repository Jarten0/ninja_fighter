mod component;
mod error;
mod object_data;
mod object_id;
mod scene_manager;
mod serialized_scene;
#[cfg(test)]
mod test;

use bevy_ecs::{component::Component, world::Mut};
use bevy_reflect::{GetTypeRegistration, Reflect};
pub use component::{
    add_entity_to_scene, load_scene, new_scene, save_scene, to_serialized_scene, unload_scene,
    validate_name,
};

pub use component::Scene;
pub use error::SceneError;
pub use object_data::ReflectTestSuperTrait;
pub use object_data::SceneData;
pub use object_data::TestSuperTrait;
pub use object_id::{CounterType, ObjectID};
pub use scene_manager::SceneManager;
pub use serialized_scene::ToReflect;

use crate::space;

pub fn register_scene_types(world: &mut bevy_ecs::world::World) {
    world.init_resource::<scene_manager::SceneManager>();
    world.resource_scope(|world, mut res: Mut<SceneManager>| {
        register::<object_data::SceneData>(world, &mut res);
        register::<space::Position>(world, &mut res);
        register::<space::Rotation>(world, &mut res);
        register::<space::Scale>(world, &mut res);
        register::<space::TransformSettings>(world, &mut res);
        register::<space::Velocity>(world, &mut res);
    });
}

pub fn register<T>(world: &mut bevy_ecs::world::World, res: &mut Mut<SceneManager>)
where
    T: Component
        + Reflect
        + GetTypeRegistration
        + bevy_reflect::TypePath
        + bevy_reflect::TypePath
        + serde::Serialize,
{
    world.init_component::<T>();
    res.type_registry.register::<T>();
    res.type_registry
        .register_type_data::<T, ReflectTestSuperTrait>();
}
