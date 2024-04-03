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
use collider::mesh_editor::MeshEditor;
use engine::scene::serialize_component;
use engine::scene::SceneManager;

// #[allow(unused)]
pub mod collider;
pub mod protag;
pub mod render;

pub fn init_components(world: &mut World) -> () {
    world.insert_resource(MeshEditor {
        focus: collider::mesh_editor::FocusState::Idle,
    });

    world.resource_scope(|world: &mut World, mut manager: Mut<SceneManager>| {
        let register = &mut manager.type_registry;
        serialize_component::<render::Renderer>(world, register);
        serialize_component::<collider::Collider>(world, register);
        serialize_component::<collider::GravitySettings>(world, register);
        serialize_component::<protag::Protag>(world, register);
        serialize_component::<protag::ProtagController>(world, register);
    });
}
