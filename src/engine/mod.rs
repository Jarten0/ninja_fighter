//! The main module for having game logic through components interact with [`bevy_ecs`] and [`ggez`]
//! If you need to access the main engine, this is how you do it.
//!
//!

mod game_root;
mod input;
mod render_resource;
mod schedule;
pub(crate) mod space;

pub(super) use game_root::GameRoot;
pub(super) use input::input_cli_editor;
pub(crate) use input::{Action, Input, Key, KeyStatus};
pub(crate) use render_resource::MainCanvas;
