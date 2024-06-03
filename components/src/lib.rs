//! Home to all game logic and custom built scripts.
//!
//! Any component files can be added and removed without worry.
//!
//! If you wish to make use of the engine, [`crate::engine`] is where you can access some of the public resources needed for interacting with the lower level logic.
//!
//! When developing any component library, it's reccomended to have a wildcard-like function that initializes every component in a library
//! so that the end user can hand off that small but bothersome responsibility of looking through the crate to find every component.
//! It can also be inlined later for removing any unused components if initialization performance is critical.
#![allow(unused)]

use bevy_ecs::prelude::*;
use bevy_reflect::ReflectSerialize;
use collider::MeshType;
use engine::assets::SceneAssetID;
use engine::{register_component, scene::SceneManager};
use engine::{register_custom_inspection, register_enum};
use std::collections::HashMap;

pub mod collider;
#[cfg(feature = "editor_features")]
pub mod editor_windows;
pub mod protag;
pub mod render;
pub mod text_renderer;

pub mod schedules {
    /// Systems that update game logic
    pub mod tick {
        pub use crate::collider::collider_update;
        pub use crate::protag::protag_init;
        pub use crate::protag::protag_update;
        pub use crate::render::renderer_update;
    }

    /// Systems that draw to the screen
    pub mod frame {
        pub use crate::collider::mesh_renderer::mesh_renderer_draw;
        pub use crate::render::renderer_draw;
        pub use crate::text_renderer::render_text_renderers;
    }
}

/// Adds all of the components to the type registry.
pub fn initialize_component_types(world: &mut World) -> () {
    world.resource_scope(|world: &mut World, mut manager: Mut<SceneManager>| {
        let type_registry = &mut manager.type_registry;
        register_component::<render::Renderer>(world, type_registry);
        register_custom_inspection::<render::Renderer>(world, type_registry);

        register_component::<text_renderer::TextRenderer>(world, type_registry);
        register_custom_inspection::<text_renderer::TextRenderer>(world, type_registry);

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
