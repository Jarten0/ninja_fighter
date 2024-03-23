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
pub use logging::LogData;
pub use render::render_type::RenderType;
pub use root::GameRoot;

#[derive(Debug, Clone)]
pub struct EngineConfig {
    pub scene_paths: &'static [&'static str],
    pub world_init: fn(&mut bevy_ecs::prelude::World) -> (),
    pub schedule_builder_functions:
        fn() -> Vec<fn() -> (bevy_ecs::schedule::Schedule, schedule::ScheduleTag)>,
    pub ticks_per_second: u32,
    pub debug_cli: Option<fn(&mut GameRoot)>,
}

pub enum EngineConfigError {
    NoScenePaths,
    InvalidScenePath(&'static str),
    InvalidTicksPerSecond,
    MissingSchedule,
}

pub enum SomeError {
    Scene(crate::scene::SceneError),
    Ggez(ggez::GameError),
    IO(std::io::Error),
    Misc(String),
    EngineConfig(EngineConfigError),
}

impl ToString for SomeError {
    fn to_string(&self) -> String {
        todo!()
    }
}
