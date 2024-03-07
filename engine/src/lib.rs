//! The main module for having game logic through components interact with [`bevy_ecs`] and [`ggez`]
//! If you need to access the main engine, this is how you do it.
//!
//! Several components are stored here as well, built directly into the engine.
//! The [`Transform`] and [`camera::Camera`] are good examples of that.
// #![allow(unused)]
mod assets;
mod camera;
mod engine;
mod freeze;
pub mod input;
mod render;
mod root;
pub mod scene;
pub mod schedule;
pub mod space;

pub use assets::Assets;
pub use camera::Camera;
pub use engine::GgezInterface;
pub use input::input_cli_editor;
pub use input::{Action, Input, Key};
pub use render::render_type::RenderType;
pub use root::GameRoot;

pub mod systems {
    pub use crate::space::transform::update;
}

#[allow(dead_code)]
fn having_fun() {
    let mut world = bevy_ecs::world::World::new();
    scene::register_scene_types(&mut world);
}

/// Initializes all of the types used by the engine for any given world.
/// Automatically called during engine operation, unless directly requested not to.
fn register_types(world: &mut bevy_ecs::world::World) {
    scene::register_scene_types(world);
}
