//! The main module for having game logic through components interact with [`bevy_ecs`] and [`ggez`]
//! If you need to access the main engine, this is how you do it.
//!
//! Several components are stored here as well, built directly into the engine.
//! The [`Transform`] and [`camera::Camera`] are good examples of that.
pub mod input;
pub mod logging;
pub mod scene;
pub mod schedule;
pub mod space;

mod assets;
mod camera;
mod engine;
mod freeze;
mod render;
mod root;

pub use assets::Assets;
pub use camera::Camera;
pub use engine::GgezInterface;
pub use input::input_cli_editor;
pub use input::{ActionData, Input, Key};
pub use render::render_type::RenderType;
pub use root::GameRoot;

use bevy_ecs::schedule::Schedule;
use bevy_ecs::world::World;
use schedule::ScheduleTag;

#[derive(Debug, Clone)]
pub struct EngineConfig {
    pub scene_paths: &'static [&'static str],
    pub world_init: fn(&mut World) -> (),
    pub schedule_builder_functions: fn() -> Vec<fn() -> (Schedule, ScheduleTag)>,
    pub ticks_per_second: u32,
    pub debug_cli: Option<fn(&mut GameRoot)>,
}
