use super::action::{Action, KeyStatus};
use super::key::Key;
use bevy_ecs::system::Resource;
use ggez::input::keyboard::KeyCode;
use std::collections::HashMap;
use std::env::current_dir;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

/// A module for dealing with player input. Add it as a [`bevy_ecs::system::Res<>`] or [`bevy_ecs::system::ResMut<>`] parameter inside your system function to make use of it.
///
/// Unlike other engines, this input module makes use of Rust's rich type system to convey extra information without dealing with invalid state.
/// Once you have the resource, you can call `get_action(key)`, using `key` to try and find the action you want, ex. "Jump" or "Fire".
/// Then, you can store a reference to that action. From there, simply query any actions you want using `action_status()` to get a [`KeyStatus`] enum variant.
/// Read the [`KeyStatus`] documentation for further details on inquiring about actions.
#[derive(Resource)]
pub struct Input {
    actions: HashMap<&'static str, Action>,
    keylist: HashMap<KeyCode, Key>,
}

impl Input {
    /// Returns a new [`Input`] resource.
    pub fn new() -> Self {
        Self {
            actions: HashMap::new(),
            keylist: HashMap::new(),
        }
    }

    pub fn action_status(&self, action: &Action) -> KeyStatus {
        action.status
    }

    /// Wrapper function for `HashMap::Insert` but making sure that the hash key equals the actions name.
    /// Thus, it also returns [`Some(Key)`] if the key already had a value. Otherwise, it returns [`None`].
    pub fn new_action(&mut self, action: Action) -> Option<Action> {
        HashMap::insert(&mut self.actions, action.name, action)
    }

    /// Wrapper function for HashMap::get. Returns [`Some(&Action)`] if the action exists, and returning [`None`] if not.
    pub fn get_action(&mut self, action_name: &str) -> Option<&Action> {
        self.actions.get(action_name)
    }

    /// Wrapper function for HashMap::get_mut. Returns [`Some(&Action)`] if the action exists, and returning [`None`] if not.
    pub fn get_action_mut(&mut self, action_name: &str) -> Option<&mut Action> {
        self.actions.get_mut(action_name)
    }

    fn load_keys(&mut self) {
        let dir = match current_dir() {
            Ok(path) => path,
            Err(err) => panic!("Path directory error! What? {}", err),
        };

        let path = dir.join(PathBuf::from("/src/input/keyData.txt"));

        let mut file = match File::open(path) {
            Ok(path) => path,
            Err(err) => panic!("Key file could not be opened! {}", err),
        };

        let mut buf = String::new();
        match file.read_to_string(&mut buf) {
            Ok(_) => (),
            Err(err) => panic!("Invalid file read! {}", err),
        }

        for (index, char) in buf.char_indices() {}
    }
}

impl ToString for Input {
    fn to_string(&self) -> String {
        let mut string = String::new();

        for (_, action) in self.actions.iter() {
            string.push_str(action.to_string().as_str());
            string.push_str("; ");
        }

        string
    }
}
