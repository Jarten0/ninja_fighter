//! Home to all game logic and custom built scripts.
//!
//! Any component files can be added and removed without worry.
//!
//! If you wish to make use of the engine, [`crate::engine`] is where you can access some of the public resources needed for interacting with the lower level logic.
//!
//! When developing any component library, it's reccomended to have a wildcard-like function that initializes every component in a library
//! so that the end user can hand off that small but bothersome responsibility of looking through the crate to find every component.
//! It can also be inlined later for removing any unused components if initialization performance is critical.

use bevy_ecs::world::World;

#[allow(unused)]
pub mod collider;
pub mod debug;
pub mod protag;
pub mod render;

pub fn init_components(world: &mut World) -> () {
    world.init_component::<render::Renderer>();
    world.init_component::<collider::collider_mesh::ColliderMesh>();
    world.init_component::<collider::gravity_settings::GravitySettings>();
    world.init_component::<protag::Protag>();
    world.init_component::<debug::DebugComponent>();
}
