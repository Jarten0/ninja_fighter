#![allow(unused)]

//! The main module for having game logic through components interact with [`bevy_ecs`] and [`ggez`]
//! If you need to access the main engine, this is how you do it.
//!
//! Several components are stored here as well, built directly into the engine.
//! The [`Transform`] and [`camera::Camera`] are good examples of that.
pub mod assets;
pub mod input;
pub mod logging;
pub mod scene;
pub mod schedule;
pub mod space;

// TODO: Private when finished developing
pub mod camera;
pub mod engine;
pub mod freeze;
pub mod render;
pub mod root;

#[cfg(feature = "editor_features")]
pub mod editor;

pub use camera::Camera;
pub use engine::GgezInterface;
pub use input::input_cli_editor;
pub use input::{ActionData, Input, Key};
pub use logging::LogData;
pub use render::render_type::RenderType;
pub use root::GameRoot;

/// A list of settings that the engine needs in order to operate exactly as you want it to.
///
/// See each field's specific documentation for information about what each of them is for.
#[derive(Debug, Clone)]
pub struct EngineConfig {
    /// Set a list of file paths that lead to serialized [`Scene`](crate::scene::Scene)'s. The first one will be loaded on startup if given,
    /// and others will automatically be added to the list of immediately accessable scenes.  
    // TODO: Update when I redo the scene loading system to replace JSON with ron and when this gets replaced with a file path leading to a file containing a list of scenes.
    pub scene_paths: &'static [&'static str],
    /// An initiation function that should be run on the world to register/add components, custom resources, and schedules.
    ///
    /// You are required to add at least 2 schedules, that being [`ScheduleTag::Tick`](crate::schedule::ScheduleTag::Tick) and
    /// [`ScheduleTag::Frame`](crate::schedule::ScheduleTag::Frame), which handle game logic and rendering logic respectively.
    ///
    /// See also [`ScheduleTag::Init`](crate::schedule::ScheduleTag::Init) and [`ScheduleTag::FreezeTick`](crate::schedule::ScheduleTag::FreezeTick)
    /// which are run in specific cases if given, as well as other debug schedules that run alongside normal schedules for special functionality, if enabled.
    pub world_init: fn(&mut bevy_ecs::prelude::World) -> (),
    /// How many times should game logic be updated per second.
    ///
    /// Notice that this is not equal to the amount of frames rendered and displayed to the screen, this handles game logic specifically.
    ///
    /// Also notice that this is simply a cap, and can't force the engine to run faster if the computer is having a hard time processing logic.
    /// You can also leave it uncapped and operate using delta time, if that's what you prefer.
    pub ticks_per_second: u32,
    /// Should normal game logic be paused when the window is not in focus?
    ///
    /// Rendering will still apply as normal, though delta will work a little differently.
    pub freeze_on_unfocus: bool,
    /// Should normal game logic be paused when the window is hidden from the user?
    ///
    /// Rendering will be paused when minimized.
    ///
    /// Note that minimizing counts as being unfocused, so this rule will override `freeze_on_unfocus` when the window is minimized.
    pub freeze_on_minimize: bool,
    /// Should debug schedules be run when debug mode is enabled?
    ///
    /// Notice that if this is turned on while debug schedules have not been inserted into the world, this will cause a panic.
    ///
    /// Do not enable unless you add the various debug schedules.
    /// Any [`ScheduleTag`](crate::schedule::ScheduleTag) variant that starts with `Debug-` must exist and must be added to the world using
    /// [`World::add_schedule()`](bevy_ecs::prelude::World::add_schedule).
    pub run_debug_schedules: bool,
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
