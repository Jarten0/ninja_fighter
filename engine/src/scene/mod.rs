mod component;
mod error;
mod scene_manager;
mod scene_object;
mod serialized_scene;
#[cfg(test)]
mod test;
mod traits;

use bevy_ecs::{component::Component, world::Mut};
use bevy_reflect::{GetTypeRegistration, Reflect};
pub use component::{
    add_entity_to_scene, load_scene, new_scene, save_scene, to_serialized_scene, unload_scene,
    validate_name,
};

pub use component::Scene;
pub use error::SceneError;
pub use scene_manager::SceneManager;
pub use scene_object::{CounterType, SceneObjectID};
pub use serialized_scene::ToReflect;
pub use traits::SceneData;

use crate::space;

pub fn register_scene_types(world: &mut bevy_ecs::world::World) {
    world.init_resource::<scene_manager::SceneManager>();
    world.resource_scope(|world, mut res: Mut<SceneManager>| {
        register::<traits::SceneData>(world, &mut res);
        register::<space::Position>(world, &mut res);
        register::<space::Rotation>(world, &mut res);
        register::<space::Scale>(world, &mut res);
        register::<space::TransformSettings>(world, &mut res);
        register::<space::Velocity>(world, &mut res);
        // register::<render::render_type::RenderType>();
        // register::<>();
    });
}

pub fn register<T: Component + Reflect + GetTypeRegistration>(
    world: &mut bevy_ecs::world::World,
    res: &mut Mut<SceneManager>,
) {
    world.init_component::<T>();
    res.type_registry.register::<T>();
}
