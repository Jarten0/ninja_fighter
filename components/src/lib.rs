#![allow(unused)]

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
use collider::MeshType;
use engine::{register_component, scene::SceneManager};
use engine::{
    register_custom_inspection, register_custom_serialize, register_enum, register_primitive_value,
};

// #[allow(unused)]
pub mod collider;
pub mod editor_windows;
pub mod protag;
pub mod render;

pub fn init_components(world: &mut World) -> () {
    world.insert_resource(MeshEditor::default());

    world.resource_scope(|world: &mut World, mut manager: Mut<SceneManager>| {
        let register = &mut manager.type_registry;
        register_component::<render::Renderer>(world, register);
        register_custom_inspection::<render::Renderer>(world, register);

        register_component::<collider::Collider>(world, register);
        register_component::<collider::mesh_renderer::MeshRenderer>(world, register);

        register_component::<collider::GravitySettings>(world, register);
        register_component::<protag::Protag>(world, register);
        register_component::<protag::ProtagController>(world, register);

        register_enum::<MeshType>(register);
    });
}
