//! Home to all game logic and custom built scripts.
//!
//! Any component files can be added and removed without worry.
//!
//! If you wish to make use of the engine, [`crate::engine`] is where you can access some of the public resources needed for interacting with the lower level logic.
//!
//! When developing any component library, it's reccomended to have a wildcard-like function that initializes every component in a library
//! so that the end user can hand off that small but bothersome responsibility of looking through the crate to find every component.
//! It can also be inlined later for removing any unused components if initialization performance is critical.

use bevy_ecs::prelude::*;
use bevy_reflect::TypeRegistry;
use engine::scene::{ReflectTestSuperTrait, SceneManager};

#[allow(unused)]
pub mod collider;
pub mod debug;
pub mod protag;
pub mod render;

pub fn init_components(world: &mut World) -> () {
    world.resource_scope(|world: &mut World, mut manager: Mut<SceneManager>| {
        let register = &mut manager.type_registry;
        serialize_component::<render::Renderer>(world, register);
        serialize_component::<collider::collider_mesh::ColliderMesh>(world, register);
        serialize_component::<collider::gravity_settings::GravitySettings>(world, register);
        serialize_component::<protag::Protag>(world, register);
        serialize_component::<protag::ProtagController>(world, register);
        serialize_component::<debug::DebugComponent>(world, register);
    });
}

/// Enables the component to be serialized
fn serialize_component<
    T: bevy_ecs::component::Component
        + bevy_reflect::GetTypeRegistration
        + bevy_reflect::Reflect
        + serde::Serialize
        + bevy_reflect::TypePath,
>(
    world: &mut World,
    register: &mut TypeRegistry,
) {
    world.init_component::<T>();
    register.register::<T>();
    register.register_type_data::<T, ReflectTestSuperTrait>()
}
