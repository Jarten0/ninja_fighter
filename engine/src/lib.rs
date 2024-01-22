#![allow(dead_code)]
#![allow(unused)]

//! The main module for having game logic through components interact with [`bevy_ecs`] and [`ggez`]
//! If you need to access the main engine, this is how you do it.
//!
//!

mod assets;
mod camera;
mod engine;
mod input;
mod render;
mod root;
pub mod scene;
pub mod schedule;
pub mod space;

pub use assets::Assets;
pub use engine::Engine;
pub use input::input_cli_editor;
pub use input::{Action, Input, Key};
pub use render::render_type::RenderType;
pub use root::GameRoot;
