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

use std::any::Any;
use std::collections::HashMap;

use bevy_ecs::prelude::*;
use bevy_reflect::{Reflect, ReflectSerialize, TypeRegistration, TypeRegistry};
use collider::mesh_editor::MeshEditor;
use collider::MeshType;
use engine::assets::SceneAssetID;
use engine::{register_component, scene::SceneManager};
use engine::{
    register_custom_inspection, register_custom_serialize, register_enum, register_primitive_value,
};

// #[allow(unused)]
pub mod collider;
pub mod editor_windows;
pub mod protag;
pub mod render;
pub mod theo_matthew_game;

pub fn init_components(world: &mut World) -> () {
    world.insert_resource(MeshEditor::default());

    world.resource_scope(|world: &mut World, mut manager: Mut<SceneManager>| {
        let type_registry = &mut manager.type_registry;
        register_component::<render::Renderer>(world, type_registry);
        register_custom_inspection::<render::Renderer>(world, type_registry);

        register_component::<theo_matthew_game::TextRenderer>(world, type_registry);
        register_custom_inspection::<theo_matthew_game::TextRenderer>(world, type_registry);

        register_component::<collider::Collider>(world, type_registry);
        register_component::<collider::mesh_renderer::MeshRenderer>(world, type_registry);

        register_component::<collider::GravitySettings>(world, type_registry);
        register_component::<protag::Protag>(world, type_registry);
        register_component::<protag::ProtagController>(world, type_registry);

        register_enum::<MeshType>(type_registry);

        type_registry.register::<MeshType>();
        type_registry.register::<HashMap<SceneAssetID, MeshType>>();
        type_registry.register_type_data::<MeshType, ReflectSerialize>();
        type_registry.register_type_data::<HashMap<SceneAssetID, MeshType>, ReflectSerialize>();
    });
}

fn register_for_generic_types<T>(type_registry: &mut TypeRegistry) {}
