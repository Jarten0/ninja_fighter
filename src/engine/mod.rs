#![allow(dead_code)]

//! The main module for having game logic through components interact with [`bevy_ecs`] and [`ggez`]
//! If you need to access the main engine, this is how you do it.
//!
//!

mod engine;
mod input;
mod root;
mod schedule;
pub(crate) mod space;

pub(crate) use engine::Engine;
pub(super) use input::input_cli_editor;
#[allow(unused_imports)]
pub(crate) use input::{Action, Input, Key};
pub(super) use root::GameRoot;
