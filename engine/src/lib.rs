#![allow(dead_code)]
#![allow(unused)]

//! The main module for having game logic through components interact with [`bevy_ecs`] and [`ggez`]
//! If you need to access the main engine, this is how you do it.
//!
//!

mod camera;
mod game_root;
mod input;
mod render_resource;
mod schedule;
pub mod space;

pub use game_root::GameRoot;
pub use input::input_cli_editor;
pub use input::{Action, Input, Key};
pub use render_resource::Engine;
