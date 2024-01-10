//! The input module for handling player input.
//!
//! To get started, read the docs for [`resource::Input`] to crawl your way around.
//!
//! # Structs
//! * [`Input`] - Input [`bevy_ecs::system::Resource`] that handles all interactions between [`ggez`] and user scripts through [`Action`]'s and [`Key`]'s.
//!
//! * [`Key`] - Minimal container for storing info about [`ggez::event`]'s.
//!
//! * [`Action`] - Basic container for assigning [`Key`]'s to user script actions.
//!
#[cfg(test)]
pub(self) mod test;

pub(self) mod action;
pub(self) mod input_resource;
pub(self) mod key;
pub(self) mod main;

mod input_update_scheduler {
    pub(in crate::engine) struct InputUpdateScheduler {}

    impl InputUpdateScheduler {
        pub fn push_update(&mut self) {}
    }
}

#[allow(unused_imports)]
pub use action::{Action, KeyStatus};
pub use input_resource::Input;
pub use key::keycode_converter::KeycodeType;
pub use key::Key;
pub use main::main as input_cli_editor;
